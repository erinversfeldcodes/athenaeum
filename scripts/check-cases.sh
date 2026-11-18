#!/usr/bin/env bash
# Grade a contract's schema against its conformance cases.
#
#   check-cases.sh manifest
#   check-cases.sh gate-result
#   check-cases.sh lock
#   check-cases.sh baseline
#   check-cases.sh baseline-answers
#   check-cases.sh review
#
# The example and every file under cases/valid must be accepted. Every file
# under cases/invalid must be rejected, for exactly the one reason it states:
#
#       <path> <keyword>
#
# where <path> is where the rule is broken, written $.a.b[0], and <keyword> is
# the JSON Schema keyword that rejects it. A manifest is TOML and states its
# reason on its first line, after "# violates:". A gate result is JSON, which
# has no comments, so it states its reason in a file beside it, with the same
# name and the extension .violates.
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

contract=${1:-}
case $contract in
  manifest)
    schema=manifest/schema.json
    example=manifest/example.toml
    cases=manifest/cases
    extension=toml
    ;;
  lock)
    schema=contracts/lock/schema.json
    example=contracts/lock/example.toml
    cases=contracts/lock/cases
    extension=toml
    ;;
  baseline)
    schema=contracts/baseline/schema.json
    example=contracts/baseline/example.toml
    cases=contracts/baseline/cases
    extension=toml
    # The real list is held to the schema it is written against.
    also=baseline/items.toml
    ;;
  baseline-answers)
    schema=contracts/baseline-answers/schema.json
    example=contracts/baseline-answers/example.toml
    cases=contracts/baseline-answers/cases
    extension=toml
    ;;
  gate-result)
    schema=contracts/gate-result/schema.json
    example=contracts/gate-result/example.json
    cases=contracts/gate-result/cases
    extension=json
    ;;
  review)
    schema=contracts/review/schema.json
    example=contracts/review/example.json
    cases=contracts/review/cases
    extension=json
    ;;
  *)
    echo "usage: check-cases.sh manifest|gate-result|lock|baseline|baseline-answers|review" >&2
    exit 2
    ;;
esac

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
    minItems | minLength) echo 'should be non-empty|is too short' ;;
    uniqueItems) echo 'has non-unique elements' ;;
    minProperties) echo 'should be non-empty|does not have enough properties' ;;
    not) echo 'should not be valid under' ;;
    anyOf) echo 'is not valid under any of the given schemas' ;;
    contains) echo 'does not contain items matching the given schema' ;;
    *) return 1 ;;
  esac
}

# The statement a case makes about itself: "<path> <keyword>". Prints it, or
# says why there is none.
statement_of() {
  local file=$1 first
  if [ "$extension" = toml ]; then
    first=$(head -n 1 "$file")
    if [ "${first#\# violates: }" = "$first" ]; then
      echo "the first line must be '# violates: <path> <keyword>', but is '$first'" >&2
      return 1
    fi
    echo "${first#\# violates: }"
  else
    local sidecar=${file%."$extension"}.violates
    if [ ! -f "$sidecar" ]; then
      echo "has no ${sidecar##*/} stating the rule it breaks" >&2
      return 1
    fi
    head -n 1 "$sidecar"
  fi
}

if [ ! -f "$schema" ]; then
  echo "$schema does not exist, so no case can be graded" >&2
  exit 1
fi

status=0

valid=("$example" "$cases"/valid/*."$extension" ${also:+"$also"})
if ! output=$("${VALIDATOR[@]}" --schemafile "$schema" "${valid[@]}" 2>&1); then
  echo "an instance that must be accepted was rejected:" >&2
  echo "$output" >&2
  status=1
fi

invalid=("$cases"/invalid/*."$extension")
report=$("${VALIDATOR[@]}" --schemafile "$schema" --output-format json "${invalid[@]}" 2>/dev/null || true)

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

for orphan in "$cases"/invalid/*.violates; do
  [ -e "$orphan" ] || continue
  if [ ! -f "${orphan%.violates}.$extension" ]; then
    echo "$orphan: states a rule for a case that does not exist" >&2
    status=1
  fi
done

for file in "${invalid[@]}"; do
  if ! statement=$(statement_of "$file" 2>&1); then
    echo "$file: $statement" >&2
    status=1
    continue
  fi
  read -r path keyword rest <<<"$statement"
  if [ -z "${keyword:-}" ] || [ -n "${rest:-}" ]; then
    echo "$file: the stated reason must be '<path> <keyword>', but is '$statement'" >&2
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

echo "$contract: ${#valid[@]} accepted, ${#invalid[@]} rejected for the reason each states"
