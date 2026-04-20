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
- treats storage-media durability, authenticity, or crash recovery as ambient
  platform behavior instead of a first-class verified contract
- allows multi-tenant isolation, quota boundaries, or repair actions to become
  ambiguous under failure or operator pressure

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
- Security, authenticity, backup/restore, and disaster-recovery posture must
  appear as explicit milestone scope somewhere in the roadmap rather than
  remaining implied by integrity language.
- Extensibility must never weaken authority, replay, compatibility, retention,
  or certification boundaries.
- Tenant isolation, quota enforcement, and blast-radius control must remain
  visible as first-class platform concerns even when they are enabled by
  deeper Forge graph semantics.
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

- `Milestone 1` -> `Milestone 2` -> `Milestone 3` -> `Milestone 3.5` ->
  `Milestone 3.6` -> (`Milestone 4` and `Milestone 5`) -> `Milestone 6` ->
  `Milestone 7` -> (`Milestone 8` and `Milestone 10`) -> `Milestone 11` ->
  `Milestone 12` -> `Milestone 14` -> `Milestone 15` -> `Milestone 20` ->
  `Milestone 22` -> certification

Parallel tracks:

- `Milestone 9` can overlap with late `Milestone 6` once the physical chunk
  model is honest enough for canonical chunking.
  Milestone 6 closeout now explicitly includes a three-lane layout-support
  posture (`ProofOnly`, `OnDemandMaterialized`, `PolicyEagerMaterialized`)
  with requested-vs-resolved lane evidence and explicit publication
  disposition, so overlapping Milestone 9 work may depend on the materialized
  lane without treating proof-only or policy resolution as ambient storage
  behavior.
- `Milestone 10` should be treated as concurrent with `Milestone 8`.
  Milestone 10 depends on `Milestone 4`, `Milestone 5`, and `Milestone 6` to
  make retention, compaction, and reclaim honest, while Milestone 8 depends on
  `Milestone 7` for stable-basis and durable-cursor vocabulary.
  The concurrency boundary is that Milestone 10 may publish basis-survival
  conclusions and retained-range rules, but it must not absorb live-query or
  cursor semantics.
- `Milestone 13` can start after `Milestone 10` stabilizes rebuild and
  retention rules. It may progress concurrently with `Milestone 11` once
  placement work classes, recall posture, and non-authority boundaries are
  explicit. The concurrency boundary is that Milestone 13 owns placement
  meaning, tier classes, and recall semantics, while Milestone 11 owns pacing,
  isolation, and debt-escalation policy for that work.
- `Milestone 15` can begin once replication and rebuild contracts are stable,
  but must close before advanced derived-family proliferation makes extension
  containment ambiguous.
- `Milestone 16`, `Milestone 17`, `Milestone 18`, and `Milestone 19` are late
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

## Milestone 3.5: Durable Media Semantics And Write-Path Certification

Engineering spec: [milestone-3.5-3.6.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-3.5-3.6.md)

Closeout: [milestone-3.5-3.6-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-3.5-3.6-closeout.md)

### Goal

Make backend durability semantics exact enough that acknowledged truth does not
depend on optimistic filesystem folklore.

### Adversarial Constraint

Torn writes, truncated tails, reordered persistence, incomplete rename
durability, backend-specific flush semantics, and directory-entry loss must not
allow the store to acknowledge truth it cannot later localize, reject, or
recover honestly.

### Must Ship

- explicit record framing and tail/truncation detection for durable media paths
- backend-specific durability-barrier contracts:
  - append
  - flush
  - directory entry persistence
  - rename or equivalent publication boundary
- typed torn-write, partial-write, and truncated-tail failure families
- exact acknowledgment preconditions per backend family
- machine-checkable write-path certification for embedded-file and SQLite
  families

### Must Preserve

- authoritative commit meaning remains above physical write layout
- backend variation may change mechanics, not durable acknowledgment meaning
- integrity and authenticity remain distinct concepts: valid bytes are not
  automatically trusted bytes

### Complexity / Proof Obligations

- name append-frame scan, durable flush, and startup-tail validation contracts
- expose exact counters for truncated-tail detection, torn-write detection,
  startup write-path rejection, and acknowledged writes by backend barrier class

### Allowed Debt

- backend-specific performance tuning may remain `Debt`; durability semantics
  and acknowledgment barriers may not

### Sequencing Notes

This belongs immediately after the initial WAL milestone because later crash,
snapshot, retention, and replication stories are only as honest as the media
contract beneath them.

### Parallelization Notes

This stays on the authority path. Later milestones may not claim crash exactness
without it.

## Milestone 3.6: Adversarial Crash Recovery And Recovery Source Precedence

Engineering spec: [milestone-3.5-3.6.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-3.5-3.6.md)

Closeout: [milestone-3.5-3.6-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-3.5-3.6-closeout.md)

### Goal

Promote crash recovery from "WAL restart works" to a full recovery program with
explicit source precedence, salvage boundaries, and interrupted-maintenance
recovery rules.

### Adversarial Constraint

Crashes during WAL publication, snapshot publication, compaction, reclaim,
replication capsule creation, or background maintenance must converge to one
typed recovery conclusion without reopening closed work, inventing shadow
authority, or trusting the wrong source of truth.

### Must Ship

- explicit crash-class taxonomy
- explicit recovery source precedence across:
  - canonical authoritative artifacts
  - WAL artifacts
  - snapshot families
  - compaction products
  - replication/import capsules
  - other derived durable families
- typed recovery-mode matrix:
  - automatic crash restart
  - authoritative rebuild
  - integrity-audit rebuild
  - salvage/quarantine
  - snapshot-plus-tail fast restore
  - replication/bootstrap recovery
- interrupted-maintenance recovery rules for snapshot, compaction, reclaim, and
  replication publication paths
- restart quiescence guarantees for already-closed work
- operator-visible recovery source and degraded-state reporting

### Must Preserve

- recovery remains subordinate to canonical authority
- derived families never outrank authoritative truth during recovery
- recovery decisions remain deterministic and machine-checkable

### Complexity / Proof Obligations

- name crash restart, salvage evaluation, and interrupted-maintenance recovery
  contracts
- expose exact counters for restart scans, salvage invocations, quiescent
  restarts, degraded recoveries, and source-precedence fallbacks

### Allowed Debt

- advanced operator ergonomics may remain `Debt`; recovery source precedence and
  quiescent restart semantics may not

### Sequencing Notes

This belongs before snapshots and branch-delta physical programs because the
system needs a complete recovery story before derived storage families multiply.

### Parallelization Notes

This stays on the critical path with `Milestone 3.5`.

## Milestone 4: Snapshot Persistence And Point-In-Time Restore

Engineering spec: [milestone-4.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-4.md)

Closeout: [milestone-4-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-4-closeout.md)

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

Engineering spec: [milestone-5.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-5.md)

Closeout: [milestone-5-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-5-closeout.md)

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

- none remaining in the milestone 5 closeout lane

### Sequencing Notes

This must land before aspect-aware layout, compaction, replication, and bulk
chunking.

### Parallelization Notes

Can run in parallel with `Milestone 4` after `Milestone 3`. It should also be
treated as concurrent with `Milestone 7` so long as branch-delta physical
layout remains separate from schema, lineage, cursor, and checkpoint authority
semantics.

## Milestone 6: Aspect-Aware Physical Layout And Content-Addressed Structural Blocks

Engineering spec: [milestone-6.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-6.md)

Closeout: [milestone-6-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-6-closeout.md)

### Goal

Make aspect-scoped reads and cross-branch structural dedup physically honest.

### Adversarial Constraint

Partial reads and cross-branch reuse must not secretly degrade into full-state
scans or duplicate storage of identical structural regions.

### Must Ship

- aspect-aware physical layout for admitted partial reads and CDC narrowing
- content-addressed structural block identity
- cross-branch deduplication over structural blocks
- explicit `ProofOnly` and `OnDemandMaterialized` Milestone 6 layout-support
  lanes

### Must Preserve

- aspect-aware layout does not become a second semantic schema
- structural blocks remain derived from canonical commits
- callers never get silent ambient materialization on cheap-looking read paths

### Complexity / Proof Obligations

- name aspect-read and structural-block lookup contracts
- expose exact counters for whole-state fallbacks, block reuse hits, and block
  decode breadth

### Allowed Debt

- some admitted fast paths may remain `Debt` if fallback classes are explicit
  and mechanically observable
- proof-only layout lanes may remain `Debt`; on-demand materialized lanes may
  not silently degrade

### Sequencing Notes

This belongs after delta layering because content-addressed physical reuse needs
an honest branch/delta model first.

### Parallelization Notes

Can progress alongside late `Milestone 5` once delta identity is stable.
`Milestone 9` may begin concurrently once this milestone freezes the chunk
model honestly enough for canonical bulk chunking, but Milestone 6 still owns
the physical chunk identity and non-authority contract.

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

It may also progress concurrently with `Milestone 10` once `Milestone 4`,
`Milestone 5`, and `Milestone 6` are already honest, so long as the two
milestones keep their boundaries:

- `Milestone 8` freezes durable read/sync basis meaning
- `Milestone 10` freezes retention/compaction/reclaim behavior over retained
  authority and derived families

`Milestone 10` must not redefine stable-basis or cursor-continuation meaning,
and `Milestone 8` must not absorb retention policy ownership.

## Milestone 9: Deterministic Bulk Ingest And Bulk Transform Paths

Closeout: [milestone-9-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/milestone-9-closeout.md)

Current state:

- implemented and closed through the named Milestone 9 certification lane
- deterministic planning, persisted support artifacts, durable bulk execution,
  resume surfaces, bulk-specific recovery, and machine-checkable certification
  evidence are present in `forge-store`
- the historical concurrency boundary with late Milestone 6 remains true, but
  Milestone 9 is no longer operationally incomplete

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

Can begin once `Milestone 6` stabilizes the chunk model and should be treated
as concurrent with late `Milestone 6` work so long as Milestone 9 depends only
on the stable chunk contract rather than backend-local layout internals.

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

Should be treated as concurrent with `Milestone 8` once `Milestone 7` has made
stable-basis and durable-cursor vocabulary explicit. The concurrency boundary
is that Milestone 10 owns retention, compaction, reclaim, and basis-survival
conclusions, while Milestone 8 owns live-query continuation semantics.

## Milestone 11: Background Maintenance Isolation And Scheduling Contracts

### Goal

Make compaction, rebuild, snapshotting, replication preparation, and other
maintenance programs operationally safe under foreground load.

### Adversarial Constraint

Background maintenance must not silently steal foreground latency, create
hidden starvation, or allow debt growth to become the real scheduler of truth
visibility.

### Must Ship

- explicit maintenance work classes and priorities
- bounded pacing for compaction, rebuild, snapshot, and replication-prep work
- foreground vs background isolation rules
- starvation and debt-escalation policy triggers
- operator-visible maintenance debt and scheduling state

### Must Preserve

- maintenance remains derived and policy-driven
- foreground truth mutation and foreground reads do not inherit hidden
  background cost by default

### Complexity / Proof Obligations

- name maintenance pacing, foreground isolation, and debt-escalation contracts
- expose exact counters for background queue depth, deferred work, policy
  trigger causes, and foreground work broadened by maintenance interference

### Allowed Debt

- heuristic policy tuning may remain `Debt`; bounded isolation and policy
  visibility may not

### Sequencing Notes

This belongs immediately after retention/compaction because large-scale store
correctness is not enough without operational isolation.

### Parallelization Notes

Can progress alongside late `Milestone 13` once placement work classes, tier
recall posture, and scheduler handoff boundaries are explicit.

## Milestone 12: Artifact Format Evolution And Rolling Compatibility

### Goal

Make authoritative and derived artifact families evolvable across rolling
upgrades without semantic ambiguity.

### Adversarial Constraint

Old artifacts with new code, new artifacts with old readers, mixed-version
replicas, and rolling upgrades must either remain semantically exact or fail
explicitly and typed without partial truth acceptance.

### Must Ship

- authoritative artifact format evolution contracts
- derived artifact compatibility and explicit rebuild invalidation rules
- rolling upgrade and mixed-version store/replica compatibility rules
- typed incompatibility and explicit reader rejection surfaces
- machine-checkable version-skew and compatibility reporting
- backup/restore compatibility posture and disaster-recovery version rules

### Must Preserve

- older authoritative meaning may not drift when new fields are introduced
- deserialization success may not be treated as compatibility proof
- backup and restore must preserve authoritative meaning across admitted version
  windows

### Complexity / Proof Obligations

- name compatibility-check, rolling-upgrade, and restore-version contracts
- expose exact counters for compatibility accepts, typed rejects, rebuilds
  forced by version drift, and version-skew lanes exercised

### Allowed Debt

- optional convenience migration tooling may remain `Debt`; compatibility truth
  may not

### Sequencing Notes

This belongs before replication and late artifact families because forever
systems need explicit format evolution before they spread artifacts widely.

### Parallelization Notes

Can progress in parallel with late `Milestone 11` once retention and rebuild
rules are stable.

## Milestone 13: Tiering And Durable Working-Set Intelligence

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

Can progress in parallel with `Milestone 11` once `Milestone 10` has stabilized
retention and rebuild rules. The concurrency boundary is that Milestone 13 owns
placement semantics, tier classes, and recall meaning, while Milestone 11 owns
pacing, foreground isolation, and debt-escalation policy for those placement
work units.

## Milestone 14: Replication, Capsules, And Integrity Verification

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

## Milestone 15: Extensible Durable Artifact Families And Storage Strategies

### Goal

Turn Forge Store into a durable-artifact platform by allowing new derived
storage families and storage strategies to plug in without weakening authority
or certification.

### Adversarial Constraint

An extension-defined artifact family that is stale, buggy, non-deterministic,
or over-privileged must not become accidental authority, bypass rebuild rules,
skip retention policy, or evade compatibility and certification boundaries.

### Must Ship

- extension registration for derived durable artifact families
- declared contracts per family for:
  - authority classification
  - accuracy class
  - rebuild basis
  - retention and compaction participation
  - replication/export participation
  - compatibility/versioning
  - diagnostics and certification outputs
- storage-strategy containment rules
- extension authenticity and trust-boundary rules for shipped artifacts
- machine-checkable extension-family certification and rejection surfaces

### Must Preserve

- extensions may not create authoritative truth families
- extensions may not bypass replay, rebuild, retention, or compatibility rules
- extensibility changes platform breadth, not authority shape

### Complexity / Proof Obligations

- name extension registration, rebuild, and export/retention participation
  contracts
- expose exact counters for extension-family rebuilds, typed extension-family
  rejection, stale-extension detection, and extension-caused fallback breadth

### Allowed Debt

- additional extension ergonomics may remain `Debt`; extension containment and
  declared contracts may not

### Sequencing Notes

This belongs after replication/integrity and compatibility rules are explicit,
because extensibility without those guardrails would harden drift into the
platform.

### Parallelization Notes

Can begin once `Milestone 12` and `Milestone 14` are stable.

## Milestone 16: Time-Travel Diff Acceleration And Merge-Assistance Artifacts

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

Can run in parallel with `Milestone 17`, `Milestone 18`, and `Milestone 19`.

## Milestone 17: Derived Durable Artifact Families And Accuracy Taxonomy

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

Can run in parallel with `Milestone 16`, `Milestone 18`, and `Milestone 19`
once replication and rebuild contracts are stable.

## Milestone 18: Analysis Lanes

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

Can run in parallel with `Milestone 16`, `Milestone 17`, and `Milestone 19`.

## Milestone 19: Correspondence Indexes, Structural Fingerprints, And Locality Clustering

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

Can run in parallel with `Milestone 16`, `Milestone 17`, and `Milestone 18`.

## Milestone 20: Native Blob And Object Storage

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

Can begin in parallel with late `Milestone 17` through `Milestone 19`.

## Milestone 21: Admission Control And Budget Contracts

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

Can proceed in parallel with late `Milestone 20`, but should close after major
artifact families and tiers are known.

## Milestone 22: Operator Repair, Audit, And Forensic Recovery Tooling

### Goal

Make the store operable under real corruption, recovery, and compliance
pressure by giving operators explicit audit, repair, and quarantine tools.

### Adversarial Constraint

When corruption, drift, version mismatch, tenant pressure, or damaged media are
present, the operator must be able to determine what truth is still trusted,
what can be rebuilt, what must be quarantined, and what repair action is
admissible without reading ambiguous logs or improvising on production data.

### Must Ship

- offline audit and integrity-walk surfaces
- repair-plan generation and typed repair-action contracts
- quarantine and salvage modes
- explicit trusted-truth / degraded-derived reporting
- machine-checkable forensic bundles for operator and certification use
- tenant-scoped blast-radius and quota diagnostics for repair and recovery work
- authenticity-aware audit surfaces in addition to raw integrity reporting

### Must Preserve

- repair tooling may not mutate authority implicitly
- operator actions must remain auditable artifacts
- tenant isolation and quota boundaries remain visible during repair and
  recovery

### Complexity / Proof Obligations

- name audit-walk, repair-plan, and quarantine contracts
- expose exact counters for audited artifacts, proposed repairs, quarantined
  families, tenant-scoped repair actions, and operator-visible degraded states

### Allowed Debt

- operator UX polish may remain `Debt`; typed audit and repair contracts may not

### Sequencing Notes

This belongs before final certification because trust-grade systems need
operator-grade recovery and forensic truth, not just internal correctness.

### Parallelization Notes

Can progress alongside late `Milestone 21`, but must close before certification.

## Milestone 23: Generic Store Certification Program

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

## Milestone 24: Domain Store Certification Program

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
- media durability, crash recovery, repair/audit, compatibility, and extension
  containment are all explicit, tested, and operator-visible
- tenant isolation, quota boundaries, authenticity checks, backup/restore, and
  disaster-recovery posture are explicit platform contracts rather than
  implied side effects of lower-level integrity work

## Companion Documents

- [forge_store_vision.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_vision.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements.md)
- [forge_runtime_bridge_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-runtime-bridge/forge_runtime_bridge_roadmap.md)
- [forge_relational_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_roadmap.md)
- [MENTALITY.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/MENTALITY.md)
- [architectural_guidelines.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/architectural_guidelines.md)
- [domain_standards.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/domain_standards.md)
- [performance_guidelines.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/coding_guidelines/performance_guidelines.md)
