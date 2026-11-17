//! The crate must hand out what the repository holds for a repository's baseline
//! answers, no more and no less. The same comparison as for the manifest: the
//! answers are TOML and state why a case is rejected on its first line.

use std::collections::BTreeSet;
use std::path::Path;

use athenaeum::Expectation;
use athenaeum::baseline_answers::{CASES, EXAMPLE, SCHEMA};

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

fn case(name: &str) -> &'static athenaeum::baseline_answers::Case {
    CASES
        .iter()
        .find(|case| case.name == name)
        .unwrap_or_else(|| panic!("the case {name} to be exposed"))
}

#[test]
fn every_valid_case_file_is_exposed_as_accepted() {
    let mut on_disk = names_in("contracts/baseline-answers/cases/valid", "toml");
    on_disk.insert("example".to_owned());

    assert_eq!(names_expecting(true), on_disk);
}

#[test]
fn every_invalid_case_file_is_exposed_as_rejected() {
    assert_eq!(
        names_expecting(false),
        names_in("contracts/baseline-answers/cases/invalid", "toml")
    );
}

#[test]
fn a_case_carries_the_text_of_its_file() {
    let on_disk = std::fs::read_to_string(
        Path::new(ROOT).join("contracts/baseline-answers/cases/invalid/empty-contact.toml"),
    )
    .expect("the case file to be readable");

    assert_eq!(case("empty-contact").toml, on_disk);
}

#[test]
fn a_rejected_case_carries_the_violation_its_first_line_states() {
    assert_eq!(
        case("empty-contact").expectation,
        Expectation::Rejected {
            path: "$.contact",
            keyword: "minLength",
        }
    );
    assert_eq!(
        case("duplicate-check").expectation,
        Expectation::Rejected {
            path: "$.required_checks",
            keyword: "uniqueItems",
        }
    );
}

#[test]
fn the_schema_and_the_example_are_the_files_in_the_repository() {
    assert_eq!(
        SCHEMA,
        std::fs::read_to_string(Path::new(ROOT).join("contracts/baseline-answers/schema.json"))
            .expect("the schema")
    );
    assert_eq!(
        EXAMPLE,
        std::fs::read_to_string(Path::new(ROOT).join("contracts/baseline-answers/example.toml"))
            .expect("the example")
    );
}
