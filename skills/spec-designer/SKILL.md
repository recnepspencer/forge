---
name: spec-designer
description: Design or revise Forge milestone specs and roadmap entries. Use when authoring a new milestone spec, refining an existing milestone plan, inserting a milestone into a crate roadmap, or turning a product goal into a Forge-quality engineering specification grounded in the coding-guideline documents and the target crate roadmap.
---

# Spec Designer

Use this skill when the task is specification design, not implementation.

## Mandatory reading order

Read these in this order before designing anything:

1. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
2. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\arch_laws.md`
3. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\perf_laws.md`
4. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_laws.md`

`MENTALITY.md` is first on purpose. It governs how the spec should be conceived, not just how it should be formatted.

## Required subsystem document set

You must identify the target crate or subsystem and read all of these before writing the spec:

- the subsystem vision document
- the subsystem roadmap document
- the subsystem test requirements document, if one exists
- adjacent milestone docs and closeouts, if they exist

Prefer files such as:
- `_docs/<crate>/<crate>_vision.md`
- `_docs/<crate>/<crate>_roadmap.md`
- `_docs/<crate>/test-requirements.md`
- `_docs/<crate>/dx_plan.md`
- `_docs/<crate>/milestone-*.md`

If the crate has vision, roadmap, and test requirements, read all three:
- the vision to understand the full product thesis
- the roadmap to understand sequencing and capability boundaries
- the test requirements to understand what proof bar closes the work

If you cannot identify the vision or roadmap for the target crate, stop and report that clearly rather than guessing.

## Required document summaries

Before drafting the spec, write a short working summary for each document you read.

The summary should answer:
- what is the single most important thing this document is trying to protect?
- what constraint or expectation from this document most strongly shapes the spec?

At minimum, this summary set must include one short summary each for:
- `MENTALITY.md`
- `arch_laws.md`
- `perf_laws.md`
- `domain_laws.md`
- the subsystem vision doc
- the subsystem roadmap doc
- the subsystem test requirements doc, if one exists

These summaries are not optional. They are part of the design process and help
ensure the later spec is actually grounded in the governing sources rather than
only loosely inspired by them.

## Core design stance

The spec must reflect Forge standards:
- adversarial constraint first
- hard problem first
- enforcement over convention
- architecture and spec aligned structurally
- authority separate from derivation
- production-grade proof, not MVP-grade plausibility

Do not produce milestone specs that are:
- feature checklists without structural design
- implementation todo lists without governing invariants
- vague narrative without concrete types, phases, boundaries, and proof obligations
- roadmap filler that does not earn its place in the crate's sequencing

## What a good milestone spec must do

A milestone spec must:
- define a real capability boundary
- explain why that capability matters in the crate roadmap
- identify the adversarial constraint the milestone must survive
- break the work into explicit ordered phases
- define what must ship
- define what must be preserved
- define acceptance evidence
- state the architectural shape clearly enough that code can map to it honestly
- distinguish what is authoritative, what is derived, and what the bridge/framework/mechanism is allowed to own

## Phase requirement

Milestone specs must be phase-structured.

Use as many numbered `Phase N` sections as the real milestone requires.

Phase rules:
- start at `Phase 1`
- continue linearly as `Phase 2`, `Phase 3`, `Phase 4`, and so on
- use the minimum number of phases that still makes the implementation sequence honest
- add more phases whenever collapsing steps would hide a real structural dependency, proof boundary, or implementation gate

Phases must:
- be ordered intentionally
- be followed in order
- each solve a real structural step
- each leave the system in a coherent state for the next phase

Do not write specs as unordered work buckets.
Do not treat phases as interchangeable.
Do not compress a milestone into too few phases just to make the document look tidy.
Do not create phase splits that are cosmetic rather than architectural.
Do not use alternate numbering schemes such as `M1.1`, `M1.2`, or other nested milestone-local identifiers.

## Required workflow

1. Read the mandatory coding-guideline files in the required order.
2. Identify the target crate/subsystem.
3. Read the subsystem vision, roadmap, and test requirements docs.
4. Read adjacent milestone docs and closeouts if they exist.
5. Write the short per-document summaries before drafting.
6. State the adversarial constraint before drafting the milestone.
7. Design the milestone so it solves the hard structural problem first.
8. Write or revise the milestone spec.
9. Check the resulting spec against the governing docs before considering it done.

## Patch-writing rule

When creating or revising a spec file, write it in multiple smaller patches
rather than one giant write.

Reason:
- large single writes are more likely to be rejected by Windows tooling or app
  limits
- smaller patches make it easier to keep structure aligned while revising

Expected behavior:
- create the file skeleton first
- fill major sections in follow-up patches
- add later sections, refinements, and corrections in additional patches
- prefer several safe writes over one huge write

Do not try to dump the full spec into one oversized patch when the document is
large.

## Required self-check before finalizing a spec

Ask these questions explicitly:
- Does the milestone solve a real structural problem or just package work cosmetically?
- Is the adversarial constraint precise and load-bearing?
- Does the milestone preserve crate authority boundaries?
- Does the milestone define proof obligations, not just implementation tasks?
- Could a competent engineer map this spec into honest types, modules, and tests?
- Does the milestone belong in this roadmap sequence, or is it out of order?

If any answer is no, revise the spec before presenting it.

## Preferred milestone structure

Use the local crate style when one already exists. Otherwise, prefer this structure:

```text
# Milestone N: <title>

## Goal

## Why This Milestone Exists

## Adversarial Constraint

## Phases

### Phase 1: <title>

### Phase 2: <title>

### Phase 3: <title>

### Phase N: <title>

## Must Ship

## Must Preserve

## Acceptance Evidence

## Architectural Notes

## Sequencing Notes
```

If the crate's existing docs use a different but clearly stronger structure, follow that local structure instead of forcing this one.
Preserve `Phase 1`, `Phase 2`, `Phase 3`, etc. as the phase naming standard, but continue to as many phases as the milestone honestly needs.

## Roadmap insertion rule

When adding a new milestone:
- place it in the roadmap sequence intentionally
- explain why it belongs there
- update the roadmap language so the milestone is not an orphan

Do not just create a standalone milestone doc without considering roadmap sequencing.

## Output rule

When reporting the design work back, include:
- what vision, roadmap, test-requirements, and guideline docs were read
- the short summary takeaway from each governing doc
- the adversarial constraint chosen
- why the milestone belongs where it does
- what doc(s) were created or updated
- any unresolved uncertainty that still needs user judgment

Keep the report concise, but the spec itself should be rigorous.
