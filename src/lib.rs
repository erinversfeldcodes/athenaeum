//! Athenaeum's contracts, as text a reader can compile in.

/// The manifest: the file in which a managed repository describes itself.
pub mod manifest {
    /// The schema, as JSON Schema text.
    pub const SCHEMA: &str = "";

    /// A worked example for an invented repository.
    pub const EXAMPLE: &str = "";

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

    /// What a reader must do with a case.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Expectation {
        /// The manifest is valid.
        Accepted,
        /// The manifest breaks exactly one rule.
        Rejected {
            /// Where in the manifest the rule is broken, as `$.a.b[0]`.
            path: &'static str,
            /// The JSON Schema keyword that rejects it.
            keyword: &'static str,
        },
    }

    /// Every case.
    pub const CASES: &[Case] = &[];
}
