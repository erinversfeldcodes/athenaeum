//! Turn the case files into the lists the crate hands out.
//!
//! A case is a file under a contract's `cases` directory. Each list is
//! generated from the directory so that adding a file is the whole of adding a
//! case: there is no second place to register it, and so no way for a case to
//! exist that a reader is not graded against.
//!
//! An invalid case states the one rule it breaks as `<path> <keyword>`. A
//! manifest is TOML and states it on its first line, after `# violates:`. A
//! gate result is JSON, which has no comments, and states it in a file beside
//! it with the same name and the extension `.violates`.
//!
//! A case that does not is refused here, by name, at build time. A reader
//! cannot be asked to reject an instance for a reason nobody wrote down.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

/// Where an invalid case states the rule it breaks.
#[derive(Clone, Copy)]
enum Statement {
    /// On the first line of the case itself, after `# violates:`.
    FirstLine,
    /// In a file beside the case, with the extension `.violates`.
    Sidecar,
}

/// One contract whose cases the crate hands out.
struct Contract {
    /// The name of the generated file, and of the module's list of cases.
    output: &'static str,
    /// The directory the contract lives in, relative to the crate root.
    directory: &'static str,
    /// The worked example, relative to that directory.
    example: &'static str,
    /// The extension of an instance.
    extension: &'static str,
    /// The field of the generated `Case` that holds the instance's text.
    field: &'static str,
    statement: Statement,
}

const CONTRACTS: &[Contract] = &[
    Contract {
        output: "manifest_cases.rs",
        directory: "manifest",
        example: "example.toml",
        extension: "toml",
        field: "toml",
        statement: Statement::FirstLine,
    },
    Contract {
        output: "gate_result_cases.rs",
        directory: "contracts/gate-result",
        example: "example.json",
        extension: "json",
        field: "json",
        statement: Statement::Sidecar,
    },
];

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));

    println!("cargo:rerun-if-changed=build.rs");

    for contract in CONTRACTS {
        let directory = root.join(contract.directory);
        println!("cargo:rerun-if-changed={}", directory.display());

        let mut cases = String::from("&[\n");

        accepted(
            &mut cases,
            contract,
            "example",
            &directory.join(contract.example),
        );
        for file in case_files(&directory.join("cases/valid"), contract.extension) {
            accepted(&mut cases, contract, &name_of(&file), &file);
        }
        for file in case_files(&directory.join("cases/invalid"), contract.extension) {
            rejected(&mut cases, contract, &name_of(&file), &file);
        }

        cases.push_str("]\n");
        std::fs::write(out.join(contract.output), cases)
            .expect("to write the generated list of cases");
    }
}

/// The files with an extension in a directory, in name order, so the
/// generated list does not depend on the order the file system happens to
/// return them in.
fn case_files(directory: &Path, extension: &str) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{} should be readable: {error}", directory.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| path.extension().is_some_and(|found| found == extension))
        .collect();
    files.sort();
    files
}

fn name_of(file: &Path) -> String {
    file.file_stem()
        .expect("a case file to have a name")
        .to_string_lossy()
        .into_owned()
}

/// The path as it must appear in the generated source: a string literal for
/// `include_str!`. Rust source is UTF-8, so a path that is not cannot be
/// written into it.
fn literal(file: &Path) -> &str {
    file.to_str()
        .unwrap_or_else(|| panic!("{} should be a UTF-8 path", file.display()))
}

fn accepted(cases: &mut String, contract: &Contract, name: &str, file: &Path) {
    let field = contract.field;
    let file = literal(file);
    let _ = writeln!(
        cases,
        "    Case {{ name: {name:?}, {field}: include_str!({file:?}), \
         expectation: crate::Expectation::Accepted }},"
    );
}

fn rejected(cases: &mut String, contract: &Contract, name: &str, file: &Path) {
    let field = contract.field;
    let (path, keyword) = violation_stated_by(contract, file);
    let file = literal(file);

    let _ = writeln!(
        cases,
        "    Case {{ name: {name:?}, {field}: include_str!({file:?}), \
         expectation: crate::Expectation::Rejected {{ path: {path:?}, keyword: {keyword:?} }} }},"
    );
}

/// The path and keyword an invalid case says it breaks, or a build failure
/// naming the case that does not say.
fn violation_stated_by(contract: &Contract, file: &Path) -> (String, String) {
    let statement = match contract.statement {
        Statement::FirstLine => {
            let text = std::fs::read_to_string(file)
                .unwrap_or_else(|error| panic!("{} should be readable: {error}", file.display()));
            let first_line = text.lines().next().unwrap_or_default().to_owned();
            match first_line.strip_prefix("# violates:") {
                Some(rest) => rest.to_owned(),
                None => panic!(
                    "{}: the first line must be `# violates: <path> <keyword>`, but is `{first_line}`",
                    file.display()
                ),
            }
        }
        Statement::Sidecar => {
            let sidecar = file.with_extension("violates");
            std::fs::read_to_string(&sidecar)
                .unwrap_or_else(|_| {
                    panic!(
                        "{}: has no {} stating the rule it breaks",
                        file.display(),
                        sidecar.display()
                    )
                })
                .lines()
                .next()
                .unwrap_or_default()
                .to_owned()
        }
    };

    let words: Vec<&str> = statement.split_whitespace().collect();
    let [path, keyword] = words.as_slice() else {
        panic!(
            "{}: the stated rule must be `<path> <keyword>`, but is `{}`",
            file.display(),
            statement.trim()
        );
    };
    ((*path).to_owned(), (*keyword).to_owned())
}
