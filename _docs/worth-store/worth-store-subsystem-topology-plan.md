# Worth Store Subsystem Topology Plan

## Purpose

Give the entire Worth Store workspace one permanent, navigable domain topology
before performing broad mechanical moves.

This plan and `worth-store-topology-inventory.csv` are paired artifacts:

- this document defines the target subsystem boundaries;
- the CSV gives every current file a disposition against those boundaries;
- a later migration tool may execute only rows marked mechanically ready;
- content-level splits, merges, deletions, authority corrections, and public API
  changes remain explicit engineering work.

The initial inventory is intentionally conservative. It classifies all 3,636
current non-ignored workspace files without claiming that filenames alone can
prove semantic ownership. Hash-bound semantic overlays now record the deeper
review of every row that the initial path-only pass left unresolved.

## Inventory Contract

The inventory records:

- current path, crate, directory, source set, kind, and line count;
- inferred current subsystem and semantic role;
- whether the file appears authoritative, observational, boundary-facing, or
  test/courtroom-only;
- proposed and candidate actions;
- target subsystem, directory, and path when deterministic;
- confidence and whether content review is required.

The action vocabulary is:

| Action | Meaning |
| --- | --- |
| `keep` | Current location plausibly expresses a permanent responsibility. |
| `move` | The responsibility is sufficiently clear for deterministic relocation. |
| `split` | One file contains responsibilities with different structural fate. |
| `merge` | Separate files model one responsibility without a useful boundary. |
| `delete` | The file represents displaced, duplicate, or provenance-only structure. |
| `review` | Filename and location are insufficient to choose safely. |

Only `move` rows with `mechanical_ready=true` may enter the first automated
migration. `keep` means "no initial physical move," not "architecturally
certified." No `review` row may be silently converted into a move.

## Semantic Review Status

The 1,284 rows selected for content review have been classified in 16 semantic
batches. The batches follow dependency and domain ownership rather than equal
numeric sizes: shared semantics, physical substrate, integrity/isolation/WAL,
IO scheduling, layout, blobs, deltas/snapshots/tiering, maintenance/operations,
recovery, readiness, certification, physical certification, test support, and
three integration-test families.

Each reviewed row is bound to the SHA-256 digest of the source content used for
classification. Inventory regeneration fails when a reviewed source changes,
when two move targets collide, or when a classified target retains milestone
vocabulary. The current review state has:

- 1,284 classified rows;
- zero unresolved review rows;
- zero low-confidence classifications;
- zero colliding move/keep targets;
- zero milestone-shaped target paths.

This completes classification, not migration. `split`, `merge`, and `delete`
rows require explicit implementation, and all moves remain mechanically gated
until module, visibility, dependency, and facade consequences are checked in
their bounded migration batch.

## Workspace Architecture

Worth Store is organized into seven dependency bands. These bands express
direction of knowledge; they are not ceremonial layers.

### Band 0: Durable Vocabulary

`worth-store-contracts` contains only dependency-neutral durable identities,
artifact classifications, and representation-stable contracts genuinely shared
for the same semantic reason.

It must not contain milestone readiness, closeout, roadmap scope, synthetic
handoffs, or owner-specific transition authority.

### Band 1: Admission And Shared Semantics

These crates define reusable Store-native meaning without knowing physical
implementations:

- `worth-store-aspect-native`
- `worth-store-authority`
- `worth-store-budgets`
- `worth-store-claim-boundaries`
- `worth-store-modes`
- `worth-store-operations-vocabulary`
- `worth-store-security`

Physical encodings of their values belong in `worth-store-physical-format`.
Band 1 crates must not depend on physical format, layout indexes, recovery,
readiness, or certification after migration.

### Band 2: Physical Substrate

These crates own bytes, media, memory residency, integrity, concurrency, and IO:

- `worth-store-physical-format`
- `worth-store-physical-backend`
- `worth-store-buffer-pool`
- `worth-store-wal`
- `worth-store-physical-integrity`
- `worth-store-physical-isolation`
- `worth-store-io-scheduler`

They may consume Band 0 and Band 1 contracts. They must not depend on layout,
blob, recovery, operations, readiness, or certification merely to advertise a
later milestone handoff. Domain-specific adapters point downward from the
higher owner.

### Band 3: Durable Structures

These crates own reconstructible and authoritative storage structures:

- `worth-store-layout-indexes`
- `worth-store-blob-chunks`
- `worth-store-branch-deltas`
- `worth-store-snapshots`
- `worth-store-schema-lineage`
- `worth-store-reclaim-policy`
- `worth-store-retention`
- `worth-store-tiering`

They consume physical capabilities but cannot redefine physical authority.
Indexes, materializations, and derived blobs remain reconstructible from their
declared source authority.

### Band 4: Recovery And Operations

These crates coordinate multiple lower owners without absorbing their law:

- `worth-store-recovery-physics`
- `worth-store-offline-verifier`
- `worth-store-maintenance`
- `worth-store-operations`
- `worth-store-compatibility`
- `worth-store-bulk`
- `worth-store-replication`
- `worth-store-live-query`
- `worth-store-subscription-support`

An orchestration outcome may attest that the orchestration ran. It may not
reconstruct the lower capabilities it consumed.

### Band 5: Product Composition

- `worth-store-readiness` owns only real process/startup admission that has a
  runtime consumer. Milestone completion and certification closeout do not
  belong here.
- `worth-store-extensions` owns extension declaration, admission, registry, and
  compatibility boundaries.
- `worth-store-analysis` owns durable analysis-lane contracts and execution.
- `worth-store` is the thin product facade and composition root.

### Band 6: Courtroom And Engineering Evidence

- `worth-store-certification` owns verdicts and acceptance programs.
- `worth-store-physical-certification` owns reusable adversarial simulation and
  physical harness machinery.
- `worth-store-formal-models` owns executable abstract models.
- `worth-store-test-support` owns inputs, fixtures, drivers, and assertions.

These crates may observe lower production behavior. They may never issue a
production capability unavailable through an ordinary production operation.

## Crate Dispositions

| Crate | Disposition | Permanent responsibility |
| --- | --- | --- |
| `worth-store` | keep thin | Public product facade and composition root. |
| `worth-store-analysis` | freeze until built | Durable analysis lanes, basis admission, and analysis artifact lifecycle. |
| `worth-store-aspect-native` | reorganize | Aspect-native authority, canonical basis, ingress readmission, terminal projection, and receipts. |
| `worth-store-authority` | reorganize | Current authority, source admission, retention, projection, and readmission. |
| `worth-store-blob-chunks` | preserve domains, remove scaffolding | Blob identity, chunking, lifecycle, publication, reachability, retention, streaming, export/import, and compaction. |
| `worth-store-branch-deltas` | grow by domain | Delta identity, layering, base sharing, reads, compaction, and publication. |
| `worth-store-bulk` | freeze until built | Bulk planning, admission, execution, checkpointing, and publication. |
| `worth-store-budgets` | reorganize | Resource units, envelopes, pre-execution admission, counters, and denials. |
| `worth-store-buffer-pool` | reorganize root | Residency, pinning, record access, dirty pages, eviction, allocation, and bounded background work. |
| `worth-store-certification` | major reorganization | Courtroom programs, evidence projections, scenarios, oracles, and verdicts by permanent subsystem. |
| `worth-store-claim-boundaries` | reorganize | Claim declarations, forbidden claims, classification, promotion, and accounting. |
| `worth-store-compatibility` | grow by domain | Version compatibility, migration bridges, legacy readmission, and disposition. |
| `worth-store-contracts` | shrink and reorganize | Lowest stable shared vocabulary only. |
| `worth-store-extensions` | preserve and sharpen | Extension declaration, admission, registry, posture, and compatibility. |
| `worth-store-formal-models` | freeze until built | Executable models and refinement checks. |
| `worth-store-io-scheduler` | reorganize | Resource accounting, foreground reservation, background pacing, queues, and backend dispatch. |
| `worth-store-layout-indexes` | major reorganization | Catalog, keyspace, strategies, materialization, access, maintenance, evolution, and integrity adaptation. |
| `worth-store-live-query` | grow by domain | Durable query basis, resume position, invalidation input, and observation. |
| `worth-store-maintenance` | reorganize | Maintenance planning, admission, scheduling, execution, and publication. |
| `worth-store-modes` | preserve | Durable, embedded, and absent lifecycle contracts. |
| `worth-store-offline-verifier` | reorganize | Offline discovery, bounded scanning, verification, findings, and reports. |
| `worth-store-operations` | reorganize | Backup/export/import, repair planning, quarantine coordination, and operator-facing execution outcomes. |
| `worth-store-operations-vocabulary` | shrink | Dependency-neutral operation declarations shared across owners. |
| `worth-store-physical-backend` | reorganize | Backend capabilities, access policy, durability ordering, placement observation, and physical execution. |
| `worth-store-physical-certification` | preserve engine, remove milestone topology | Actors, scenarios, schedules, faults, drivers, observations, oracles, coverage, and transcripts. |
| `worth-store-physical-format` | reorganize | Binary identity, framing, records, manifests, references, security encoding, and decode admission. |
| `worth-store-physical-integrity` | major root reorganization | Pre-decode admission, checksums, container/page/chunk/WAL integrity, quarantine, and scrub. |
| `worth-store-physical-isolation` | preserve domains, remove later-owner knowledge | Epochs, latches, hazards, read stability, compaction, publication, reclaim, and checkpoint interlocks. |
| `worth-store-readiness` | shrink aggressively | Real runtime composition/startup admission only. |
| `worth-store-reclaim-policy` | reorganize | Candidate classification, reachability, holds, admission, permits, and accounting. |
| `worth-store-recovery-physics` | reorganize | Entry admission, source precedence, replay, checkpoint cutover, publication recovery, and readmission. |
| `worth-store-replication` | freeze until built | Artifact publication, peer admission, transfer, verification, cursoring, and convergence evidence. |
| `worth-store-retention` | grow by domain | Retention declarations, holds, anchors, evaluation, and expiry decisions. |
| `worth-store-s0-reclassification` | retire after migration | Move permanent claim behavior to its owner; move closeout evidence to certification. |
| `worth-store-schema-lineage` | freeze until built | Durable schema boundaries, lineage records, lookup, and compatibility inputs. |
| `worth-store-security` | major root reorganization | Scope, authenticity, custody/readmission, trust boundaries, and durable security declarations. |
| `worth-store-snapshots` | grow by domain | Snapshot identity, materialization, publication, lookup, and restore inputs. |
| `worth-store-subscription-support` | freeze until built | Durable cursors, resume contracts, retention interaction, and compatibility. |
| `worth-store-test-support` | reorganize | Cross-crate worlds, fixtures, deterministic drivers, faults, and assertions only. |
| `worth-store-tiering` | reorganize | Tier posture, placement, transition admission, IO readiness, and observation. |
| `worth-store-wal` | reorganize | Record vocabulary, append, flush/durability, publication, topology, and recovery reads. |

## Target Production Skeletons

The following trees are rough but opinionated targets. A named file may become
a directory when the responsibility requires decomposition. Implementers may
refine local shape, but may not replace permanent domains with milestone,
`layout_access`, generic handoff, or generic proof topology.

### Shared Semantics

```text
worth-store-contracts/src/
  artifact/
    identity.rs
    classification.rs
    compatibility.rs
  physical/
    identity.rs
    location.rs
  operation/
    identity.rs
  lib.rs

worth-store-aspect-native/src/
  authority/
  canonical_basis/
  ingress/
  projection/
  receipts/
  facade.rs
  lib.rs

worth-store-authority/src/
  source/
  current/
  retention/
  projection/
  readmission/
  facade.rs
  lib.rs

worth-store-security/src/
  declarations/
  scope/
  authenticity/
  custody/
  trust_boundary/
  readmission/
  projection/
  facade.rs
  lib.rs

worth-store-budgets/src/
  units/
  envelope/
  pre_execution/
  counters/
  denial.rs
  facade.rs
  lib.rs
```

`worth-store-aspect-native`, `worth-store-authority`, and
`worth-store-security` must lose physical-format imports. Their physical byte
representations become adapters owned by physical format.

### Physical Substrate

```text
worth-store-physical-format/src/
  format_identity/
  binary/
    framing/
    header/
    payload/
  records/
    page/
    extent/
    blob_manifest/
  manifest/
  reference/
  generation/
  security_encoding/
  decode_admission/
  access/
    page/
    frame/
    extent/
    segment/
    manifest/
    free_space/
    counters.rs
  bootstrap/
  facade.rs
  lib.rs

worth-store-physical-backend/src/
  capability/
  access_policy/
  durability/
  placement/
  operation/
  execution/
  facade.rs
  lib.rs

worth-store-buffer-pool/src/
  residency/
  pinning/
  record_access/
  dirty_pages/
  eviction/
  allocation/
  background_work/
  speculative_work/
  counters/
  facade.rs
  lib.rs

worth-store-wal/src/
  records/
    transaction/
    blob/
    security/
  append/
  checkpoint/
  durability/
  publication/
  topology/
  recovery_read/
  facade.rs
  lib.rs

worth-store-physical-integrity/src/
  admission/
    entry/
    physical_scope/
    pre_decode/
  authority/
  checksums/
  containers/
  manifests/
  index_pages/
  blob_chunks/
  wal_frames/
  quarantine/
  scrub/
  evidence/
  facade.rs
  lib.rs

worth-store-physical-isolation/src/
  epoch/
  latch/
  hazard/
  generation/
  reads/
    planning/
    stability/
    execution/
  checkpoint/
  compaction/
  publication/
  reclaim/
  byte_guard/
  counters/
  facade.rs
  lib.rs

worth-store-io-scheduler/src/
  resources/
  admission/
  foreground/
  background/
  queue/
  interference/
  dispatch/
  security_scope/
  counters/
  facade.rs
  lib.rs
```

The physical crates expose capabilities named for their own operation. They do
not export S.6/S.7/S.8 readiness, later-milestone handoffs, or layout-access
inventories.

### Durable Structures

```text
worth-store-layout-indexes/src/
  catalog/
    system_families/
  keyspace/
  strategy/
    registry/
    btree/
    lsm/
  materialization/
  access/
    shape/
    planning/
    budget/
    lowering/
    readiness/
    execution/
    degraded/
    counters/
  maintenance/
  evolution/
    migration/
    rollback/
    compatibility/
    rebind/
  integrity/
  bootstrap/
  customization/
  compaction_projection/
  facade.rs
  lib.rs

worth-store-blob-chunks/src/
  identity/
  integrity/
  lifecycle/
  placement/
  dedupe/
  publication/
  reachability/
  retention/
  reclaim/
  streaming/
    ingest/
    read/
    resume/
  export/
  import/
  recovery/
  compaction/
  capsule/
  facade.rs
  lib.rs

worth-store-branch-deltas/src/
  identity/
  layering/
  base_sharing/
  access/
  publication/
  compaction/
  facade.rs
  lib.rs

worth-store-snapshots/src/
  identity/
  materialization/
  publication/
  lookup/
  restore/
  facade.rs
  lib.rs
```

`layout_access` directories dissolve. A lower owner exports its ordinary
facade; layout indexes adapts the capability inside the exact domain that uses
it. Cross-crate qualification matrices live in certification.

### Recovery And Operations

```text
worth-store-recovery-physics/src/
  entry/
    declaration/
    admission/
  source_precedence/
  replay/
    redo/
    blob/
  checkpoint/
  publication/
  durability/
  corruption_readmission/
  security_scope/
  budget/
  evidence_projection/
  facade.rs
  lib.rs

worth-store-offline-verifier/src/
  discovery/
  scan/
  verification/
  findings/
  report/
  budget/
  facade.rs
  lib.rs

worth-store-maintenance/src/
  planning/
  admission/
  scheduling/
  execution/
  publication/
  counters/
  facade.rs
  lib.rs

worth-store-operations/src/
  backup/
    export/
    import/
  repair/
    blast_radius/
    quarantine/
    execution/
  audit/
  facade.rs
  lib.rs
```

Repair blast-radius readiness proves what physical scope repair may observe. It
does not establish operator authorization. Backup/import declarations crossing
a trust boundary are raw until admitted by their current owners.

### Policy, Lifecycle, And Extension Boundaries

```text
worth-store-claim-boundaries/src/
  declaration/
  classification/
  forbidden_claims/
  promotion/
  counters/
  facade.rs
  lib.rs

worth-store-modes/src/
  durable.rs
  embedded.rs
  absent.rs
  lifecycle.rs
  facade.rs
  lib.rs

worth-store-operations-vocabulary/src/
  backup/
  repair/
  replication/
  maintenance/
  lib.rs

worth-store-reclaim-policy/src/
  candidate/
  reachability/
  holds/
  admission/
  permit/
  counters/
  facade.rs
  lib.rs

worth-store-retention/src/
  declaration/
  anchors/
  holds/
  evaluation/
  expiry/
  facade.rs
  lib.rs

worth-store-tiering/src/
  posture/
  placement/
  transition/
  io_admission/
  observation/
  facade.rs
  lib.rs

worth-store-compatibility/src/
  format/
  schema/
  artifact/
  legacy_readmission/
  disposition/
  facade.rs
  lib.rs

worth-store-extensions/src/
  declaration/
  admission/
  registry/
  posture/
  compatibility/
  facade.rs
  lib.rs
```

Vocabulary crates contain declarations consumed by multiple owners. They do
not contain the operation, transition result, or authority merely because two
operations mention the same noun.

### Semantic Durability Programs

These crates are currently small, but their target structure is declared now
so future milestones grow into permanent domains rather than roadmap topology.

```text
worth-store-schema-lineage/src/
  schema_boundary/
  lineage_record/
  lookup/
  compatibility_projection/
  facade.rs
  lib.rs

worth-store-live-query/src/
  basis/
  resume_position/
  invalidation_input/
  observation/
  facade.rs
  lib.rs

worth-store-subscription-support/src/
  cursor/
  resume/
  retention_interlock/
  compatibility/
  facade.rs
  lib.rs

worth-store-replication/src/
  peer_admission/
  publication/
  transfer/
  verification/
  cursor/
  convergence/
  facade.rs
  lib.rs

worth-store-bulk/src/
  planning/
  admission/
  execution/
  checkpoint/
  publication/
  counters/
  facade.rs
  lib.rs

worth-store-analysis/src/
  declaration/
  basis/
  admission/
  execution/
  artifact_lifecycle/
  facade.rs
  lib.rs

worth-store-formal-models/src/
  durability/
  recovery/
  isolation/
  retention/
  refinement/
  lib.rs
```

Empty or placeholder crates remain frozen until their permanent owner operation
is implemented. A placeholder may not accumulate readiness recaps, speculative
contracts, or milestone handoffs in anticipation of future work.

### Product Composition And Runtime Admission

```text
worth-store-readiness/src/
  startup/
  durability/
  recovery/
  security/
  resources/
  denial.rs
  facade.rs
  lib.rs

worth-store/src/
  facade.rs
  composition.rs
  lib.rs
```

Every readiness module must name an actual runtime consumer and the owner-issued
capabilities it requires. If no production operation consumes the readiness,
the item is certification evidence or historical documentation and leaves the
crate. The product crate composes owners; it does not become a second owner of
their transitions.

### Courtroom And Harnesses

```text
worth-store-certification/src/
  courtroom/
    foundational/
    physical_integrity/
    durability/
    memory/
    scheduling/
    recovery/
    security/
    blobs/
    layout/
    cross_cutting/
  evidence/
    foundational/
    physical_integrity/
    durability/
    memory/
    scheduling/
    recovery/
    security/
    blobs/
    layout/
  scenario/
    definitions/
    planning/
    execution/
    transcripts/
  oracles/
  compile_fail/
  public_api.rs
  lib.rs

worth-store-physical-certification/src/
  actors/
  scenarios/
  schedules/
  faults/
  drivers/
  observation/
  oracles/
  coverage/
  transcripts/
  fixtures/
  facade.rs
  lib.rs

worth-store-test-support/src/
  worlds/
  fixtures/
  inputs/
  drivers/
  faults/
  assertions/
  scale/
  compile_fail/
  facade.rs
  lib.rs
```

Certification names permanent production domains. Milestone numbers remain in
specifications, runner state, and historical test reports, not in reusable Rust
vocabulary or directory topology.

## Cross-Crate Corrections

### Remove Layout Access As A Parallel Architecture

The repeated `layout_access` directories currently let many crates advertise
their participation in S.8 through a second API beside their ordinary domain
API. These directories are not a permanent subsystem.

Each file receives one of three dispositions:

1. move the real capability into its existing owner domain;
2. move a read-only vocabulary projection into layout indexes;
3. move qualification-only inventory into certification.

No replacement `layout_integration`, `layout_handoff`, or similarly generic
directory may preserve the same parallel graph.

### Remove Milestone Readiness From Production

`worth-store-readiness` is retained only for real runtime composition admission.
An item stays there only when a production startup or runtime operation consumes
it. Otherwise:

- production capability returns to its owner;
- certification conclusion moves to the courtroom;
- historical recap moves to documentation;
- synthetic handoff is deleted.

### Restore Lower-Crate Independence

Current manifests contain dependencies from lower physical crates into layout,
recovery, readiness, and other later owners. Some may be dev-dependencies, but
none is accepted merely because Cargo permits it.

The target production graph requires:

- aspect-native, authority, and security independent of physical format;
- physical format independent of layout indexes;
- physical isolation independent of recovery and layout indexes;
- IO scheduling independent of blobs, layout, recovery operations, and
  milestone readiness;
- certification and test support at the top only.

Adapters move to the higher-level consumer. Shared vocabulary moves downward
only when it has genuinely shared authority and no owner-specific behavior.

### Preserve Public API Stability Deliberately

Mechanical moves may retain an intentional public API through explicit facade
re-exports. They may not preserve deep module paths, milestone aliases, or raw
constructors solely to reduce migration work.

Every retained re-export must identify:

- the permanent owner;
- whether the export is authority or projection;
- the callers that require it;
- the eventual compatibility policy if its name is changing.

## Mechanical Migration Eligibility

A row is mechanically eligible only when:

1. source and target responsibilities are the same;
2. the file does not need to be split, merged, or deleted;
3. the target path is unique;
4. the move does not promote visibility;
5. public compatibility can be preserved through the owning facade;
6. no milestone or parallel-catalog abstraction survives merely under a new
   directory;
7. the affected crate can be compiled in the same bounded move batch.

The later mover must use `git mv`, update Rust module declarations and imports,
format affected crates, and run a focused `cargo check` after each batch. It
must stop on target collision, unclassified source, unexpected public API
change, or dependency inversion.

## Review Order Before Moving

Content review should refine the CSV in this order:

1. Shared contracts, authority, aspect-native, and security, because incorrect
   lower ownership contaminates every higher target.
2. Physical format, backend, buffer pool, WAL, integrity, isolation, and IO
   scheduling.
3. Layout indexes and all `layout_access` families.
4. Blob chunks and its cross-crate lifecycle adapters.
5. Recovery, offline verification, maintenance, and operations.
6. Readiness, certification, physical certification, and test support.
7. Future semantic crates and placeholders.

This ordering does not require implementation to wait for a perfect inventory.
It prevents high-level files from being assigned against a lower architecture
that is still changing underneath them.

## Completion Gates For These First Two Steps

The inventory and subsystem declaration are complete when:

- every current non-ignored Worth Store file has exactly one CSV row;
- every row names its crate and current directory;
- every row carries a proposed disposition, confidence, and review posture;
- every existing crate has a permanent responsibility or explicit retirement
  posture;
- the target trees cover every currently implemented critical subsystem;
- dependency direction is declared independently of current Cargo precedent;
- milestone and `layout_access` topology have explicit elimination rules;
- no mechanical move has yet modified production code.

They are not a claim that every file has been semantically audited. They are a
complete map of what exists, an opinionated declaration of where the system is
going, and a bounded queue for the deeper decisions that filenames cannot make.
