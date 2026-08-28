# Branching, Merging, And Commit Vocabulary

This folder documents the Milestone 5 transition surface in
`worth-foundational`.

Use these docs when you need to answer questions like:

- How do I represent branch-local work without accidentally claiming
  authority?
- How do I plan a merge, admit a merge verdict, and keep strategy and basis
  influence visible?
- How do I cross into proof-bearing committed authority and then issue a real
  commit receipt?
- How do I canonicalize transition artifacts and re-admit them after a trust
  boundary?
- What exactly is frozen by the Milestone 5 readiness artifact?

Read the docs in this order if you are new to the surface:

1. [Exact Branch References](./branch-references.md)
2. [Branch-Local Candidates And Staged Branches](./branch-local-candidates-and-staged-branches.md)
3. [Merge Planning And Verdicts](./merge-planning-and-verdicts.md)
4. [Transition Strategy And Basis Semantics](./transition-strategy-and-basis-semantics.md)
5. [Scoped Merge And Cherry-Pick Vocabulary](../scoped-merge-adoption.md)
6. [Committed Authority Transitions](./committed-authority-transitions.md)
7. [Commit Receipts, Discard, And Transition Bundles](./commit-receipts-discard-and-transition-bundles.md)
8. [Transition Canonical Basis, Locators, And Current-Basis](./transition-canonical-basis-locators-and-current-basis.md)
9. [Transition Production Readiness](./transition-production-readiness.md)

The order matters.

- Exact branch references are the only shared grammar for an operational
  owner observation. They remain descriptive until Relational or Signal
  readmits them and issues its concrete authority.
- Branch-local surfaces are descriptive only.
- Merge planning and merge verdicts are still not authority.
- Scoped merge and cherry-pick vocabulary names selected scope before any
  adopting runtime executes it.
- Committed authority is the first proof-bearing authority lane.
- Receipts and bundles derive from committed authority.
- Canonical basis and current-basis behavior strengthen already-honest
  transition artifacts instead of redefining them.
- Readiness freezes the machine-checkable closure contract for the milestone.

These docs are capability-first on purpose. They are not milestone notes or
test tours. If a transition capability shipped, it should have a stable home in
this folder.

Milestone 5's epoch/equivalence candidate forms and the exact reference
grammar are intentionally different families. There is no compatibility
constructor from `BoundaryEpoch`, `EquivalenceBasisId`, or a staged candidate
to an exact operational reference.
