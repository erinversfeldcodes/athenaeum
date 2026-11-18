---
name: mutation-probe
description: Find out whether a test is a real guarantee by breaking the behaviour it claims to guard and seeing whether it fails. Plant a small deliberate fault, run the test, and record whether it caught it; a test that stays green guards nothing. Use when a test's green result is being trusted, when reviewing new or changed tests, when a test was written after the code, or when asked whether a suite would notice a regression.
---

# mutation-probe

A test is a claim: "if this behaviour breaks, I fail". A green run cannot tell
you whether the claim is true, because a test that checks nothing is green too.
The only way to find out is to break the behaviour and look. This skill is how,
and how to avoid being fooled by the probe itself.

The conventions in [the skills README](../README.md) apply.

## Steps

1. **Start from a state you can restore.** Commit or stash real work first, so
   that putting the probe back cannot destroy it. If you cannot, diff the file
   before and after and put back only the probe.

2. **Take a baseline.** Run the test you are probing and see it pass. Record the
   command and the count. A test that is not green to begin with proves nothing
   about a probe.

3. **Choose the fault.** Make the smallest change that breaks the behaviour the
   test claims to guard, the kind a real mistake makes: invert a condition, return
   the wrong value, skip a side effect, drop a validation, move a boundary by
   one. One fault per probe, aimed at one claim. Aim at the load-bearing tests,
   the ones whose green result people trust, and not at every test: a suite of
   hundreds does not need hundreds of probes.

4. **Confirm the fault is really there.** Look at the file, or at the diff. A
   fault that failed to apply (a pattern that did not match the formatted code,
   an edit to the wrong copy) leaves the test green, and that looks exactly like
   a test that guards nothing.

5. **Run the test and classify what happened.** There are four outcomes, and only
   one is a catch.
   - **Caught.** The test failed, and the failure is about the behaviour you
     broke: its assertion, not an incidental error. This is the guarantee.
   - **Survived.** The test stayed green. It claims a guarantee it does not give.
     This is the finding, and the output is its evidence.
   - **Not exercised.** The fault did not compile, or crashed before reaching the
     test, or was caught only by a different test than the one you probed. The
     probe says nothing about the test. Make a different fault.
   - **Equivalent.** The fault changed no observable behaviour, so no test could
     catch it (a guard that another check already implies). Say so. Do not write a
     test to kill a fault that changes nothing.

6. **Put it back, and prove you did.** Restore the file, then check that nothing
   of the probe remains (`git diff` shows nothing from it) and that the baseline
   is green again. Do this before the next probe and before you report.

7. **Report what you did.** For each probe: the fault, the test, the outcome, and
   for a survivor the assertion that would have caught it.

## Rules that exist because probes go wrong

- **One probe at a time, never two at once.** Two probes editing the same files
  corrupt each other and leave a fault behind that looks like a real failure.
  Do not run probes in parallel in one working tree. If you must, give each its
  own copy.
- **Never leave a probe behind.** A report delivered while a fault is still in
  the tree is a bug shipped. Restore from the commit, not from memory, so a probe
  that is interrupted still comes out.
- **A fault that does not apply is not a survivor.** See step 4. Check before you
  conclude a test is weak.
- **Make sure the run uses the code you edited.** Some toolchains cache compiled
  output by file size and modification time, so a probe that changes a file to
  another of the same size within the same second can run the old code and report
  a survivor that is nothing of the kind. Clear the cache, or run with it
  disabled, for probes, and check one known fault is caught before trusting a run
  of survivors.
- **Do not probe in a shared or live environment.** A probe edits code and runs
  it: do it only where an accident harms nothing, and with every external
  program stood in for. A probe of a safety check must not use the real thing it
  guards.
- **A test that passes before the feature exists is not guarding the feature.**
  Probing is the same question asked of a finished test. Writing the test first
  and seeing it fail for the right reason asks it earlier, and is cheaper.
