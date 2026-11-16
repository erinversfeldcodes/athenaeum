//! The crate must hand out what the repository holds, no more and no less.
//!
//! The cases are files; the crate's list of them is generated. These tests
//! compare the two, so a file the generator skipped, or a case it invented,
//! is caught here and not by a reader that quietly graded itself against
//! fewer cases than exist.

use std::collections::BTreeSet;
use std::path::Path;

use athenaeum::manifest::{CASES, Expectation};

fn names_in(directory: &str) -> BTreeSet<String> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join(directory);
    std::fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("{} should be readable: {error}", directory.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "toml")
        })
        .map(|path| {
            path.file_stem()
                .expect("a case file to have a name")
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

fn names_expecting(accepted: bool) -> BTreeSet<String> {
    CASES
        .iter()
        .filter(|case| matches!(case.expectation, Expectation::Accepted) == accepted)
        .map(|case| case.name.to_owned())
        .collect()
}

#[test]
fn every_valid_case_file_is_exposed_as_accepted() {
    let mut on_disk = names_in("manifest/cases/valid");
    on_disk.insert("example".to_owned());

    assert_eq!(names_expecting(true), on_disk);
}

#[test]
fn every_invalid_case_file_is_exposed_as_rejected() {
    assert_eq!(names_expecting(false), names_in("manifest/cases/invalid"));
}

#[test]
fn a_case_carries_the_text_of_its_file() {
    let case = CASES
        .iter()
        .find(|case| case.name == "zero-timeout")
        .expect("the zero-timeout case to be exposed");
    let on_disk = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("manifest/cases/invalid/zero-timeout.toml"),
    )
    .expect("the case file to be readable");

    assert_eq!(case.toml, on_disk);
}

#[test]
fn a_rejected_case_carries_the_violation_its_first_line_states() {
    let case = CASES
        .iter()
        .find(|case| case.name == "zero-timeout")
        .expect("the zero-timeout case to be exposed");

    assert_eq!(
        case.expectation,
        Expectation::Rejected {
            path: "$.gates.regression.timeout_seconds",
            keyword: "minimum",
        }
    );
}

#[test]
fn the_schema_and_the_example_are_the_files_in_the_repository() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert_eq!(
        athenaeum::manifest::SCHEMA,
        std::fs::read_to_string(root.join("manifest/schema.json")).expect("the schema")
    );
    assert_eq!(
        athenaeum::manifest::EXAMPLE,
        std::fs::read_to_string(root.join("manifest/example.toml")).expect("the example")
    );
}
