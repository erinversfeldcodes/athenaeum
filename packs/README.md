# Packs

A pack holds what is true for any repository on one stack: Rust, Elixir, and
later the others. It says nothing a different repository on the same stack
would disagree with. What is true of one product, its modules, its deploy
target or its domain rules, belongs in that repository's manifest and
documents, not here.

A repository names the pack a component uses in its manifest
(`manifest:components.*.pack`).

## Layout

```
packs/<name>/review.md     the review lens for the stack, or for every change
```

More files will sit beside it as they are moved in: the specialist's content,
the stack's standards, and the default lint and test commands.

## A review lens

`review.md` is the stack's lens under [the reviewer contract](../skills/review/SKILL.md).
The skill says how to review; the lens says what to look for in this stack.
It begins with front matter:

```
---
name: <name>
description: <what it reviews and when it applies, in a sentence or two>
blocking: true
---
```

- The name is the directory's name.
- `blocking` says whether the lens's verdict stops a change by default. A
  repository can override it in its own manifest.
- `applies: always`, optionally, says the lens is asked for on every change that is
  reviewed, and not only when a component names its pack. A component names one
  pack, so a lens about the change itself (does it do what its ticket asked) or
  about the system it runs in cannot be chosen by a path. No other value is
  accepted. A lens that does not say it applies only where a component names it.

Then it lists the **axes**, each with what to check and why. An axis is
either mechanical or a judgement:

- A **mechanical** axis is settled by a gate's result (the formatter, the
  linter, the audit, the tests). The lens names it so the reviewer knows the
  gate covers it, and the reviewer does not re-run it or re-judge it.
- A **judgement** axis is argued from the code. These are where a lens
  earns its place, because no tool settles them.

Last, it says what each severity means for this stack, so that two reviewers
reading the same change reach the same verdict.

The conventions in [the skills README](../skills/README.md) apply to a lens as
they do to a skill: ask the manifest and never name a path, cite evidence, and
say nothing a different repository would disagree with.

## Checks

`scripts/check-skills.py` fails the build when a lens's name does not match
its directory, when it has no description, when it does not say whether it
blocks, or when it cites a manifest key the schema does not declare.
