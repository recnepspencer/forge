---
name: spec-designer
description: Design or revise Forge milestone specs and roadmap entries. Use when authoring a new milestone spec, refining an existing milestone plan, inserting a milestone into a crate roadmap, or turning a product goal into a Forge-quality engineering specification grounded in the coding-guideline documents and the target roadmap.
---

# Spec Designer

Use this skill when the task is specification design, not implementation.

## Mandatory reading order

Read these in this order before designing anything:

1. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\MENTALITY.md`
2. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\arch_laws.md`
3. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\composition_laws.md` if it is populated
4. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\domain_structure_laws.md`
5. `C:\Users\Esther\Documents\Programming\forge_workspace\forge\_docs\coding_guidelines\perf_laws.md`

After the coding guidelines, read the target subsystem roadmap.

`MENTALITY.md` is first on purpose. It governs how the spec should be conceived, not just how it should be formatted.

## Default reading boundary

By default, read only:

- the coding-guideline files above
- the target roadmap

Do not read, cite, summarize, or rely on any other docs unless the user explicitly tells you to read them.

That means:

- do not automatically read vision docs
- do not automatically read test-requirements docs
- do not automatically read adjacent milestone docs
- do not automatically read closeouts
- do not automatically read design notes, audits, plans, or other subsystem docs

If the user points you at additional docs, then read those docs too. Otherwise, stay within the default reading boundary.

If you cannot identify the roadmap for the target crate or subsystem, stop and report that clearly rather than guessing.

## Required document summaries

Before drafting the spec, write a short working summary for each document you read.

Each summary should answer:

- what is the single most important thing this document is trying to protect?
- what constraint or expectation from this document most strongly shapes the spec?

At minimum, this summary set must include one short summary each for:

- `MENTALITY.md`
- `arch_laws.md`
- `composition_laws.md`, if it is populated
- `domain_structure_laws.md`
- `perf_laws.md`
- the subsystem roadmap doc

These summaries are not optional. They are part of the design process and help ensure the spec is grounded in governing sources rather than loosely inspired by them.

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
- vague narrative without concrete boundaries, phases, proof, and denials
- roadmap filler that does not earn its place in the roadmap sequence

## Structural reference pattern

Use the following pattern as a style reference:

- concise framing sections at the top
- a phase-dominant main body
- one conceptual detail or boundary per phase
- rich per-phase content rather than thin phase stubs
- summary sections at the end that compress, not replace, the phase content

This is a structural pattern, not a document dependency. Do not cite or require any specific non-roadmap spec unless the user explicitly asks for it.

## What a good milestone spec must do

A milestone spec must:

- define a real capability boundary
- explain why that capability matters in the roadmap
- identify the adversarial constraint the milestone must survive
- break the work into explicit ordered phases
- define what must ship
- define what must be preserved
- define acceptance evidence
- state the architectural shape clearly enough that code can map to it honestly
- distinguish what is authoritative, what is derived, and what the mechanism is allowed to own

## Phase dominance rule

The phases should hold most of the information in the document.

This is mandatory.

The top-level sections exist to frame the work, but the detailed design must primarily live inside the phase plan.

That means:

- keep `Goal`, `Why This Milestone Exists`, `Governing Summaries`, `Adversarial Constraint`, and `Product Decision Lock` concise
- do not bury the real design in giant top-level `Must Ship`, `Architectural Notes`, or `Sequencing Notes` sections
- put the concrete structural content into the phases
- if the phases feel like a minor section of the document, the spec is probably wrong
- the phases should usually be the majority of the document

## Phase requirement

Milestone specs must be phase-structured.

Use as many numbered `Phase N` sections as the milestone honestly requires.

Phase rules:

- start at `Phase 1`
- continue linearly as `Phase 2`, `Phase 3`, `Phase 4`, and so on
- use one phase per conceptual detail or boundary by default
- add more phases whenever collapsing steps would hide a real structural dependency, authority transition, proof boundary, or denial boundary
- prefer more honest phases over fewer overloaded phases

Phases must:

- be ordered intentionally
- be followed in order
- each solve one real conceptual detail, boundary, or proof-bearing transition
- each leave the system in a coherent state for the next phase

Do not write specs as unordered work buckets.
Do not treat phases as interchangeable.
Do not compress a milestone into too few phases just to make the document look tidy.
Do not create phase splits that are cosmetic rather than architectural.
Do not use alternate numbering schemes such as `M1.1`, `M1.2`, or other nested milestone-local identifiers.

## Per-phase structure

Each phase should be rich and self-contained.

By default, each phase should include:

- a phase title naming the boundary or conceptual detail
- a short statement of what that phase freezes, admits, or closes
- `Relevant subsystems`
- `Relevant APIs` or equivalent source surfaces when known
- `Warnings`
- `Test requirements`
- `Engineering decisions`
- `Open questions`

Do not reduce phases to thin `Purpose / Must ship / Gate` blocks unless the user explicitly asks for a lighter spec style.

## Per-phase adversarial test rule

Each phase must include at least 2 adversarial tests by default.

Those tests normally belong inside the phase's `Test requirements` section.

The default expectation is:

- one adversarial equivalence, parity, convergence, or replay-honesty test
- one adversarial rejection, denial, drift, leakage, residue, or boundary-localization test

Add more than 2 tests whenever the phase has more than two meaningful failure modes.

Do not leave a phase with zero named hostile proof just because the milestone has a global acceptance section.

## Phase splitting heuristics

Split phases whenever any of these are true:

- authority changes
- the proof family changes
- one step produces a typed artifact that a later step consumes
- one step can deny or become unavailable before later execution
- request vocabulary, planning, execution, certification, and diagnostics are being mixed together
- one conceptual detail could reasonably be implemented, reviewed, or tested independently of another

Default toward narrower phases.

## Required workflow

1. Read the mandatory coding-guideline files in the required order.
2. Identify the target crate or subsystem.
3. Read the subsystem roadmap.
4. Write the short per-document summaries before drafting.
5. State the adversarial constraint before drafting the milestone.
6. Design the milestone so it solves the hard structural problem first.
7. Write or revise the milestone spec with the document loaded mostly by phases.
8. Check the resulting spec against the coding guidelines and roadmap before considering it done.

If the user explicitly points you to extra docs, read them and incorporate them. Otherwise do not expand the reading set on your own.

## Patch-writing rule

When creating or revising a spec file, write it in multiple smaller patches rather than one giant write.

Reason:

- large single writes are more likely to be rejected by Windows tooling or app limits
- smaller patches make it easier to keep structure aligned while revising

Expected behavior:

- create the file skeleton first
- fill major sections in follow-up patches
- add phase sections in additional patches
- refine the later summary sections after the phase plan is stable
- prefer several safe writes over one huge write

Do not try to dump the full spec into one oversized patch when the document is large.

## Required self-check before finalizing a spec

Ask these questions explicitly:

- Does the milestone solve a real structural problem or just package work cosmetically?
- Is the adversarial constraint precise and load-bearing?
- Does the roadmap justify this milestone now?
- Does the spec preserve crate authority boundaries?
- Are the phases carrying most of the real design information?
- Is each phase centered on one conceptual detail or boundary?
- Does each phase contain at least 2 adversarial tests by default?
- Could a competent engineer map this spec into honest types, modules, and tests?
- Does the milestone belong in this roadmap sequence, or is it out of order?

If any answer is no, revise the spec before presenting it.

## Preferred milestone structure

Use the local crate style when one already exists. Otherwise, prefer this structure:

```text
# Milestone N: <title>

## Goal

## Why This Milestone Exists

## Governing Summaries

## Adversarial Constraint

## Product Decision Lock

## Phase Plan

### Phase 1: <boundary or conceptual detail>

[short phase statement]

**Relevant subsystems**
- <subsystem>
- <subsystem>

**Relevant APIs**
- <api or source surface>
- <api or source surface>

**Warnings**
- <warning>
- <warning>

**Test requirements**
- <adversarial equivalence/parity/convergence test>
- <adversarial denial/drift/leakage/localization test>

**Engineering decisions**
- <decision>
- <decision>

**Open questions**
- None.
```

Continue with `Phase 2`, `Phase 3`, and so on until the milestone is honestly covered.

Then end with:

```text
## Must Ship

## Must Preserve

## Acceptance Evidence

## Sequencing Notes
```

Interpret this structure in the following way:

- the top sections are concise
- the phase plan is the main body of the document
- `Must Ship`, `Must Preserve`, and `Acceptance Evidence` summarize rather than carry the primary design payload

## Roadmap insertion rule

When adding a new milestone:

- place it in the roadmap sequence intentionally
- explain why it belongs there
- update the roadmap language so the milestone is not an orphan

Do not just create a standalone milestone doc without considering roadmap sequencing.

## Output rule

When reporting the design work back, include:

- what coding-guideline docs were read
- what roadmap doc was read
- the short summary takeaway from each governing doc
- the adversarial constraint chosen
- why the milestone belongs where it does
- what doc(s) were created or updated
- whether the user explicitly pointed you at any extra docs beyond the default reading set

Keep the report concise, but the spec itself should be rigorous.
