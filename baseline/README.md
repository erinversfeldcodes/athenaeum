# The baseline

What every repository must have, and the templates that write what is missing.

- [`items.toml`](./items.toml) is the list: each item, how it is checked, and for a
  file item, which templates satisfy it. How an item is written is defined by
  [the baseline contract](../contracts/baseline/README.md).
- [`templates/`](./templates) is the pack: the files a repository needs, with what is
  specific to it left as a variable.

## Templates

A template is a file with `{{name}}` where a value goes. The names are a closed list,
and a tool that applies a template fills in each from the repository:

| Variable | Where it comes from |
|---|---|
| `owner`, `repository` | the GitHub repository, as `OWNER/NAME` |
| `year` | the year it is applied |
| `licence` | the repository's [answers](../contracts/baseline-answers/README.md) |
| `holder` | the answers, and needed only by a template that uses it |
| `contact` | the answers: where a vulnerability is reported |

A template that uses a value the repository has not given is an error when it is
applied, and nothing is written.

Templates that depend on the licence are under `templates/licence/<name>/`, and an
item reaches them through `{{licence}}` in its template path. A repository's licence
chooses its `LICENSE`, its `CONTRIBUTING.md` and its issue forms, because the licence
decides whether pull requests are accepted. The only licence so far is
`source-available`, which permits reading and nothing else, and so closes the
repository to outside pull requests.

The pack was taken from the files Hephaestium already had, with what is specific to it
replaced by a variable. `scripts/check-baseline-templates.py` holds the pack to its
rules on every change: every template an item names exists, every template is used by
some item, and every variable a template uses is in the list above.

## What a template does not do

A generated README is a skeleton that says what to fill in. The baseline check cannot
tell a stub from a finished README, so finishing it is the repository's job.

`dependabot.yml` has no template. What goes in it depends on the ecosystems the
repository uses, so it is written by a tool that looks.

The security policy and the issue forms' security link assume GitHub's private
vulnerability reporting; a repository that uses another route gives it as its `contact`
and edits the link.
