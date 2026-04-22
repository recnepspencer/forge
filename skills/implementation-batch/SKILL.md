---
name: implementation-batch
description: Continue a Forge milestone by deriving the next unfinished implementation slice from the milestone document, planning it, building it, and verifying it in one turn.
---

# Implementation Batch

Start from the milestone document.

Find the next unfinished slice by comparing:
- milestone phase order
- acceptance evidence
- current code
- current tests

## Batch Size Standard

An implementation batch must be a meaningful milestone advance, not the
smallest technically defensible patch.

Default expectation:

- choose work that advances a complete proof-bearing capability, not an amount
  of elapsed time
- cover one coherent phase subsection or one complete proof-bearing capability
  across model, lowering/admission/execution, facade, evidence, and tests where
  those layers apply
- include at least one production behavior change and at least one certification,
  regression, or compile-fail proof update
- advance multiple milestone obligations together when they are structurally
  coupled

Structural minimum:

- touch at least three meaningful surfaces when the milestone has them, such as
  type/model, admission/lowering/execution, facade/API, counters/evidence,
  persistence/replay, diagnostics, certification, unit tests, and compile-fail
  tests
- close or materially advance at least two milestone obligations or one named
  acceptance/proof lane
- include hostile or denial coverage for the new behavior unless the work is
  purely internal refactoring with no new admitted behavior
- update the machine-checkable evidence surface when the milestone claims one
- leave the next agent with a stronger completed boundary, not merely a longer
  TODO list

Do not take micro-slices such as:

- adding only names, enums, counters, or placeholder structs with no behavior
- adding only facade exports for already-existing internals
- adding one happy-path test while leaving the denial, replay, counter, or
  boundary surface untouched
- documenting debt without implementing the next honest proof surface
- stopping after a patch that changes only one surface when adjacent coupled
  surfaces are still required for the milestone claim
- using speed of completion as evidence that the batch was acceptable; elapsed
  time is irrelevant

If the next obvious task is too small, expand it to include the nearest coupled
obligation. Examples:

- vocabulary plus constructors plus denial tests plus compile-fail privacy
- admission logic plus counters plus facade exposure plus hostile rejection
- execution path plus replay evidence plus certification row plus diagnostics
- persistence shape plus reopen/load validation plus corruption rejection

If you cannot form a substantial coherent batch because the milestone is nearly
closed, say so explicitly and perform a closeout-readiness pass instead of
pretending a tiny patch is an implementation batch.

Write the implementation plan first. Make it specific enough that the next QA loop can judge it literally.

The plan must include:
- slice name
- milestone obligations covered
- concrete artifacts to add or change
- facade/API changes
- tests and compile-fail fixtures
- verification commands
- explicit out-of-scope items
- why the slice is large enough to count as a real implementation batch
- what adjacent tiny tasks were intentionally bundled so the batch is not a
  cosmetic micro-slice

Then implement the plan immediately.

During implementation:
- follow existing local patterns
- preserve sealed proof construction
- expose milestone protocol through the facade when required
- add counters where the milestone names observable work
- add compile-fail tests where invalid phase transitions should be uncallable
- update tests that certify the changed behavior
- keep expanding within the chosen coherent scope if implementation reveals an
  unhandled denial, replay, counter, persistence, or facade boundary that is
  necessary for the claim to be honest

After implementation:
- run formatting
- run focused milestone-area tests
- run compile-fail tests when type boundaries changed
- run the package test suite when shared facade, identity, counter, replay, or protocol surfaces changed
- self-check that the result would not look embarrassing as a standalone
  implementation batch in the milestone history

Final response:
- built slice
- changed files or artifacts
- verification results
- remaining milestone work
- best next QA target
- confirmation that the batch was substantial, or an explicit explanation if it
  was a final-cleanup exception
