---
name: qa-loop
description: Review and correct completed WORTH implementation work through a requirement-and-evidence closure ledger. Use after a coherent implementation phase or slice to audit specification fidelity and causally relevant risks, fix root causes, reopen affected guarantees, and continue until the implementation is genuinely proved.
---

# QA Loop

Attempt to falsify the implementation's claims. Treat the ledger as the
authority for closure. Green tests, completed fixes, and finding counts do not
define completion.

## Establish scope

Read the repository instructions, governing specification, changed files, and
causally relevant engineering laws. Inspect adjacent producers, consumers,
authority owners, persistence boundaries, public facades, and test support
where they can invalidate the implementation's guarantees.

Freeze the source state from which the audit begins and preserve unrelated
worktree changes.

## Build the closure ledger

Translate the scoped specification into logical guarantees. Use appropriate
vertical guarantees rather than tiny implementation tasks or combinations of
input dimensions.

For each row, record:

- a stable identifier
- the exact closure claim
- the evidence needed to prove it
- the current result and evidence
- `OPEN`, `PROVED`, `DEFECT`, `BLOCKED`, or justified `N/A`

Start uncertain rows as `OPEN`. Existing tests are candidate evidence until
their world, boundary, oracle, consequential state, and fault sensitivity have
been inspected.

The ledger enumerates guarantees, not review categories or test cases. One
piece of evidence may prove several rows, and one row may require several forms
of evidence.

## Build the risk map

Choose review lenses according to the guarantees and failure surfaces
represented in the ledger. Possible lenses include:

- semantic and specification correctness
- authority, security, privacy, and disclosure
- architecture, phase progression, lifecycle, and dependency direction
- failure, cancellation, concurrency, recovery, and migration
- performance and resource behavior
- test and fixture honesty
- composition and domain topology
- public DX and operability

For each possible lens, ask whether a defect in that category could plausibly
invalidate an in-scope guarantee. Apply the lens deeply when it is causally
relevant, lightly when only a boundary check is warranted, and omit it when it
has no meaningful connection to the work.

Do not privilege any category or mechanically multiply categories across
ledger rows. The implementation's actual claims and risks determine the
review.

## Plan proof economically

Reuse one causally valid canonical world with isolated semantic deltas. Group
equivalent inputs by the production behavior they exercise.

Prefer the cheapest honest proof:

- type and dependency enforcement for impossible structure
- compile-fail evidence for inaccessible public authority
- pure or property tests for broad validation spaces
- one-axis metamorphic tests for causal behavior
- targeted runtime tests for lifecycle and consequential state
- pairwise cases where two axes genuinely interact
- exhaustive matrices only for small, closed decision lattices
- a small number of realistic integrated journeys for cross-phase composition
- dedicated race tests only for real shared-state concurrency

Test combinations only when production behavior couples the axes. Every added
test must detect a distinct plausible fault. Consolidate redundant fixtures,
scenarios, compiler sessions, and integration journeys.

## Discover before declaring closure

Trace real execution, denial, failure, cancellation, and teardown paths across
the whole ledger. Do not stop discovery after finding the first defect.

Look for violated invariants, unearned authority, premature mutation, hidden
effects, incomplete lifecycle behavior, dishonest fallback, stale derived
state, incompatible evolution, resource escape, fixture theatre, and
implementation that satisfies wording while defeating intent.

Record every finding against the ledger rows it invalidates. State:

1. severity and affected guarantees
2. the concrete defect and evidence
3. the governing requirement or invariant
4. the required root-cause correction
5. the proof that would close it

Do not report preferences or speculative concerns as defects.

## Correct and reopen

Fix root causes rather than padding tests or adding local guards around a
broken authority model. Search the affected semantic family for the same
defect.

After a correction:

- retain the finding as audit history
- reopen every row whose evidence or assumptions changed
- reopen causally downstream rows when phase progression changed
- update fixtures and tests whose earlier proof became stale
- rerun the affected evidence family
- reassess the corrected production path directly

Do not rerun the Cartesian product of the entire suite when only a bounded
proof family was affected. Do not leave downstream guarantees proved merely
because their old tests remain green.

## Complete the loop

Continue discovery, correction, reopening, and verification until every ledger
row is resolved and no known in-scope defect remains.

Before declaring completion:

- verify the final source state, not an earlier snapshot
- run the required functional, compiler, architectural, and repository checks
- confirm relevant warm paths and test costs remain sane
- preserve resolved findings and their closure evidence
- record any unrelated repository failure as an explicit scoped caveat
- ensure the ledger contains no stale `OPEN` or `DEFECT` rows

Report closure from the ledger. Never infer closure from the absence of new
findings in a single pass.
