# Skills

A skill teaches an agent one method. It is used unchanged in every managed
repository, by a person working in an agent runtime and by an unattended
workload.

## Layout

```
skills/<name>/SKILL.md
```

The file begins with front matter:

```
---
name: <name>
description: <what it does and when to use it, in a sentence or two>
---
```

The name is the directory's name. The description is how an agent decides
whether the skill applies, so it says when to use it and not only what it is.

## Conventions

These are stated here once. A skill follows them and does not repeat them.

### Ask the manifest, never name a path

A skill does not know where a repository keeps anything. It names what it
needs by its key in the repository's manifest, written like this:

- `manifest:docs.architecture`, the architecture document
- `manifest:components.*.test`, the test command of whichever component is in
  question
- `manifest:gates.*.run`, the command of a gate

The manifest is `.athenaeum/manifest.toml` in the repository. Its keys are
defined in [`manifest/schema.json`](../manifest/schema.json). When a
repository's manifest does not declare a key a skill asks for, say so and
carry on without it; do not guess a path.

### Refer to another skill by linking to it

A skill that hands work to another links to that skill's file. A link to a
skill that does not exist fails the build, so a skill never promises a method
nobody wrote.

### Say nothing a different repository would disagree with

No product, no module, no stack, no deploy target. A rule that is true only
for one stack belongs in that stack's pack. A fact about one product belongs
in that repository.

Nothing that only one agent runtime has, either. Say "ask the maintainer",
not the name of one runtime's tool for asking.

### When nobody can answer

A skill may need a decision from a person. A workload may be running with
nobody there.

- If someone can answer, ask, and recommend a default with every question.
- If nobody can, do not guess and do not wait. Stop, and end with the
  questions, each with the default you would recommend. The run's caller
  tells you which situation you are in.

A question the repository already answers is never asked in either case.

### Evidence

A claim that something exists or works cites how you know: a file and line, a
command and its output, or something you observed. Reading code shows what it
says, not what it does.

## Checks

`scripts/check-skills.py` fails the build when a skill's name does not match
its directory, when it has no description, or when it cites a manifest key
the schema does not declare.
