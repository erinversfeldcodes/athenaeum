---
name: contract
description: The review lens for the shape of data that crosses a boundary: an API response, an event payload, a message between services, a webhook. Catches the bugs that happen when two components agree on a name and disagree on its shape: inconsistent envelopes and errors, a consumer that reads a field the producer does not send, an event that lacks what its subscribers need, a change that breaks consumers already deployed. Applies to a change that touches an endpoint, an event, a message schema or a client of one, whatever the stack.
blocking: true
---

# contract lens

You are the contract lens of [a review](../../skills/review/SKILL.md). That skill
says how to review; this says what to look for. Review the diff against these
axes, name the axis on each finding, and cite the file and line.

Where the stack lens judges the language, you judge the agreement between two
programs. For every boundary the change touches, read **both sides** and compare
them: the producer and the consumer, the emitter and each subscriber, the caller
and the callee. A finding that reads one side only is a guess.

## Axes

### 1. Consistency across the change (judgement)

Endpoints and messages that one change adds or alters follow one set of
conventions. If they differ, that is a finding, because a consumer will have to
special-case each.

- The envelope: success responses are wrapped the same way, or none are.
- The error shape: every error has the same structure and the same fields.
- Pagination: every list endpoint paginates the same way, or none do.
- Key naming: one case convention, with no mixing.
- Absence: an optional value is a `null` or an absent key, but one of them, always.

### 2. Consumer alignment (judgement)

For every producer and consumer pair the change touches, compare what is sent with
what is read: the field names, the types, the nesting, which fields are optional,
and what happens on `null`. Name the mismatch and say which side should move.

Common failures:

- A producer sends one spelling, the consumer reads another.
- A producer sends `null` for an optional value and a consumer that treats the
  field as required crashes.
- A producer nests a collection under its parent and a consumer expects it flat,
  or the reverse.
- A producer adds a field. That is safe for a consumer that ignores what it does
  not know and a crash for one that is strict, so check which this consumer is.

### 3. Event payload completeness (judgement)

For every event or message the change emits:

- Find each subscriber and list the fields it reads. A field a subscriber reads
  that the emitter does not send is a failure waiting for the first event.
- Where events form a chain, a correlation identifier and a causation identifier
  tie them together, so that what happened can be followed afterwards.
- The payload conforms to the schema, if there is one.

### 4. The declared shape against the real one (judgement)

Where a schema describes the data (a protobuf message, a JSON schema, an API
description), the data that the producer actually emits has that shape: the same
names (on a JSON wire, the names are the contract), the same enum spellings,
arrays where the schema says repeated, an absent key where it says optional.

### 5. Service boundaries (judgement)

For each call between services, read the request the caller builds and the request
the callee accepts, and the response the callee builds and the response the caller
parses. Check:

- the authentication that one side sends is the one the other verifies, in the same
  format;
- for an incoming webhook, the signature is verified before the body is trusted, and
  a repeat of the same delivery has no second effect;
- a field's units are the same on both sides.

### 6. Breaking changes (judgement)

For any change to an endpoint, an event or a message that already exists:

- Would it break a consumer that is already deployed, or a subscriber that is
  already running?
- Would it need the two sides to be deployed together? If so, the change says how,
  and which side goes first.
- If it would break something, the finding recommends a strategy: a versioned
  endpoint, an additive field with the old one kept, a deprecation period.

### 7. The right entity (judgement)

In a parent and child relationship, data belongs to one of them. A field attached
to the wrong entity (a per-item value on the order, a per-order value on each item)
is correct for the first case tested and wrong for the second. Check that the data
the change adds is on the entity it describes, and that the consumer's model of the
nesting matches the producer's.

## Severity for this lens

- **P0**: a change that breaks a deployed consumer or a running subscriber with no
  migration path; a subscriber that reads a field the emitter never sends; a
  required item the ticket names that is missing.
- **P1**: error shapes or envelopes that differ within one change; a consumer that
  crashes on a value the producer is allowed to send; a webhook accepted before its
  signature is verified, or one with a second effect on a repeat; a field on the
  wrong entity.
- **P2**: pagination or key naming that differs from the rest of the API; a missing
  correlation identifier on a chain of events; a payload that differs from its
  declared schema in a way no consumer reads yet.
- **P3**: naming a reader would trip over.

A P2 finding does not block unless the repository's manifest says this lens's P2
findings block.
