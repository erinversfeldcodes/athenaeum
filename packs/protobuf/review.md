---
name: protobuf
description: The review lens for Protocol Buffers schemas. Checks that a schema evolves without breaking anything already deployed (field numbers kept, removed fields reserved, no type changes), that a suppression of the breaking-change gate is justified and still needed, that names, enums and field types follow the rules that keep the wire stable, that every message and field is documented, and that wire schemas have round-trip tests. Applies to a change that touches a `.proto` file or the configuration of the tool that lints it.
blocking: true
---

# protobuf lens

You are the protobuf lens of [a review](../../skills/review/SKILL.md). That skill
says how to review; this says what to look for. Review the diff against these
axes, name the axis on each finding, and cite the file and line.

The schema linter, the breaking-change check and the generated-code drift check
are gates (`manifest:gates`). Take their results as given, with one exception
that is the second axis below: a gate that passes because of a suppression has
not told you the change is safe.

Do not review a generated file for its contents. A hand edit to one
(`manifest:components.*.generated`) is a finding in itself: the edit belongs in the
schema it is generated from.

## Axes

### 1. Evolution (judgement, backed by the breaking-change gate)

A schema is a promise to every program that has already been built against it.

- A field number is permanent. It is never reused, not even after the field is
  removed. Taking a number that a removed field held makes old data read as the
  new field.
- A removed field has its number, and its name, listed as reserved, with a
  comment that says why it went.
- A field's type is never changed. Add a new field and migrate readers to it.
- Changes are additive: new fields, new messages, new enum values at the end.
- If the schema is also carried as JSON, the names are the wire format, not only
  the numbers. Renaming a field or an enum value breaks JSON consumers while the
  binary stays compatible, and the binary check will not say so. Treat a rename as
  a breaking change.

### 2. Suppressions (judgement, settled by removing them)

Every entry that silences the lint or the breaking-change check is a promise
that a known, bounded exception is being tolerated until some work removes it.

- It says what it covers, why, how wide the silence is (a suppression scoped to a
  whole file also hides every other violation in that file), and the work that
  removes it. One with none of these is a finding.
- It is still needed. Remove it and run the gate. If the gate still passes, the
  entry hides nothing and should go: left in place it will silently cover the next
  real violation. See [mutation-probe](../../skills/mutation-probe/SKILL.md).
- A suppression **added in this change** is read as a claim that the change breaks
  something. A change whose breaking-change gate passes only because the change
  also suppressed that gate is a breaking change.

### 3. Names and layout (mechanical, judgement where the linter is silent)

- The package is versioned. Messages are in PascalCase, fields in snake_case,
  enum values in UPPER_SNAKE_CASE prefixed with the enum's name.
- An enum's zero value is `<NAME>_UNSPECIFIED` and means nothing. A meaningful
  value at zero is read as the default for every message that omits it.
- One family of messages per file, not a single file that grows forever.

### 4. Field design (judgement)

- A point in time is a timestamp type, and a span a duration type, not an integer
  of seconds or a string.
- Money is an integer in the smallest unit with its currency stated, never a
  float. A quantity that must be exact is never a float.
- A list is a repeated field, not a delimited string. Mutually exclusive variants
  are a `oneof`, not several optional fields and a convention.
- Field presence (`optional`) is used only where the reader must tell "not set"
  from the zero value.
- A field that will often carry a large payload holds a reference to it, not the
  payload.

### 5. Documentation (judgement)

- Every message says what it is for and when it is sent. Every field says what it
  means, its units and its constraints. A schema that other teams or other
  organisations read is their documentation, and gaps in it are findings.

### 6. Tests (judgement)

- A schema that travels over the wire has a round-trip test: serialise a message,
  read it back, and the values survive.
- The edges are covered: an optional field unset, a repeated field empty, each
  variant of a `oneof`, and the zero value of each enum.
- Where the same schema is generated for several languages, a consumer in each
  language reads a message the others wrote.

### 7. Validation and trust (judgement)

- A schema fixes a shape. It does not enforce a range, a format or a business
  rule, and an unknown value from a newer sender must not crash an older reader.
  The application validates what the schema cannot, at the boundary.
- A schema that crosses a trust boundary carries no secrets or credentials, and
  no field the receiving side must not see.
- Where several versions of a message are live at once, there is a stated way to
  read the old ones.

## Severity for this lens

- **P0**: a field number reused, or a type changed, or a removed field not
  reserved; a change that breaks the wire and passes only because it also
  suppressed the check; a secret in a schema; a required item the ticket names
  that is missing.
- **P1**: a meaningful value at an enum's zero; money or an exact quantity as a
  float; a renamed field on a JSON wire; a hand edit to a generated file; a wire
  schema with no round-trip test; a suppression with no reason or no way out.
- **P2**: a timestamp as an integer; a delimited string for a list; a missing
  comment; a suppression that is dead; a large payload inline.
- **P3**: naming that a reader would trip over.

A P2 finding does not block unless the repository's manifest says this lens's P2
findings block.
