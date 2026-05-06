---
name: qa-loop
description: Run a Forge-quality QA and correction loop on completed work. Use when reviewing a just-finished phase or implementation batch against the milestone spec and Forge coding-guideline docs, producing findings-first feedback, fixing the findings, and repeating until no meaningful findings remain.
---

# QA Loop

Use this skill after implementation work appears complete enough for hostile review.

## Mandatory reading order

Read these in this order before running the QA loop:

1. `_docs/coding_guidelines/MENTALITY.md`
2. `_docs/coding_guidelines/arch_laws.md`
3. `_docs/coding_guidelines/composition_laws.md` if it is populated
4. `_docs/coding_guidelines/domain_structure_laws.md`
5. `_docs/coding_guidelines/perf_laws.md`

Then reread the governing milestone or implementation spec for the work under review.

## Standard

Review as a hostile engineer, not as a congratulatory assistant.

The bar is production grade:
- aerospace-grade
- chip-simulator-grade
- no MVP softness
- no "good enough" closure

Look for things that technically pass while violating the deeper intent of the milestone, the spirit of the architecture, or the proof obligations implied by the spec.

## Scope of this skill

This skill is for implementation QA and correction.

It should focus on:
- implementation integrity
- architectural integrity
- domain correctness
- performance posture
- proof strength
- obvious test omissions that block the work

This skill is not the dedicated final milestone adversarial test-audit pass. If the task is specifically to do the big end-of-milestone test review, that should be a separate step.

## Required workflow

1. Read the mandatory guideline docs in the required order.
2. Reread the governing spec.
3. Perform a hostile review.
4. Report findings first.
5. Fix the findings.
6. Reassess for remaining related weaknesses.
7. Repeat until no meaningful findings remain.

Do not stop at one review pass if real issues still exist.

## Required output discipline

When reporting findings:
- findings first
- concrete, not vague
- no celebration
- no meta commentary about the workflow

For each finding, state:
1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

## Canonical QA prompt

Preserve this wording exactly when you use it internally as your review frame.

```text
Perform a brutal QA of this phase.

Evaluate the phase against:
- the implementation spec
- `arch_laws.md`
- `composition_laws.md`, if it is populated
- `domain_structure_laws.md`
- `perf_laws.md`
- the spirit of the system vision, not merely literal checkbox compliance

Assume the bar is production-grade. Look for implementation gaps, architectural dishonesty, hidden complexity, semantic weakness, proof weakness, performance blindness, shallow tests, and anything that technically passes while violating the deeper intent.

Report findings first.

For each finding, state:
1. what is wrong
2. why it matters
3. what authority it violates
4. what correction is required

If there are no findings, say so explicitly only after genuinely hostile review.
```

## Canonical correction-loop prompt

Preserve this wording exactly when you use it internally as your correction frame.

```text
Address these QA findings completely.

Do not negotiate with them, work around them, or minimize them. Correct them in a way that preserves the governing architecture and improves the phase in substance, not appearance.

After corrections, reassess whether any related weakness remains, including small but real ones. Then continue until the phase is ready for another hostile QA pass.
```

## Completion rule

The QA loop is complete only when:
- no meaningful findings remain
- the work still matches the spec
- the architecture still holds
- domain correctness is intact
- performance posture is honest
- no obvious in-scope cleanup remains

Do not declare victory early.
