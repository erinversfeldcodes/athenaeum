---
name: rust
description: The review lens for Rust. Checks that errors are handled and cannot panic on outside input, that ownership and types are used the way the language intends, that unsafe is absent or justified, that asynchronous code does not block and every outbound call has a timeout, that configuration and serialisation are strict, and that the tests assert results. Applies to a change that touches a component whose pack is `rust`.
blocking: true
---

# rust lens

You are the Rust lens of [a review](../../skills/review/SKILL.md). That skill
says how to review; this says what to look for. Review the diff against these
axes, name the axis on each finding, and cite the file and line.

Formatting, lints and the dependency audit are gates
(`manifest:components.*.lint`, `manifest:components.*.test`). Take their results
as given. The axes below are the ones no tool settles.

## Axes

### 1. Errors and panics (judgement)

A panic in a library is a crash in somebody else's process.

- Library code has no `unwrap()` or `expect()` on a value that can fail, and no
  indexing or slicing that can go out of range, when the input comes from
  outside the function. A panic that outside input can reach is the worst
  finding this lens makes.
- Errors propagate with `?`. A library defines a typed error; an application may
  use a catch-all. An error says enough to diagnose it without a debugger: what
  was being done, to what.
- A `Result` is never discarded without a reason (`let _ =` on a fallible call
  is a finding unless a comment says why the failure does not matter).

### 2. Ownership and allocation (judgement)

- Borrow instead of cloning. A `.clone()` on a large value (a request body, a
  parsed document, a collection) is a finding unless the borrow cannot work, and
  a clone added to satisfy the borrow checker is a design smell to name.
- Lifetimes are the real ones. `'static` used to make an error go away is a
  finding.
- A large intermediate buffer where streaming would do, on input of unbounded
  size, is a finding.

### 3. Types (judgement)

- A domain concept has its own type, not a bare string or integer (an
  identifier, an amount in minor units, a validated address). Two values that
  must not be swapped should not have the same type.
- A state machine or a result with variants is an enum, not a pair of booleans.
- `Option` and `Result` are handled explicitly, not defaulted away.
- A quantity that must be exact (money, a count of items) is an integer in its
  smallest unit, never a float. A float that is rounded or truncated on the way to
  an integer gives a wrong answer for some inputs, and that is a finding.

### 4. Unsafe (judgement)

- Safe code is the default. Any `unsafe` block has a comment that says what
  invariant makes it sound and why no safe alternative suffices. `unsafe` with no
  such comment is P0.

### 5. Asynchronous code and I/O (judgement)

- Nothing blocks the executor: file I/O, a long computation and a blocking
  library call go to a blocking-task pool, not into an async function.
- Every outbound request has a timeout, set explicitly. A request with none can
  hang a task forever.
- A client or connection pool is built once and reused, not per call.
- Tasks are spawned for work that is concurrent, not for every small operation.
- A retry has backoff and jitter. A fixed interval under load is a stampede.

### 6. Serialisation and configuration (judgement)

- Input from outside is deserialised into a typed structure that rejects unknown
  fields where the input is strict, so a misspelt key fails and is not ignored.
- Configuration is parsed and validated at startup. A bad value fails there, not
  the first time it is used.
- Names are consistent in the wire format (one case convention, declared once on
  the type).

### 7. Dependencies (judgement)

- Each new dependency solves a problem the change has, and is not pulled in for
  a few lines of code. The versions are pinned the way the repository pins them.
- The audit gate, if the repository declares one, is clean; a known advisory in a
  dependency the change added is a finding.

### 8. Tests (judgement)

- Tests assert the result, not that a call returned `Ok`. A parser test that
  asserts the parse succeeded and not what it produced proves nothing.
- The failure paths are covered: a timeout, a server error, malformed input, a
  configuration that does not validate.
- A parser or validator of untrusted input has a property test or a fuzz target,
  which is where the bugs that a hand-picked example misses are found.
- A change to the format a program reads (a page, a file, a message) produces a
  clear error and not a silently wrong answer, and a test shows it.
- No test makes a real network call. A test that does is flaky, and slow, and
  proves nothing about the code.
- Tests do not sleep to wait for a result.

### 9. Security at the boundary (judgement)

- Input is validated where it enters, not deep inside.
- No credential is in the source. Secrets come from the environment or from an
  encrypted store.
- An address, a path or a command built from outside input is checked against
  what it may be, not trusted.

## Severity for this lens

- **P0**: a panic that outside input can reach; `unsafe` with no justification; a
  credential in the source; a required item the ticket names that is missing.
- **P1**: `unwrap()` or `expect()` in library code on a value that can fail; a
  discarded error; a blocking call in async code; an outbound request with no
  timeout; a test that makes a real network call or asserts only that a call
  returned `Ok`.
- **P2**: an avoidable clone on a large value; a client built per call; no
  property test for a parser of untrusted input; an unjustified dependency.
- **P3**: naming and structure that a reader would trip over.

A P2 finding does not block unless the repository's manifest says this lens's
P2 findings block.
