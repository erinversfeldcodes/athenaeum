# Baseline

What every repository must have, as a list of items a tool can check. The list is
[`baseline/items.toml`](../../baseline/items.toml); this directory defines how an
item is written.

- [`schema.json`](./schema.json) is the definition. Every field and rule is stated
  there and nowhere else.
- [`example.toml`](./example.toml) is a worked example.
- [`cases/`](./cases) holds baselines that must be accepted and baselines that must be
  rejected. See [how a contract is held to account](../README.md).

## What an item is

An item has a title, a `kind`, a `severity` and a `check`. It is keyed by its id, so
an id cannot be written twice: TOML rejects a repeated key, which a list of items
could not express in a schema.

| Kind | Satisfied when |
|---|---|
| `file` | one of the listed paths, globs allowed, exists |
| `workflow` | a named rule holds for every workflow under `.github/workflows` |
| `setting` | a named repository setting on GitHub is on |
| `ruleset` | a named rule holds for an active ruleset that targets the default branch |

A `ruleset` item is satisfied by a ruleset and not by classic branch protection. Two of
its rules compare the ruleset with the repository's own
[answers](../baseline-answers/README.md): which checks it requires, and who may bypass.

A `required` item that a repository lacks fails the check. A `recommended` item is
reported and never fails it. An item can say it applies only to public or only to
private repositories, and can cite the documentation it comes from.

## What this version does not cover

What a configuration file must contain, such as the ecosystems a `dependabot.yml` covers,
is not here yet. It is added as a new rule, which is a minor version: a reader refuses a
kind or rule it does not know, so it cannot mistake a check it cannot make for one that
passed.

A repository states its own answers, such as which licence it uses and any exceptions
with their reasons, in [a file of its own](../baseline-answers/README.md).
