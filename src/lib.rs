//! Athenaeum's contracts, as text a reader can compile in.
//!
//! This crate holds no logic. It does not parse, validate or interpret
//! anything: it hands out the files in this repository, so that a reader
//! written in Rust can pin a tag of Athenaeum as a build dependency and be
//! graded against the contract it was built for. A reader that disagrees with
//! a case then fails its own tests, at build time, instead of disagreeing
//! with a repository in the field.

/// What a reader must do with a case, whichever contract it belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expectation {
    /// The instance is valid.
    Accepted,
    /// The instance breaks exactly one rule.
    Rejected {
        /// Where in the instance the rule is broken, as `$.a.b[0]`.
        path: &'static str,
        /// The JSON Schema keyword that rejects it.
        keyword: &'static str,
    },
}

/// The manifest: the file in which a managed repository describes itself.
pub mod manifest {
    /// The schema, as JSON Schema text.
    pub const SCHEMA: &str = include_str!("../manifest/schema.json");

    /// A worked example for an invented repository.
    pub const EXAMPLE: &str = include_str!("../manifest/example.toml");

    /// One manifest and what a reader must do with it.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Case {
        /// The case's file name without its extension.
        pub name: &'static str,
        /// The manifest itself.
        pub toml: &'static str,
        /// What a reader must do with it.
        pub expectation: Expectation,
    }

    pub use crate::Expectation;

    /// Every case: the worked example, each file under `manifest/cases/valid`
    /// and each file under `manifest/cases/invalid`. Generated from those
    /// directories by `build.rs`.
    pub const CASES: &[Case] = include!(concat!(env!("OUT_DIR"), "/manifest_cases.rs"));
}

/// The gate result: what a gate reports when it is run.
pub mod gate_result {
    /// The schema, as JSON Schema text.
    pub const SCHEMA: &str = include_str!("../contracts/gate-result/schema.json");

    /// A worked example: a gate that passed.
    pub const EXAMPLE: &str = include_str!("../contracts/gate-result/example.json");

    /// One gate result and what a reader must do with it.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Case {
        /// The case's file name without its extension.
        pub name: &'static str,
        /// The gate result itself, as JSON text.
        pub json: &'static str,
        /// What a reader must do with it.
        pub expectation: crate::Expectation,
    }

    /// Every case: the worked example, each file under
    /// `contracts/gate-result/cases/valid` and each under `.../invalid`.
    /// Generated from those directories by `build.rs`.
    pub const CASES: &[Case] = include!(concat!(env!("OUT_DIR"), "/gate_result_cases.rs"));
}

/// The lock: how a managed repository pins the Athenaeum revision it uses.
pub mod lock {
    /// A lock is TOML, like a manifest, so a case has the same shape.
    pub use crate::manifest::Case;

    /// The schema, as JSON Schema text.
    pub const SCHEMA: &str = include_str!("../contracts/lock/schema.json");

    /// A worked example.
    pub const EXAMPLE: &str = include_str!("../contracts/lock/example.toml");

    /// Every case: the worked example, each file under
    /// `contracts/lock/cases/valid` and each under `.../invalid`. Generated
    /// from those directories by `build.rs`.
    pub const CASES: &[Case] = include!(concat!(env!("OUT_DIR"), "/lock_cases.rs"));
}
