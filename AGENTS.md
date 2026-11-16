# AGENTS.md

Rules for any agent, and any person, changing this repository. The rules in
Hephaestium's
[AGENTS.md](https://github.com/erinversfeldcodes/hephaestium/blob/main/AGENTS.md)
apply here too. These are the ones particular to content.

## What belongs here

- A skill says nothing a different repository would disagree with. If a
  sentence names a product, one of its modules, its deploy target or its
  domain rules, it belongs in that repository's manifest or documents.
- A pack says nothing a different repository on the same stack would disagree
  with.
- One home per fact. If two files need the same rule, one states it and the
  other links to it.

## How a skill is written

- One method per skill. If a skill needs a second heading that could be used
  without the first, it is two skills.
- Say what to do and why. A rule whose reason is missing gets dropped the
  first time it is inconvenient.
- A rule the runtime enforces is not also written as an instruction. When
  Hephaestium takes over a rule, delete the sentence.
- Every path, skill or contract a file cites must exist. The link check fails
  the build when one does not.
- An example taken from a real incident keeps the lesson and drops the
  product's ticket numbers and module names.

## Changing a skill

- A change to a skill changes how agents behave in every repository that
  takes the next revision. Say in the pull request what behaviour should
  change, and how that would show up in a run.
- Never weaken a rule to make a failing run pass.
