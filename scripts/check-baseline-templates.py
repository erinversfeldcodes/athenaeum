#!/usr/bin/env python3
"""Check that the baseline's templates and the items that name them agree.

An item's `apply` names templates under `baseline/templates`. Three things must
hold, and nothing else checks them:

Every template an item names exists. A path may use `{{licence}}`, which stands
for each directory under `licence/`, so the check expands it once for each
licence and every licence must provide every file. No other variable may appear
in a path.

Every template is named by some item. A file nobody applies is a template that
will rot, and one a reader cannot tell is dead.

Every variable a template uses is in the closed list below. A template that
uses a name the tool does not know would be written out with `{{name}}` in it.
Two variables, `ecosystem` and `directory`, are filled in once for each entry a
generator writes, so only the generator's entry template may use them.

A generator names its templates here, because the pack defines them and the
tool that applies the generator reads them: `dependabot` uses
`dependabot/header.yml` and `dependabot/entry.yml`.

The list of variables is defined here and in the tool that fills them in, and
the pack's README, so a change to it is a change to the contract.
"""
import re
import sys
import tomllib
from pathlib import Path

VARIABLES = {"owner", "repository", "holder", "year", "contact", "licence"}
GENERATORS = {"dependabot": {"dependabot/header.yml", "dependabot/entry.yml"}}
ENTRY_VARIABLES = {"ecosystem", "directory"}
ENTRY_TEMPLATES = {"dependabot/entry.yml"}
ROOT = Path(__file__).resolve().parent.parent
TEMPLATES = ROOT / "baseline" / "templates"
ITEMS = ROOT / "baseline" / "items.toml"
VARIABLE = re.compile(r"\{\{([^{}]*)\}\}")


def main() -> int:
    problems: list[str] = []
    items = tomllib.loads(ITEMS.read_text())["items"]
    licences = sorted(p.name for p in (TEMPLATES / "licence").iterdir() if p.is_dir()) if (TEMPLATES / "licence").is_dir() else []

    named: set[Path] = set()
    for item_id, item in items.items():
        generate = item.get("apply", {}).get("generate")
        if generate:
            wanted = GENERATORS.get(generate["generator"])
            if wanted is None:
                problems.append(f"{item_id}: this check does not know the generator {generate['generator']}")
            for name in sorted(wanted or ()):
                path = TEMPLATES / name
                named.add(path)
                if not path.is_file():
                    problems.append(f"{item_id}: the generator {generate['generator']} needs the template {name}, which does not exist")
        for entry in item.get("apply", {}).get("files", []):
            template = entry["template"]
            names = set(VARIABLE.findall(template))
            unknown_in_path = names - {"licence"}
            if unknown_in_path:
                problems.append(f"{item_id}: the template path {template} uses {sorted(unknown_in_path)}; only {{{{licence}}}} may appear in a path")
                continue
            expansions = [template.replace("{{licence}}", licence) for licence in licences] if names else [template]
            if names and not licences:
                problems.append(f"{item_id}: {template} needs a licence directory, and there is none")
            for expanded in expansions:
                path = TEMPLATES / expanded
                named.add(path)
                if not path.is_file():
                    problems.append(f"{item_id}: the template {expanded} does not exist")

    for path in sorted(p for p in TEMPLATES.rglob("*") if p.is_file()):
        relative = path.relative_to(TEMPLATES)
        if path not in named:
            problems.append(f"{relative}: no item names it")
        text = path.read_text()
        allowed = VARIABLES | (ENTRY_VARIABLES if str(relative) in ENTRY_TEMPLATES else set())
        for name in VARIABLE.findall(text):
            if name not in allowed:
                problems.append(f"{relative}: uses the variable {{{{{name}}}}}, which is not one of {sorted(VARIABLES)}")
        stray = VARIABLE.sub("", text)
        if "{{" in stray or "}}" in stray:
            problems.append(f"{relative}: has a '{{{{' or '}}}}' that is not a whole variable")

    if problems:
        print("\n".join(problems), file=sys.stderr)
        return 1
    print(f"{len(items)} items, {len(named)} templates, {len(licences)} licence(s): every template exists, is used and uses only known variables")
    return 0


if __name__ == "__main__":
    sys.exit(main())
