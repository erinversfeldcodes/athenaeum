# Contracts

A contract fixes the shape of something that crosses a boundary, so that
whatever produces it and whatever reads it agree.

| Contract | What crosses | Written as |
|---|---|---|
| [`gate-result`](./gate-result/schema.json) | What a gate reports when it is run | JSON |
| [`lock`](./lock/schema.json) | The Athenaeum revision a repository pins, and the skills it takes | TOML |
| [`baseline`](./baseline/schema.json) | What every repository must have, as a list of items a tool can check | TOML |

The manifest is a contract too, and lives in [`manifest/`](../manifest/README.md)
because it is the one a person writes by hand.

## How a contract is held to account

Each has a JSON Schema, a worked example, and a set of conformance cases:
instances that must be accepted and instances that must be rejected.

- An invalid case breaks **exactly one rule** and says which: where in the
  instance, and by which JSON Schema keyword, written `$.exit_code const`.
- A TOML case says so on its first line, after `# violates:`. A JSON case has no
  comments, so it says so in a file beside it with the same name and the
  extension `.violates`.
- `scripts/check-cases.sh <contract>` grades the schema against the cases and
  fails a case that is accepted, that breaks more than one rule, or that is
  rejected for a different reason than it states.

Any other reader of the contract is graded against the same cases. A reader
written in Rust gets them from the crate at the repository root.

## Adding or changing a rule

1. Add the case first, and see the check fail on it.
2. Change the schema until the check passes.
3. A new optional field is a `feat`. A change that makes an instance which was
   valid invalid is a breaking change: it takes a new `schema_version`, and the
   commit says so with `!`.

## Known quirk

The reference validator reports a `false` subschema at the path of its parent
and not of the key, where another validator reports the key. So a rule that a
field must be absent is written with `not` and `required`, which both agree on.
Keep that in mind when a rule seems to be reported in the wrong place: the case
that finds the disagreement is doing its job.
