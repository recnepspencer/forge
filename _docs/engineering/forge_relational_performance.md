# Forge Relational Performance Hardening Spec

## Purpose

This document defines the engineering plan for hardening `forge-relational` toward credible support for:

- geometry kernels
- chip simulators
- game-engine-style reactive worlds

The goal is not generic benchmark improvement. The goal is a certifiable runtime that can support very high-end workloads while preserving lineage, replay, and auditability.

This spec covers the work up to the point where a hybrid AoSoA decision can be made from evidence rather than instinct.

Primary supporting documents:

- [forge_relational_coverage_and_api_inventory.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_coverage_and_api_inventory.md)
- [forge_relational_pre_hardening_scope.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_pre_hardening_scope.md)
- [forge_relational_performance_baseline.jsonl](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_performance_baseline.jsonl)
- [VISION.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/OG/VISION.md)

## Non-Goals

This spec does not yet include:

- the hybrid AoSoA implementation plan
- DX cleanup and public API redesign
- arbitrary memory minimization

We are optimizing first for throughput, bounded latency, recoverability, and trustworthy tracing on large machines.

## Operating Assumptions

Target users for the hardest workloads can afford:

- `256+` cores
- `1TB+` RAM
- high-end local NVMe

The runtime may spend more memory to preserve speed and recoverability.

Perfect lineage and replay are allowed to cost real overhead, but the acceptable premium is bounded:

- thin operational path should remain the primary performance target
- rich tracing should ideally stay within roughly `25-50%` overhead versus thin operational runs for the same workload
- anything materially beyond that must be justified as deferred, optional, or moved off the hot path

## Current Read

Current state from the certified matrix:

- primitive runtime performance is strong
- hot/cold separation is now measured
- artifact recoverability is now measured
- merged relational-plus-signal execution is now measured at small invalidation width
- Phase 1 diagnostics policy is now proven in code and baseline data
- geometry-scale hot paths no longer flood the synchronous path with `DetailedTrace` artifacts
- packetization and locality still matter, but the next hardening wall is profile enforcement plus broader merged relational-plus-signal scaling

## Readiness Targets

These are target envelopes for credibility, not final theoretical limits.

### Geometry Kernel Targets

Narrow hot operational geometry work:

- `<= 100us` median for small local topology edits on small resident worlds
- `<= 250us` median for small local connectivity/query follow-up
- `<= 250ms` median for narrow hot updates on `100k`-node pseudo-realistic worlds

Rich geometry certification work:

- `<= 1.5x` hot operational cost on small and medium workloads
- `<= 2.0x` hot operational cost on large resident worlds
- rich artifacts must not scale linearly with total resident graph size for narrow local edits

Merged relational-plus-signal geometry waves:

- `<= 1ms` median for narrow invalidation regions
- explicit target envelope for recomputation width:
  - downstream recompute should scale with affected region, not total world size

### Chip Simulator Targets

Thin hot compile/update loop:

- `<= 25us` median compile for narrow local recompute
- `<= 250us` median narrow update-plus-compile step
- `<= 2ms` median dense local fanout wave under operational-thin profile

Recoverability:

- checkpoint/recover/replay must remain available without polluting the hot stepping loop
- compiled views must be reconstructable from replay with parity certification

Rich diagnostics:

- rich profile should remain bounded and intentionally slower
- rich compile/update should not exceed `2x` thin compile/update on the same local workload

### Game Engine Targets

We are less domain-proven here, so the first targets are frame-like credibility targets:

- `<= 250us` median for narrow hot state update
- `<= 500us` median for local reactive propagation wave
- `<= 2ms` median for medium mixed read/write frame-shape churn on bounded working sets

The harder requirement is structural:

- broad scene/world size must not force broad recomputation for local state changes
- derived state must remain region-local

## Gating Principles

Before any layout refactor decision:

1. hot-path truth must be isolated from rich trace publication
2. replay-reconstructable artifacts must be explicitly classified
3. merged relational-plus-signal envelopes must be certified at broader invalidation widths
4. measurement overhead must be kept out of benchmark timings

Only after those are complete do we decide whether locality/layout pressure still justifies hybrid AoSoA.

## Phase Plan

## Phase 1: Hot-Path Diagnostics Policy

### Goal

Move non-essential tracing and publication work off the synchronous hot path while preserving perfect lineage.

### Deliverables

- explicit artifact classification:
  - `must_be_hot`
  - `can_defer`
  - `reconstructable_from_replay`
- publication policy implementation for geometry and chip profiles
- profile-specific artifact caps and/or sampling rules
- certification cases proving the new policy preserves truth

### Required Work

1. classify every hot-path diagnostic artifact used by geometry and chip profiles
2. keep minimal truth summaries synchronous
3. move detailed traces, history traces, and explanatory artifacts behind deferred or replay-backed paths
4. cap artifact fanout on narrow local edits

### Success Criteria

- rich geometry hot updates on `100k`-node pseudo-realistic worlds improve by at least `3x`
- narrow rich-vs-thin overhead falls into a target band closer to `25-50%` rather than orders of magnitude
- recoverability policy tests still pass unchanged
- current status:
  - achieved in the certified matrix
  - `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_narrow_round_trip` now keeps `diagnostic_artifact_count = 30` and `detailed_trace_entries = 0`
  - `rocketship_scale_matrix/hundred_k_nodes_geometry_profile_propagation_wave` now keeps `diagnostic_artifact_count = 32` and `detailed_trace_entries = 0`
  - `geometry_artifact_decomposition_matrix/hundred_k_nodes_pseudorealistic_rich_artifact_classes` now reports `artifact_count_total = 32` and `artifact_kind_detailed_trace_count = 0`

### Exit Criteria

- geometry-rich artifact explosion is materially reduced
- artifact policy is documented and enforced
- no regression in replay parity or compiled-view parity
- status: complete enough to move to Phase 2

## Phase 2: Profile Boundary Enforcement

### Goal

Turn runtime profiles into hard operational boundaries instead of soft presets.

### Current Status

- started in code and now visible in the committed perf baseline
- profile boundary intent is now represented explicitly in the config layer
- default profile diagnostics now resolve through a single policy surface instead of being duplicated ad hoc in preset branches
- resolved configs can now assert whether they still match their intended boundary defaults
- `profile_matrix`, `workflow_matrix`, `chip_simulator_matrix`, and `rocketship_scale_matrix` now emit profile boundary metrics directly:
  - `profile_execution_lane_code`
  - `profile_diagnostics_boundary_code`
  - `profile_matches_defaults`
- the committed Phase 2 summary currently reports no profile drift against the promoted baseline
- boundary metrics are now enforced as exact-match certification signals in both the in-test baseline gate and the external baseline checker

### Deliverables

- explicit profile policy table for:
  - `CertificationCore`
  - `GeometryKernel`
  - `ChipSimulation`
  - `AiWorkflow`
- thin operational variant rules
- rich interactive/debug variant rules
- audit/replay-heavy variant rules

### Required Work

1. define per-profile defaults for diagnostics, publication, replay, and checkpoint behavior
2. make profile drift visible in perf reporting
3. prevent rich debug behavior from silently entering thin hot runs

### Success Criteria

- thin profiles become stable certification targets
- rich profiles remain certifiable without becoming the default hot loop
- profile deltas are predictable and intentional

### Exit Criteria

- profile behavior is documented
- profile-specific performance expectations are codified in the baseline and docs
- config-level drift checks exist for the named runtime profiles
- perf reporting surfaces profile drift in the domain and workflow suites, not just synthetic profile tests

## Phase 3: Runtime Bridge Scaling

### Goal

Certify realistic truth-to-reactive invalidation and recomputation envelopes for the actual kernel direction without violating crate boundaries.

### Deliverables

- broader `runtime_bridge_mock_matrix` families in `forge-relational`
- mirrored real-integration certification in the future bridge crate
- region-local invalidation workloads
- medium and broad downstream recomputation workloads
- explicit recompute-width budgets

### Required Work

1. add larger invalidation-region mock-bridge cases
2. add mixed locality mock-bridge cases
3. measure:
  - relational commit width
  - relational query width
  - downstream recompute width
  - scheduled/pruned tasks
  - history and flow diagnostics
4. define acceptable scaling for affected-region growth

### Success Criteria

- merged path remains region-local under broader invalidation
- downstream recompute scales with affected region instead of resident world size
- operational merged wave remains within target envelope for narrow and medium cases
- mixed-locality explicit-plus-traversal reads remain additive instead of exploding downstream schedules

### Exit Criteria

- merged-kernel target envelopes are documented and certified
- we understand whether the next wall is publication, recompute width, or locality

### Current Status

Phase 3 is now materially in progress:

- `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_operational`
  - median elapsed `81us`
  - `affected_bridge_sources = 3`
  - `bridge_nodes_recomputed = 10`
- `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_medium_region_operational`
  - median elapsed `187us`
  - `affected_bridge_sources = 16`
  - `bridge_nodes_recomputed = 49`
  - `bridge_tasks_scheduled = 33`
- `runtime_bridge_mock_matrix/geometry_commit_bridge_wave_mixed_locality_operational`
  - median elapsed `208us`
  - `affected_bridge_sources = 15`
  - `explicit_result_entities = 4`
  - `traversal_result_entities = 11`
  - `bridge_tasks_scheduled = 31`

The current read is that the bridge seam is still region-local under broader invalidation and mixed locality. The next bridge-specific question is whether these same proportionality bounds hold once we add broader resident worlds and later mirror the cases in the real bridge crate.

For `forge-relational` itself, Phase 3 is complete at the mock-bridge seam. The remaining integration work belongs in the future bridge crate, where these cases should be mirrored against the real runtime boundary without reintroducing a crate dependency here.

## Phase 4: Large-Scale Domain Hardening

### Goal

Harden geometry and chip realism using the full matrix rather than isolated micro-fixes.

### Deliverables

- expanded rocketship-scale geometry cases
- expanded chip stepping and event-wave cases
- first game-engine-like frame-shape workloads
- updated baseline and hotspot report

### Current Status

Phase 4 is now started. The first hotspot and endurance read is captured in [forge_relational_phase4_hotspots_and_endurance.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_relational_phase4_hotspots_and_endurance.md).

The current ranking is:

1. 100k-world geometry propagation and explicit query latency
2. 100k-world bootstrap cost
3. remaining rich-vs-thin geometry spread at large scale
4. hot geometry durability tail
5. chip checkpoint/recovery cadence
6. long-horizon replay-window endurance

### Required Work

1. geometry:
   - more subsystem propagation shapes
   - more local-vs-broad topology edit waves
2. chip:
   - larger stepping windows
   - denser fanout and adjacency churn
   - repeated rollback/recover patterns
3. game-engine-like:
   - mixed read/write frame-shape churn
   - local scene graph propagation
   - bounded regional recompute

### Success Criteria

- we have credible domain-shaped operational targets for all three domains
- hotspot ranking is based on domain realism, not only generic primitive tests

### Exit Criteria

- geometry, chip, and game-engine target families exist and are baselined

## Phase 5: Measurement Hygiene Sweep

### Goal

Eliminate remaining ambiguity about test overhead contaminating benchmark numbers.

### Deliverables

- migration of hand-rolled measurement blocks onto shared helpers
- harness self-audit retained in the suite
- measurement hygiene note in the perf docs

### Required Work

1. convert remaining hand-rolled `PerfMeasurement` timing sites onto shared helpers where practical
2. ensure metric-building and JSON emission stay out of timed paths
3. keep the harness audit case in the suite

### Success Criteria

- benchmark timings are mechanically isolated from reporting work
- future perf regressions are easier to trust immediately

### Exit Criteria

- no major benchmark family relies on ad hoc timing shape

## Phase 6: AoSoA Decision Gate

### Goal

Decide, from evidence, whether hybrid AoSoA or another chunk-local execution layout is justified.

### Questions This Gate Must Answer

1. After hot-path diagnostics cleanup, are narrow operational runs still too slow?
2. Are merged relational-plus-signal waves still showing locality-driven cost?
3. Are materialization and publication widths still dominating after policy cleanup?
4. Do domain-shaped workloads still show cache-unfriendly scan behavior?

### AoSoA Should Be Considered Justified If

- hot operational geometry and chip paths remain meaningfully outside target envelope after Phases 1-5
- costs cluster around chunk-local scan/materialization behavior rather than diagnostics/publication policy
- region-local workloads still scale too broadly relative to touched working sets

### AoSoA Should Be Deferred If

- the majority of current cost disappears after policy and profile hardening
- merged waves stay narrow and proportional
- hot operational paths move into credible target ranges without layout surgery

## Implementation Order

The intended execution order is:

1. Phase 1: hot-path diagnostics policy
   Status: completed and promoted into the committed baseline
2. Phase 2: profile boundary enforcement
3. Phase 3: merged relational-plus-signal scaling
4. Phase 4: large-scale domain hardening
5. Phase 5: measurement hygiene sweep
6. Phase 6: AoSoA decision gate

## Immediate Next Tasks

The first tasks to pick up from this spec are:

1. finish Phase 2 profile boundary enforcement across all named runtime profiles
2. extend `runtime_bridge_mock_matrix` with larger invalidation-region cases and mirror them later in the bridge crate
3. add first frame-shape game-engine perf family
4. continue measurement helper rollout through `performance_profiles.rs`
5. re-baseline merged and domain-scale hotspots after the new profile boundaries land

## Decision Standard

We should consider this pre-AoSoA hardening plan successful if:

- thin operational geometry and chip paths are credibly fast
- rich tracing remains available without owning the hot path
- merged relational-plus-signal execution stays proportional
- replay and recoverability remain first-class
- AoSoA becomes either clearly necessary or clearly deferrable

That is the point where layout work becomes an informed engineering choice instead of a fear-driven one.
