# Baseline answers

A repository's own answers to the [baseline](../baseline/README.md): what the baseline
cannot decide for it, and any item it does not satisfy, with the reason. It lives at
`.athenaeum/baseline.toml`, beside the manifest and the lock.

- [`schema.json`](./schema.json) is the definition. Every field and rule is stated
  there and nowhere else.
- [`example.toml`](./example.toml) is a worked example.
- [`cases/`](./cases) holds answers that must be accepted and answers that must be
  rejected. See [how a contract is held to account](../README.md).

## What it answers

The licence the repository uses, where a vulnerability is reported, which checks the
default branch requires, and whether the administrator may bypass the default branch's
rules, and if so how. These are the things a baseline can ask for but only the
repository can answer.

## Exceptions

An exception names a baseline item by its id and gives the reason the repository does
not satisfy it. A report shows it as excepted, with the reason, so a gap that was
chosen is visible and a gap that was overlooked is not hidden behind it.

An exception is keyed by id, so the same item cannot be excepted twice. That an id
names a real item cannot be said in a schema, because the items are in another file:
the tool that reads both reports an exception for an item that does not exist as an
error, so renaming an item cannot silently drop an exception.
