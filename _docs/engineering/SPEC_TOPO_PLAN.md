# Spec-Graph-Native Topology Migration Execution Plan

## Current Mission

Forge is migrating topology truth from the legacy arena model to the spec graph. The non-negotiables are:

- The spec graph is the product and the source of truth.
- `forge-spec::SpecState` and `forge-spec::SpecDraft` own committed and mutable truth.
- `forge-topo::ProjectedTopology` is the dense B-Rep read model.
- `forge-signal` is scheduling only and never truth.
- No new arena truth, projection-only truth, or permanent dual-path architecture is allowed.
- Migration work takes priority over normal roadmap work until the migration gates in this document are met.

## Locked Architecture Decisions

- `forge-spec` owns truth, schema, snapshots, draft mutation, serialization, lineage foundations, and future merge/naming/replay hardening.
- `forge-topo` owns projection building, projected handles, topology queries, and topology validators.
- `forge-kernel` owns feature execution, geometry coupling, proof/checkpoint/fingerprint orchestration, and spec-native runtime cutover.
- `forge-signal` owns invalidation, scheduling, conditions, and transactional reactive evaluation only.
- Truth, projection, and scheduling remain separate systems.
- Stable ids belong to `forge-spec`; projection handles are dense, ephemeral, and projection-scoped.
- Heavy numeric payloads stay in payload stores; reverse indexes, `prev`, disk sidecars, radial caches, and other traversal helpers are derived.
- Undo is snapshot restore.
- Merge is deterministic graph merge with typed conflicts.
- No permanent backward-compatibility wrappers or long-term dual-truth shims will be retained.
- Kernel integration with `forge-signal` must remain transactionally correct and aspect-correct.

## Current State

### `forge-spec`

Status: `Active`

Already real:

- Stable ids, schema, `SpecState`, and `SpecDraft`
- Graph storage, snapshots, draft commit flow, serialization
- Journal/replay/naming/lineage foundations
- Schema validation
- Shell metadata as graph truth payload
- A large migrated mutation surface for lifecycle, boundary, and NMT topology work

Still incomplete:

- Remaining operator truth slices
- Finished merge implementation
- Finished naming/replay/lineage hardening at the final architecture level

### `forge-topo` projection

Status: `Active`

Already real:

- `ProjectionBuilder`
- `ProjectedTopology`
- Projected handles
- Projection query surface
- Deterministic structural signatures used in parity tests
- Projection-native validator families for:
  - loop wiring
  - radial edge
  - reference integrity
  - shell closure
  - vertex disk
  - Euler/genus
  - structural aggregate (`baseline + Euler`)

Still incomplete:

- Remaining read-side/query shaping
- Remaining direct sub-validator coverage where families are still only partially exercised
- Any substantive legacy validator families not yet fully mirrored projection-side

### Kernel spec bridge

Status: `Active`

Already real:

- `SpecEnvelope`
- Signal-backed spec-native read-side substrate
- Spec-native projection materialization
- Spec-native structure validation
- Spec-native invariant validation for the currently supported invariant surface
- Spec-native checkpoint and fingerprint helpers
- Proof/checkpoint/fingerprint/invariant paths partially widened onto the new surface

Still incomplete:

- More runtime call-path cutover so spec-native becomes the default path
- Removal of remaining arena-shaped assumptions in kernel internals
- Collapse of transitional duplicate helper paths where safe

### `forge-signal` contract correction

Status: `Done`

Already fixed:

- Transactional runtime-backed `FeatureTree`
- Monotonic semantic aspect versions
- Idempotent aspect-aware dependency registration
- Subscriber deduplication
- Explicit feature signal policy surface
- Serialization round-trip coverage and architecture guard tests
- Topology-only payload materialization fast path

### Operator migration

Status: `Active`

Already migrated operator families include:

- Foundational lifecycle:
  - `MakeVertexFace`
  - `KillVertexFace`
  - `MakeEdgeVertex`
  - `KillEdgeVertex`
  - `SplitEdge`
  - `MakeEdgeFace` (restricted simple case)
  - `MakeShellFace`
  - `KillShellFace`
  - `MakeFaceVertex`
  - `KillFaceVertex`
- Boundary / face construction:
  - `MakeFaceFromVertices`
  - `MakeFaceInShellFromVertices`
  - `MakeLoopInFaceFromVertices`
  - `MakeEdgeKillLoop`
  - `KillEdgeMakeLoop`
  - `MakeFaceKillRingHole`
  - `KillFaceMakeRingHole`
  - `JoinFaces`
  - `JoinFacesNmt`
  - `SewEdge`
  - `UnsewEdge`
- Container / restructuring:
  - `MakeSolid`
  - `DestroyBody`
  - `MakeLumpRegion`
  - `DestroyLump`
  - `MakeEmptyShell`
  - `DestroyShell`
  - `RehomeShell`
  - `SplitShell`
  - `MergeShells`
  - `RehomeLump`
  - `ExtractLump`
  - `SplitLump`
  - `MergeLumps`
  - `SplitBody`
  - `MergeBodies`

Still incomplete:

- Remaining topology mutation families not yet ported to `SpecDraft`
- Any truth semantics exposed by those remaining slices

## Execution Order

The remaining work proceeds in this order:

1. Finish projection-native validator and read-side shaping.
2. Drain remaining operator migration slices.
3. Widen kernel cutover until spec-native is the default runtime path.
4. Complete parity acceptance across migrated families.
5. Harden graph-native replay, naming, and lineage.
6. Implement graph merge and typed conflicts.
7. Remove remaining legacy truth dependencies.
8. Delete the old truth model.

This order is deliberate:

- Read-side and validator work is low-risk and batchable.
- Truth mutation slices are higher-risk and must stay vertical.
- Kernel cutover is safest after projection and operator coverage are broader.
- Merge, naming, replay, and deletion should happen against the real final architecture, not an earlier transitional shape.

## Milestone Details

### 1. Projection-native validator and read-side shaping

Status: `Active`

Goal:
- Make `forge-topo` fully own the projected read model in a clean, split, testable shape.

Why now:
- This work is low-risk, batchable, and directly unlocks operator parity and kernel cutover.

Included work:
- Split remaining large projection validator families by concept.
- Add direct sub-validator tests where only aggregate coverage exists today.
- Finish any missing kernel-facing projection queries and read helpers needed by cutover work.
- Keep validator and query façades stable at component roots.

Out of scope:
- New truth semantics in `forge-spec`
- Legacy deletion
- Broad kernel refactors unrelated to spec-native reads

Definition of done:
- Remaining large projection validator families are split by concept.
- Direct tests exist for each meaningful sub-validator family, not only aggregate calls.
- Kernel-facing projection queries needed by cutover work are available from stable projection facades.

Blocking risks:
- If a read-side gap actually requires missing truth semantics, the work moves to milestone 2 and `forge-spec` is extended explicitly.

### 2. Remaining operator migration slices

Status: `Active`

Goal:
- Finish porting the remaining topology mutation surface to `SpecDraft`.

Why now:
- The migration is not complete until truth mutation leaves the arena path.

Included work:
- Port remaining operator families in strict vertical slices:
  - spec mutation
  - spec tests
  - projection parity tests
- Extend `forge-spec` truth where a slice exposes missing semantics.
- Keep restrictions explicit where a migrated operator is intentionally narrower than eventual target behavior.

Out of scope:
- Opportunistic generalization of operators beyond current truth support
- Read-only query batching that can be handled in milestone 1

Definition of done:
- Every remaining topology mutation family needed by the kernel runs on `SpecDraft`.
- Each slice has spec tests and projection parity tests.
- No truth semantics are being faked in projection-only state.

Blocking risks:
- Missing truth semantics may be discovered mid-slice; when that happens, truth must be extended before the slice continues.

### 3. Kernel cutover

Status: `Active`

Goal:
- Make spec-native paths the default runtime path for spec-backed kernel flows.

Why now:
- Once projection and operator coverage are broad enough, cutover can proceed without introducing holes.

Included work:
- Keep widening proof, checkpoint, fingerprint, invariant, and output call paths onto `SpecEnvelope` and projection-native reads.
- Remove arena-shaped assumptions from kernel helpers and pipeline internals.
- Collapse duplicate imperative caching and transitional wrappers where safe.
- Continue respecting `forge-signal` as the execution substrate for derived kernel state.

Out of scope:
- Full legacy deletion
- Merge/naming hardening

Definition of done:
- Spec-native surfaces are the default path for spec-backed validation, fingerprint, checkpoint, and output reads.
- Legacy truth is not required for normal spec-backed kernel flows.
- Remaining legacy use is isolated and explicitly transitional.

Blocking risks:
- A kernel path that still depends on an unported operator or validator family stays transitional until the earlier milestones close it.

### 4. Parity acceptance

Status: `Partially complete`

Goal:
- Prove that migrated spec-native behavior matches or intentionally supersedes legacy behavior.

Why now:
- Parity becomes a deletion gate only after enough truth and read-side work are migrated.

Included work:
- Expand operator parity coverage across remaining families.
- Compare validator outcomes where relevant.
- Lock deterministic structural signature expectations.
- Preserve or explicitly document intentional differences where new truth semantics are correct and legacy behavior was transitional.

Out of scope:
- Graph merge implementation
- Replay/naming hardening beyond parity checks needed for migrated slices

Definition of done:
- Migrated families have legacy-vs-spec parity coverage.
- Validator outcomes are compared where relevant.
- Deterministic structural signature expectations are locked.

Blocking risks:
- Some parity gaps may expose real missing truth or read-side semantics, pushing work back into milestones 1 or 2.

### 5. Replay, naming, and lineage hardening

Status: `Partially complete`

Goal:
- Finish the graph-native identity and history story on top of the migrated truth architecture.

Why now:
- These systems should harden against the final truth path, not a speculative intermediate one.

Included work:
- Graph-native replay hardening
- Naming anchor hardening through rebuild/split/merge flows
- Lineage retargeting and auditability improvements
- Deterministic diagnostics around identity-preserving vs identity-breaking edits

Out of scope:
- Legacy truth deletion
- Broad kernel cutover work not needed for replay/naming/lineage

Definition of done:
- Graph-native replay is the authoritative path.
- Naming anchors survive expected rebuild/split/merge flows deterministically.
- Lineage retargeting is auditable and tested.

Blocking risks:
- If merge is still incomplete, some naming/lineage hardening remains provisional until milestone 6 lands.

### 6. Graph merge and typed conflicts

Status: `Not started`

Goal:
- Implement deterministic three-way graph merge with typed conflict output.

Why now:
- Merge should be built on the final truth model, not during early cutover churn.

Included work:
- Three-way merge over spec truth
- Deterministic ordering of merged output
- Typed structural, naming, and delete-vs-modify conflict reporting
- Rejection of merged graphs that would project invalidly

Out of scope:
- Legacy deletion
- UI/presentation workflows for merge conflict resolution

Definition of done:
- Deterministic three-way merge exists.
- Typed conflict output exists for structural and naming conflicts.
- Invalid merged graph states are rejected before projection.

Blocking risks:
- Merge may surface missing schema-level conflict metadata that must be added to `forge-spec`.

### 7. Legacy dependency removal

Status: `Not started`

Goal:
- Reduce legacy truth dependencies to zero in active kernel flows.

Why now:
- This is only safe after operator migration, kernel cutover, and parity work are far enough along.

Included work:
- Inventory every remaining `TopologyArena`, `TopologyState`, and `MutableDraft` truth dependency.
- Remove or isolate them behind clearly temporary seams.
- Shrink transitional adapters steadily instead of carrying them forward.

Out of scope:
- Final deletion of old truth ownership code

Definition of done:
- Remaining legacy truth dependencies are inventoried to zero in active kernel flows.
- Adapters are removed or isolated behind clearly temporary seams.

Blocking risks:
- Hidden legacy dependencies may still appear in replay, naming, or tests until milestones 5 and 6 are further along.

### 8. Legacy truth deletion

Status: `Not started`

Goal:
- Delete the old truth model and all arena-bound truth plumbing.

Why now:
- Deletion is the last step, not a parallel track.

Included work:
- Remove old truth ownership code
- Remove arena-bound replay, naming, and event truth plumbing
- Remove dual-path docs, tests, and preludes
- Ensure the canonical serialized product is the spec graph

Out of scope:
- Preserving backward-compatibility layers

Definition of done:
- Old truth ownership code is removed.
- Arena-bound replay, naming, and event truth plumbing is removed.
- Docs, tests, and preludes no longer describe dual truth.

Blocking risks:
- Any unresolved cutover, parity, naming, replay, or merge dependency blocks deletion.

## Active Work Queue

The near-term queue is:

1. Finish remaining projection validator and read-side family shaping.
2. Continue remaining operator-family migration slices.
3. Widen kernel call paths that become unlocked by those two items.

Execution rules:

- Truth mutations move in strict vertical slices:
  - spec mutation
  - spec tests
  - projection parity tests
- Read/query/validator work may batch by family.
- If migration exposes missing truth semantics, extend `forge-spec` explicitly.
- Do not invent projection-only truth.
- Do not mirror arena sidecars.
- Do not resume normal roadmap work before the migration gates in this document are met.

## Phase Crosswalk

| Phase | Title | Current rough completion | Satisfied by execution-order milestones |
| --- | --- | --- | --- |
| 0 | Spec and freeze | Done | Locked architecture decisions and migration rules in this document |
| 1 | Create `forge-spec` | 90-95% | Milestones 2, 5, and 6 finish the remaining truth/runtime hardening |
| 2 | Build B-Rep projection in `forge-topo` | 80-85% | Milestone 1 finishes projection/read-side shaping |
| 3 | Parity harness | 70-80% | Milestone 4 completes parity acceptance |
| 4 | Port entity lifecycle ops | 90%+ | Mostly already satisfied; milestone 2 closes residual truth slices |
| 5 | Port boundary and NMT ops | 75-85% | Milestone 2 closes remaining operator families |
| 6 | Kernel cutover to `SpecState` | 65-75% | Milestone 3 is the main body of this phase |
| 7 | Merge/naming hardening | 20-30% | Milestones 5 and 6 |
| 8 | Delete old truth model | 5-10% | Milestones 7 and 8 |

## Acceptance Criteria

### Truth correctness

Pass when:

- `SpecDraft` can create, mutate, and delete required truth nodes and relations.
- Commit produces immutable `SpecState`.
- Rollback and snapshot restore preserve exact prior truth.
- Stable ids survive snapshot round-trips.
- Deterministic serialization holds for identical truth.

### Projection correctness

Pass when:

- `ProjectedTopology` deterministically materializes valid spec truth for supported topology cases.
- Projection rejects invalid truth deterministically.
- Projected traversal and structural signatures remain deterministic.
- Projection validator families directly cover their meaningful failure modes.

### Operator parity

Pass when:

- Each migrated operator slice has spec mutation tests.
- Each migrated operator slice has projection parity tests against the legacy path where parity still matters.
- No migrated operator relies on projection-only truth to pass parity.

### Validator parity

Pass when:

- Projection-native validator outcomes are compared against relevant legacy outcomes where the legacy family is still authoritative.
- Structural aggregate expectations are locked.
- Kernel proof/checkpoint/invariant tests exercise the spec-native validator path.

### Kernel cutover

Pass when:

- Spec-backed kernel flows use spec-native validation, checkpoint, fingerprint, and output paths by default.
- `SpecEnvelope` and projection-native reads cover active spec-backed kernel execution.
- Legacy truth is no longer required in normal spec-backed flows.

### Replay, naming, and merge

Pass when:

- Replay is graph-native and deterministic.
- Naming anchors behave deterministically across expected rebuild, split, and merge scenarios.
- Three-way merge is deterministic and returns typed conflicts for invalid combinations.

### Legacy deletion

Pass when:

- `TopologyArena`, `TopologyState`, and `MutableDraft` are no longer topology truth.
- Arena-bound truth-specific replay, naming, and event plumbing is removed.
- Docs, tests, and public surfaces describe a single-truth spec-native architecture.

## Agent Rules

- No god files.
- Public façades only at component roots.
- No inline tests in production files.
- No projection-only truth.
- No arena sidecar mirroring.
- If migration exposes missing truth semantics, extend `forge-spec`.
- Operators stay vertical; read/query/validator work may batch by family.
- Do not resume normal roadmap work before the migration gates in this document are met.
