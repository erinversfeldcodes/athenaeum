# Review verdict

What a review lens returns for one diff. A lens is a reviewer pointed at one
kind of concern (a stack, the contracts between components, whether the tests
would fail without the change), and the runtime chooses which lenses review a
change from the files it touches. Each returns one of these, so that a runtime
can read the result instead of a person reading a report.

- [`schema.json`](./schema.json) is the definition. Every field and rule is
  stated there and nowhere else.
- [`example.json`](./example.json) is an approval with a nit and a note.
- [`cases/`](./cases) holds verdicts that must be accepted and verdicts that must
  be rejected. See [how a contract is held to account](../README.md).

The method a lens follows to produce one is [the `review` skill](../../skills/review/SKILL.md).

## The three verdicts

| Verdict | Means | The findings must hold |
|---|---|---|
| `approved` | Nothing blocks the change. | No blocking finding. Advisory ones are fine. |
| `needs_revision` | It can be fixed in place. | At least one blocking finding. |
| `failed` | It cannot be revised into shape: the approach is wrong, a required item is missing, or there is a critical flaw. | At least one P0 finding. |

So a verdict cannot say it approved a change while listing something that
blocks it, or ask for a revision without saying what to revise.

## Findings

| Severity | Means | Blocks |
|---|---|---|
| `P0` | Must not merge in any form. | Always |
| `P1` | Must be fixed before merge. | Yes |
| `P2` | Should be fixed. | Only if the lens says so |
| `P3` | A nit. | No |

Every finding has `evidence`: a file and line, or the output of a command. A
finding without evidence is an opinion and is rejected. A blocking finding also
has a `revision`, what to change, so whoever revises is told rather than left to
guess.

## The commit

A verdict names the commit it reviewed, forty lower-case hexadecimal digits. A
verdict about a diff that has since moved says nothing about the new one, so a
runtime must take a verdict for a commit only as a verdict for that commit.

## What it does not carry

The design registers of taste (the steward's) are a different vocabulary and are
not here. A lens that wants to say something that is not a finding puts it in
`notes`, which never blocks.
