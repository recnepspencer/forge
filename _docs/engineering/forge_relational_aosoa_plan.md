# Forge Relational Targeted AoSoA Plan

## Purpose

This document defines the Phase 6 implementation plan for introducing a targeted hybrid AoSoA layer into `forge-relational`.

This is not a storage rewrite. It is a narrow optimization plan for the specific paths that still dominate large-world hot updates after Phases 1-5:

- touched working-state preparation
- touched-slot publication back into authoritative storage
- optional later packet-local execution kernels if the first two changes are insufficient

The intent is to preserve the current truth model, lineage model, replay model, and broad public surface while reducing the remaining locality and clone/publish costs on very large geometry and simulator-class worlds.

Primary supporting documents:

- [forge_relational_performance.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance.md)
- [forge_relational_phase4_hotspots_and_endurance.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_phase4_hotspots_and_endurance.md)
- [forge_relational_performance_baseline.jsonl](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.jsonl)

## Current State

`forge-relational` is currently SoA-dominant.

The authoritative record substrate is still a partitioned columnar arena:

- [arena.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/substrate/record_arena/arena.rs)
- [partition.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/overlay/partition.rs)

The hot commit path still prepares detached touched-partition overlays up front:

- [prepare.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/authority/commit/phases/prepare.rs)
- [working_state.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/overlay/working_state.rs)

Publication still commits back by touched slot over the SoA substrate:

- [authority.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/logic/authority.rs)

That means the current shape is:

- authoritative SoA storage
- partition-local detached overlays
- touched-slot journals
- packetized read/execution planning
- no chunk-local AoSoA representation yet

## Why AoSoA Is On The Table

AoSoA is no longer a rescue move. It is a targeted optimization candidate because the remaining large-world hot-path costs now cluster around locality-sensitive overlay and publish work.

The main evidence in the promoted baseline is:

- `rocketship_scale_matrix/hundred_k_nodes_zero_diagnostics_narrow_round_trip`
  - `draft_preparation_micros ~= 2477`
  - `publication_storage_commit_micros ~= 993`
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_propagation_wave`
  - `draft_preparation_micros ~= 2091`
  - `publication_storage_commit_micros ~= 846`
- `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave`
  - `draft_preparation_micros ~= 1967`
  - `publication_micros ~= 943`

Meanwhile:

- propagation execution is already small
- explicit reads are already small
- hot-path diagnostics are already under control

So the decision is no longer "can AoSoA save a bad runtime?"

It is now:

"can chunk-local layout reduce the remaining overlay and publish cost enough to justify its complexity?"

## Current Phase 6 Result

The first targeted AoSoA passes have now paid off on the intended paths without forcing a resident-storage rewrite.

Current certified read from [forge_relational_performance_baseline.jsonl](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.jsonl):

- `rocketship_scale_matrix/hundred_k_nodes_zero_diagnostics_narrow_round_trip`
  - `hot_update_micros ~= 4951`
  - `draft_preparation_micros ~= 71`
  - `publication_storage_commit_micros ~= 40`
- `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave`
  - `hot_update_micros ~= 12758`
  - `draft_preparation_micros ~= 76`
  - `publication_micros ~= 84`
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_flat_entity_batch_wave`
  - `64` entity updates across `8` partitions
  - `median_elapsed_micros ~= 13848`
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_large_flat_entity_batch_wave`
  - `128` entity updates across `8` partitions
  - `median_elapsed_micros ~= 14395`
- `rocketship_scale_matrix/hundred_k_nodes_pseudorealistic_mixed_entity_relation_batch_wave`
  - `64` entity updates plus `16` local relation creates across `8` partitions
  - `median_elapsed_micros ~= 36487`
  - `draft_working_state_clone_micros ~= 687`
  - `publication_storage_commit_micros ~= 435`

That means the current AoSoA rollout has already:

- removed whole-arena clone cost from the narrow `EntityOnly` hot path
- replaced publish fallback with a real chunk-aware publish path for the main flat-entity case
- widened safely into bounded multi-partition batch waves
- materially improved mixed entity-plus-local-relation geometry commits once touched-scope widening was corrected

It has not yet justified a resident-storage rewrite, but it has justified continuing to use targeted AoSoA where the perf lane keeps showing real leverage.

## Design Principles

1. Preserve one truth model.
The authoritative runtime stays singular. AoSoA must not create an alternate semantic engine.

2. Keep SoA as the canonical source of truth initially.
AoSoA should begin as a hot-path working-set and publish optimization, not a global storage rewrite.

3. Make AoSoA opt-in by path, not by domain-wide fork.
Only the targeted paths should use the chunked representation at first.

4. Fall back cleanly.
Every AoSoA-assisted path must be able to fall back to the current SoA path until the new path is fully certified.

5. Move in phases that are perf-verifiable.
Every stage must have clear counters and phase-timing expectations in the existing certification lane.

## Target Paths

These are the paths where hybrid AoSoA is currently most justified.

### 1. Draft Preparation / Working-State Construction

Primary files:

- [prepare.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/authority/commit/phases/prepare.rs)
- [working_state.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/overlay/working_state.rs)
- [partition.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/overlay/partition.rs)

Current issue:

- touched partitions are cloned as whole overlay partitions before mutation
- even `EntityOnly` mode still clones the full entity arena vectors for the touched partition
- this is the cleanest remaining structural contributor to `draft_preparation_micros`

AoSoA goal:

- replace whole-partition detached entity-arena clones with chunk-local touched working blocks
- stage only the touched slot neighborhoods and their immediately required metadata
- allow narrow entity-only commits to operate over a sparse set of chunked working blocks instead of a full partition clone

### 2. Entity-Only Storage Publication

Primary files:

- [authority.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/logic/authority.rs)
- [arena.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-relational/src/storage/substrate/record_arena/arena.rs)

Current issue:

- entity-only publication is already much better than before, but it still performs touched-slot moves over the canonical SoA vectors
- publication is narrower now, but still a meaningful large-world cost cluster

AoSoA goal:

- publish chunk-local touched groups back into authoritative storage with fewer scattered slot operations
- reduce write amplification when the touched set is sparse but spatially clustered
- make publication cost track changed chunks more directly than changed slots alone

### 3. Optional Later Packet-Local Execution Kernels

This is explicitly phase-gated and should only happen if the first two paths do not move the needle enough.

Candidate surfaces:

- packet-local propagation or adjacency execution
- locality-heavy read/materialization kernels

This is not the initial AoSoA target because those paths are not currently the top wall.

## Non-Targets

These are explicitly out of scope for the initial AoSoA rollout.

- lineage and history storage
- durability log storage
- replay basis selection
- retention policy logic
- broad public facade or DX surface redesign
- whole-arena identity model replacement
- full relation/adjacency layout rewrite in the first pass

If the first AoSoA pass cannot justify itself without touching those areas, the scope is too broad.

## Proposed Hybrid Representation

The first implementation should be a hybrid chunk-local working-set layer over the existing SoA substrate.

### Core Concept

Introduce a chunked entity working-block representation for touched partitions:

- fixed-size entity chunks inside a partition
- each chunk contains a compact AoSoA-style block for the hottest entity fields
- each chunk carries touched masks and slot maps
- cold or rarely-touched fields can remain SoA-backed initially

The representation should be optimized for:

- narrow local mutations
- sparse touched sets
- repeated updates in the same local neighborhood
- bulk import or publish of spatially clustered slot groups

### Initial Chunk Candidates

Chunk-local entity fields likely worth co-locating first:

- generation
- lifecycle
- kind id
- payload pointer or payload handle
- created/retired version ids
- pin counts

Fields that can remain SoA-backed initially if needed:

- deep payload history
- metadata history
- diagnostics enrichment maps
- aspect version maps

This keeps the first step focused on hot operational paths rather than deep history storage.

### Operational Shape

The first hybrid shape should look like:

- canonical partition state stays SoA
- `WorkingState` can optionally hold chunked entity working blocks for selected partitions
- mutation journals gain chunk-level summaries in addition to slot-level touched sets
- publish can merge changed chunks back into the authoritative SoA substrate

This keeps replay, history, and visibility logic anchored to the current canonical model.

### Resident Storage Boundary

This plan is specifically about AoSoA for hot mutation and publication blocks.

It is not a resident-storage conversion plan.

That means:

- canonical committed partition storage remains SoA-backed in the initial rollout
- chunk-local AoSoA blocks are introduced as execution-time and publication-time accelerators
- any future resident-storage AoSoA decision requires a separate design review

If the first implementation starts pulling canonical storage itself into AoSoA form, the scope has drifted.

### Initial Chunk-Size Decision

The first implementation should not expose an open-ended tuning surface.

Use a small fixed experiment set with only `2-3` candidate chunk widths, for example:

- small
- medium
- large

The exact numeric widths should be chosen once from current touched-slot distributions in the `rocketship_scale_matrix` and then held stable during the first certification pass.

Selection criteria:

- minimize `draft_preparation_micros`
- avoid inflating `publication_storage_commit_micros`
- keep fallback frequency low
- preserve locality for repeated nearby edits

If no candidate width materially improves the target metrics, the rollout should pause rather than expanding the search space.

## Rollout Phases

## Phase 6A: AoSoA Scaffold And Counters

### Goal

Add the structural seam without changing behavior.

### Work

1. add AoSoA configuration and feature seam
2. add chunk descriptors and chunk metrics
3. add instrumentation counters for:
   - chunk blocks staged
   - chunk slots materialized
   - chunk blocks published
   - chunk fallback path count
4. add no-op code paths that still fall back to current SoA behavior

### Success Criteria

- zero behavior change
- zero perf regression
- new counters visible in perf reporting

### Stop Rule

Stop immediately if the scaffold requires semantic branching across broad mutation, replay, durability, or visibility paths just to compile.

Phase 6A only succeeds if the seam stays narrow.

## Phase 6B: Entity-Only Draft Preparation

### Goal

Move `EntityOnly` working-state preparation off whole-arena overlay clone behavior.

### Work

1. add chunked entity working blocks for touched partitions
2. teach `WorkingState::from_touched_partitions(...)` to build chunked blocks for `EntityOnly` mode
3. preserve current full clone path for:
   - `Full` clone mode
   - relation mutation
   - adjacency mutation
   - merge and graph mutation paths
4. keep slot-level mutation journals for compatibility during rollout

### Success Criteria

- meaningful reduction in:
  - `draft_preparation_micros`
  - `draft_working_state_clone_micros`
  - cloned entity slot count equivalents
- no regression in entity-only correctness or replay parity

### Decision Gate

If this phase does not move the large-world geometry hot path meaningfully, the rest of the AoSoA plan should pause and be reevaluated.

### Hard Stop Threshold

Treat this phase as unsuccessful if all of the following remain true after a clean certification pass:

- `draft_preparation_micros` improves by less than `15%` on the main `100k` geometry cases
- total `hot_update_micros` improves by less than `10%`
- fallback use remains frequent enough that the AoSoA path is not carrying most entity-only hot commits

If those conditions hold, do not continue to Phase 6C by momentum alone.

## Phase 6C: Entity-Only Publication Merge

### Goal

Publish changed entity chunks back into authoritative storage more efficiently than the current touched-slot move path.

### Work

1. add chunk-level publish merge on top of authoritative SoA storage
2. preserve current slot-level fallback for:
   - sparse pathological edits
   - non-chunk-aligned updates
   - low-confidence migration cases
3. keep free-list and lifecycle semantics exactly aligned with current behavior

### Success Criteria

- meaningful reduction in:
  - `publication_storage_commit_micros`
  - total `publication_micros` on `100k` geometry cases
- no regression in:
  - lifecycle state correctness
  - pin behavior
  - snapshot/replay consistency

### Hard Stop Threshold

Treat this phase as unsuccessful if:

- `publication_storage_commit_micros` does not improve by at least `15%` on the large-world target cases
- correctness fallback remains the dominant path
- implementation complexity spills into non-target areas such as full graph mutation or historical storage

## Phase 6D: Optional Packet-Local Execution Blocks

### Goal

Only if justified, use the same chunk-local representation in locality-heavy execution kernels.

### Work

1. evaluate whether remaining cost now clusters around propagation/read kernels
2. only then introduce packet-local execution blocks

### Success Criteria

- only proceed if the earlier phases succeed but the target envelopes still are not good enough

## Certification Requirements

The existing perf lane is already the right gate. AoSoA should not ship without explicit certification against:

- `rocketship_scale_matrix`
- `sustained_load_matrix`
- `geometry_kernel_matrix`
- `cad_topology_matrix`
- `chip_simulator_matrix`
- `game_engine_matrix`

### Metrics That Must Improve

Primary AoSoA target metrics:

- `draft_preparation_micros`
- `draft_working_state_clone_micros`
- `publication_storage_commit_micros`
- `hot_update_micros` on large-world geometry cases

### Metrics That Must Not Regress Materially

- `propagation_execution_micros`
- `explicit_query_micros`
- `durable_append_micros`
- replay and recovery parity metrics
- packet count and scope-unit stability
- artifact and profile boundary metrics

### Counters To Add

AoSoA-specific counters should be added for:

- `aosoa_entity_chunks_staged`
- `aosoa_entity_chunk_slots_materialized`
- `aosoa_entity_chunks_published`
- `aosoa_entity_slot_fallback_merges`
- `aosoa_prepare_fallback_count`
- `aosoa_publish_fallback_count`

These should be certification-visible from day one.

## Expected Wins

If the initial hybrid AoSoA pass is worth it, we should expect:

1. lower `draft_preparation_micros` on the `100k` geometry cases
2. lower `publication_storage_commit_micros` on entity-heavy local updates
3. better stability under long repeated local updates on large worlds
4. a smaller gap between narrow entity-only update cost and large-world resident size

If those things do not happen, the implementation is likely too broad or aimed at the wrong path.

## Risks

### 1. Semantic Drift

Risk:

- chunk-local working blocks accidentally behave differently from detached SoA overlays

Mitigation:

- keep the current SoA path alive as the semantic fallback
- require parity tests before switching the default on any path

### 2. Dual-Model Complexity

Risk:

- maintaining both SoA and AoSoA paths gets messy

Mitigation:

- keep AoSoA restricted to `EntityOnly` preparation and publication first
- do not apply it to full graph mutation or history/durability paths initially

### 3. Wrong Chunk Shape

Risk:

- a poor chunk size or field mix hides the benefit

Mitigation:

- make chunk size explicit and measurable
- test a very small set of candidate chunk widths rather than making it endlessly tunable

### 4. Premature Broadening

Risk:

- the project expands into a storage rewrite

Mitigation:

- hold the non-target line aggressively
- require a new decision memo before touching relation/adjacency/historical layout

### 5. Wrong Problem Selection

Risk:

- AoSoA reduces a visible subphase but does not move the end-to-end hot path enough to matter

Mitigation:

- require end-to-end movement on the rocketship target cases, not just local microbench wins
- stop after Phase 6B or Phase 6C if the improvement is too small

### 6. Hidden Bootstrap Bias

Risk:

- the team implicitly expects AoSoA to fix import/bootstrap throughput even though this plan is aimed at steady-state hot paths

Mitigation:

- keep bootstrap/import as a separate problem in reporting
- do not count bootstrap wins as the success condition for the initial AoSoA rollout

## Recommended First Implementation Order

1. Phase 6A scaffold and counters
2. Phase 6B entity-only draft preparation
3. re-run `rocketship_scale_matrix` and `sustained_load_matrix`
4. if the metrics move meaningfully, do Phase 6C entity-only publication merge
5. only after that decide whether Phase 6D packet-local execution is warranted

## Go / No-Go Standard

### Go

Proceed with targeted AoSoA implementation if:

- the team wants another serious performance push before any broader architecture change
- we are willing to keep SoA as canonical truth while adding a hot-path hybrid layer
- success will be judged primarily on large-world geometry and simulator-class local update costs

### No-Go

Defer AoSoA if:

- bootstrap/import throughput is the only pain left
- current `100k` hot path is already good enough for the near-term product goals
- the team is not willing to carry a narrow dual-path implementation with fallback and certification

## Likely Failure Modes

These are the most likely ways the first pass could disappoint:

1. The touched-slot distribution is too sparse or irregular for chunking to beat the current SoA move path.
2. Draft preparation improves, but publication does not.
3. Publication improves, but only for narrow synthetic cases and not for the pseudo-realistic rocketship workloads.
4. Fallback coverage remains so high that the new path is hard to justify.
5. Chunk-local working blocks improve steady-state hot updates but do little for the practical pain the team actually feels because bootstrap/import remains dominant in day-to-day workflows.

These are not reasons not to try it. They are the reasons the rollout should stay narrow and measurable.

## Current Recommendation

Proceed with a targeted AoSoA prototype on the `EntityOnly` working-state and publication paths.

Do not begin with:

- full graph mutation
- relation/adjacency layout
- history/durability layout
- public API changes

The right first question is not "can we make the whole engine AoSoA?"

It is:

"Can chunk-local entity working blocks materially reduce large-world draft preparation and publish cost while preserving the existing SoA truth model?"

That is the smallest high-value experiment, and the current baseline says it is the right one.
