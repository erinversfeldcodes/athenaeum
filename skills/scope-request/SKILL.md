---
name: scope-request
description: Turn a vague request into a scoped, buildable brief. Investigate the code and the documents first, right-size the work, and ask only the questions whose answers change the outcome. Use before writing a ticket or starting work that is not trivial, or when a request such as "add X" or "fix the Y flow" does not say what is in scope, where it stops, or how success will be judged.
---

# scope-request

Good scoping is investigation first and questions second. Most questions a
request seems to raise are answered by the repository, and a person's
attention should go only to the real forks. The output is a short brief.

The conventions in [the skills README](../README.md) apply.

## Steps

1. **Investigate the code.** Find the real surfaces the request touches.
   Establish which components are involved (`manifest:components`), what
   already exists and what would be new, and how far a change would reach. For
   a sweep across many files, hand it to a subagent if you have one and keep
   only its conclusion. Check a claim against the tree before you rely on it:
   that a file is said to exist is not proof that it does.

2. **Investigate the documents.** Read what the repository has written down
   about intent and constraints, wherever its manifest says each lives:
   - the architecture (`manifest:docs.architecture`)
   - the stories this would serve (`manifest:docs.stories`) and how they map
     to code (`manifest:docs.mapping`)
   - the decisions already made in this area (`manifest:docs.decisions`)
   - what the product is for and what it has ruled out (`manifest:docs.goals`)

   A decision that is recorded is a constraint, not a suggestion. A request
   that contradicts one is a finding to raise, not a detail to work around.

3. **Right-size it.** One piece of work is one concern. Propose a split when
   the work would combine concerns that could ship separately, reach into
   several components that could be changed one at a time, or be too large
   for one person to review with care in one sitting. As a rule of thumb,
   that is more than about three hundred lines of production code. When it is
   too big, propose the smallest coherent first slice and list the rest as
   work to follow.

   Then ask one more thing, whatever the size: does any part of this have to be
   merged, deployed or released before another part can be built, checked or
   reviewed by what runs today? The checks, the reviewers and the installed
   tools are the ones already running, not the ones the change would produce.
   For example: a new field in a schema or contract and its first use; a new
   version of the tool that runs the checks and a check that needs it; a data
   migration and the code that depends on it; a protocol change between two
   parts that are released separately. If so, split there however small it is,
   make the order a blocked-by link, and have the brief name what must be
   rebuilt, deployed or released between the two. A change that holds both
   halves fails before anyone can read it, and neither a retry nor a review gets
   it through. A change that is only large, or whose parts all work with what
   runs today, is not split for this reason: adding an optional field that
   nothing reads yet, with the code that will read it in the same release, is
   one piece when no running check has to understand the field first.

   A split is linked
   in the tracker and not only mentioned in the text: the original is the
   parent, each piece is created as its sub-issue, and an order between pieces
   is a blocked-by link. A ticket with children is an epic. Say in the brief
   which piece is which, so whoever creates them can make the links.

4. **Ask only what changes the outcome.** After investigating you will be
   left with a few real forks. Good candidates:
   - success criteria that could be read two ways
   - which story the work serves, when that is unclear or there is none
   - whether it is reachable by a user now, or built now and connected later
   - which edge cases are in scope and which are deferred
   - how each behaviour will be proven, especially where the obvious kind of
     test is the wrong tool

   Recommend a default with each. Do not ask what the repository already
   answers.

5. **Write the brief.** Short, and in this shape:
   - **Goal**
   - **Affected surfaces**, with file and line where known
   - **Story**, or "none" with why the work can still be validated
   - **In scope** and **Out of scope**
   - **Dependencies**
   - **How each behaviour will be proven**
   - **Decisions resolved**, and any still open

## Rules

- **Investigate before asking.** A question the tree already answers wastes a
  person's turn.
- **Never offer a bare menu.** Every question carries the answer you would
  choose and why.
- **Surface, do not bury.** If investigation contradicts the premise of the
  request, because the thing does not exist or already works, say so before
  scoping any further.
- **Scope lock.** A second concern found while scoping becomes its own piece
  of work to follow, not a larger brief.
