---
name: qa-loop
description: Review and correct completed WORTH implementation work against its governing specification and causally relevant engineering laws. Use after a coherent implementation slice is complete to find substantive defects, fix root causes, verify corrections, and repeat until the work is genuinely complete.
---

# QA Loop

Attempt to falsify the implementation's claims. Report only evidence-backed
defects; do not reward green checks or invent findings to appear rigorous.

## Establish authority

Read the repository instructions, governing specification, changed files, and
the coding laws relevant to the affected guarantees. Inspect adjacent producers,
consumers, authority owners, persistence boundaries, and test support when they
can invalidate the implementation's claims.

## Build the risk map

Identify:

- what the implementation claims to accomplish
- which invariants and authority boundaries it can affect
- which failures would be expensive, silent, irreversible, or hard to detect
- what evidence currently supports each material claim

Apply review lenses by causal relevance, not uniformly:

- semantic and specification correctness
- authority, security, privacy, and disclosure
- architecture, lifecycle, state, and dependency direction
- failure, cancellation, concurrency, recovery, and migration
- performance and resource behavior
- test and fixture honesty
- composition and domain topology
- public DX and operability

Always classify security and performance relevance. Perform a deep pass only
when the change exposes their corresponding threat or cost surface.

## Review

Trace real execution and failure paths. Look for violated invariants,
unearned authority, hidden effects, incomplete lifecycle behavior, dishonest
fallbacks, stale derived state, incompatible evolution, resource escape,
fixture theatre, and implementation that satisfies wording while defeating
intent.

Prefer the smallest decisive evidence: source tracing, type and dependency
inspection, targeted execution, adversarial tests, structural counters, or
repository enforcement. Passing tests are evidence only for claims they
honestly establish.

## Findings

Report findings before summaries. For each finding state:

1. severity and affected guarantee
2. concrete defect and evidence
3. governing source or invariant
4. required root-cause correction
5. proof that would close the finding

Do not report style preferences as correctness findings.

## Correct and repeat

Fix root causes, not symptoms. Search the affected semantic family for the same
defect, update tests and fixtures when the proof was weak, and run verification
proportional to the changed guarantees. Rebuild the risk map after material
corrections and continue until no meaningful findings remain.

Completion requires specification fidelity, preserved authority and
correctness, honest relevant performance posture, credible test evidence,
passing required enforcement, and no known in-scope defect hidden by the
harness or review boundary.
