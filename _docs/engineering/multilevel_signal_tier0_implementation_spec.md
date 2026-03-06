# Multilevel Signal Architecture: Tier-0 Implementation Spec

## 0) Scope and Intent
This spec replaces ad-hoc topo cache orchestration with a domain-free Tier-0 signal runtime in `forge-signal`, then adapts `forge-topo` through explicit router/evaluator slices.

This is intentionally aligned with `_docs/coding_guidelines/domain_standards.md`:
- Vertical slices by component (`forge-signal`, `forge-topo`, `forge-core`)
- N-tier separation (`data`, `logic`, `facade`)
- Explicit facades and restricted cross-component coupling

## 1) Target Architecture

### 1.1 Tier Semantics
- Tier-0 Entity:
  - static routing (`Effect -> Domain + Scoped Impact`)
  - batched dirty collection
  - checkpoint-triggered refresh
  - deterministic flush order
- Tier-1 Feature:
  - existing push/pull graph with read-tracked dependencies
- Tier-2 Analysis:
  - Tier-1 model; async scheduling deferred

### 1.2 Shared Domain-Free Core (`forge-signal`)
- `EvaluationTier`
- `CheckpointBarrier`
- `BatchedDirtySet<Domain, Impact>`
- `EffectMapping`
- `CheckpointEvaluator`
- `CheckpointPolicy`
- `CheckpointRuntime`

### 1.3 Topology Adapter (`forge-topo`)
- `TopoCacheDomain` and `TopoCacheTarget`
- `TopoCacheEffect` (ID-scoped by default)
- `TopoEffectMapping` (`EffectMapping` impl)
- `TopoCacheEvaluator` (`CheckpointEvaluator` impl)
- `TopoCacheRuntime` policy + telemetry + deterministic trace

## 2) Non-Negotiable Invariants
- Scoped impacts are default; global invalidation is fallback-only.
- All flush ordering is deterministic (domain order + target order).
- All radial link writes flow through arena choke-point APIs.
- Replay determinism compares cache refresh trace bytes in addition to topology hash.
- Rollback semantics are contract-locked (snapshot-restore, append-only `Reverted` lineage event).

## 3) Implementation Phases

### Phase A: Shared Signal Tier-0 Primitives
Deliverables:
- `forge-signal/src/data/{checkpoint,tier,dirty_set,effect_mapping,evaluator,checkpoint_policy}.rs`
- `forge-signal/src/logic/checkpoint_runtime.rs`
- `forge-signal/src/facade.rs` exports

Acceptance:
- unit tests for deterministic dirty merge/order
- unit tests for barrier-respecting flush
- unit tests for `ensure_fresh(domain)` domain-local refresh

Trap guard:
- Do not force Tier-0 through Tier-1 dynamic dependency graph APIs.

### Phase B: Topo Adapter and Runtime Policy
Deliverables:
- `crates/forge-topo/src/b_rep/data/storage/cache_runtime.rs`
- `TopoCachePolicy`, `TopoCacheTelemetry`, `CacheRefreshTraceEntry`
- scoped effects for radial/face/vertex/shell cache domains

Acceptance:
- policy rejects global invalidation by default
- strict per-commit freshness check enforced
- domain budget enforcement at commit
- targeted refresh parity tests vs global rebuild per domain

Trap guard:
- No unscoped effect variants except explicit `GlobalInvalidate`.

### Phase C: Mutation Chokepoints and Effect Emission
Deliverables:
- radial mutation choke-point API (`set_half_edge_radial_next`)
- operator paths emit effects via arena runtime (not direct cache mutation)

Acceptance:
- CI guard bans direct `.set_radial_next(...)` outside sanctioned files
- tests verify radial cache invalidation/refresh correctness

Trap guard:
- Prevent bypass paths that silently skip dirty marking.

### Phase D: Replay/Determinism/Lineage Coupling
Deliverables:
- replay entry cache refresh trace payload
- deterministic trace encoder
- determinism verifier compares trace payloads

Acceptance:
- determinism golden test asserts identical:
  - topology hash
  - replay bytes
  - lineage ordering
  - cache refresh trace ordering/content

Trap guard:
- Do not derive determinism from incidental map iteration; all observed lists canonicalized.

### Phase E: Rollback Contract Lock
Deliverables:
- rollback contract doc (`_docs/engineering/rollback_contract.md`)
- rollback contract types (`rollback_contract.rs`)
- `LineageEvent::EntityReverted` + store semantics

Acceptance:
- contract-lock test enforces chosen rollback strategy/version
- lineage store test verifies append-only revert event restores live lineage

Trap guard:
- No hidden “hard rewind” semantics.

### Phase F: CI Hard Gates
Deliverables:
- `scripts/ci/check_determinism_guards.py`
- `scripts/ci/check_determinism_golden.sh`
- `Makefile` targets: `determinism-guards`, `determinism-golden`, wired into `check`

Acceptance:
- lint guard blocks:
  - `println!` / `dbg!` in production topo paths
  - `HashMap`/`HashSet` in observed determinism paths
  - direct `set_radial_next` bypass usage
  - unsanctioned `GlobalInvalidate`
- golden gate runs deterministic runtime tests on every `make check`

Trap guard:
- CI must fail on policy regressions, not merely log warnings.

## 4) Performance and Scale Constraints (Codified)

### 4.1 Current Tier-0 Budgets
- Dirty marking: amortized O(log N) per scoped target (`BTreeSet`)
- Flush: O(D log D + T log T) where `D` domains, `T` scoped targets
- Refresh granularity: scoped by entity IDs; no full-domain recompute except explicit global fallback

### 4.2 Guardrails
- per-domain global invalidation budget defaults to zero
- telemetry tracks global/scoped invalidations + flush counts
- strict fresh-at-commit catches stale-domain leakage

### 4.3 Deferred Upgrades
Execution upgrades for `forge-signal` (parallel eval, allocation tuning, GC changes) remain deferred until measured performance walls are hit.

## 5) Integration with Upcoming Kernel Work
- Persistent naming/replay:
  - relies on deterministic refresh trace and canonical target ordering
- Rollback:
  - relies on explicit contract and lineage event semantics
- Future domains (topo/geom/spatial):
  - add domain adapters (router + evaluator) without changing `forge-signal` core

## 6) Definition of Done
Done means:
- all six phases above are implemented
- `make check` executes both guard lint and deterministic golden gates
- no fallback globals in production mutation paths (except sanctioned runtime fallback API)
- replay determinism includes cache-refresh trace equivalence
- rollback contract is type- and test-locked
