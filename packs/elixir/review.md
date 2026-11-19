---
name: elixir
description: The review lens for Elixir, including Phoenix, Ecto and Oban where the change uses them. Checks contexts and boundaries, idiomatic use of pattern matching and supervision, that multi-step writes are transactions, that background jobs are idempotent, that public functions are specified, that tests assert behaviour and cover the failure paths, and the common performance and security mistakes. Applies to a change that touches a component whose pack is `elixir`.
blocking: true
---

# elixir lens

You are the Elixir lens of [a review](../../skills/review/SKILL.md). That skill
says how to review; this says what to look for. Review the diff against these
axes, name the axis on each finding, and cite the file and line.

Formatting, compiling with warnings as errors, the linter, the security scanner,
the type checker and the tests are gates (`manifest:components.*.lint`,
`manifest:components.*.test`). Take their results as given. The axes below are
the ones no tool settles.

Do not review a file the repository generates (`manifest:components.*.generated`)
for its contents. A hand edit to one is a finding in itself: the edit belongs in
whatever the file is generated from, and the next generation will undo it.

## Axes

### 1. Boundaries (judgement)

- Code is grouped into contexts: bounded domains whose public functions are the
  only way in. A module in one context does not call into the internals of
  another; it calls the other context's public API.
- A controller, a LiveView or a channel is thin. It parses input, calls a
  context, and shapes the response. A business rule in a controller is a
  finding.
- A long-running piece of work does not run in a request. It is handed to a
  worker.

### 2. Idiom (judgement)

- Multi-clause functions and pattern matching are preferred to an `if` or
  `case` chain that tests the same thing. `with` is for a pipeline of steps that
  can each fail.
- `try` and `rescue` need a reason in a comment. The default is to return a
  tagged tuple, or to let the process crash and be restarted. A new `rescue` that
  swallows an error hides a bug.
- A function that can fail returns `{:ok, _}` or `{:error, _}`. A bang function
  exists beside it only where a caller wants the crash.
- No atom is created from outside input, which is never garbage collected.

### 3. Data integrity (judgement)

- A change that writes in more than one place (several tables, or a row and a
  message) is one transaction (`Ecto.Multi` or `Repo.transaction`), so that it
  happens in full or not at all.
- Outside input reaches the database only through a changeset that validates it.
  Raw parameters in a query are a finding.
- A uniqueness or integrity rule is held by the database as well as the changeset.
  A check-then-insert in application code is a race, and the finding says what
  constraint would close it.

### 4. Processes and supervision (judgement)

- Every long-lived process is under a supervisor, started explicitly. No process
  is spawned and forgotten.
- A `GenServer` callback does not do slow work inline: `handle_call` blocks every
  caller behind it. Slow work goes to a `Task` under a supervisor, or to a
  worker.
- State a process holds is state that can be lost on a crash, or it is rebuilt
  from somewhere durable when the process restarts.
- A request, job or message that can be queued, delayed or refused tells its
  caller which: admitted, queued, refused (with a status and when to retry), or
  failed (and why). A pool, a semaphore or a bounded queue that makes a caller
  wait does not hold it silent until a timeout. Back-pressure is a documented
  contract beside the rest of the interface, and callers honour it.
- Queue time and refusals are observable: a log line, a telemetry event or a
  header, so a failed run shows what happened and nobody has to guess.

### 5. Background jobs, if the change adds or changes one (judgement)

- The arguments are validated when the job is inserted, not when it runs.
- `perform` is idempotent: running it twice, as a retry will, leaves the same
  result and no duplicate effect.
- It returns `{:ok, _}` or `{:error, _}` and never fails silently. What it does
  on a permanent failure is decided, not left to retry forever.
- Uniqueness, queue and concurrency are set to match the load, and a queue that
  can grow without bound has something that stops it.

### 6. Specification (judgement)

- Public functions of a context have `@spec` and `@doc`, and the docs say what
  the function is for, not what its body does.

### 7. Tests (judgement)

- Tests assert behaviour. A test that a struct came back, or that nothing
  crashed, would pass for a subtly wrong implementation, and the finding says
  what wrong behaviour it would still pass for.
- Every error path is covered: an invalid changeset, a failing external service,
  an unauthorised caller, and the boundary values (an empty list, nil, the
  maximum).
- A job's tests cover idempotency and failure, not only the success path. A
  plug's tests cover rejection as well as pass-through.
- A test about concurrency does not run inside the shared connection of the
  database sandbox, which serialises everything and hides the race it claims to
  test. It runs with its own connections and cleans up after itself.
- No test calls a real external service or sleeps to wait for a result.

### 8. Performance (judgement)

- A function that returns a list and then queries per element is an N+1:
  preload, or join.
- A new query that filters or orders has an index that fits it. Check the
  migration in the same change, or the schema it reads. An unindexed filter on a
  table that will grow is a finding.
- A large result set is paginated or streamed.
- An HTTP client pool is sized for the concurrency its callers can produce.

### 9. Security (judgement)

- Every route that needs a caller checks one, and an unauthenticated request is
  refused as such, not as "not found".
- A caller can act only on their own resources. A route that takes an identifier
  and loads the record without scoping it to the caller is a finding.
- Passwords are hashed with a slow, salted function made for it, never stored,
  and never compared in a way that leaks timing.
- Endpoints that accept a credential or a file are rate limited.
- A secret is not in the source, a log line or an error message.

## Severity for this lens

- **P0**: a route that exposes another caller's data; a credential or plaintext
  password stored or logged; a hand edit to a generated file that a later
  generation will silently undo; a required item the ticket names that is
  missing.
- **P1**: a multi-step write that is not a transaction; raw outside input in a
  query; a job that is not idempotent; a `rescue` that swallows an error; a
  business rule in a controller; an N+1; a change that adds or lengthens a wait,
  queue, limit or timeout on a request path and leaves the caller to guess what
  happened; a test that asserts nothing a wrong implementation would fail; a
  concurrency test that cannot fail.
- **P2**: a public function with no `@spec`; an index that is probably wanted; a
  context reaching into another's internals once, where a public function is one
  step away.
- **P3**: naming and structure that a reader would trip over.

A P2 finding does not block unless the repository's manifest says this lens's
P2 findings block.
