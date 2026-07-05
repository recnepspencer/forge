# Forge Store Roadmap 2: Physical Database Foundation

## Purpose

This document extends [forge_store_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_roadmap.md).

The first roadmap established the semantic durability program for
`forge-store`: canonical authority, WAL recovery, snapshots, branch deltas,
retention, tiering, subscription-support durability, replication, extensibility,
blobs, budgets, repair, and certification.

This second roadmap corrects the missing foundation beneath that program:
`forge-store` must become a real physical database engine, not a heap-shaped
persistence harness with strong semantic evidence around it.

This roadmap is the build plan for the Forge Store physical foundation. The
planned implementation surface is the dedicated Forge Store workspace and crate
family. Roadmap 2 does not rely on any prior Store topology, persistence path,
or compatibility lane as an architectural premise.

This is a side-step program. It belongs after the active `13.x`
subscription-support cleanup arc closes and before the roadmap proceeds into
`Milestone 14`, `Milestone 15`, native blob expansion, replication closure, and
platform certification.

## Roadmap Position

Current critical path in the first roadmap says:

`Milestone 13` -> `Milestone 13.1` -> `Milestone 13.2` ->
`Milestone 13.3` -> `Milestone 14`

Roadmap 2 inserts this required aspect-native and physical foundation gate:

`Milestone 13.3` -> `Aspect-Native Workspace Gate` -> `S.0` -> `S.1`
-> `S.2` -> `S.3` -> `S.4` -> `S.4.5` -> `S.5` -> `S.5.1`
-> `S.6` -> `S.7` -> `S.8` -> `S.9` -> `S.10` -> `S.11`
-> `S.12` -> `Milestone 14`

The `S.*` numbers are storage-foundation sequence numbers, not ordinary feature
milestones. They express dependency order for the physical database substrate.

The aspect-native gate is not an `S.*` storage sequence because it is a
workspace truth-shape prerequisite. It closes before `S.0` so every source
boundary, evidence artifact, handoff, digest basis, and certification row in
the physical roadmap starts from native Foundational aspect material instead
of JSON-shaped payloads.

## Global Adversarial Constraint

`forge-store` must survive this hostile physical condition:

> A store larger than memory, carrying authoritative commits, branch deltas,
> snapshots, subscription-support artifacts, indexes, derived artifacts, and
> large blobs, while under foreground read/write load, background compaction,
> checkpointing, scrub, replication preparation, power-loss crashes, torn
> writes, media corruption, version skew, tenant pressure, and operator repair,
> must preserve canonical truth, bounded memory, bounded recovery, localized
> corruption, explicit physical stability, and machine-checkable diagnostics
> without relying on full-store heap materialization, serde-loaded domain
> objects, OS writeback folklore, or backend-private residue guessing.

If any supported path:

- reads or writes by loading the whole store into heap domain structures
- treats artifact digests as substitutes for per-page or per-frame integrity
- lets logical maintenance scheduling stand in for physical I/O isolation
- makes blob storage a late sidecar rather than native physical storage
- treats key scope, tenant scope, authenticity class, or custody posture as
  deployment folklore instead of typed physical metadata
- allows compaction, checkpointing, reclaim, or repair to observe unstable
  bytes
- recovers by trusting backend residue instead of framed, checksummed,
  versioned, and replayable physical records
- cannot state memory, recovery, allocation, read-amplification,
  write-amplification, and foreground-interference bounds
- claims financial, aerospace, or platform-grade posture without formal,
  operational, and certification evidence

then the store has not earned the database claim.

## Roadmap Rules

- Roadmap 2 is about physical database architecture, not new semantic features.
- Roadmap 2 is greenfield for the physical Store foundation. Planned code lands
  in the dedicated Store workspace/crate family.
- The dedicated Store workspace must close the
  [Aspect-Native Workspace Gate](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-aspect-native-gate.md)
  before `S.0` implementation proceeds. JSON may exist only as an explicitly
  named terminal projection or hostile/readmission test input.
- No compatibility backend, prior persistence path, historical topology, or
  external semantic harness is a Roadmap 2 foundation unless a later sequence
  introduces it as new typed work with explicit authority.
- `forge-foundational` supplies shared platform vocabulary, id categories,
  proof-bearing construction expectations, and boundary language where those
  concepts are cross-platform rather than Store-specific.
- `forge-proof` supplies shared proof progression, receipt, evidence, suite,
  and certification vocabulary where Store needs to report what was proven.
  Store still owns physical durability, media layout, corruption localization,
  recovery physics, and byte survival.
- `forge-relational` continues to own semantic MVCC, transaction meaning,
  snapshot visibility, branch truth, and identity semantics.
- `forge-store` owns physical byte survival: pages, segments, frames, buffer
  pool residency, WAL/checkpoint physics, physical read stability, corruption
  localization, I/O pacing, blob chunks, backup, repair, and audit evidence.
- Physical layout may optimize access, but it may never become semantic
  authority.
- No storage-foundation sequence may close on functionality alone. It must close
  on correctness, boundedness, latency isolation, corruption behavior,
  operational diagnosis, and certification evidence for its declared operating
  envelope.
- Every storage-foundation sequence must expose exact counters for the work it
  claims to bound.
- Every sequence must name the hardware/backend assumptions under which its
  durability and performance claims are valid.

## Governing Read

- `MENTALITY.md`
  protects hard-problem-first architecture. Roadmap 2 exists because the
  physical database substrate must be built before more replication, blob,
  extension, and certification work compounds around a weak foundation.
- `arch_laws.md`
  protects proof-bearing boundaries. Roadmap 2 separates semantic authority
  from physical byte authority and requires typed physical proof before logical
  decode, recovery, or repair.
- `perf_laws.md`
  protects visible, testable cost. Roadmap 2 requires resident-memory,
  allocation, I/O, read-amplification, write-amplification, and interference
  counters instead of throughput folklore.
- `domain_structure_laws.md`
  protects decomposition by responsibility. Roadmap 2 splits pages, buffer
  pool, integrity, recovery, physical isolation, I/O, blobs, layout, formal
  models, operations, security, and certification into separate storage
  programs because they fail and evolve differently.
- `forge_foundational_roadmap.md`
  protects shared Forge vocabulary from being reinvented locally. Roadmap 2
  uses Foundational terms for cross-platform ids, classifications, construction
  guarantees, and public contract language, while keeping Store-specific byte
  survival concepts inside Store.
- `forge_proof_roadmap.md`
  protects shared proof progression, evidence, receipt, suite, and
  certification vocabulary. Roadmap 2 uses Proof where Store reports proof, but
  does not move Store-owned media, recovery, corruption, or durability authority
  into Proof.
- `forge_store_vision.md`
  says Store makes truth survive. Roadmap 2 clarifies that survival includes
  physical media, bounded memory, native blobs, corruption localization, and
  operability, not only semantic replay.
- `forge_store_roadmap.md`
  remains the semantic durability roadmap. Roadmap 2 extends it by adding the
  missing physical database foundation before later platform milestones.
- `test-requirements.md`
  remains the certification baseline. Roadmap 2 requires new physical
  certification suites to be added before the store can claim beta,
  financial-platform, or aerospace-grade readiness.

## Aspect-Native Workspace Gate

Engineering spec: [storage-foundation-aspect-native-gate.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-aspect-native-gate.md)

### Goal

Make the dedicated Forge Store workspace aspect-native before `S.0` and all
physical database foundation work.

### Boundary

This is not page, WAL, blob, or recovery implementation. This is the
truth-shape gate that ensures Store workspace facts are native Foundational
aspect material and Store-owned physical witnesses, with JSON confined to
explicit terminal projection or hostile/readmission inputs.

### Adversarial Constraint

No Store workspace authority, evidence, digest basis, handoff, recovery input,
certification row, or ordinary test fixture may require or accept JSON-shaped
state, `serde_json::Value`, arbitrary serde serialization, raw string identity,
or terminal projection text as semantic authority.

### Must Ship

- current JSON/serde residue inventory for old and new Store surfaces
- native Store boundary vocabulary over Foundational `AspectValue`,
  `StructAspectValue`, `AspectKey`, validated aspect values, authoritative
  aspect state, authoritative patches, masks, locators, canonical basis,
  receipts, diagnostics, and performance evidence
- Store-owned physical witness wrappers where byte-survival authority is local
  to Store
- terminal projection and terminal JSON projection quarantine
- explicit JSON ingress readmission lane that lowers to native aspects before
  authority consumption
- native canonical basis and digest path with no JSON serialization basis
- compile-fail and scan proofs for JSON residue, generic serde authority,
  terminal projection re-entry, raw string identity, and neutral string
  accessors
- S0 handoff artifact that is typed/native and cannot be satisfied by JSON,
  logs, markdown, or terminal projections

### Must Preserve

- Store owns physical byte survival
- Foundational owns shared aspect and boundary vocabulary
- Proof owns shared proof/reporting vocabulary
- Relational owns semantic truth/MVCC meaning
- physical pages, frames, records, WAL, manifests, and blob chunks remain
  binary Store structures, not JSON documents

### Proof Obligations

- unclassified production JSON/serde residue fails the gate
- terminal JSON modules are the only JSON homes in the dedicated Store
  workspace
- ordinary Store tests author native aspect values and Store physical witnesses
- canonical basis and digest evidence are derived from native facts
- terminal projections cannot reconstruct authority without Store readmission
- S0 can consume only the typed native readiness artifact

### Closeout Gate

The Aspect-Native Workspace Gate is not closed until the dedicated Store
workspace can prove that all ordinary source, evidence, handoff, digest,
recovery, certification, and harness paths are aspect-native, and that JSON is
confined to named terminal projection or hostile/readmission boundaries.

## S.0: Foundation Source Boundary And Claim Vocabulary

Engineering spec: [storage-foundation-s0.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s0.md)

### Goal

Establish the source boundary, claim vocabulary, shared Foundational/Proof
adoption, deferred physical guarantee map, and S.1 handoff contract for the
Forge Store physical foundation.

### Boundary

This is not physical page work. This is the contract foundation that makes later
page, backend, verifier, recovery, and certification work consume typed,
aspect-native Store facts instead of prose, local folklore, or ad hoc JSON.

### Adversarial Constraint

No milestone, closeout, backend, certification lane, or roadmap phrase may
claim platform-grade physical database behavior without a Store-owned physical
witness, a declared capability tier, accepted proof evidence, and a mapped
Roadmap 2 sequence for any deferred guarantee.

### Must Ship

- declared source-set contracts for the Forge Store workspace crate family
- typed/aspect-native input manifest and terminal projection rules
- capability-tier vocabulary for Roadmap 2 backend and evidence claims
- claim scanner and report vocabulary for language that implies physical
  database guarantees
- explicit physical guarantees deferred to `S.1` through `S.12`
- terminology cleanup for any "production-grade embedded backend" language
  that currently overstates the physical substrate
- shared-vocabulary adoption map for `forge-foundational` and `forge-proof`,
  distinguishing imported platform vocabulary from Store-owned physical
  durability vocabulary

### Must Preserve

- later physical work must respect semantic authority boundaries owned by
  `forge-relational`
- runtime bridge and signal evidence may support Store claims only through
  typed cross-crate surfaces
- Foundational and Proof vocabulary is imported where shared, while Store keeps
  byte survival authority

### Proof Obligations

- every new Roadmap 2 backend has a declared capability tier and forbidden
  claims
- every deferred physical guarantee maps to one or more `S.*` sequences
- every shared-language adoption point names whether it comes from
  `forge-foundational`, `forge-proof`, or Store-owned physical vocabulary
- S.1 cannot consume a terminal projection, stale artifact, or raw string in
  place of an accepted native witness

### Closeout Gate

`S.0` is not closed until the Forge Store workspace has typed contracts for
source manifests, capability tiers, claim reports, deferred guarantees, shared
vocabulary adoption, certification evidence, and S.1 handoff readiness.

## S.1: Physical Page, Segment, And Extent Substrate

Engineering spec: [storage-foundation-s1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s1.md)

### Goal

Define the physical byte universe of Forge Store.

### Boundary

This is not adding page-shaped structs around the existing heap store. This is
making page, segment, extent, frame, manifest, and physical reference addressing
the normal storage substrate for the platform-grade backend.

### Adversarial Constraint

No normal read, write, recovery, compaction, replication-preparation, audit, or
repair path may require full-store heap materialization, full-domain-struct
deserialization, or backend-private layout knowledge.

### Must Ship

- fixed-size page and variable-size extent model
- page ids, segment ids, extent ids, epochs, generation counters, and physical
  references
- explicit binary physical format law for byte order, integer widths,
  magic/version fields, page-size classes, alignment, reserved fields, and
  forward compatibility
- physical page/frame headers with kind, version, length, checksum slot,
  generation, and publication state
- slot directories or equivalent page-local record addressing
- record framing independent of serde domain object materialization
- segment manifests, allocation classes, free-space maps, and physical root
  manifests
- explicit `forge-foundational` adoption for S.1 canonical basis, digest
  derivation, diagnostic rows, profile/materialization posture,
  provenance/support truth, completed-boundary receipts, and counter-backed
  performance receipts at Store evidence boundaries
- crate-local public facades that map to the new Store workspace boundaries
  and keep physical format authority separate from semantic authority
- `S2PhysicalSubstrateReadiness` handoff so buffer-pool work consumes typed
  physical substrate proof rather than raw pages or backend handles

### Must Preserve

- canonical commit envelopes remain semantic authority
- physical pages are byte containers and access structures, not truth semantics
- backend variation may alter placement, not artifact meaning
- format contracts remain Store-owned even when they consume shared
  Foundational ids or Proof evidence vocabulary

### Proof Obligations

- exact page-read, page-write, frame-decode, allocation, and manifest-lookup
  counters
- no required lane performs whole-store heap materialization
- physical references remain stable across restart and detectable across stale
  generation reuse
- locate, append, manifest lookup, root open, verifier walk, and evidence
  materialization have verified complexity contracts and exact counter proof
- S.1 evidence consumes `forge-foundational` public/grouped APIs where shared
  boundary vocabulary exists, while Store-owned physical witnesses remain the
  only authority for physical byte survival

### Closeout Gate

`S.1` is not closed until the primary platform-grade backend can persist,
reopen, scan, and locate records through page/segment/extent identifiers without
deserializing the whole store into domain structs.

## S.2: Buffer Pool, Memory Budgets, And Zero-Copy Record Access

Engineering spec: [storage-foundation-s2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s2.md)

### Goal

Make memory residency bounded and explicit.

### Boundary

This is not a cache in front of heap-loaded state. This is a bounded buffer pool
and physical record-access regime where every admitted storage path knows its
resident bytes, pinned pages, dirty pages, and allocation envelope.

### Adversarial Constraint

A store larger than available memory must continue to read, write, recover,
compact, and stream within declared resident-memory and allocation ceilings.

### Must Ship

- bounded buffer pool with page leases, pin/unpin, dirty tracking, and eviction
- explicit dirty-page, pinned-page, and resident-byte budgets
- allocation scopes for foreground operations, maintenance, recovery, scrub,
  import/export, and blob streaming
- zero-copy or bounded-copy record views for hot physical paths
- read-ahead and write-behind policies with counters and admission rules
- OOM avoidance by admission, defer, or typed denial before expensive
  materialization

### Must Preserve

- Store does not replace `forge-relational`'s in-memory runtime arenas
- semantic reconstruction may allocate domain objects at admitted boundaries,
  but physical storage cannot require full-store domain allocation

### Proof Obligations

- resident-memory, pinned-page, dirty-page, allocation, eviction, cache-hit, and
  cache-miss counters
- allocation-free or exact-allocation hot-path contracts
- large-store tests where data size exceeds memory budget

### Closeout Gate

`S.2` is not closed until an admitted workload with store size greater than the
configured memory budget completes reads, writes, recovery, compaction planning,
and blob streaming without exceeding exact resident-byte, pin, dirty-page, and
allocation counters.

## S.3: Physical Integrity, Scrub, Quarantine, And Corruption Localization

Engineering spec: [storage-foundation-s3.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s3.md)

### Goal

Detect damaged bytes before logical decode and localize physical corruption.

### Boundary

This is not artifact-level digest validation with better error messages. This
is physical integrity enforcement where damaged pages, frames, manifests,
indexes, WAL records, and blob chunks are rejected before logical decoding can
turn bad bytes into ambiguous semantic failure.

### Adversarial Constraint

A flipped bit, torn frame, stale generation, damaged page, corrupted blob chunk,
or mismatched manifest must fail at the physical boundary with typed
localization rather than surfacing as ambiguous semantic decode failure.

### Must Ship

- page and frame checksums, preferably CRC32c or a stronger declared algorithm
- WAL frame, manifest, index-page, and blob-chunk checksums
- digest chains where logical identity and physical integrity must compose
- torn-write and stale-generation detection
- online and offline scrub surfaces
- quarantine records and artifact-boundary localization reports
- repair-plan inputs for rebuildable derived damage vs damaged authority

### Must Preserve

- checksums prove physical integrity, not authenticity
- cryptographic artifact digests prove identity, not physical locality
- damaged derived artifacts never outrank intact authority

### Proof Obligations

- byte-flip, torn-write, stale-generation, missing-frame, and damaged-chunk
  injection lanes
- exact counters for checked pages, checksum failures, quarantines, rebuildable
  damage, unrecoverable authority damage, and skipped logical decodes

### Closeout Gate

`S.3` is not closed until injected byte flips, torn frames, stale generations,
manifest corruption, index-page corruption, WAL-frame corruption, and blob-chunk
damage all localize to typed physical boundaries before any semantic decoder is
allowed to consume the bytes.

## S.4: WAL, Checkpoint, LSN, And Recovery Physics

Engineering spec: [storage-foundation-s4.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s4.md)

### Goal

Rebuild WAL and checkpointing around database-grade physical recovery.

### Boundary

This is not adding LSN fields to the current WAL vocabulary. This is making
WAL, pageLSN, checkpoint manifests, flush ordering, and idempotent replay the
physical recovery law for acknowledged durable bytes.

### Adversarial Constraint

A crash at any byte publication boundary must recover through deterministic
physical rules without trusting stale pages, replaying closed work twice,
losing acknowledged truth, or requiring backend residue guessing.

### Must Ship

- WAL segments, LSNs, pageLSNs, durable frame headers, and replay cursors
- redo policy, or an explicitly justified alternative
- checkpoint manifests with durable publication and validation rules
- idempotent replay and closed-work quiescence
- page flush ordering, WAL-before-data rules, and durable directory/rename
  barriers
- recovery source precedence over pages, WAL, checkpoints, snapshots,
  compaction products, and derived families
- recovery-time budget tied to checkpoint interval and WAL tail, not full store
  size

### Must Preserve

- runtime semantics remain above WAL mechanics
- WAL is recovery machinery, not an alternate truth source
- recovery conclusions remain deterministic and machine-checkable

### Proof Obligations

- crash matrix around WAL append, page flush, checkpoint publication, manifest
  cutover, compaction cutover, and acknowledgment
- exact replayed-frame, skipped-frame, page-redo, checkpoint-validated, and
  recovery-time counters

### Closeout Gate

`S.4` is not closed until crash recovery is bounded by checkpoint interval and
WAL tail, every acknowledged write is recoverable exactly once, every
unacknowledged partial publication is rejected or completed through typed rules,
and recovery never relies on scanning backend residue as authority.

## S.4.5: Physical Database Simulation Harness

Engineering spec: [storage-foundation-s4-5.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s4-5.md)

### Goal

Turn Roadmap 2's physical adversarial harness doctrine into a reusable,
aspect-native simulation and certification substrate before S.5 begins.

### Boundary

This is not physical isolation, compaction, checkpointing, reclaim, blob
lifecycle, I/O QoS, backup, repair, security, or full S.12 certification. This
is the typed harness foundation that lets later sequences express those
behaviors through production-facing drivers, deterministic schedules,
certification-owned oracles, replayable transcripts, exact counters, and
Foundational/Proof-backed evidence.

### Adversarial Constraint

No S.5 through S.12 physical claim may close on logs, same-run
self-comparison, timing luck, fixture labels, hand-mutated private structs,
test-support-owned oracle meaning, synthetic in-memory stores, JSON scenario
authority, or post-hoc broad assertions. Hostile physical claims must be
replayable from typed aspect-native scenarios, deterministic schedules,
production-boundary drivers, fault/corruption/interleaving events,
certification-owned oracles, exact counters, transcripts, and evidence bundles.

### Must Ship

- public golden-path scenario authoring API backed by aspect-native scenario,
  schedule, actor, fault, oracle, transcript, counter, fixture, and
  evidence-bundle vocabulary
- deterministic scheduler and replay identity for physical database simulations
- production-facing driver contracts for storage, crash, corruption, memory,
  I/O, offline verification, maintenance, named yieldpoints, and later
  blob/security/repair lanes
- reusable certification-owned oracle families and generated coverage matrix so
  proof meaning never lives in test support or hand-authored coverage prose
- counter-strength posture so exact counters are used for forbidden behavior
  and deterministic event structure, while implementation-sensitive costs use
  weakest-sufficient expectations
- S.4 recovery and shortcut-rejection dogfood slices through the public
  authoring API, plus an S.5 readiness shape-probe through the same API with
  explicit non-claim evidence that the probe does not certify unbuilt S.5
  isolation behavior
- reusable profile lanes for smoke, CI certification, soak, release
  certification, and hardware qualification
- S.5 readiness handoff proving the harness can express protect-before-observe,
  root swaps, byte guards, reclaim barriers, crash/restart, deterministic
  interleavings, and forbidden shortcuts

### Must Preserve

- Store owns physical database behavior.
- Test support owns mechanics, not proof meaning.
- Certification owns oracle verdicts and closeout evidence.
- Foundational owns shared boundary/evidence vocabulary, not Store authority.
- Proof owns proof-bearing progression law, not simulation semantics.
- JSON remains confined to terminal projection or hostile/readmission lanes.

### Proof Obligations

- scenario lowering is typed and cannot skip resolution, scheduling,
  admission, execution, transcript, oracle, or evidence stages
- deterministic replay reproduces actor steps, injected events, counters,
  transcripts, and oracle verdicts across runs
- production-backed fixtures and drivers exercise production-facing boundaries
  rather than private state mutation
- certification-owned oracles deny logs, same-run self-comparison, fixture
  labels, test-support verdicts, and JSON-shaped authority
- S.5 cannot start without `S5SimulationHarnessReadiness`

### Closeout Gate

`S.4.5` is not closed until the dedicated Store workspace has a reusable
physical simulation harness that can run typed, aspect-native, deterministic,
production-boundary-backed, adversarial scenarios with certification-owned
oracles, exact counters, replayable transcripts, and an S.5 readiness handoff.

## S.5: Physical Isolation, Latches, Epochs, And Stable Read Plans

Engineering spec: [storage-foundation-s5.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s5.md)

### Goal

Provide physical byte stability without duplicating semantic MVCC.

### Boundary

This is not rebuilding `forge-relational` MVCC in the store. This is physical
isolation for bytes: latches, epochs, stable read plans, copy-on-write
publication, and reachability barriers that keep page and chunk reads valid
while maintenance moves storage underneath them. It consumes the S.4.5
simulation harness rather than inventing one inside the isolation milestone.

### Adversarial Constraint

Foreground reads, recovery reads, compaction, checkpointing, tier movement,
blob migration, and reclaim may interleave without bytes disappearing, roots
changing invisibly, or readers observing half-published physical structure.

### Must Ship

- page latch and manifest/root epoch discipline
- stable physical read plans carrying page/segment/root generation proofs
- copy-on-write or equivalent publication for moved pages and rewritten roots
- compaction/read, checkpoint/read, reclaim/read, and blob-migration/read
  safety rules
- deadlock prevention or detection for physical latches
- hazard, lease, or reachability barriers for pages, extents, and blob chunks

### Must Preserve

- `forge-relational` owns semantic MVCC and truth visibility
- Store physical isolation answers whether bytes remain stable while a physical
  plan reads them

### Proof Obligations

- hostile interleavings for read-during-compaction, read-during-checkpoint,
  read-during-reclaim, read-during-blob-migration, and restart during cutover
- exact latch, lease, epoch-retry, stale-plan-rejection, and blocked-reclaim
  counters

### Closeout Gate

`S.5` is not closed until foreground reads can run during compaction,
checkpointing, reclaim, tier movement, and blob migration without observing
half-published roots, missing pages, reused stale generations, or reclaimed
chunks that were still protected by an admitted read plan.

## S.5.1: Cryptographic Boundary Seeds And Tenant Scope Metadata

Engineering spec: [storage-foundation-s5-1.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s5-1.md)

### Goal

Backfill the cryptographic, authenticity, and tenant-scope metadata that later
Roadmap 2 work must consume, without rewriting the already-planned `S.1`,
`S.3`, or `S.4` milestone scopes.

### Boundary

This is not full encryption, full tenant isolation, an identity provider, or
the `S.11` security/compliance program. This is the typed physical metadata
foundation that makes those later claims structurally possible: key scope, key
version, tenant scope, authenticity class, custody posture, encrypted-frame
compatibility, and readiness witnesses for blobs, backup/export, repair, and
certification.

### Adversarial Constraint

Later Store work must not be able to introduce encrypted pages, authenticated
frames, tenant-scoped blobs, backup capsules, PITR bundles, export capsules, or
repair plans through raw strings, ambient deployment assumptions, terminal JSON
projections, or digest-only equivalence. Any page, frame, WAL record, manifest,
stable read plan, blob chunk, backup/export bundle, or repair plan that lacks
typed key scope, tenant scope, authenticity class, and custody posture must fail
platform-grade admission.

### Must Ship

- Store-owned key scope, key version, tenant scope, authenticity class, and
  custody posture vocabulary
- physical metadata compatibility for page/frame headers, WAL/checkpoint
  records, manifests, physical roots, and stable read plans
- explicit separation between checksum/integrity success and authenticity
  success
- readiness witnesses for S.7 blob chunks, S.10 backup/PITR/repair, S.11 key
  lifecycle, and Roadmap 1 replication/blob/repair milestones
- S.4.5/S.5 harness scenarios for stale key versions, wrong tenant scopes,
  missing authenticity requirements, unsupported capability posture, and
  cross-scope repair rejection
- typed `S.6` and `S.11` handoffs proving later physical I/O and security work
  consume these metadata surfaces instead of inventing parallel ones

### Must Preserve

- Store owns physical byte survival and cryptographic boundary evidence
- `forge-relational` owns semantic MVCC, transaction meaning, and identity
  semantics
- external identity systems may provide admission evidence, but Store does not
  become an identity provider
- this milestone backfills metadata for already-sequenced foundations rather
  than pretending `S.1`, `S.3`, or `S.4` shipped it
- JSON remains confined to terminal projection or hostile/readmission lanes

### Proof Obligations

- raw strings, semantic ids, JSON projections, lower-authority digests, and
  terminal labels cannot satisfy key-scope, tenant-scope, authenticity, or
  custody APIs
- checksum-valid bytes can still be authenticity-failed,
  authenticity-unavailable, or authenticity-unsupported, and those outcomes are
  machine-distinguishable
- stable read plans, manifests, WAL/checkpoint records, blob readiness,
  backup/export readiness, and repair readiness preserve security scope through
  hostile physical interleavings
- identical blob or page content across tenant/key scopes does not collapse
  into a shared physical claim unless an admitted equivalence policy proves it
  safe
- exact counters expose security-scope admissions, key-version observations,
  tenant-scope drift, authenticity failures/unavailable results,
  unsupported-capability denials, and cross-scope repair rejections

### Closeout Gate

`S.5.1` is not closed until the physical Store foundation can carry typed key
scope, tenant scope, authenticity class, key-version posture, and custody
posture through page/frame, WAL/checkpoint, manifest, stable-read-plan, blob
readiness, backup/export readiness, and repair-readiness paths, with hostile
tests proving wrong-scope, stale, missing, unsupported, and terminal-projection
inputs are rejected before later `S.6`, `S.7`, `S.10`, or `S.11` work can
consume them.

## S.6: Hardware-Aware I/O, QoS, And Background Work Pacing

### Goal

Make physical I/O behavior an explicit store contract.

### Boundary

This is not a configuration enum for buffered vs direct I/O. This is a
capability-gated media contract with measured foreground reservations,
background pacing, flush semantics, queue-depth control, and declared hardware
assumptions.

### Adversarial Constraint

Background compaction, checkpointing, scrub, replication preparation, blob
migration, and cold-tier movement must not rely on OS writeback folklore and
must not unpredictably freeze foreground operations inside the declared
operating envelope.

### Must Ship

- backend capability tiers for buffered file, mmap, direct I/O, and optional
  async I/O
- media capability tiers for encrypted/authenticated frame compatibility and
  unsupported secure posture denial from `S.5.1`
- fsync/fdatasync, directory sync, durable rename, alignment, and sector
  atomicity assumptions per backend
- queue-depth, write-grouping, read-ahead, write-back, and flush scheduling
  contracts
- foreground I/O reservation and background maintenance pacing
- page-cache policy, trim/punch-hole policy, and cold-tier movement rules
- tail-latency and interference counters at foreground boundaries

### Must Preserve

- backend capability changes may affect cost, not durability meaning
- unsupported durability or QoS claims fail typed rather than silently degrading
- I/O paths preserve `S.5.1` key scope, tenant scope, authenticity class, and
  custody posture instead of treating them as side metadata

### Proof Obligations

- foreground p99/p999 or equivalent envelope tests under compaction,
  checkpoint, scrub, blob ingest, and blob migration
- exact foreground-wait, background-yield, flush, sync, queue-depth, and
  interference counters
- encrypted/authenticated-frame readiness lanes proving unsupported secure I/O
  posture is rejected before platform-grade admission

### Closeout Gate

`S.6` is not closed until foreground read/write latency remains inside the
declared envelope under admitted compaction, checkpoint, scrub, replication
preparation, blob ingest, and blob migration pressure, with exact counters
showing when background work yielded, paced, or was denied.

## S.7: Native Blob/Object Chunk Store

Engineering spec: [storage-foundation-s7.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/storage-foundation-s7.md)

### Goal

Make blob storage native to the physical substrate before Roadmap 1's blob
milestone expands the product surface.

### Boundary

This is not storing blob metadata in the database and bytes beside it. This is
making large-object storage a native chunk-tree, streaming, checksummed,
deduped, resumable, retention-aware part of the same physical database
substrate.

### Adversarial Constraint

Multi-GB blobs must be stored, read, verified, deduped, replicated, compacted,
and reclaimed without whole-object memory residency or a second storage system
beside Forge Store.

### Must Ship

- content-addressed chunk trees
- chunk metadata that carries `S.5.1` key scope, key version, tenant scope,
  authenticity class, and custody posture
- streaming ingest, read, verify, export, and import paths
- resumable blob writes and interrupted-upload recovery
- chunk checksums plus content digests
- dedupe indexes and collision handling
- chunk reachability, reference tracking, or equivalent retention-safe GC
- inline, external, and cold chunk placement on the same physical foundation
- partial replication and capsule-readiness for blob-bearing artifacts
- explicit cross-tenant/cross-key dedupe admission policy; digest equality
  alone is not enough to merge physical blob claims across security scopes

### Must Preserve

- primary blobs may be authoritative artifacts
- derived blobs remain rebuildable and accuracy-classed
- blob storage is not a file-server sidecar with metadata in the database
- blob chunks may not erase tenant/key/authenticity posture during streaming,
  verification, dedupe, export, import, tier movement, or reclaim

### Proof Obligations

- constant-memory large-blob lanes
- interrupted write/resume lanes
- dedupe, corruption, reclaim, tier-move, partial-export, and missing-chunk
  lanes
- exact chunk-read, chunk-write, dedupe-hit, reachability, orphan, and
  streaming-memory counters
- wrong-key-scope, wrong-tenant-scope, stale-key-version, and cross-scope
  dedupe-rejection lanes consuming `S.5.1` readiness

### Closeout Gate

`S.7` is not closed until a multi-GB blob can be streamed in, verified, resumed
after interruption, deduped, exported, imported, tier-moved, partially
replicated, and reclaimed with constant memory and chunk-level corruption
localization.

## S.8: Index, Layout, And Access-Path Discipline

### Goal

Make physical access structures explicit per artifact family.

### Boundary

This is not picking one generic index structure and routing everything through
it. This is forcing every artifact family to declare the physical layout,
access paths, rebuild source, evolution policy, and read/write amplification
model that match its workload.

### Adversarial Constraint

No artifact family may accidentally inherit a generic layout whose cost model,
rebuild model, corruption behavior, or evolution path is dishonest for its
workload.

### Must Ship

- declared layout family per durable artifact family:
  append log, heap file, B-tree, LSM-like structure, sparse index, chunk tree,
  or another explicit strategy
- index/page format versioning and compatibility rules
- secondary-index consistency and rebuild contracts
- range, prefix, point, scan, and streaming access-path declarations
- read-amplification and write-amplification accounting
- layout migration and rollback posture

### Must Preserve

- physical indexes and layouts remain derived unless explicitly classified as
  authoritative artifacts
- index rebuild must derive from canonical authority or declared physical
  authority

### Proof Obligations

- access-path counter tests for each admitted layout family
- broad-scan rejection where a bounded index is required
- index corruption and rebuild parity lanes
- exact page-touch, index-probe, write-amplification, and read-amplification
  counters

### Closeout Gate

`S.8` is not closed until each admitted durable artifact family has a declared
layout strategy, bounded access-path counters, corruption/rebuild behavior, and
format-evolution posture, and no required family falls back to an implicit
whole-store scan where the roadmap claims indexed or locality-bounded access.

## S.9: Formal Models For Crash, Recovery, Compaction, And Repair

### Goal

Model the state machines tests cannot exhaustively cover.

### Boundary

This is not adding formal-methods theater after implementation. This is making
the critical crash, recovery, compaction, repair, import, and physical-lease
state machines small enough to model and aligned enough that code states map
back to checked model states.

### Adversarial Constraint

The physical state machines most likely to fail under crash plus concurrency
must be proven against illegal transitions rather than trusted to curated test
coverage.

### Must Ship

- TLA+, PlusCal, or equivalent formal models for:
  - WAL/checkpoint/page flush ordering
  - recovery source precedence
  - compaction cutover
  - physical read leases and reclaim barriers
  - repair/quarantine transitions
  - replication/import admission where it depends on physical evidence
- model-to-code vocabulary mapping
- checked invariants and counterexample documentation

### Must Preserve

- formal models prove physical and recovery protocols, not domain semantics
  already owned by `forge-relational`
- implementation vocabulary must stay aligned with model states

### Proof Obligations

- checked model artifacts committed with the roadmap implementation evidence
- every modeled state machine has matching code states, typed transitions, and
  certification lanes

### Closeout Gate

`S.9` is not closed until the named physical state machines have checked model
artifacts, explicit invariants, implementation vocabulary mapping, and at least
one certification lane that would fail if the implementation violated the
modeled transition rules.

## S.10: Operational Safety, Backup, PITR, Disaster Recovery, And Forensics

### Goal

Make the store operable under real production damage and recovery pressure.

### Boundary

This is not nicer operator logs around existing recovery. This is offline
verification, backup, PITR, disaster recovery, repair planning, forensic
bundling, and trusted/degraded/quarantined truth reporting that can be used
when the live store binary or part of the media cannot be trusted.

### Adversarial Constraint

When production storage is damaged, partially restored, operator-touched,
rolled back, or disaster-recovered, the system must explain exactly what truth
is trusted, what is degraded, what is rebuildable, and what is quarantined.

### Must Ship

- online backup and restore
- point-in-time recovery over physical checkpoints plus WAL tail
- backup, PITR, disaster-recovery, and forensic bundle declarations that carry
  `S.5.1` key scope, tenant scope, authenticity class, and custody posture
- replica/bootstrap recovery paths
- disaster-recovery bundles
- offline verifier that can inspect store files without trusting the live store
  runtime
- bad-sector and damaged-page localization reports
- operator repair plans, rollback plans, forensic bundles, and audit trails
- trusted-truth, degraded-derived, rebuildable, quarantined, and unrecoverable
  reporting
- key-custody-unavailable, authenticity-unavailable, wrong-tenant-scope, and
  unsupported-secure-posture reporting

### Must Preserve

- repair tooling may not mutate authority implicitly
- operator actions are auditable artifacts
- recovery and verification do not require ambiguous log interpretation
- repair, restore, and forensic plans may not cross tenant or key scope without
  an admitted `S.5.1` custody and blast-radius witness

### Proof Obligations

- backup/restore, PITR, offline-verify, bad-sector, partial-restore,
  damaged-authority, damaged-derived, and operator-repair lanes
- exact verifier-read, repaired-page, quarantined-page, restored-LSN,
  trusted-artifact, and degraded-artifact counters
- key-custody-missing, tenant-scope-drift, authenticity-unavailable, and
  cross-scope-repair-rejection lanes

### Closeout Gate

`S.10` is not closed until an offline verifier can inspect a damaged store,
identify trusted authority, degraded derived artifacts, quarantined physical
regions, PITR candidates, and admissible repair plans without trusting the live
store runtime or ambiguous human log interpretation.

## S.11: Security, Compliance, Tenant Boundaries, And Auditability

### Goal

Make platform trust a store contract rather than a deployment wish.

### Boundary

This is not deferring encryption, tenancy, audit, deletion, key lifecycle,
operator access, and provenance to deployment policy. This is making those
concerns explicit physical and operational contracts of the store, with typed
behavior when a backend or deployment cannot satisfy them. It consumes `S.5.1`
security-scope metadata rather than inventing key, tenant, authenticity, or
custody vocabulary locally.

### Adversarial Constraint

Physical access, tenant leakage, key compromise, audit tampering, stale
credentials, and deletion promises must be explicit store concerns with typed
behavior, not ambient assumptions above the database.

### Must Ship

- native envelope-encryption hierarchy for deployment, tenant, artifact, page,
  WAL/checkpoint, blob/chunk, backup, export, and repair scopes
- key versioning, rotation, rewrap, recovery, compromise, and custody posture
- BYOK, HYOK, KMS, HSM, local-keystore, and unsupported-key-management
  capability tiers with typed admission and rejection
- tenant-scoped physical placement, quota, and repair blast-radius accounting
- tamper-evident audit logs for authority-path and operator actions
- secure deletion policy with backend capability assumptions
- authenticity surfaces distinct from checksums
- provenance for store binary, format version, configuration, and certification
  bundle
- workload identity and proof-of-possession admission evidence for service and
  operator actions, without making Store an identity provider
- cryptographic erasure posture for tenant offboarding, blob deletion, backup
  retirement, and secure repair disposal

### Must Preserve

- encryption and access control do not redefine canonical truth
- checksum success is not authenticity success
- tenant isolation survives repair, backup, restore, replication, and audit
- key material and identity-provider assertions do not become semantic
  authority
- S.11 consumes the `S.5.1` typed metadata foundation instead of accepting raw
  token strings, deployment labels, or terminal projections

### Proof Obligations

- key-rotation, rewrap-without-semantic-rewrite, wrong-key, stale-key,
  cross-tenant-decrypt, replayed-credential, tenant-boundary, audit-tamper,
  secure-delete, cryptographic-erasure, backup-restore-with-keys,
  export-capsule-key mismatch, and operator-action provenance lanes
- exact encrypted-page, decrypted-page, rewrapped-page, encrypted-chunk,
  decrypted-chunk, key-version, tenant-scope, key-custody, audit-chain,
  proof-of-possession, and authenticity-failure counters

### Closeout Gate

`S.11` is not closed until encrypted storage, envelope key hierarchy, key
rotation and rewrap, BYOK/HYOK/KMS/HSM capability admission, tenant-scoped
physical boundaries, audit tamper evidence, authenticity checks, secure-delete
and cryptographic-erasure capability declarations, proof-of-possession
operator/service admission, and provenance survive backup, restore, repair,
replication, export/import, and operator-action lanes.

## S.12: Physical Database Certification And Performance Program

### Goal

Certify the physical foundation as a database substrate before the roadmap
resumes platform expansion.

### Boundary

This is not a large integration test pack. This is the readiness gate that ties
physical correctness, semantic parity, performance envelopes, hardware
assumptions, fault injection, hazard analysis, and reproducible evidence into a
machine-checkable certification program.

### Adversarial Constraint

The physical store may not claim production, financial-platform, aerospace, or
beta posture until hostile physical behavior has machine-checkable proof tied
to declared hardware, backend, workload, configuration, and operating envelope
assumptions.

### Must Ship

- physical database certification suite
- power-loss, torn-write, byte-flip, stale-generation, and partial-flush fault
  injection
- long-running physical soak with foreground traffic and background maintenance
- bounded-memory proof for stores larger than memory
- allocation-free or exact-allocation hot-path proof
- recovery-time, checkpoint-interval, and WAL-tail proof
- read-amplification and write-amplification proof
- tail-latency and foreground-interference proof
- blob-scale proof
- cross-backend parity and capability-matrix proof
- FMEA/STPA-style hazard analysis with detection, containment, recovery action,
  proof lane, and residual-risk fields
- reproducible certification bundles tied to source, binary, config, format,
  backend, and hardware assumptions

### Must Preserve

- certification proves physical behavior and semantic parity together where
  they meet
- no suite may rely on logs, same-run self-comparison, or successful completion
  as proof

### Proof Obligations

- all Roadmap 2 certification lanes pass with machine-checkable evidence
- every claimed performance envelope has exact counters and an admitted
  workload class
- every unsupported backend/hardware capability fails typed or is marked debt
  before use

### Closeout Gate

`S.12` is not closed until Roadmap 2 produces reproducible certification
bundles for power loss, torn writes, byte flips, stale generations, memory
bounds, allocation bounds, recovery bounds, foreground latency under background
work, blob scale, cross-backend parity, and hazard-analysis residual risk.

## Platform Readiness Gate

`forge-store` may resume the first roadmap's post-13.3 platform path only when:

- the Aspect-Native Workspace Gate is closed, proving JSON is confined to
  terminal projection or hostile/readmission boundaries
- `S.0` through `S.12`, including `S.4.5` and `S.5.1`, are implemented or
  explicitly scoped with named, non-platform-grade debt
- [test-requirements-2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements-2.md)
  adversarial harness requirements are satisfied for every closed `S.*`
  sequence
- Roadmap 2 workspace crates expose the typed source, claim, handoff, and
  certification contracts required for S.1 entry
- new Store work lives in the dedicated workspace/crate family and does not
  inherit undeclared module topology as precedent
- Forge Foundational and Forge Proof vocabulary have been reviewed and adopted
  where they prevent local folklore without stealing Store-owned physical
  authority
- no platform-grade backend requires full-store heap materialization
- physical integrity exists below logical artifact digests
- memory, allocation, recovery, latency, read-amplification, and
  write-amplification envelopes are declared and tested
- blob storage is native to the physical substrate
- backup, PITR, repair, audit, tenant, security, and offline verification
  posture are explicit
- key scope, tenant scope, authenticity class, key-version posture, and custody
  posture are typed physical metadata from `S.5.1`, not late feature-local
  strings
- formal models exist for the crash/concurrency state machines named in `S.9`
- Roadmap 2 physical certification passes

## Relationship To Existing Milestones

- `Milestone 14` replication must consume Roadmap 2 physical integrity,
  chunking, manifests, offline verification, `S.5.1` security-scope metadata,
  `S.11` key lifecycle evidence, and blob substrate instead of defining
  replication over backend-local rows or files.
- `Milestone 15` extensibility must register storage strategies against the
  physical layout, integrity, recovery, security-scope, key-lifecycle, and
  certification contracts from Roadmap 2.
- `Milestone 20` native blob storage should be rewritten as a product-layer
  expansion over `S.7` and `S.5.1`, not as first introduction of the blob
  substrate or blob security-scope metadata.
- `Milestone 21` budgets must include Roadmap 2 physical budgets: resident
  pages, dirty pages, WAL tail, chunk footprint, I/O queue depth, compaction
  debt, recovery time, latency interference, key-version pressure, rewrap debt,
  audit-chain growth, secure-delete backlog, and tenant export footprint.
- `Milestone 22` operator repair must build on `S.10`, `S.5.1`, and `S.11`
  rather than inventing repair after platform features exist.
- `Milestone 23` and `Milestone 24` certification must include Roadmap 2
  physical certification outputs before the store can be called beta-ready,
  financial-platform grade, or aerospace-grade.

## Completion Standard

Roadmap 2 is complete only when `forge-store` can honestly say:

- canonical truth is semantically authoritative
- physical bytes are page/chunk/frame structured
- memory residency is bounded
- corruption is detected before logical decode
- recovery is LSN/checkpoint/page aware and bounded
- physical reads are stable under maintenance
- foreground latency is protected from background work inside declared
  envelopes
- blobs are native, content-addressed, streaming, and retention-safe
- physical access paths are declared per artifact family
- critical crash/concurrency protocols are formally modeled
- backup, PITR, disaster recovery, repair, forensics, tenant isolation,
  security, key lifecycle, cryptographic erasure, and auditability are platform
  contracts
- certification evidence is machine-checkable, reproducible, and tied to
  declared hardware/backend assumptions

Only then should the store resume the higher-level platform roadmap.
