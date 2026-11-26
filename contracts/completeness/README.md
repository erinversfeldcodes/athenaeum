# Completeness verdict

What judging whether a ticket's goal is met returns for one ticket at one commit
of its default branch. A review judges one change at one commit. This judges the
ticket as a whole, against the state it is in now and the evidence gathered since
its changes merged, so that a runtime can read the result and a person can read
the rest.

- [`schema.json`](./schema.json) is the definition. Every field and rule is
  stated there and nowhere else.
- [`example.json`](./example.json) is a ticket whose requirements are all met by
  observed evidence.
- [`cases/`](./cases) holds verdicts that must be accepted and verdicts that must
  be rejected. See [how a contract is held to account](../README.md).

The method that produces one is [the `ticket-completeness` skill](../../skills/ticket-completeness/SKILL.md).

## The four verdicts

The worst status among the requirements decides it, in this order.

| Verdict | Means | The requirements must hold |
|---|---|---|
| `not_met` | Something is not there, or is shown only by structure or a claim. | At least one `not_met`. |
| `cannot_judge` | The evidence may exist and could not be seen. | At least one `cannot_judge`, and none `not_met`. |
| `owed` | Nothing fails, but something cannot be settled yet. | At least one `owed`, none `not_met` or `cannot_judge`, and at least one entry in `owed`. |
| `met` | The evidence supports closing the ticket. | Every one `met`, and the goal `achieved`. |

A `met` requirement has evidence of the kind `observed` or `test`. Structure and a
claim show something was built or said, not that it works, so they cannot make a
requirement met. Evidence that says it was not `checked` in this evaluation cannot
either.

## Who decides

`decides` says who should decide whether to close the ticket, and `decides_because`
says why. It is `evidence` only when the verdict is `met`, every requirement is
`settled_by` evidence, and the ticket is not an `epic`; otherwise it is `person`.
A verdict cannot say the evidence decides while a requirement needs a person, or
for an epic, or when it is not met.

## What is owed

An owed item has what is owed, the command that produces it, the condition under
which it can be run and the threshold that decides it, so that a later evaluation
can pick up only what is owed. It may say its `arrival`: whether it `arrives` by
itself or is `needs_action`, and then what the action is and who does it, so that
nobody waits for evidence that will never come.

`checked` and `arrival` are optional, because the reader that validates a verdict
can be an older release than the skill that asks for it.

## The commit

A verdict names the commit of the default branch it judged, forty lower-case
hexadecimal digits. A verdict about a state that has since moved says nothing
about the new one, so a runtime must take a verdict only as a verdict for that
commit.
