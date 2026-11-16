#!/usr/bin/env bash
# Grade the manifest schema against its conformance cases.
#
#   manifest/example.toml and manifest/cases/valid/*.toml must be accepted.
#   manifest/cases/invalid/*.toml must each be rejected, for exactly the one
#   reason the case states on its first line:
#
#       # violates: <path> <keyword>
#
#   where <path> is where in the manifest the rule is broken and <keyword> is
#   the JSON Schema keyword that rejects it.
#
# "Rejected" is not enough. A case that is rejected for some other reason
# proves nothing about the rule it was written for, and would keep passing if
# that rule were deleted from the schema. So a case must produce one error,
# at the stated path, of the stated kind.
#
# The cases are the contract; this script is one reader of them. It uses
# check-jsonschema as the reference validator, which reports a path and a
# message but not the keyword, so the table below says how this validator
# words each keyword. Another validator graded against the same cases brings
# its own table and leaves the cases alone.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

SCHEMA=manifest/schema.json
read -r -a VALIDATOR <<<"${CHECK_JSONSCHEMA:-check-jsonschema}"

wording_of() {
  case $1 in
    required) echo 'is a required property' ;;
    additionalProperties) echo 'Additional properties are not allowed' ;;
    type) echo 'is not of type' ;;
    const) echo 'was expected' ;;
    enum) echo 'is not one of' ;;
    pattern | propertyNames) echo 'does not match' ;;
    minimum) echo 'is less than the minimum' ;;
    minItems) echo 'should be non-empty|is too short' ;;
    minProperties) echo 'should be non-empty|does not have enough properties' ;;
    *) return 1 ;;
  esac
}

if [ ! -f "$SCHEMA" ]; then
  echo "$SCHEMA does not exist, so no case can be graded" >&2
  exit 1
fi

status=0

valid=(manifest/example.toml manifest/cases/valid/*.toml)
if ! output=$("${VALIDATOR[@]}" --schemafile "$SCHEMA" "${valid[@]}" 2>&1); then
  echo "a manifest that must be accepted was rejected:" >&2
  echo "$output" >&2
  status=1
fi

invalid=(manifest/cases/invalid/*.toml)
report=$("${VALIDATOR[@]}" --schemafile "$SCHEMA" --output-format json "${invalid[@]}" 2>/dev/null || true)

if ! jq -e . >/dev/null 2>&1 <<<"$report"; then
  echo "the validator did not return a report for the invalid cases:" >&2
  echo "$report" >&2
  exit 1
fi

if [ "$(jq '.parse_errors | length' <<<"$report")" -ne 0 ]; then
  echo "a case could not be parsed, so it was not graded:" >&2
  jq -r '.parse_errors[] | "  \(.filename): \(.message)"' <<<"$report" >&2
  status=1
fi

for file in "${invalid[@]}"; do
  read -r hash label path keyword rest < "$file" || true
  if [ "$hash $label" != "# violates:" ] || [ -z "${keyword:-}" ] || [ -n "${rest:-}" ]; then
    echo "$file: the first line must be '# violates: <path> <keyword>'" >&2
    status=1
    continue
  fi
  if ! wording=$(wording_of "$keyword"); then
    echo "$file: no wording is recorded for the keyword '$keyword'" >&2
    status=1
    continue
  fi

  errors=$(jq -c --arg f "$file" '[.errors[] | select(.filename == $f)]' <<<"$report")
  count=$(jq 'length' <<<"$errors")

  if [ "$count" -eq 0 ]; then
    echo "$file: was accepted, but must be rejected at $path by $keyword" >&2
    status=1
  elif [ "$count" -ne 1 ]; then
    echo "$file: must break one rule, but broke $count:" >&2
    jq -r '.[] | "  \(.path): \(.message)"' <<<"$errors" >&2
    status=1
  elif [ "$(jq -r '.[0].path' <<<"$errors")" != "$path" ] ||
    ! jq -r '.[0].message' <<<"$errors" | grep -qE "$wording"; then
    echo "$file: rejected for the wrong reason" >&2
    echo "  stated:   $path $keyword" >&2
    jq -r '.[0] | "  reported: \(.path): \(.message)"' <<<"$errors" >&2
    status=1
  fi
done

if [ "$status" -ne 0 ]; then
  exit 1
fi

echo "${#valid[@]} manifests accepted, ${#invalid[@]} rejected for the reason each states"
