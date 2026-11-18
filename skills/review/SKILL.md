---
name: review
description: Review one change, pinned to a commit, as one lens (a stack, the contracts between components, whether the tests would fail without the change, and so on), and return a verdict in the review-verdict contract. Use when asked to review a diff or a pull request, or when a runtime picks lenses for a change. A reviewer never writes code: it audits the ticket's requirements before reading the implementer's report, takes the gate results as given, checks the lens's axes, and returns findings with evidence.
---

# review

A review is a second pair of eyes. It is only worth having if it is independent
of the first pair, so a reviewer reads what was asked before it reads what was
claimed, and it returns what it found and nothing it fixed. The output is a
[review verdict](../../contracts/review/README.md): a verdict and findings that
a runtime can act on.

The conventions in [the skills README](../README.md) apply.

## What a lens is

You are one lens: a reviewer pointed at one kind of concern. A stack lens knows
what good code in that stack looks like. The `test-truth` lens asks whether each
test would fail if the feature were removed. A lens supplies the checks, its
axes, and this skill supplies everything else. For a component, the lens is
its stack's pack (`manifest:components.*.pack`). Name the lens in your verdict,
and name the axis a finding belongs to.

Review only what your lens covers. A concern outside it is a note for the lens
that covers it, not a finding of yours.

## Steps

1. **Pin the review to a commit.** Review one commit, say which, and put it in
   the verdict. If the work has moved since you started, your verdict is about
   the old commit and says so; it is not about the new one.

2. **Do not write code.** Do not edit the ticket, the plan, the tests or the
   implementation. The one exception is a probe: the test-truth lens breaks the
   behaviour a test guards to see whether the test notices
   ([mutation-probe](../mutation-probe/SKILL.md)), and puts it back. A probe is
   evidence-gathering, is never part of what you deliver, and is never left behind. A reviewer that fixes what it finds has stopped reviewing,
   and the person or run that wrote the change never learns what was wrong with
   it. Say what to change, in the finding.

3. **Audit the requirements before you read the report.** The implementer's
   completion report is a claim. The ticket is the ground truth. Before reading
   the report, list what the ticket requires: every behaviour, interface,
   command and file it names, and every acceptance criterion. Then check each
   against the diff and the tree.
   - A required item that is absent is a P0 finding, whatever the report says.
   - An item that is present and not tested is a finding.
   - Name `requirements` as the axis of the findings this step makes, so they are
     told apart from the lens's own.
   - Do this with [completion-audit](../completion-audit/SKILL.md) in mind: it
     is the author's version of the same pass, and a reviewer should not trust
     that it was done.

4. **Take the gate results as given.** The repository's gates
   (`manifest:gates`) are run for you, on this commit, and their results are
   supplied. Do not run the suite again to confirm them. If a result is missing,
   or a gate failed or timed out, return `needs_revision` and say which: an item
   with no passing evidence is not satisfied. A gate that was skipped is
   acceptable only if the manifest allows it to be.

5. **Check that the tests are first-class.** Every required behaviour has a test
   that asserts the behaviour, not that the code exists or returned without
   error. Where a recorded failing run from before the implementation is
   supplied, check it fails for the reason the behaviour is absent. Where none
   is supplied and the change is not documentation, ask for it: no evidence the
   tests ever failed is a blocking finding.

6. **Walk the lens's axes.** Take them from the lens, in order, and for each say
   what you looked at. A judgement axis (is this the right design) is argued
   from the code. A mechanical axis (is it formatted, does it lint) is settled by
   the gate results and not re-judged.

7. **Trace the stories end to end.** For every story the ticket serves
   (`manifest:docs.stories`), follow the interaction through the code from entry
   to result and say whether the acceptance criteria are met. Do not stop at
   the first story. Reading code shows what it says; where you can observe what
   it does, do.

8. **Research alternatives, and keep them out of the findings.** For a
   significant design choice, look for an alternative: what it is, the trade-off
   against what was done, and whether it is worth raising with a person now or
   later. Put these in `notes`. They never block, because the change was judged
   against the requirements, not against what you would have built.

9. **Check what is built on this.** Find the tickets that depend on this one
   and the work that comes next. For each, say what it needs from this change
   and whether the change provides it. A gap is a finding.

10. **Return the verdict.** Choose it by the rules in
    [the contract](../../contracts/review/README.md), not by feel:
    - `approved` when nothing blocks. Advisory findings and notes are welcome.
    - `needs_revision` when something blocks and can be fixed in place.
    - `failed` when the approach cannot be revised into shape, a required item
      is missing, or there is a critical flaw. It carries a P0 finding.

    Every finding has evidence: a file and line, or a command's output. A
    blocking finding says what to change. If you cannot cite evidence, you have
    an impression, not a finding: look until you have the evidence or drop it.

## Reviewing a revision

A review of a revised commit is a new review of the new commit, not a check that
the old findings were touched. Re-read what the ticket requires, check whether
each earlier blocking finding is now resolved, and review the new changes as you
would any. If a blocking finding comes back unchanged, say so in the finding:
it is the signal that revising is not working.

## What a reviewer does when nobody can answer

A reviewer does not ask. If the ticket is ambiguous in a way that decides the
verdict, say which reading you took, and return the verdict that reading
supports. The question belongs in a note, with the default you would
recommend.
