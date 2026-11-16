# Lock

How a managed repository pins the Athenaeum revision it uses, and says which
skills it takes from it. It lives at `.athenaeum/lock.toml`, beside the manifest.

- [`schema.json`](./schema.json) is the definition. Every field and rule is stated
  there and nowhere else.
- [`example.toml`](./example.toml) is a worked example.
- [`cases/`](./cases) holds locks that must be accepted and locks that must be
  rejected. See [how a contract is held to account](../README.md).

## What makes it a pin

A pin has to be something that cannot drift without being noticed.

- `rev` is a release tag, never a branch, because a branch moves.
- `commit` is the full commit the tag resolved to when the lock was written, forty
  lower-case hexadecimal digits. A tag that is later moved to a different commit no
  longer matches the lock, and a reader refuses it. An abbreviation is ambiguous, so
  it is not accepted.

`skills` names what is taken, and `harnesses` names the agent runtimes the skills
are linked for. Everything else about a skill is in Athenaeum, at the pinned
revision.
