# Athenaeum

> The skills, contracts and stack packs that [Hephaestium](https://github.com/erinversfeldcodes/hephaestium) works from.

Athenaeum is the content half of my agent harness. It holds what an agent
needs to be told in order to plan, build, review and finish a piece of work,
written so that it is true in any repository and not only the one it came from.
It contains no logic: the only code is the scripts that check its own content
and a crate that hands its contracts, unchanged, to a reader that pins them.

> **Status: the manifest and gate result schemas are defined and four skills have been moved in. No packs yet.**
> The source is publicly visible under a restrictive licence. You may read it,
> but may not use, copy or distribute it. See [LICENSE](./LICENSE).

## Where it comes from

The content is being extracted from the agent harness inside
[The Stacks](https://github.com/erinversfeldcodes/thestacks). There it grew as
role specifications of over a thousand lines each, in which process, knowledge
of a stack and facts about one product share the same file. The same fact is
often stated in several of those files, and they have drifted apart. I've used Claude Code to help with this extraction though the intention is for everything in this repo and [Hephaestium](https://github.com/erinversfeldcodes/hephaestium) to work with any of OpenRouter, Claude Code and Snowflake Cortex.

Athenaeum separates the three:

| Kind of fact | Example | Where it lives |
|---|---|---|
| Process, true anywhere | A new test fails for the right reason before the code that satisfies it is written. | A skill, here |
| Knowledge of a stack | A migration that adds an index to a large table does so concurrently. | A pack, here |
| A fact about one product | Which module owns account deletion. | The managed repository's manifest or its own documents |

## Layout

```
manifest/    the manifest schema, a worked example and its conformance cases
contracts/   the other contracts, each with a schema, an example and conformance cases
skills/      one directory per skill, each a SKILL.md, and the conventions they share
packs/       planned: one directory per stack or cross-cutting concern
```

Start with [`manifest/`](./manifest/README.md), then [`contracts/`](./contracts/README.md) and [`skills/`](./skills/README.md).

## Pinning it from Rust

A reader written in Rust takes Athenaeum at a tag as a build dependency:

```toml
[dependencies]
athenaeum = { git = "https://github.com/erinversfeldcodes/athenaeum", tag = "v0.3.0" }
```

`athenaeum::manifest::SCHEMA` is the schema text, and
`athenaeum::manifest::CASES` is every case with what it expects. The gate result
is the same in `athenaeum::gate_result`. The reader
compiles the schema in and runs the cases as its own tests, so it cannot
disagree with the contract it was built for without failing its build. The
crate's own version is never bumped; the tag is the version.

- A **skill** teaches one method and is small enough that a run either loaded
  it or did not. That is what lets Hephaestium measure whether it matters.
- A **contract** fixes the shape of something that crosses a boundary, so a
  specialist's report and a reviewer's verdict are stated once.
- A **pack** holds what is true for any repository on one stack.

The vocabulary is defined in Hephaestium's
[`docs/CONTEXT.md`](https://github.com/erinversfeldcodes/hephaestium/blob/main/docs/CONTEXT.md),
and the decision to keep content and code apart is
[ADR-0002](https://github.com/erinversfeldcodes/hephaestium/blob/main/docs/adr/0002-athenaeum-is-content-hephaestium-is-code.md)
there.

## How a repository uses it

A managed repository pins one revision of Athenaeum in a lock file and receives
a later revision as a change to that lock, which can be reviewed and reverted
like any other change. The same skills are used by a person working in an agent
runtime by hand and by Hephaestium running unattended.

Under the current licence, only the copyright holder's own repositories may do
this.

## Releases

A version is derived from the conventional commits on `main`: a breaking change
to a skill, a contract or the manifest schema bumps the major version, `feat`
the minor, and `fix` the patch. When the checks pass on `main`, a workflow
creates the `v<version>` tag and a GitHub release with notes. That tag is what
a lock pins. The rule is recorded in Hephaestium's
[ADR-0008](https://github.com/erinversfeldcodes/hephaestium/blob/main/docs/adr/0008-versions-come-from-conventional-commits.md).

## Feedback

Bug reports and ideas are welcome as issues. Pull requests from outside are not
accepted. See [CONTRIBUTING.md](./CONTRIBUTING.md) and
[SECURITY.md](./SECURITY.md).

## Licence

Source-available, all rights reserved. See [LICENSE](./LICENSE).

## Why the name

An athenaeum is a place of learning dedicated to Athena, goddess of craft and
practical skill. She shared a temple in Athens with Hephaestus, god of the
forge, for whom the runtime is named.
