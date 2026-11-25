# The manifest

A managed repository describes itself in a manifest. Skills ask the manifest
where things are and how to run them, so that no skill has to name a product's
paths or commands, and Hephaestium runs the commands the manifest declares.

- [`schema.json`](./schema.json) is the definition. Every key, its type and
  its rule is stated there and nowhere else, with a description beside it.
- [`example.toml`](./example.toml) is a worked example for an invented
  repository.
- [`cases/`](./cases) holds manifests that must be accepted and manifests that
  must be rejected.

## What it covers

Version 0 holds only what something reads today.

| Section | What it answers |
|---|---|
| `schema_version` | Which version of the schema this manifest follows. |
| `toolchain` | How to enter the repository's pinned toolchain before running anything. |
| `components` | What the buildable parts are, which files belong to each, and how each is tested and linted. |
| `gates` | Which commands decide whether a change may proceed, which changes each applies to, which environment variables and secrets each needs, and which check on a pull request exercises the same work. |
| `bootstrap` | The steps that prepare a fresh workspace so the gates can run in it. |
| `branch` | How a ticket's branch is named: a `template` containing `{key}`, and optionally `{slug}`. |
| `docs` | Where the documents a skill may need are kept. |

## Testing only what changed

`components.<name>.test_changed` is optional. It is a command, as `test` is
(a list, never a string), and the paths of the changed files under the
component's root are appended to it as arguments. The paths are relative to
the component's root, not the repository's. It is not run when no file of the
component changed. `test` is unchanged: it still runs the component's whole
suite.

Review policy, workspace bootstrap and the drive are not in version 0. Each is
added when something reads it.

## Four rules that shape it

- **An unknown key is an error.** A misspelt `when_paths` that was silently
  ignored would make a gate run for every change, the opposite of what its
  author meant.
- **A command is a list of arguments.** A single string would need a shell to
  split it, and then quoting decides what runs.
- **A gate says what it needs.** `requires` names the environment variables a
  gate depends on, as `env` or as `secret`. A reader that finds one missing does
  not run the gate, so a missing prerequisite is never mistaken for a failing
  test. A `secret` is never printed or logged. Naming each variable once, as a
  key, makes a duplicate or a variable that is both kinds impossible to write.
- **A gate can name the check that stands in for it.** `ci_check` is the name of a
  check on a pull request that runs the same work. A runner that cannot run the gate
  where it is may leave it to that check and say so, and the pull request is then not
  settled until that check has passed. The repository is the one that says the two are
  equivalent. A gate that runs and fails is a failure whatever check it names.
- **A path is relative and stays inside the repository.** The manifest then
  means the same thing in every checkout.

## The cases are the contract

Each file under `cases/invalid/` breaks exactly one rule and says which on its
first line:

```
# violates: $.gates.regression.timeout_seconds minimum
```

The path says where in the manifest the rule is broken, and the keyword is the
JSON Schema keyword that rejects it. `scripts/check-cases.sh manifest` grades
the schema against the cases, and fails a case that is accepted, that breaks
more than one rule, or that is rejected for a different reason than it states.
Being rejected is not enough: a case rejected for the wrong reason would keep
passing if its rule were deleted.

Any other reader of manifests, Hephaestium's command-line tool included, is
graded against the same cases. A reader written in Rust gets them from the
crate at the repository root, which generates its list from this directory.

## Changing the schema

1. Add the case first, and see the check fail on it.
2. Change the schema until the check passes.
3. A new optional key is a `feat`. A change that makes a manifest which was
   valid invalid is a breaking change: it takes a new `schema_version`, and
   the commit says so with `!`.
