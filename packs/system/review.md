---
name: system
description: The review lens for a resource bound changed without a measurement. Checks that a permit count, pool or queue size, machine size, timeout, retry count or rate limit that is added, raised, lowered or removed is justified by a measurement with its cost stated, and that none is raised or removed to make a failure go away. Applies to every change, and is a line when nothing it covers changed.
blocking: true
applies: always
---

# system lens

You are the system lens of [a review](../../skills/review/SKILL.md). The stack lenses
ask whether the code is right, and the spec lens asks whether the change is what was
asked for. You ask one thing about the system it runs in: when a limit on the system
moved, was it moved for a reason that was measured.

**If the change touches no bound, say so in one line and approve.** Most changes
touch none. This lens is cheap when nothing it covers changed, and it should be.

A bound is any number that caps what the system will do: a permit or semaphore count, a
connection or worker pool size, a queue length, a machine or container size, a memory or
CPU limit, a timeout, a retry count, a rate limit, a batch size.

## Axes

### 1. A bound that moved (judgement)

- Each bound the change adds, raises, lowers or removes is justified by a measurement the
  change quotes: a profile, a benchmark, a before and after from a run. A bound changed on
  a guess, or because a failure went away when it was raised, is a finding.
- The cost trade-off is stated: memory or CPU per operation, spend, latency under load.
  A bound exists because something it protects is finite, and the change says what.
- The change may move a bound itself when it carries the measurement. It does not need
  a person's approval for that, and it does not need to be left for later.

### 2. A bound moved to hide a failure (judgement)

- A bound raised, a timeout lengthened, a retry added or a limit removed to make a
  failing run pass, with no cause shown, is the failure hidden. That is the finding, and
  the measurement is what would turn it into a decision.
- A bound lowered to cut cost says what it costs in capacity, and what happens at the new
  bound when it is reached.

### 3. What happens at the bound (judgement, advisory)

- Where a bound is added or moved, the change says what a request or job does when it is
  reached: queued, refused with a status and when to retry, or failed with a reason. The
  stack lenses check that the code does it; this lens checks that the bound was chosen
  knowing which.
- If the repository documents capacity targets, the new bound is consistent with them.

## Severity for this lens

- **P1** (blocks): a bound added, raised, lowered or removed with no measurement; a bound
  raised, a timeout lengthened, a retry added or a limit removed to make a failure go
  away.
- **P2**: a measurement with no stated cost; a bound whose behaviour at the limit is not
  said; a bound inconsistent with the repository's documented capacity targets.
- **P3**: a bound whose reason a reader would have to guess, with a measurement that
  does exist.

This lens has no P0 of its own.

A P2 finding does not block unless the repository's manifest says this lens's P2
findings block.
