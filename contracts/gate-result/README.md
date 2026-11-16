# Gate result

What a gate reports when it is run. A gate is a command a repository's
manifest declares; the command-line tool and, later, the runtime run it and
report one of these.

- [`schema.json`](./schema.json) is the definition. Every field and rule is
  stated there and nowhere else.
- [`example.json`](./example.json) is a passing result.
- [`cases/`](./cases) holds results that must be accepted and results that must
  be rejected. See [how a contract is held to account](../README.md).

## The four outcomes

| Outcome | Exit code | Also |
|---|---|---|
| `passed` | 0 | |
| `failed` | not 0 | |
| `timed_out` | none | The command was stopped, so it has no exit code of its own. |
| `skipped` | none | A `reason` is required: why the gate was not run. |

The outcome decides which fields must be present, so a result cannot say it
passed with a failing exit code. `command` is a list of arguments, with the
toolchain wrapper already applied, so what is recorded is what ran.

`report` carries the gate's own structured result when the manifest says it
gives one. Its shape belongs to the gate.
