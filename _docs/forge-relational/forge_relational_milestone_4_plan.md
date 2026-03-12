# Milestone 4 Plan: Invariant Engine

## Purpose

This document is the implementation plan for Milestone 4 from
[forge_relational_architecture_roadmap.md](./forge_relational_architecture_roadmap.md),
covering Phase D in
[relational_architecture.md](./relational_architecture.md).

This milestone is a clean-break rewrite of invariant execution. The goal is not
to preserve the current validation plumbing. The goal is to replace broad,
execution-point-driven invariant passes with a real invariant engine that is
explicit about groups, cost, applicability, failure policy, and execution
context.

## Architectural Goal

Invariants should stop behaving like a bag of checks attached to a few runtime
helper functions.

They should become:

- explicitly classified
- cheaply dispatchable
- tied to mutation intent contracts
- derived from authoritative runtime state
- deterministic in selection and ordering
- able to distinguish pass, fail, and not-applicable

The execution model should preserve a thin boundary layer above the engine.
That boundary exists to select invariant policy for a runtime phase, workload,
or domain profile. It must not become a second engine or a helper bucket.

The split of responsibility is:

- the engine answers: "given this request, what happened?"
- the policy boundary answers: "for this phase/profile/domain, what request is correct?"

This is required for future workloads like geometry kernels, chip simulators,
and game engines, which need configurable invariant pressure without leaking
request assembly into every caller.

## Ordered Slices

### Slice 1: Invariant Taxonomy Rewrite

Introduce the classification layer:

- `InvariantGroup`
- `InvariantGroupSet`
- `InvariantCostClass`
- `InvariantExecutionPointSet`
- metadata registration per invariant

Each invariant must declare:

- groups
- supported execution points
- cost class
- failure effect

This slice should eliminate hardcoded rule-bucket dispatch from the current
runner.

### Slice 2: Intent Contract System

Introduce a merged-plan contract model that describes which semantic surfaces a
mutation plan can affect.

The contract should answer things like:

- entity existence affected
- relation existence affected
- uniqueness affected
- adjacency affected
- publication surface affected
- historical visibility affected

This should be computed once per merged plan and reused by invariant dispatch.

### Slice 3: State-Derived Invariant Context

Introduce a named invariant execution context derived from runtime truth:

- committed state view
- working/overlay state view when present
- version context
- execution point
- plan contract
- publication context when relevant

Commit phases should stop building bespoke invariant context by hand.

### Slice 4: Engine and Dispatch

Introduce:

- `InvariantEngine`
- `InvariantExecutionRequest`
- `InvariantExecutionResult`
- `InvariantVerdict`
- a thin invariant policy boundary above the engine

Dispatch should be driven by:

- execution point
- group mask
- cost policy
- intent contract applicability

Selection must be deterministic and cheap.

The boundary above the engine should expose only phase/profile-owned entrypoints.
It should not expose a broad "run arbitrary invariants" helper API.

### Slice 5: Commit/Publication Integration

Rewire commit-boundary, mutation-sensitive, and publication checks so the
pipeline invokes the engine instead of helper-specific rule scans.

Blocking/publication-failure/audit-only policy must be centrally encoded.
Phase callers should choose named policy entrypoints, not assemble raw engine
requests inline.

### Slice 6: Rule Migration and Deletion Pass

Port every current invariant to the new engine shape and delete:

- old catalog plumbing
- duplicated execution-point branching
- old filtering helpers
- compatibility wrappers between old and new invariant result surfaces

No long-lived dual system.

## Must Preserve

- deterministic invariant ordering
- blocking vs publication-only semantics
- no hidden mutation during checks
- authoritative state as the source of truth
- concrete diagnostic detail on failure

## May Be Deleted Freely

- current broad invariant helper flow
- legacy bucket dispatch logic
- duplicated execution-point filtering
- old result-wrapper plumbing
- temporary compatibility surfaces
- generic validation helper APIs that bypass the policy boundary

## Main Risks

### Silent applicability drift

If intent contracts are incomplete, invariants may stop running when they
should.

Mitigation:

- add contract tests early
- add representative mutation-plan coverage for each invariant family

### Failure-routing drift

If failure policy moves carelessly, commits may block or succeed incorrectly.

Mitigation:

- centralize failure policy in the engine
- keep integration tests at each execution point

### Decorative metadata

If group/cost metadata exists but dispatch ignores it, the design is fake.

Mitigation:

- make dispatch depend on the metadata from the start

## Test Plan

- dispatch selection by execution point
- dispatch selection by group membership
- cost filtering behavior
- deterministic ordering
- `NotApplicable` distinct from `Pass`
- policy-boundary request selection
- intent-contract coverage
- commit-boundary blocking behavior
- mutation-sensitive blocking behavior
- publication-failure behavior
- full `cargo test -p forge-relational --lib`

## Immediate Execution Order

1. build taxonomy types and registrations
2. make current validation flow use registrations instead of hardcoded bucket tuples
3. add intent-contract types
4. add invariant execution context
5. introduce the engine request/result shell
6. migrate commit-boundary first
7. migrate mutation-sensitive
8. migrate snapshot-publication
9. thin the boundary into a phase/profile policy surface
10. delete the old path
