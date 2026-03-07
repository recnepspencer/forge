# Forge Signal Foundation Execution Plan

## Objective
Build `forge-signal` as a domain-independent, library-grade reactive runtime that can support spec-graph-native kernel evolution without coupling to B-Rep/topology internals.

This plan intentionally excludes full topo migration/cutover work. It focuses on signal-layer foundations only.

## Source Alignment
- `VISION.md`: spec graph is truth; derived projections are disposable.
- `_docs/coding_guidelines/domain_standards.md`: component organization, N-tier layering, facade boundaries, domain naming, cohesion.

## Scope
1. Keep `forge-signal` independent from kernel/topo semantics.
2. Complete N-granularity tier model as policy-driven runtime behavior.
3. Add conditional node evaluation primitives in a domain-free way.
4. Harden deterministic and transactional behavior for mission-critical usage.
5. Reorganize crate structure and tests for domain standards compliance.

## Non-Goals
1. No full topology migration in this phase.
2. No async/multithread execution runtime in this phase.
3. No geometry or topology domain logic inside `forge-signal`.

---

## Architectural Contracts

## C1. Two Graph Contract
`forge-signal` owns **evaluation graph** edges only (must be DAG).

It does not own or model cyclic structural graphs (B-Rep pointers). Structural state is opaque input from host domains.

## C2. Tier Policy Contract (N-Granular)
Tier behavior is defined by policy, not hardcoded enum semantics:
- dependency mode (`Static`, `AutoDiscovered`)
- dirty propagation (`Batched`, `Immediate`)
- evaluation trigger (`Checkpoint`, `LazyPull`, `OnDemand`, reserved `Async`)

No Forge-specific tier vocabulary in runtime internals.

## C3. Comparator Contract
Version propagation must support pluggable comparators:
- `Exact`
- `Tolerance { epsilon }`
- custom comparator callback

Default remains deterministic and exact.

## C4. Transaction/Checkpoint Contract
Subscriber/event outputs stage first, commit atomically after successful flush.
On failure: rollback staged data, preserve committed snapshot, propagate error.

## C5. Determinism Contract
Deterministic ordering must not depend on registration order or hash maps:
- stable topo sort
- stable tie-break by `SubscriberId`
- deterministic collection iteration

---

## Domain Standards Mapping

## Component Structure
`forge-signal` remains one component with:
- `presentation/` public API boundaries
- `logic/` orchestration/evaluation behavior
- `data/` schemas/state models
- `facade.rs` external entry point

## Cohesion Refactor Requirements
Current broad files must be split into single-concept modules where needed (especially event runtime and tests).

## Facade Discipline
External crates import through `forge_signal::facade`.
Deep imports from `data/*` and `logic/*` are disallowed cross-crate.

---

## Target Directory Layout

```text
crates/forge-signal/src/
  facade.rs
  lib.rs
  presentation/
    mod.rs
    api.rs
  data/
    mod.rs
    error.rs
    aspect/
      mod.rs
      aspect.rs
      version.rs
      mask.rs
    node/
      mod.rs
      node.rs
      condition.rs
      comparator.rs
    graph/
      mod.rs
      handle.rs
      dependency.rs
      entry.rs
      graph.rs
    tier/
      mod.rs
      policy.rs
      trigger.rs
    checkpoint/
      mod.rs
      checkpoint.rs
      policy.rs
      dirty_set.rs
    events/
      mod.rs
      subscriber.rs
      subscriber_context.rs
      effect_mapping.rs
      ids.rs
    trace/
      mod.rs
      summary.rs
  logic/
    mod.rs
    evaluation/
      mod.rs
      evaluate.rs
      invalidate.rs
      context.rs
    checkpoint/
      mod.rs
      runtime.rs
    events/
      mod.rs
      bus.rs
      ordering.rs
```

Note: this is the target shape. We can land it incrementally to keep PRs reviewable.

---

## Test Layout (Required)

Move from `src/tests.rs` to dedicated concept files:

```text
crates/forge-signal/src/tests/
  mod.rs
  graph_basics.rs
  invalidation_semantics.rs
  evaluation_skip.rs
  tier_policy.rs
  node_conditions.rs
  checkpoint_runtime.rs
  event_bus_ordering.rs
  event_bus_rollback.rs
  determinism.rs
```

Each file tests one concept family. No mixed omnibus test files.

---

## PR Sequence

## PR1: Structural Reorganization + Test Directory Split
### Changes
1. Introduce directory structure for cohesive submodules (no behavior changes).
2. Move `src/tests.rs` into `src/tests/` concept files.
3. Keep `facade.rs` exports stable.

### Acceptance
1. `cargo test -p forge-signal --lib` green.
2. No external crate import breakage.
3. No behavior changes in deterministic tests.

## PR2: Aspect Mask Core + Typed Aspect API
### Changes
1. Add internal aspect bitmask representation.
2. Keep typed aspect API at facade boundary.
3. Route dirty checks via mask operations in runtime internals.

### Acceptance
1. Existing aspect behavior unchanged.
2. Determinism tests unchanged.
3. New tests validate mask/typed equivalence.

## PR3: Comparator Policy Integration
### Changes
1. Add `VersionComparator` in data layer.
2. Support `Exact`, `Tolerance`, and custom comparator hook.
3. Wire comparator into evaluate/propagate path.

### Acceptance
1. `Exact` path is bit-for-bit unchanged vs baseline.
2. Tolerance tests show suppressed phantom recompute.
3. Deterministic ordering unaffected.

## PR4: Tier Policy Completion (N-Granularity Runtime)
### Changes
1. Ensure runtime behavior is fully policy-driven.
2. Remove residual hardcoded assumptions tied to `Entity/Feature/Analysis`.
3. Keep compatibility enum as deprecated adapter only (if retained).

### Acceptance
1. Tests pass for custom caller-defined tier sets (N tiers).
2. Static/batched and autodiscovered/immediate flows both validated.

## PR5: Event Runtime Hardening
### Changes
1. Split event bus internals into ordering/registry/flush modules.
2. Enforce rich DAG diagnostics:
   - full cycle chain
   - missing provider details
   - duplicate provider details
3. Validate reverse-order rollback semantics.

### Acceptance
1. Ordering deterministic independent of registration order.
2. Reverse rollback order tests pass.
3. Error diagnostics snapshot tests pass.

## PR6: Checkpoint Runtime Hardening
### Changes
1. Ensure stage/finalize semantics are explicit and atomic.
2. Add invariants around failure behavior (no partial commit).
3. Keep runtime data heap-backed for cheap `mem::take` compatibility.

### Acceptance
1. Flush failure never leaves partial committed staged outputs.
2. Recovery/rollback tests pass.
3. Existing host integrations compile unchanged.

## PR7: CI Guardrails for Signal Quality
### Changes
1. Add checks for:
   - deterministic ordering stability
   - no `HashMap` iteration in observable ordering paths
   - no direct deep import usage from external crates (facade-only policy)
2. Add microbenchmark gate for signal runtime overhead (budgeted threshold).

### Acceptance
1. CI fails on deterministic drift.
2. CI fails on facade boundary violations.
3. CI fails on regression above defined benchmark threshold.

## PR8: Signal-Layer Contracts Doc + Integration Guide
### Changes
1. Add canonical `forge-signal` contract doc:
   - what runtime guarantees
   - what runtime does not own
   - host integration rules (transaction boundary, structural state as opaque input)
2. Add migration notes for topo/kernel consumers.

### Acceptance
1. Doc published and linked from `facade` crate docs.
2. Integration guide validated by topo/kernel maintainers.

---

## Risks and Plan-Tied Mitigations

1. **Cycle confusion between structural and evaluation graphs**
   - Mitigation: C1 contract + docs + tests in PR8.

2. **Granularity explosion from over-node-ization**
   - Mitigation: keep signal graph macro-level; document opaque heavy numeric state contract in PR8.

3. **Mid-operation reactive explosions**
   - Mitigation: checkpoint barrier policies remain explicit; staged runtime tested in PR6.

4. **Determinism drift due to container/order choices**
   - Mitigation: PR2/PR5/PR7 deterministic ordering gates + container audit.

5. **Performance regression from abstraction expansion**
   - Mitigation: PR7 benchmark gate + heap-backed runtime state contract in PR6.

---

## Immediate Next Actions
1. Execute PR1 (reorganization + test split) first, no behavior change.
2. Land PR2 and PR3 before any further topo/kernel signal integration work.
3. Freeze additional topo migration until PR1-PR3 are green.
