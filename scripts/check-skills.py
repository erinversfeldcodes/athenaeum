#!/usr/bin/env python3
"""Check that every skill is well formed and cites only what exists.

A skill is `skills/<name>/SKILL.md`. Two things are checked here.

Its front matter names it and describes it. The name must match the
directory, because the directory is how a skill is found and the name is how
it is cited; two spellings is one too many.

Every manifest key it cites exists in the manifest schema. A skill refers to
a document or a command through the manifest, written `manifest:docs.goals`
or `manifest:gates.*.run`, and never as a path. A key that the schema does
not have is a skill telling an agent to look up something no repository can
declare, which fails silently in the field: the agent finds nothing and
improvises.

Links from one skill to another are ordinary relative links, so the link
check covers those.
"""

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SCHEMA = json.loads((ROOT / "manifest" / "schema.json").read_text())

CITATION = re.compile(r"manifest:([A-Za-z0-9_.*-]+)")
FRONT_MATTER = re.compile(r"\A---\n(.*?)\n---\n", re.DOTALL)


def resolve(node):
    """Follow a local `$ref` to the definition it names."""
    while "$ref" in node:
        target = SCHEMA
        for part in node["$ref"].removeprefix("#/").split("/"):
            target = target[part]
        node = target
    return node


def key_exists(key):
    """Whether a dotted key, with `*` for any name, is declared by the schema."""
    node = resolve(SCHEMA)
    for part in key.split("."):
        node = resolve(node)
        if part == "*":
            extra = node.get("additionalProperties")
            if not isinstance(extra, dict):
                return False
            node = extra
        elif part in node.get("properties", {}):
            node = node["properties"][part]
        else:
            return False
    return True


def problems_in(skill_file):
    directory = skill_file.parent.name
    text = skill_file.read_text()
    where = skill_file.relative_to(ROOT)

    front = FRONT_MATTER.match(text)
    if not front:
        yield f"{where}: must begin with front matter between two '---' lines"
        return

    fields = dict(
        (key.strip(), value.strip())
        for key, _, value in (line.partition(":") for line in front.group(1).splitlines())
        if value
    )
    if fields.get("name") != directory:
        yield f"{where}: its name is '{fields.get('name')}', but its directory is '{directory}'"
    if not fields.get("description"):
        yield f"{where}: has no description, so nothing can tell when to use it"

    for key in sorted(set(CITATION.findall(text))):
        if not key_exists(key):
            yield f"{where}: cites manifest:{key}, which the manifest schema does not declare"


def main():
    skills = sorted((ROOT / "skills").glob("*/SKILL.md"))
    problems = [problem for skill in skills for problem in problems_in(skill)]

    for directory in sorted(p for p in (ROOT / "skills").iterdir() if p.is_dir()):
        if not (directory / "SKILL.md").is_file():
            problems.append(f"skills/{directory.name}: has no SKILL.md")

    for problem in problems:
        print(problem, file=sys.stderr)
    if problems:
        return 1

    print(f"{len(skills)} skills, each named for its directory and citing only declared manifest keys")
    return 0


if __name__ == "__main__":
    sys.exit(main())
