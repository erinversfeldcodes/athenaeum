---
name: close-the-gap
description: When a ticket is not complete, question the person one question at a time to find what is missing, what needs rework and what was misjudged, then propose the tickets that close the gap so the work can go back to agents. Each run is one step of the conversation: it reads the thread, then asks the next question, proposes the tickets, or says it has nothing more to ask. Use after a completeness verdict of not met or cannot judge, when a person judges an epic incomplete, or when a person reopens a ticket that was closed.
---

# close-the-gap

A ticket that is not complete leaves a gap, and the person who knows what the
gap is has rarely written it down. Reporting "not met" and stopping leaves them
to work it out again. This skill finds the gap by asking, and ends in tickets
small enough to give to an agent.

It follows a [completeness verdict](../ticket-completeness/SKILL.md) that
found the ticket not complete, or a person's judgement that it is not. It does
not repeat that skill's judging.

The conventions in [the skills README](../README.md) apply.

## One step at a time

The conversation takes days, and nobody is waiting on it. So each run is one
step and must stand alone: it reads the whole thread, the verdict, the
sub-tickets and every earlier question and answer, and does not rely on
remembering any of them. It ends in exactly one of three ways, and says which
by its heading.

- `## Next question`: one question, with the answer you would choose and why.
- `## Proposed tickets`: the tickets that close the gap.
- `## Nothing more to ask`: the gap is already clear, or the person has said
  they have nothing to add. Say what you take the gap to be.

## What to find

Look for these in turn, and only ask about what the ticket, the verdict, the
code and the earlier answers do not already tell you.

- **Missing**: a requirement the ticket asked for that is not there. What work
  would make it so?
- **Needs rework**: a requirement that was met in a way the person does not
  want, or met by evidence that does not show it. What should change?
- **Misjudged**: a requirement that was wrong, or could be read two ways, or
  that the person no longer wants. What should the ticket have said?
- **Owed**: something that cannot be known yet. Who will know, and when?
- **Beyond the ticket**: something found on the way that is a new concern and
  not a larger version of this one.

## How to ask

- **Investigate before asking.** A question the repository already answers
  wastes a person's turn.
- **One question at a time**, the one whose answer changes the most. Do not
  list five.
- **Every question carries the answer you would choose, and why.** A bare menu
  makes the person do the work.
- **Ask in the ticket's words and the person's**, not in the skill's.
- A person who answers a different question than the one asked has told you
  something. Read it before asking again.

## The proposal

When the gap is clear, propose the tickets that close it. Each one is scoped as
[scope-request](../scope-request/SKILL.md) scopes work: one concern, what it is
for, what is in and out of scope, and how each behaviour will be proven. Write
them in this shape, so a runtime can create them and a person can read them.

```
## Proposed tickets

### <title>
Parent: <the ticket this closes a gap in>
Depends on: <other proposed tickets, or none>

<the ticket's body: goal, scope, and how each behaviour will be proven>
```

- A ticket is small enough for one change a person can review with care.
- Say the order, and what can go in parallel.
- A requirement that cannot be settled now becomes a ticket that says what it is
  waiting for, not a loose sentence.

## Rules

- **Never approve your own tickets, and never start the work.** Approval to begin
  is the person's.
- **Never close, reopen, label or edit** the ticket. You ask and you propose.
- **Never push the person to an answer.** If they have nothing to add, stop.
- **Do not decide for the person** what is a matter of taste, design or choice.
  Put it to them with your recommendation.
- **When nobody can answer**, end with the questions and the answer you would
  choose for each, as the conventions say.

## Where this comes from

The loop of questioning a person about a plan until each branch is settled, one
question at a time, is the practice of the `grill-me` skill in
[mattpocock/skills](https://github.com/pipedrive/mattpocock-skills). The
change here is that it starts from a verdict that something is missing, runs
across days, and ends in tickets that wait for a person's approval.
