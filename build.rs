//! Turn the case files into the list the crate hands out.
//!
//! A case is a file under `manifest/cases`. The list is generated from the
//! directory so that adding a file is the whole of adding a case: there is no
//! second place to register it, and so no way for a case to exist that a
//! reader is not graded against.
//!
//! An invalid case states the one rule it breaks on its first line:
//!
//! ```text
//! # violates: <path> <keyword>
//! ```
//!
//! A case that does not is refused here, by name, at build time. A reader
//! cannot be asked to reject a manifest for a reason nobody wrote down.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let manifest = root.join("manifest");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", manifest.display());

    let mut cases = String::from("&[\n");

    accepted(&mut cases, "example", &manifest.join("example.toml"));
    for file in case_files(&manifest.join("cases/valid")) {
        accepted(&mut cases, &name_of(&file), &file);
    }
    for file in case_files(&manifest.join("cases/invalid")) {
        rejected(&mut cases, &name_of(&file), &file);
    }

    cases.push_str("]\n");

    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR")).join("manifest_cases.rs");
    std::fs::write(&out, cases).expect("to write the generated list of cases");
}

/// The `.toml` files in a directory, in name order, so the generated list
/// does not depend on the order the file system happens to return them in.
fn case_files(directory: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{} should be readable: {error}", directory.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "toml")
        })
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

fn accepted(cases: &mut String, name: &str, file: &Path) {
    let file = literal(file);
    let _ = writeln!(
        cases,
        "    Case {{ name: {name:?}, toml: include_str!({file:?}), expectation: Expectation::Accepted }},"
    );
}

fn rejected(cases: &mut String, name: &str, file: &Path) {
    let text = std::fs::read_to_string(file)
        .unwrap_or_else(|error| panic!("{} should be readable: {error}", file.display()));
    let first_line = text.lines().next().unwrap_or_default();

    let file_literal = literal(file);
    let words: Vec<&str> = first_line.split_whitespace().collect();
    let ["#", "violates:", path, keyword] = words.as_slice() else {
        panic!(
            "{}: the first line must be `# violates: <path> <keyword>`, but is `{first_line}`",
            file.display()
        );
    };

    let _ = writeln!(
        cases,
        "    Case {{ name: {name:?}, toml: include_str!({file_literal:?}), \
         expectation: Expectation::Rejected {{ path: {path:?}, keyword: {keyword:?} }} }},"
    );
}
