//! The crate must hand out what the repository holds for the gate result, no
//! more and no less. The same comparison as for the manifest, for a contract
//! that is JSON and states why a case is rejected in a file beside it.

use std::collections::BTreeSet;
use std::path::Path;

use athenaeum::Expectation;
use athenaeum::gate_result::{CASES, EXAMPLE, SCHEMA};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn names_in(directory: &str, extension: &str) -> BTreeSet<String> {
    let directory = Path::new(ROOT).join(directory);
    std::fs::read_dir(&directory)
        .unwrap_or_else(|error| panic!("{} should be readable: {error}", directory.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| path.extension().is_some_and(|found| found == extension))
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

fn case(name: &str) -> &'static athenaeum::gate_result::Case {
    CASES
        .iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("the case {name} to be exposed"))
}

#[test]
fn every_valid_case_file_is_exposed_as_accepted() {
    let mut on_disk = names_in("contracts/gate-result/cases/valid", "json");
    on_disk.insert("example".to_owned());

    assert_eq!(names_expecting(true), on_disk);
}

#[test]
fn every_invalid_case_file_is_exposed_as_rejected() {
    assert_eq!(
        names_expecting(false),
        names_in("contracts/gate-result/cases/invalid", "json")
    );
}

#[test]
fn a_case_carries_the_text_of_its_file() {
    let on_disk = std::fs::read_to_string(
        Path::new(ROOT).join("contracts/gate-result/cases/invalid/negative-duration.json"),
    )
    .expect("the case file to be readable");

    assert_eq!(case("negative-duration").json, on_disk);
}

#[test]
fn a_rejected_case_carries_the_violation_its_sidecar_states() {
    assert_eq!(
        case("failed-with-zero-exit").expectation,
        Expectation::Rejected {
            path: "$.exit_code",
            keyword: "minimum",
        }
    );
    assert_eq!(
        case("missing-outcome").expectation,
        Expectation::Rejected {
            path: "$",
            keyword: "required",
        }
    );
}

#[test]
fn the_schema_and_the_example_are_the_files_in_the_repository() {
    assert_eq!(
        SCHEMA,
        std::fs::read_to_string(Path::new(ROOT).join("contracts/gate-result/schema.json"))
            .expect("the schema")
    );
    assert_eq!(
        EXAMPLE,
        std::fs::read_to_string(Path::new(ROOT).join("contracts/gate-result/example.json"))
            .expect("the example")
    );
}
