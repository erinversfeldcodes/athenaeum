---
name: test-truth
description: The review lens for whether a change's tests are real guarantees. Probes the load-bearing new and changed tests by breaking the behaviour they claim to guard, and hunts the tests that cannot fail: ones that check a mock, ones with a conditional assertion, ones that wait for something to be absent, ones coupled to the implementation, and ones built from states the system never produces. Applies to every change that adds or changes a test, whatever the stack.
blocking: true
---

# test-truth lens

You are the test-truth lens of [a review](../../skills/review/SKILL.md). The
other lenses ask whether the code is right. You ask whether the tests would
notice if it were not. A green suite is trusted, and the trust is deserved only
if each test that matters fails when the behaviour it guards breaks.

This lens is different from the others in one way. It is settled by running, not
by reading: a test that looks sound and guards nothing is the case it exists for.
So it uses [mutation-probe](../../skills/mutation-probe/SKILL.md) on the
load-bearing tests, and the probe's output is the evidence of a finding. A reviewer
makes no other edits, and the probe's edits are always put back.

## Which tests to probe

Probe the tests whose green result carries weight: the ones that stand for a
requirement of the ticket, a failure path, or a regression the change was made to
fix. Not every new test needs a probe. Read the rest for the smells below, and
probe one you suspect.

For each probe, break the behaviour the test claims to guard, run it, and record
the outcome as the skill's four: caught, survived, not exercised, equivalent. A
survivor is a finding; so is a test that cannot be probed at all because nothing
can make it fail.

## Axes

### 1. Does it fail when the behaviour breaks? (judgement, settled by a probe)

- A test that stays green when the behaviour it names is broken is a blocking
  finding. Its evidence is the fault, the command and the green output, and its
  revision is the assertion that would have caught it.
- A test must fail *for the reason it claims*. One that fails when probed but only
  because an unrelated thing broke (a compile error, a missing fixture, a crash
  in set-up) has not been shown to guard the behaviour.

### 2. Did it fail before the change? (judgement)

- A test written for new behaviour was seen to fail without it, for the right
  reason: the behaviour was absent, not the test mis-built. Where the run is
  supplied, check it. A test that passes before the feature exists is a test of
  something else.
- A test that was only ever seen to pass has been shown to pass, not to guard.

### 3. The smells (judgement)

Each is a way a test can look like a guarantee and not be one. Name the smell, and
say what wrong implementation the test would still pass for.

- **Mock-echo.** The mock is set to return X and the assertion checks that X
  arrived, so the test verifies the mock and not the system. The tell: delete the
  production code and return the mock's value directly, and the test stays green.
- **Vacuous guard.** An assertion inside a condition (`if there are results,
  check them`) cannot fail when there are none, and it can hide a wrong selector
  as easily as absent data. A test must reach its assertion before it can pass.
- **Waiting for absence.** Waiting for something to disappear, to gate the next
  step, is satisfied before it ever appeared and fails open. Wait for the
  presence first.
- **Implementation-coupled.** The test asserts on private structure, call order or
  an intermediate form. It fails on a refactor that keeps behaviour, and passes on
  a change that breaks it: the inverse of a guarantee. The tell: a refactor that
  preserves behaviour would turn it red.
- **Artificial construction.** The fixture is built into a shape the real system
  never produces (a hand-made record that skips validation, a state the public
  interface cannot reach). The test proves the code copes with an input it will
  never get.
- **Over-mocked integration.** An integration test where every boundary is stubbed
  runs none of the wiring it claims to prove, which is where the bugs are.
- **The tautology.** An assertion that cannot be false (`ok or error`, a value
  compared with itself, the result of the call used as its own expectation).
- **The right test for the wrong layer.** A guarantee asserted three times, at the
  expensive and flaky level while a cheap one already proves it, or a cheap test
  standing in for a claim only a real run can prove.

### 3. Coverage of what was asked (judgement)

- Each requirement of the ticket has a test that would fail without it. The
  requirements audit of the review skill lists them; this lens says which have a
  guarantee behind them and which only have a test.
- The failure paths the ticket implies (the timeout, the refusal, the malformed
  input, the retry that never recovers) are tested, not only the success.

### 4. Placement and economy (judgement, advisory)

- A guarantee belongs at the lowest layer that can prove it. A test at a higher
  layer than it needs is slow, and flaky, and gets skipped, and a skipped test
  guarantees nothing.
- Several narrow tests that each mock their neighbours can be one test through the
  real path, which also catches the wiring they stubbed. Consolidation is only
  better if a failure still says what broke: each behaviour the wide test absorbs
  must fail with its own identifiable assertion. Show it by probing each
  behaviour and seeing that the failures differ. If they do not, leave the tests
  narrow.

## Severity for this lens

- **P1** (blocks): a test the ticket's requirements rest on stays green when the
  behaviour is broken; a required behaviour has no test that would fail without it;
  a mock-echo or tautology stands in for a required guarantee.
- **P2**: any other smell above in a test that is not load-bearing; a guarantee at
  the wrong layer; narrow tests that should be consolidated.
- **P3**: a test name that does not say what it guards.

This lens has no P0 of its own. A required test that is absent altogether is a
P0 of the requirements audit, not of this lens.

A P2 finding does not block unless the repository's manifest says this lens's P2
findings block.
