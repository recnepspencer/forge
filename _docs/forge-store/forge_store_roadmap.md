# Forge Store Future Roadmap

## Purpose

This document defines the future work for `forge-store`.

It is a future-only roadmap. It exists to sequence the work required to make
the Forge runtime durable without weakening authority boundaries, replay,
branch semantics, lineage semantics, live-query semantics, or recovery honesty.

The operating rule is:

`persist canonical authority once, parallelize derived storage work around it`

## Global Adversarial Constraint

`forge-store` must survive this hostile condition:

> A long-lived system with deep branch history, active mutation, crash-restart
> loops, retention pressure, schema evolution, lineage-bearing truth,
> resumable subscribers, live-query consumers, bulk ingest pressure, and
> multiple physical backends must recover the same canonical truth, replay the
> same observable history, and explain the same durable conclusions regardless
> of whether it restored from WAL, snapshots, delta-layer materializations, or
> rebuilt derived storage from canonical artifacts.

If any supported path:

- makes physical layout authoritative
- makes snapshots, materializations, live-query bases, or derived artifacts
  unrebuildable from canonical truth
- makes branch storage scale with copied full state by default
- loses schema, lineage, cursor, or basis meaning across restart
- allows backend variation to change recovery or replay conclusions
- lets compaction, tiering, or reclamation silently destroy truth retention the
  policy said must survive
- lets approximate or advisory derived artifacts masquerade as exact durable
  truth

then the store has failed.

## Roadmap Rules

- Each milestone must describe a real store capability boundary, not a bag of
  chores.
- `forge-relational` owns truth semantics; `forge-store` owns durable survival.
- Every durable artifact must be classified as authoritative, derived durable,
  or ephemeral.
- Every derived durable artifact family must declare an accuracy class:
  `Exact`, `Conservative`, `Approximate`, `Heuristic`, or `Advisory`.
- Sequence numbers express dependency order, not staffing order.
- Every milestone must say what can run in parallel and what stays on the
  critical path.
- Every milestone must declare its own adversarial constraint.
- Every hot-path milestone must declare named complexity contracts and exact
  counter proof obligations.
- Any knowingly incomplete first ship must be marked as explicit debt rather
  than implied completeness.
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements.md)
  is the authoritative milestone-closeout test document; roadmap milestones are
  not closed until their required named suites pass.

## Operating Modes

The roadmap preserves all three operating modes explicitly:

- `Durable mode`: store owns an internal runtime instance and crash-safe commit
  acknowledgment.
- `Embedded mode`: an external runtime owns semantics; store persists commit
  envelopes, checkpoints, and related artifacts without taking lifecycle
  authority.
- `Absent mode`: the runtime runs with no store at all; store integration
  remains optional rather than ambient.

## Critical Path And Parallel Tracks

Critical path:

- `Milestone 1` -> `Milestone 2` -> `Milestone 3` -> (`Milestone 4` and
  `Milestone 5`) -> `Milestone 6` -> (`Milestone 7` and `Milestone 8`) ->
  `Milestone 10` -> `Milestone 12` -> `Milestone 17` -> certification

Parallel tracks:

- `Milestone 9` can overlap with late `Milestone 6` once the physical chunk
  model is honest enough for canonical chunking.
- `Milestone 11` can start after `Milestone 10` stabilizes rebuild and
  retention rules.
- `Milestone 13`, `Milestone 14`, `Milestone 15`, and `Milestone 16` are late
  platform programs and can progress in parallel once replication, integrity,
  and rebuild contracts are stable.

## Milestone 1: Canonical Commit Persistence And Artifact Authority

Engineering spec: [milestone-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-1.md)

Closeout: [milestone-1-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-1-closeout.md)

### Goal

Make canonical durable truth explicit before any optimization or physical
layout is allowed to exist.

### Adversarial Constraint

Two backends, one original store, and one rebuild path from canonical artifacts
must all converge to the same commit history, branch heads, and replay truth.

### Must Ship

- one `forge-store` facade
- canonical commit-envelope append and fetch
- explicit durable artifact classification
- persistence of canonical commit envelopes, version DAG records, branch heads,
  and ordered parent metadata
- backend abstraction with a production-grade embedded backend baseline
- canonical digest and identity surfaces for authoritative artifacts

### Must Preserve

- runtime semantics stay owned by `forge-relational`
- commit envelopes remain the only semantic durability authority
- backend variation does not change authoritative artifact meaning

### Complexity / Proof Obligations

- name the append and fetch complexity contracts
- expose exact counters for envelope append count, parent-record writes, and
  authoritative artifact reads
- prove backend parity by exact artifact-digest comparison

### Allowed Debt

- backend-specific fast paths may ship as `Debt` if authoritative behavior is
  already parity-proven through the baseline path

### Sequencing Notes

This belongs first because every later milestone depends on one authoritative
durable artifact model.

### Parallelization Notes

Once authoritative artifact identity is frozen, `Milestone 7` can begin in
parallel.

## Milestone 2: Operating Modes And Lifecycle Contracts

Engineering spec: [milestone-2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-2.md)

Closeout: [milestone-2-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-2-closeout.md)

### Goal

Make durable mode, embedded mode, and absent mode explicit architectural
contracts rather than implied deployment conventions.

### Adversarial Constraint

The same canonical truth committed in durable mode and embedded mode must
persist with the same artifact meaning even though runtime lifecycle ownership
differs.

### Must Ship

- explicit surfaces for durable, embedded, and absent mode
- durable-mode runtime lifecycle boundary
- embedded-mode commit-envelope and checkpoint reception contracts
- typed lifecycle errors for cross-mode misuse

### Must Preserve

- embedded mode does not force durable-mode orchestration
- durable mode does not redefine runtime semantics
- absent mode remains first-class valid

### Complexity / Proof Obligations

- construction and mode-selection paths must expose exact mode-choice counters
- prove no ambient store dependency exists in absent mode

### Allowed Debt

- none on mode semantics; mode ambiguity is structural, not polish

### Sequencing Notes

This belongs before WAL and snapshots because later proofs differ by lifecycle
ownership.

### Parallelization Notes

No major parallel dependency beyond enabling later mode-specific work honestly.

## Milestone 3: WAL-Coordinated Durable Mode And Crash Recovery

Engineering spec: [milestone-3.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-3.md)

Closeout: [milestone-3-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-3-closeout.md)

### Goal

Make durable mode real: every acknowledged durable-mode commit survives process
failure and recovers to the same committed truth.

### Adversarial Constraint

A crash at any point around the durable commit boundary must not duplicate,
lose, or partially publish acknowledged truth.

### Must Ship

- store-owned runtime lifecycle for durable mode
- append-only WAL
- "log before acknowledge" durability contract
- crash recovery from WAL plus canonical commit artifacts
- typed recovery modes for crash recovery and full rebuild
- typed recovery diagnostics

### Must Preserve

- runtime owns transaction semantics and commit legality
- recovery conclusions derive from canonical commit truth, not WAL alone
- no partial transaction truth publishes after crash

### Complexity / Proof Obligations

- name WAL append, recovery replay, and recovery scan complexity contracts
- expose exact counters for appended WAL entries, replayed WAL entries, and
  acknowledged commit recovery count

### Allowed Debt

- recovery acceleration can remain `Debt`; recovery correctness cannot

### Sequencing Notes

This is the first hard authority-path milestone after lifecycle boundaries are
locked.

### Parallelization Notes

`Milestone 4` and `Milestone 5` start after this. `Milestone 7` may integrate
transactionally-coupled artifacts after this.

## Milestone 4: Snapshot Persistence And Point-In-Time Restore

### Goal

Make immutable snapshots and snapshot-plus-tail restore first-class derived
durable artifacts.

### Adversarial Constraint

Any point-in-time restore path must converge to the same truth as replay from
canonical commits alone, even after snapshot deletion and rebuild.

### Must Ship

- immutable persisted full snapshots
- point-in-time snapshot reads
- snapshot-plus-tail restore
- basis and identity records tying each snapshot to exact commit ranges

### Must Preserve

- snapshots are rebuildable from authoritative artifacts
- restore cannot bypass canonical replay rules

### Complexity / Proof Obligations

- name snapshot capture, snapshot load, and tail-replay contracts
- expose exact counters for snapshot bytes/materialized records and tail replay
  breadth

### Allowed Debt

- snapshot creation profitability heuristics may remain `Debt`

### Sequencing Notes

This must precede multi-resolution materialization and retention because they
depend on a clean snapshot basis.

### Parallelization Notes

Can run in parallel with `Milestone 5` after `Milestone 3`.

## Milestone 5: Structural Delta Storage And Branch Delta Layering

### Goal

Make branch persistence scale with semantic delta instead of copied full state.

### Adversarial Constraint

Branch creation and branch-local storage growth must remain proportional to
change, not baseline state size, even under deep branch trees.

### Must Ship

- structural delta storage
- near-free branch creation from shared bases
- explicit branch-local delta layering
- deterministic rewrite rules for delta stacks
- read-amplification counters and typed stack-management policies

### Must Preserve

- branch persistence does not redefine branch semantics
- deltas remain derived from canonical commits

### Complexity / Proof Obligations

- name branch-create, delta-read, and delta-rewrite contracts
- expose exact counters for delta layers traversed, branch-base reuse, and
  rewrite breadth

### Allowed Debt

- advanced rewrite profitability may remain `Debt` if read-amplification truth
  is already visible and tested

### Sequencing Notes

This must land before aspect-aware layout, compaction, replication, and bulk
chunking.

### Parallelization Notes

Can run in parallel with `Milestone 4` after `Milestone 3`.

## Milestone 6: Aspect-Aware Physical Layout And Content-Addressed Structural Blocks

### Goal

Make aspect-scoped reads and cross-branch structural dedup physically honest.

### Adversarial Constraint

Partial reads and cross-branch reuse must not secretly degrade into full-state
scans or duplicate storage of identical structural regions.

### Must Ship

- aspect-aware physical layout for admitted partial reads and CDC narrowing
- content-addressed structural block identity
- cross-branch deduplication over structural blocks

### Must Preserve

- aspect-aware layout does not become a second semantic schema
- structural blocks remain derived from canonical commits

### Complexity / Proof Obligations

- name aspect-read and structural-block lookup contracts
- expose exact counters for whole-state fallbacks, block reuse hits, and block
  decode breadth

### Allowed Debt

- some admitted fast paths may remain `Debt` if fallback classes are explicit
  and mechanically observable

### Sequencing Notes

This belongs after delta layering because content-addressed physical reuse needs
an honest branch/delta model first.

### Parallelization Notes

Can progress alongside late `Milestone 5` once delta identity is stable.

## Milestone 7: Durable Schema, Lineage, Cursor, And Checkpoint Artifacts

### Goal

Persist the non-commit authoritative artifacts needed for replay, identity
resolution, resume, and embedded-mode checkpoint use.

### Adversarial Constraint

Restart must not erase schema-boundary meaning, lineage continuity, or cursor
position truth required to continue from stored state honestly.

### Must Ship

- schema evolution boundary persistence
- lineage event persistence and historical identity resolution support
- durable CDC cursor and subscriber checkpoint persistence
- transactionally coherent cursor advancement
- embedded-mode checkpoint artifact persistence

### Must Preserve

- schema and lineage semantics remain owned by the runtime
- cursor meaning remains above store

### Complexity / Proof Obligations

- name cursor resume and lineage-lookup contracts
- expose exact counters for cursor resume steps, lineage-resolution breadth, and
  checkpoint artifact reads

### Allowed Debt

- cursor acceleration surfaces may remain `Debt`; cursor truth cannot

### Sequencing Notes

This belongs before live-query substrate because live query depends on durable
cursor and basis truth already existing.

### Parallelization Notes

Can begin once `Milestone 1` and `Milestone 2` freeze boundaries; durable-flow
integration lands after `Milestone 3`.

## Milestone 8: Live-Query Substrate And Durable Sync Basis

### Goal

Make "read current truth and stay synced" a real store capability rather than a
purely upper-layer convenience.

### Adversarial Constraint

A client that reads from a stable basis and advances by durable cursor must
converge to the same truth as a fresh full read, regardless of restart point or
fetch width.

### Must Ship

- basis-pinned live-query surfaces:
  - stable basis read
  - durable cursor continuation
  - explicit basis mismatch detection
- storage-visible CDC narrowing support for admitted common shapes

### Must Preserve

- store accelerates declared narrowing bases; it does not invent new change
  meaning
- live-query substrate does not become a second query runtime

### Complexity / Proof Obligations

- name stable-basis read and basis-to-cursor continuation contracts
- expose exact counters for continuation batches, narrowed items, and fallback
  broadening

### Allowed Debt

- narrowing support may begin as partial admitted fast paths with explicit
  `Debt` markers for unsupported shapes

### Sequencing Notes

This belongs after durable cursor and basis artifacts because otherwise the
store would fake live-query semantics with ambient conventions.

### Parallelization Notes

Can run in parallel with late `Milestone 6`.

## Milestone 9: Deterministic Bulk Ingest And Bulk Transform Paths

### Goal

Make large imports, migrations, and rewrites first-class store programs rather
than ad hoc utility flows.

### Adversarial Constraint

Interrupted bulk programs must resume to the same final canonical truth, with
bounded memory and no alternate commit model.

### Must Ship

- deterministic bulk-ingest path
- deterministic bulk-transform path
- resumable progress checkpoints
- bounded-memory chunking and chunk diagnostics
- canonical commit parity between bulk and ordinary transaction paths
- WAL-safe bulk execution in durable mode

### Must Preserve

- bulk paths do not invent a second commit format
- resumability does not weaken replay or recovery guarantees

### Complexity / Proof Obligations

- name chunking, resume, and bounded-memory contracts
- expose exact counters for chunk width, resumed checkpoints, and peak in-flight
  memory units

### Allowed Debt

- bulk scheduling profitability may remain `Debt`; chunk determinism cannot

### Sequencing Notes

This belongs after the physical chunk model is honest enough to support
canonical chunking.

### Parallelization Notes

Can begin once `Milestone 6` stabilizes the chunk model.

## Milestone 10: Retention, Compaction, And Reclamation

### Goal

Make retention policy physically real without letting compaction or reclaim
silently erase durable truth that policy said must survive.

### Adversarial Constraint

Retention pressure must not destroy replayable authority, and compaction must
not create a second source of truth.

### Must Ship

- explicit retention policies over branches, history depth, and artifact classes
- compaction products as derived durable artifacts
- reclaim tied to retention policy rather than pressure alone
- typed compaction and reclaim diagnostics
- rebuild debt and compaction debt counters

### Must Preserve

- retained authoritative truth remains replayable
- compaction never becomes an authority source

### Complexity / Proof Obligations

- name compaction breadth, reclaim breadth, and rebuild-debt contracts
- expose exact counters for rewritten blocks/layers, reclaimed artifacts, and
  retained authoritative ranges

### Allowed Debt

- aggressive compaction policies may remain `Debt` if conservative policy paths
  are already verified

### Sequencing Notes

This belongs before tiering, replication, blob retention, and most late derived
artifact programs.

### Parallelization Notes

Depends on `Milestone 4`, `Milestone 5`, and `Milestone 6`.

## Milestone 11: Tiering And Durable Working-Set Intelligence

### Goal

Add hot/warm/cold placement and working-set adaptation without turning
placement policy into semantic truth.

### Adversarial Constraint

Artifact movement across tiers and working-set adaptation must not change
replay, restore, or visible logical branch meaning.

### Must Ship

- hot/warm/cold branch or artifact tiering
- durable working-set intelligence for hot branches, repeated materialization
  bases, and hot regions

### Must Preserve

- tiering and working-set decisions remain advisory/derived
- no hidden eviction of authoritative truth

### Complexity / Proof Obligations

- name tier-move and working-set classification contracts
- expose exact counters for tier moves, hotness reclassification, and tier
  misses

### Allowed Debt

- adaptive heuristics may remain `Debt` if placement effects are already
  explicit and bounded

### Sequencing Notes

This belongs after compaction because placement policy must build on stable
retention and rebuild rules.

### Parallelization Notes

Can run in parallel with late `Milestone 10`.

## Milestone 12: Replication, Capsules, And Integrity Verification

### Goal

Make canonical artifacts shippable, verifiable, and replayable across machines
without inventing a second truth format.

### Adversarial Constraint

Import/export and replication paths must reconstruct the same canonical truth
and artifact identities as the original store, even for partial scopes.

### Must Ship

- immutable artifact publishing for replication
- deterministic import/export capsules
- snapshot-plus-tail replication
- partial branch and bounded artifact-range replication
- cross-artifact digest graph or equivalent integrity surface
- integrity-audit rebuild mode

### Must Preserve

- replication ships canonical artifact meaning, not backend-local layout
- partial replication stays explicit about what it includes and excludes

### Complexity / Proof Obligations

- name capsule-build, replication-apply, and integrity-audit contracts
- expose exact counters for shipped artifacts, verified digests, and partial
  scope omissions

### Allowed Debt

- replication acceleration paths may remain `Debt`; replication parity cannot

### Sequencing Notes

This belongs after retention and rebuild rules are stable.

### Parallelization Notes

Depends on `Milestone 10` and `Milestone 8`.

## Milestone 13: Time-Travel Diff Acceleration And Merge-Assistance Artifacts

### Goal

Persist durable help for historical diff and merge-heavy workflows without
turning assistance artifacts into merge authority.

### Adversarial Constraint

Diff acceleration and merge-assistance artifacts must speed up historical and
merge workflows without changing canonical diff or merge conclusions.

### Must Ship

- durable time-travel diff acceleration artifacts
- durable merge-assistance artifacts such as prior reconciliation or
  correspondence support records where admitted

### Must Preserve

- these artifacts remain derived durable artifacts
- merge authority remains outside store

### Complexity / Proof Obligations

- name diff-acceleration lookup and assistance-artifact rebuild contracts
- expose exact counters for acceleration hits, misses, and fallback full-diff
  breadth

### Allowed Debt

- some acceleration families may remain `Debt` if their fallback full path is
  explicit and parity-proven

### Sequencing Notes

This belongs after replication/integrity because export, replay, and rebuild
need to know how these artifacts travel and rehydrate.

### Parallelization Notes

Can run in parallel with `Milestone 14`, `Milestone 15`, and `Milestone 16`.

## Milestone 14: Derived Durable Artifact Families And Accuracy Taxonomy

### Goal

Make derived durable artifact families explicit and enforce the full accuracy
taxonomy across them.

### Adversarial Constraint

A derived artifact with weak guarantees must never be consumed as though it had
exact truth-grade guarantees.

### Must Ship

- explicit rebuild rules for every derived artifact family added here
- the full enforced accuracy taxonomy:
  - `Exact`
  - `Conservative`
  - `Approximate`
  - `Heuristic`
  - `Advisory`
- diagnostics and counters for rebuild debt, basis drift, and accuracy class

### Must Preserve

- derived artifacts never become authoritative truth
- no artifact may claim a stronger accuracy class than its rebuild basis proves

### Complexity / Proof Obligations

- name rebuild and drift-detection contracts for each admitted derived family
- expose exact counters for stale detections, rebuilds, and class-specific
  fallback usage

### Allowed Debt

- additional derived families may remain absent; shipped families may not remain
  unclassified

### Sequencing Notes

This belongs before analysis lanes and locality clustering so later programs
inherit an already-honest accuracy model.

### Parallelization Notes

Can run in parallel with `Milestone 13`, `Milestone 15`, and `Milestone 16`
once replication and rebuild contracts are stable.

## Milestone 15: Analysis Lanes

### Goal

Add basis-anchored analysis checkpoint lanes distinct from authoritative truth.

### Adversarial Constraint

Analysis checkpoints must survive interruption and resume against the exact
truth basis they claim, without becoming shadow truth.

### Must Ship

- basis-anchored cached analysis artifacts
- analysis checkpoint lanes distinct from authoritative truth

### Must Preserve

- analysis artifacts never become authoritative truth
- basis drift is explicit rather than hidden

### Complexity / Proof Obligations

- name basis-check and checkpoint-resume contracts
- expose exact counters for basis matches, basis drifts, and resumed analysis
  artifacts

### Allowed Debt

- advanced analysis families may remain `Debt`; basis-pinning truth may not

### Sequencing Notes

This belongs after the accuracy taxonomy is enforced so analysis outputs cannot
pretend to be more trustworthy than they are.

### Parallelization Notes

Can run in parallel with `Milestone 13`, `Milestone 14`, and `Milestone 16`.

## Milestone 16: Correspondence Indexes, Structural Fingerprints, And Locality Clustering

### Goal

Add durable correspondence/fingerprint infrastructure and region-aware locality
clustering as honest derived storage programs.

### Adversarial Constraint

Correspondence indexes and locality-aware placement must improve lookup and
placement without changing lineage, diff, replay, or restore truth.

### Must Ship

- persistent correspondence indexes
- structural fingerprint tables
- region-aware locality clustering or equivalent locality-aware placement
  substrate

### Must Preserve

- correspondence indexes do not redefine identity or lineage authority
- locality clustering changes placement, not semantics

### Complexity / Proof Obligations

- name fingerprint lookup, correspondence lookup, and locality-placement
  contracts
- expose exact counters for locality hits, correspondence hits, and fallback
  broad scans

### Allowed Debt

- adaptive locality heuristics may remain `Debt` if placement truth and fallback
  breadth are explicit

### Sequencing Notes

This belongs after accuracy taxonomy so correspondence and locality artifacts
ship with explicit trust classes.

### Parallelization Notes

Can run in parallel with `Milestone 13`, `Milestone 14`, and `Milestone 15`.

## Milestone 17: Native Blob And Object Storage

### Goal

Make content-addressed blob/object storage a native store capability without
splitting the system into "truth store plus external file server."

### Adversarial Constraint

Blob retention, replication, and tiering must preserve blob identity and live
references without creating a second retention or replication system.

### Must Ship

- content-addressed blob/object storage
- tiered blob placement: inline, external, cold
- typed blob references from entities, commits, and branches
- blob retention and reclaim integrated with the store retention model
- blob replication and capsule export/import integration
- authoritative-versus-derived blob classification

### Must Preserve

- blob references remain explicit typed artifacts
- blob storage does not create a second replication or retention system

### Complexity / Proof Obligations

- name blob fetch, blob tier move, and blob dedup contracts
- expose exact counters for blob dedup hits, tier reads, and orphan-reclaim
  breadth

### Allowed Debt

- optional blob-serving fast paths may remain `Debt`; blob identity parity may
  not

### Sequencing Notes

This belongs after retention and replication semantics are stable.

### Parallelization Notes

Can begin in parallel with late `Milestone 14` through `Milestone 16`.

## Milestone 18: Admission Control And Budget Contracts

### Goal

Make store growth, debt, and resource risk explicit enough that the system can
fail honestly before it silently degrades.

### Adversarial Constraint

Unbounded branch depth, history growth, snapshot density, derived artifact
footprint, WAL growth, and blob growth must trigger explicit policy outcomes
before they become silent correctness or performance failures.

### Must Ship

- budget contracts for at least:
  - branch depth and count
  - retained history depth
  - snapshot/materialization density
  - derived durable artifact footprint
  - compaction debt
  - rebuild debt
  - WAL growth
  - blob footprint by tier
- typed admission-control surfaces and rejection diagnostics
- budget visibility surfaces for operators and certification
- policy hooks for archive, compact, defer, deny, or explicit degradation

### Must Preserve

- admission control does not redefine truth semantics
- no hidden eviction of authoritative truth

### Complexity / Proof Obligations

- name budget-check and admission-decision contracts
- expose exact counters for rejections, deferrals, degradation choices, and
  policy-trigger causes

### Allowed Debt

- threshold tuning may remain `Debt`; silent overrun may not

### Sequencing Notes

This belongs late because budget contracts must know the real artifact families
and tiers they govern.

### Parallelization Notes

Can proceed in parallel with late `Milestone 17`, but should close after major
artifact families and tiers are known.

## Milestone 19: Generic Store Certification Program

### Goal

Prove the completed store under generic hostile durability scenarios rather
than only by milestone-local evidence.

### Adversarial Constraint

Replay equivalence must hold across every recovery mode, rebuild mode, export
mode, and admitted fast path the store claims to support.

### Acceptance Evidence

- crash recovery parity
- full rebuild parity
- snapshot-plus-tail restore parity
- compaction/reclaim/tiering parity
- backend parity
- replication/import/export parity
- cursor resume and live-query basis parity
- schema/lineage durability parity
- authoritative-versus-derived artifact rebuild distinction
- bulk ingest/transform parity
- budget/admission-control honesty
- replay-equivalence across recovery modes, not just within each mode

Each certification run must emit machine-checkable artifact bundles.

## Milestone 20: Domain Store Certification Program

### Goal

Prove that the generic store is actually fit for the domains the Forge stack is
meant to serve.

### Acceptance Evidence

- geometry/CAD session durability, branch persistence, region materialization,
  and analysis-basis reuse
- web/data crash recovery, CDC resume, and live-query continuation
- AI branch/workspace persistence, basis-pinned analysis reuse, and historical
  diff support
- chip/history durability with snapshot-safe analysis restore and locality-aware
  materialization

## Completion Standard

`forge-store` is roadmap-complete only when:

- canonical durable authority is established and replay-safe
- durable mode is crash-safe
- embedded mode is a real persisted-artifact contract rather than a side note
- snapshots, materializations, deltas, aspect-aware layouts, structural blocks,
  compaction, schema, lineage, cursors, replication, and budget controls are
  all honest about what is authoritative versus derived
- live-query substrate, diff acceleration, merge assistance, analysis lanes,
  locality clustering, and native blob storage remain rebuildable from their
  declared authority basis
- derived artifacts carry enforced accuracy classifications strong enough for
  downstream trust decisions
- all required named suites in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements.md)
  pass with machine-checkable evidence
- beta readiness additionally requires all cross-cutting beta suites in
  [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements.md)
  to pass with machine-checkable evidence
- generic and domain certification both pass with machine-checkable evidence

## Companion Documents

- [forge_store_vision.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_vision.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements.md)
- [forge_runtime_bridge_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md)
- [forge_relational_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_roadmap.md)
- [MENTALITY.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/MENTALITY.md)
- [architectural_guidelines.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/architectural_guidelines.md)
- [domain_standards.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/domain_standards.md)
- [performance_guidelines.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/performance_guidelines.md)
