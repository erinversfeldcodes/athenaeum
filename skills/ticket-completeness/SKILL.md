---
name: ticket-completeness
description: Judge whether a ticket's goal is met, requirement by requirement, against the state the repository is in now and the evidence gathered since its changes merged. Ends in a verdict (met, not met, owed or cannot judge) with the evidence for each requirement, and for anything owed the command, the condition and the threshold that will settle it. Use when deciding whether a ticket can be closed, when a ticket stays open after its pull request merged, when its owed measurements may have come due, or when asked whether a ticket is really finished.
---

# ticket-completeness

A change that is green, reviewed and merged says that a change is ready. It does
not say the ticket is finished. A ticket can have every line of its pull
request approved and still be open for good reason: a measurement that can
only be taken after the merge, a cause stated as a guess, a goal that the
requirements did not quite capture. This skill judges the ticket as a whole and
says which of those it is.

It is not a review. A [review](../review/SKILL.md) judges one change at one
commit. [completion-audit](../completion-audit/SKILL.md) tries to disprove a
claim of done before it is made. This skill judges the ticket against what is
true now, including evidence that did not exist when the change was reviewed,
and ends in a verdict that a person or a runtime can act on.

The conventions in [the skills README](../README.md) apply.

## What it reads

- The ticket: its body, its comments, and the specification it names. The
  ticket's own words are the reference. Do not restate its requirements from
  memory.
- Every change that refers to it, merged or not, with its description.
- The default branch as it is now. A change that merged and was later reverted
  or reworked is not evidence.
- Evidence gathered since: runs, measurements, logs, what a person said.

Whatever you cannot read, say so. A requirement that depends on it is
*cannot judge*, never *met*.

## Method

1. **List the requirements.** Number each outcome the ticket asks for, each
   proof it says will show it (a row in a table of proofs is one), and each
   thing it rules out. Quote its words. Keep what the ticket wants separate
   from how it says that will be shown, and from why it wants it, which is its
   goal. A requirement that can be read two ways is recorded both ways, not
   resolved to the easier one.

2. **Find the evidence for each**, and rank it. A requirement about behaviour
   needs one of the first two kinds.
   1. *Observed in the real system after the change*: a measurement, a drive
      through the real interface, a run on the default branch.
   2. *A test that failed before the change and passes after it*, named and
      run, whose strength has been probed with
      [mutation-probe](../mutation-probe/SKILL.md).
   3. *Structure*: the code exists, the setting is changed, the document says
      so. It shows a thing was built, not that it works.
   4. *A claim*: a description, a ticked box. It is nothing until you have
      looked.

3. **Check the evidence is what it appears to be.**
   - It is about the default branch as it is, not about a branch that has
     since changed.
   - The merged changes did not remove, skip or weaken a test to get to green.
     Read the diff of the tests, not only the result.
   - A figure can be reproduced from a command, says what it leaves out, and
     compares like with like. An after sample smaller than the before sample
     cannot show a rate fell.
   - A stated cause is shown with evidence quoted from a failing run, or it is
     called a hypothesis. A change that alters a wait, a limit or a retry
     count while the cause is elsewhere is a mitigation, whatever it is called.
   - What the evidence shows is the requirement, and not something nearby.

4. **Give each requirement a status.**
   - **met**: evidence of the first two kinds, checked as above.
   - **not met**: the evidence shows it is not, or there is only structure or a
     claim where behaviour was asked for.
   - **owed**: it cannot be settled yet, because the evidence does not exist
     yet and will when something happens: a measurement after the merge, a
     deploy, a decision that is a person's. An owed item is not a failure, but
     it is not met either. It must name what produces the evidence, the
     command, the condition under which that can be run, and the threshold
     that decides it.
   - **cannot judge**: the evidence may exist and you cannot see it.

5. **Judge the goal, not only the list.** With every line met, ask whether what
   the ticket is for has happened. A check made to pass, with the failure it
   caught still there, meets the line and misses the goal. If the goal can only
   be judged later, from users or from production, it is owed, and says what
   will show it.

6. **Account for what is left.** Look for work claimed that the ticket did not
   ask for, and for words such as "remains", "follow-up" and "later" with no
   ticket behind them. A deferral with no ticket is unfinished work and is
   listed. The same sweep is in [completion-audit](../completion-audit/SKILL.md).

## The verdict

The worst status among the requirements decides it, in this order: *not met*,
then *cannot judge*, then *owed*, then *met*. Write it in this shape, so a
runtime can read the verdict and a person can read the rest.

```
## Verdict: <met | not met | owed | cannot judge>

State judged: <the default branch's commit, and the time>

| # | Requirement (the ticket's words) | Status | Evidence (kind, and where) | Gap or next step |
|---|---|---|---|---|

## Goal
<achieved, not achieved, or owed, and why in a sentence or two>

## Owed
- owed: <what> | run: <command> | when: <condition> | settled-by: <threshold>

## Not filed
- <residue with no ticket behind it>

## Could not check
- <what was out of reach>
```

- **met** means the evidence supports closing the ticket. Nothing here closes it.
  Closing is the decision of whoever called this skill.
- **owed** means the ticket stays open and the next evaluation is only of the
  owed items. Write each one so a later run can pick it up without redoing the
  rest.
- A verdict is about a named commit at a named time. The next evaluation may
  differ, and should say what changed.

## Rules

- **Do not take the description's word.** Read the evidence it cites, run what
  can be run, and cite what you saw.
- **Do not fault a requirement for evidence nobody could have produced yet.**
  Name it as owed. Equally, do not let owed turn into met because time passed:
  the threshold decides.
- **Judge only.** Do not edit the ticket, close it, label it or open a change.
  A finding that needs work becomes a ticket for a person or the runtime to
  take on.
- **When nobody can answer a question** about what the ticket meant, end with the
  question and the reading you would choose, as the conventions say.

## Where this comes from

Each practice here was taken from published work, and the changes to it are
this skill's own.

- The requirement-by-evidence table is the coverage view of Spec Kit's analyze
  step, applied after the build and not before it. See
  [Spec Kit](https://github.github.io/spec-kit/quickstart.html).
- Ranking evidence by whether the real system was observed, and turning what is
  found into a permanent test, is from Simon Willison's
  [agentic manual testing](https://simonwillison.net/guides/agentic-engineering-patterns/agentic-manual-testing).
- Reading the tests' diff for deleted or disabled tests follows Kent Beck's
  account of agents making a suite pass by removing tests, in
  [Augmented Coding: Beyond the Vibes](https://www.kentbeck.com/summaries/augmented-coding-beyond-the-vibes/).
- Keeping the ticket as the reference that the change is judged against, and
  not the change's own account of itself, is close to the spec-anchored level
  in Birgitta Böckeler's
  [Kiro, spec-kit and Tessl](https://www.martinfowler.com/articles/exploring-gen-ai/sdd-3-tools.html).
- Citing a run before making a claim is the discipline of the
  [verification-before-completion](https://tessl.io/registry/skills/github/obra/superpowers/verification-before-completion)
  skill.
