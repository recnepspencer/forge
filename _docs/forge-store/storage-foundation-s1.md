# Storage Foundation S.1 Engineering Spec: Physical Page, Segment, And Extent Substrate

> **Status:** Planned
>
> **Roadmap parent:** [forge_store_roadmap_2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_roadmap_2.md)
>
> **Vision parent:** [forge_store_vision.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_vision.md)
>
> **Test requirements:**
> - [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements.md)
> - [test-requirements-2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/test-requirements-2.md)
>
> **Prerequisite storage-foundation sequence:** `S.0`
>
> **Follow-on storage-foundation sequence:** `S.2`
>
> **Primary architectural driver:** define the physical byte universe of
> `forge-store` before any platform-grade backend can claim to be a database.

## Goal

Make page, segment, extent, frame, manifest, and physical-reference addressing
the mandatory substrate for the platform-grade Forge Store backend.

S.1 is complete when the platform-grade backend can persist, reopen, scan, and
locate representative authoritative and derived records through physical ids
without deserializing the whole store into heap domain objects, and when all
legacy heap/file/SQLite paths are fenced into explicit non-platform-grade
capability tiers.

## Why This Sequence Exists

The first Forge Store roadmap proved a large semantic durability program:
canonical authority, WAL-shaped recovery, snapshots, branch deltas, retention,
tiering, subscription support, and certification surfaces. That work remains
valuable, but it does not by itself make the store a physical database.

A real database needs a byte substrate that can be read, written, reopened,
verified, repaired, and evolved without relying on whole-store heap
materialization or backend-private residue. S.1 is the first foundation sequence
because every later Roadmap 2 capability depends on stable physical artifacts:

- S.2 needs pages and frames to bound resident memory.
- S.3 needs page/frame/chunk boundaries to reject damaged bytes before decode.
- S.4 needs physical references and manifests for WAL/checkpoint recovery.
- S.5 needs generation-bearing roots and references for stable read plans.
- S.8 needs artifact families to attach honest layout strategies.
- S.10 needs an offline verifier that can inspect bytes without live runtime
  construction.

If S.1 is weak, the rest of Roadmap 2 becomes decoration around a heap-shaped
store. This sequence exists to remove that ambiguity first.

## Governing Summaries

- `MENTALITY.md`
  protects adversarial, hard-problem-first foundation work. S.1 therefore
  starts with full-store materialization and backend-residue dependency as the
  enemy, rather than wrapping current persistence with nicer structs.
- `arch_laws.md`
  protects proof-bearing boundaries and authority separation. S.1 must create
  typed physical proof before logical decode and keep physical placement from
  becoming semantic truth.
- `perf_laws.md`
  protects visible cost and honest access boundaries. S.1 must expose exact
  page-read, page-write, frame-decode, allocation, scan, and manifest-lookup
  counters instead of calling physical access "fast" by assertion.
- `domain_laws.md`
  protects decomposition by reason-to-change. Pages, frames, segments, extents,
  allocation, manifests, references, legacy classification, and verification
  must be separate responsibilities because they fail and evolve differently.
- `forge_store_vision.md`
  protects the thesis that Store makes truth survive without owning truth
  semantics. S.1 gives survival a physical byte substrate while preserving
  canonical commit envelopes as semantic authority.
- `forge_store_roadmap.md`
  protects the first semantic durability sequence and now gates post-13.3
  platform work on Roadmap 2. S.1 belongs after S.0 reclassification and before
  S.2 memory residency because physical ids must exist before pages can be
  leased, pinned, or evicted.
- `forge_store_roadmap_2.md`
  protects the database-foundation backtrack. Its S.1 sketch requires fixed
  pages, variable extents, physical headers, slot directories, manifests,
  allocation classes, free-space maps, root manifests, and legacy-backend
  fencing.
- `test-requirements.md`
  protects machine-checkable closeout evidence. S.1 is not closeable until the
  `Physical page/segment/extent substrate test` proves physical access through
  persisted bytes and forbids heap-shaped shortcuts.
- `test-requirements-2.md`
  protects the adversarial harness realism. S.1 must use the storage-boundary
  interposer, adversarial backend, offline verifier, evidence bundle system,
  and coverage matrix rather than direct private-state mutation.

## Adversarial Constraint

S.1 must survive this hostile condition:

> A platform-grade backend stores representative authoritative and derived
> artifacts across multiple segments and extents, closes, reopens from persisted
> bytes, scans manifests, locates records by stable physical references, rejects
> stale generation references, emits physical layout evidence, and proves that
> no required lane used full-store heap materialization, full-domain-object
> deserialization, backend-private residue guessing, or legacy bootstrap paths
> to satisfy the database claim.

If a record can be found only by loading the whole store into domain structs,
if a physical reference can silently point to reused bytes, if the root manifest
is not self-describing enough for an offline verifier to walk the store, or if a
legacy backend can still present itself as platform-grade, S.1 is not closed.

## Product Decision Lock

- physical pages and extents are byte containers and access structures, not
  semantic truth
- canonical commit envelopes remain the semantic durability authority
- physical ids are stable within their generation and invalid across detected
  generation reuse
- page/frame headers carry physical decode evidence before any logical artifact
  decoder runs
- the root manifest is the entry point for physical discovery; backend-local
  directory layout is not authority
- slot directories or equivalent page-local record tables are mandatory for
  record addressing
- variable extents are admitted only through typed allocation classes and
  manifest membership
- legacy heap/file/SQLite paths may remain useful for bootstrap,
  compatibility, or semantic-certification lanes, but they may not satisfy S.1
  platform-grade physical-substrate claims
- S.1 reserves integrity fields such as checksum slots and generation fields,
  but full checksum enforcement belongs to S.3
- S.1 defines physical references and persisted record framing; buffer-pool
  pinning, eviction, resident-memory budgets, and zero-copy hot-path leases
  belong to S.2

## Scope

### In Scope

- physical id vocabulary for pages, segments, extents, frames, manifests, root
  manifests, allocation classes, free-space maps, and physical references
- fixed-size page model for ordinary records
- variable-size extent model for large records and future chunk families
- page and frame headers with kind, version, length, checksum slot,
  generation, publication state, and reserved LSN fields
- slot directory or equivalent page-local record addressing
- record framing independent of serde domain object materialization
- segment manifests, extent manifests, free-space maps, and root manifests
- platform-grade backend facade for physical record append, scan, locate,
  reopen, and manifest traversal
- stale-generation detection for physical references
- explicit capability-tier fencing for existing heap/file/SQLite paths
- first offline-verifier read path for S.1 physical structures
- S.1 evidence bundle outputs and exact physical counters

### Explicitly Out Of Scope

- bounded buffer pool, eviction, page leases, dirty-page budgets, and
  resident-memory certification, which are S.2
- full checksum, scrub, quarantine, and corruption-localization behavior, which
  are S.3
- WAL segment redesign, pageLSN replay law, checkpoint manifests, and recovery
  source precedence, which are S.4
- latch discipline, stable physical read plans, COW publication, and reclaim
  barriers, which are S.5
- hardware-aware I/O QoS, queue-depth management, and fsync profile
  qualification, which are S.6
- native blob chunk trees and streaming blob lifecycle, which are S.7
- per-artifact-family index/layout strategy selection, which is S.8
- formal models, operator repair, security, tenant boundaries, and final
  physical certification, which are S.9 through S.12
- query semantics, subscription semantics, truth validation, identity
  semantics, and runtime MVCC, which remain outside Store physical authority

## Required Contracts And Counters

### Physical Identity Contract

Required surfaces:

- `PhysicalPageId`
- `PhysicalSegmentId`
- `PhysicalExtentId`
- `PhysicalFrameId`
- `PhysicalGeneration`
- `PhysicalEpoch`
- `PhysicalRecordSlot`
- `PhysicalReference`
- `PhysicalRootReference`
- `StalePhysicalReference`

Rules:

- no physical reference may omit segment id, page or extent id, slot/frame
  position, and generation
- stale generation reuse must produce a typed stale-reference failure before
  logical decode
- physical ids are placement identities, not semantic artifact identities
- semantic artifact ids may point to physical references, but physical
  references may not replace semantic ids in canonical artifacts
- physical references may be serialized only through versioned framing owned by
  the physical substrate

Naive trap this prevents:

- reusing a row id, filename, SQLite rowid, or vector index as a durable
  database reference and discovering after compaction or restart that it was a
  placement hint, not a stable physical address.

### Page And Frame Header Contract

Required surfaces:

- `PhysicalPageHeader`
- `PhysicalFrameHeader`
- `PhysicalPageKind`
- `PhysicalFrameKind`
- `PhysicalFormatVersion`
- `PhysicalPublicationState`
- `PhysicalHeaderDecodeReport`
- `PhysicalHeaderDecodeWitness`

Header fields must include at minimum:

- kind
- format version
- header length
- payload length
- checksum slot or checksum descriptor reserved for S.3
- generation
- publication state
- segment id
- page or extent id
- reserved pageLSN or recovery cursor field for S.4

Rules:

- a header decode witness is required before frame payload access
- logical artifact decoders may consume only framed payload views, not raw
  backend bytes
- unknown page or frame kinds fail typed unless the format version declares an
  admitted compatibility posture
- publication state may describe only physical publication, never semantic
  commit truth
- S.1 may reserve checksum fields but must not pretend reserved checksum slots
  prove S.3 integrity behavior

Naive trap this prevents:

- storing a serde blob with a digest beside it and treating successful
  deserialization as proof that the physical storage substrate exists.

### Page-Local Record Addressing Contract

Required surfaces:

- `SlotDirectory`
- `SlotDirectoryEntry`
- `FramedRecordView`
- `FramedRecordPayload`
- `RecordPlacementClass`
- `RecordPlacementWitness`
- `RecordLocateReport`

Rules:

- ordinary records are located through page-local slot entries or an explicitly
  equivalent page-local addressing table
- record location must not require deserializing all records on a page
- deleted, moved, free, and reserved slots must be distinguishable in the slot
  directory
- record length and frame length must be checked before payload view creation
- a record view may expose bytes for later logical decode, but may not perform
  semantic interpretation inside the physical substrate
- extent-backed records must carry an extent reference from the page or
  manifest record that owns the large payload

Naive trap this prevents:

- scanning a whole file into `Vec<DomainRecord>` and then claiming the resulting
  vector offsets are page-local physical addressing.

### Segment, Extent, And Manifest Contract

Required surfaces:

- `SegmentManifest`
- `ExtentManifest`
- `PhysicalRootManifest`
- `SegmentMembershipRecord`
- `ExtentMembershipRecord`
- `AllocationClass`
- `FreeSpaceMap`
- `ManifestTraversalPlan`
- `ManifestTraversalReport`

Rules:

- every allocated page belongs to exactly one segment manifest
- every variable extent belongs to exactly one extent manifest or segment-owned
  extent membership record
- the root manifest is sufficient for a verifier to discover admitted segments,
  extents, format versions, allocation classes, and free-space maps
- allocation classes are physical-placement classes only; they may not encode
  semantic artifact authority
- free-space maps may report candidate placement and reuse state, but stale
  generation checks must still protect physical references
- manifest traversal emits exact counters for manifest reads, segment entries,
  extent entries, allocation-class entries, and free-space-map entries

Naive trap this prevents:

- letting the host filesystem directory tree, SQLite schema tables, or current
  backend object map be the real manifest while the Store manifest is only a
  decorative summary.

### Legacy Capability Fence Contract

Required surfaces:

- `StoreBackendCapabilityTier`
- `BootstrapBackend`
- `SemanticCertificationBackend`
- `CompatibilityBackend`
- `PhysicalFoundationBackend`
- `PlatformGradeBackend`
- `ForbiddenPlatformClaim`
- `LegacyBackendClassificationReport`

Rules:

- an existing heap/file/SQLite path may continue to run only under an explicit
  non-platform-grade capability tier until it satisfies Roadmap 2 gates
- certification lanes must be able to assert that the platform-grade S.1 suite
  did not route through legacy heap-shaped paths
- a backend may be useful and non-platform-grade at the same time; the tier is
  a claim boundary, not a value judgment
- a platform-grade backend must expose S.1 physical structures through the
  physical facade and offline verifier
- any API or documentation phrase that implies platform-grade posture for a
  legacy backend must map to an S.0/S.1 cleanup item

Naive trap this prevents:

- keeping old persistence paths around for compatibility while accidentally
  allowing tests, demos, or docs to use them as evidence that the real database
  substrate exists.

### S.1 Counter Contract

Required counters:

- `physical_page_read_count`
- `physical_page_write_count`
- `physical_frame_decode_count`
- `physical_header_decode_count`
- `physical_record_locate_count`
- `physical_manifest_lookup_count`
- `physical_manifest_traversal_count`
- `physical_segment_manifest_read_count`
- `physical_extent_manifest_read_count`
- `physical_allocation_count`
- `physical_free_space_map_update_count`
- `physical_root_manifest_read_count`
- `physical_root_manifest_publish_count`
- `physical_stale_generation_rejection_count`
- `physical_logical_decode_after_invalid_header_count`
- `physical_whole_store_materialization_attempt_count`
- `physical_legacy_backend_platform_claim_rejection_count`

Rules:

- required certification lanes must assert exact counter values or exact zeroes
  where the scenario demands them
- `physical_logical_decode_after_invalid_header_count` must remain zero
- `physical_whole_store_materialization_attempt_count` must remain zero for
  platform-grade S.1 lanes
- `physical_legacy_backend_platform_claim_rejection_count` must match hostile
  legacy-claim lanes exactly

## Phases

### Phase 1: Establish Physical Vocabulary And Capability Fences

Phase 1 creates the naming, tiers, and forbidden-claim boundaries that keep the
rest of S.1 honest.

Required work:

- define physical id, generation, epoch, reference, page, frame, segment,
  extent, manifest, allocation-class, and free-space-map vocabulary
- define `StoreBackendCapabilityTier` and platform-grade eligibility rules
- classify existing heap/file/SQLite paths as bootstrap, semantic
  certification, compatibility, physical foundation, or platform-grade
  candidates
- add forbidden-claim reports for any backend or test lane that attempts to
  satisfy S.1 through heap-shaped persistence
- define the S.1 physical facade surface separately from existing semantic
  store facade methods
- define the S.1 evidence bundle schema fields:
  - `physical_layout_report`
  - `artifact_digest`
  - `failure_digest`
  - `counter_snapshot`
  - `resource_envelope_report`
  - `hardware_assumption_report`

Exit condition:

- the repo can state which backends are allowed to claim S.1 evidence, and
  every old path has a typed non-platform-grade posture until proven otherwise.

### Phase 2: Define Physical References And Header Decode Proofs

Phase 2 makes raw bytes unobservable until the physical substrate has proven
what they are.

Required work:

- implement sealed construction for physical ids and references
- implement stale-generation-aware reference validation
- define page and frame header formats with reserved integrity and recovery
  fields
- define header decode reports and witnesses
- reject unknown or incompatible page/frame kinds before logical decode
- expose header-decode and stale-generation counters
- add compile-fail coverage for synthesizing physical decode witnesses outside
  the physical substrate authority

Exit condition:

- no logical decoder can consume bytes without a physical header decode witness,
  and stale generation reuse fails typed before payload access.

### Phase 3: Implement Page-Local Record Framing

Phase 3 turns pages into addressable containers rather than serialized bags.

Required work:

- implement fixed-size page storage for ordinary record families
- implement slot directory or equivalent page-local addressing
- distinguish occupied, deleted, moved, free, and reserved slot states
- implement framed record views with checked length and placement class
- support extent-backed record references for large payloads without loading the
  extent into a domain object
- expose page-read, page-write, frame-decode, record-locate, and page-local
  scan counters
- add negative lanes for out-of-range slots, moved records without follow-on
  extent reference, and length mismatch

Exit condition:

- representative records can be appended and located by physical page and slot
  without deserializing all records in the page or store.

### Phase 4: Implement Segments, Extents, Allocation Classes, And Manifests

Phase 4 gives pages and extents a discoverable physical universe.

Required work:

- implement segment manifests and segment membership records
- implement variable-size extent manifests and extent membership records
- implement root manifest discovery for admitted physical structures
- implement allocation classes for ordinary pages, large extents, manifests,
  free-space maps, and reserved future chunk families
- implement free-space maps with generation-safe reuse posture
- implement manifest traversal plans and reports
- expose allocation, free-space-map, root-manifest, segment-manifest, and
  extent-manifest counters
- add reopen-from-bytes lanes that discover the store through the root manifest,
  not backend-private directory or object maps

Exit condition:

- a fresh opener or verifier can discover segments, extents, free-space maps,
  allocation classes, and physical roots from persisted manifests alone.

### Phase 5: Build The Platform-Grade Physical Backend Facade

Phase 5 defines the backend-facing shape later Roadmap 2 sequences will harden.

Required work:

- implement physical append, read, scan, locate, root-manifest publish, and
  reopen operations for the S.1 platform-grade backend candidate
- keep semantic commit-envelope authority above the physical facade
- route physical operations through production-like storage boundaries so the
  test interposer can observe reads, writes, appends, flushes, renames, and
  opens
- reject full-store heap materialization in platform-grade physical lanes
- reject backend-private residue guessing as a locate or reopen strategy
- expose platform-grade physical counters at the facade boundary
- preserve bootstrap/compatibility paths behind explicit tier checks

Exit condition:

- the backend can persist and reopen representative records through physical
  facade operations, while forbidden shortcut lanes fail typed.

### Phase 6: Add S.1 Offline Verification

Phase 6 creates the first independent byte-inspection path required by Roadmap
2.

Required work:

- implement an S.1 verifier that walks root manifests, segment manifests,
  extent manifests, page headers, frame headers, slot directories, and
  free-space maps without constructing the live store runtime
- allow the verifier to produce a physical layout report independently from
  the live backend facade
- compare verifier-discovered physical references with runtime-discovered
  physical references
- emit disagreement reports rather than hiding verifier/runtime mismatch
- ensure verifier parsing does not call semantic artifact decoders except where
  a test explicitly asks for semantic parity after physical validation

Exit condition:

- S.1 evidence can be inspected from persisted bytes by a read-only verifier
  path that does not rely on live runtime state or backend-private memory.

### Phase 7: Prove The Physical Page/Segment/Extent Substrate

Phase 7 closes S.1 with the named suite and hostile shortcut lanes.

Required work:

- run the `Physical page/segment/extent substrate test`
- write representative authoritative and derived artifact records
- close and reopen the store from persisted bytes
- scan and locate records by physical identifiers
- assert page/frame headers, generation counters, root manifests, segment
  manifests, extent manifests, allocation classes, and free-space maps are
  internally consistent
- reject stale-generation physical references
- disable or instrument full-store heap materialization and prove platform-grade
  lanes do not use it
- run legacy-backend forbidden-claim lanes
- run offline verifier comparison lanes
- emit the required S.1 evidence bundle

Exit condition:

- S.1 closeout evidence proves physical persistence, reopen, scan, locate,
  stale-generation rejection, legacy-tier fencing, and forbidden-shortcut
  rejection through machine-checkable reports and exact counters.

## Must Ship

- physical id and reference model for pages, segments, extents, frames, slots,
  roots, epochs, and generations
- fixed-size page substrate for ordinary framed records
- variable-size extent substrate for large framed records and future chunk
  families
- physical page and frame headers with kind, version, length, checksum slot,
  generation, publication state, and reserved recovery fields
- header decode witnesses required before payload access
- slot directories or equivalent page-local record addressing
- framed record views independent of serde domain object materialization
- segment manifests, extent manifests, root manifests, allocation classes, and
  free-space maps
- manifest traversal plans and physical layout reports
- S.1 platform-grade physical backend facade for append, read, scan, locate,
  root publication, and reopen
- stale-generation detection and typed stale-reference failures
- legacy backend capability-tier classification and forbidden platform-grade
  claim reports
- S.1 offline verifier for root, segment, extent, page, frame, slot, and
  free-space structures
- exact counters for all S.1 physical access and shortcut-rejection claims
- machine-checkable S.1 certification bundle

## Must Preserve

- canonical commit envelopes remain semantic authority
- physical pages, frames, extents, manifests, and free-space maps remain byte
  substrate, not domain truth
- `forge-relational` remains owner of semantic MVCC, identity, transaction
  meaning, and truth validation
- existing semantic certification value from earlier Store milestones remains
  valid even when old backends are reclassified
- backend variation may change physical placement and cost, not artifact
  meaning
- physical publication state does not imply semantic commit visibility
- S.1 does not claim S.2 memory bounds, S.3 corruption localization, S.4
  recovery physics, S.5 read isolation, S.6 QoS, S.7 blob scale, or S.8 access
  strategy discipline

## Acceptance Evidence

S.1 is complete only when the store satisfies the Roadmap 2 named suite:

- `Physical page/segment/extent substrate test`

Required machine-checkable outputs:

- `physical_layout_report`
- `artifact_digest`
- `failure_digest`
- `counter_snapshot`
- `resource_envelope_report`
- `hardware_assumption_report`

Minimum certification matrix rows:

- `single_page_authority_reopen`
  persists an authoritative record on one page, closes, reopens from bytes, and
  locates it by physical reference.
- `multi_segment_authority_scan`
  persists records across multiple segments and proves manifest traversal, page
  scan, and record locate counters match expected breadth.
- `derived_record_non_authority`
  stores a representative derived record and proves its physical placement does
  not alter semantic artifact authority.
- `extent_backed_large_record`
  stores a large framed record through an extent reference without whole-store
  domain materialization.
- `root_manifest_discovery`
  opens the store through the root manifest and discovers segment manifests,
  extent manifests, allocation classes, and free-space maps.
- `offline_verifier_layout_match`
  walks persisted bytes through the S.1 verifier and matches runtime physical
  layout reports for admitted structures.
- `offline_verifier_runtime_disagreement_reported`
  injects a controlled verifier/runtime mismatch and proves disagreement is
  surfaced as evidence rather than hidden.
- `stale_generation_reference_rejected`
  attempts to use a physical reference after generation reuse and proves typed
  stale-reference rejection before logical decode.
- `unknown_frame_kind_rejected_before_decode`
  attempts to decode an unknown frame kind and proves logical decode is skipped.
- `length_mismatch_rejected_before_payload`
  corrupts or misdeclares frame length inside an S.1 lane and proves payload
  view construction is rejected before semantic interpretation.
- `slot_directory_locate_bounded`
  locates records by page-local slot without deserializing all records on the
  page.
- `free_space_generation_reuse_detectable`
  reuses free space with a new generation and proves old references do not
  silently resolve.
- `legacy_heap_backend_platform_claim_rejected`
  attempts to satisfy S.1 through a heap-shaped backend and proves the platform
  claim is rejected typed.
- `legacy_file_sqlite_backend_tier_fenced`
  proves existing file/SQLite paths are classified as bootstrap,
  compatibility, or semantic-certification unless they expose the S.1 physical
  substrate.
- `whole_store_materialization_forbidden`
  disables or instruments full-store heap materialization and proves all
  platform-grade S.1 lanes still pass with zero forbidden materialization
  attempts.
- `backend_residue_guessing_forbidden`
  attempts reopen or locate through backend-local residue not represented in
  the root/segment/extent manifests and proves rejection.
- `physical_counter_bundle_exact`
  proves page reads, page writes, frame decodes, header decodes, record locates,
  manifest lookups, allocations, stale-generation rejections, and forbidden
  shortcut counters match exact expected values.

Milestone-specific proof obligations:

- page, frame, segment, extent, root manifest, and free-space structures are
  self-describing enough for S.1 verification
- record location uses physical references and page-local addressing, not
  heap-domain scans
- physical references carry generation evidence sufficient to reject stale
  reuse
- logical decoders cannot consume bytes without header decode witnesses
- legacy backends cannot satisfy platform-grade S.1 evidence accidentally
- offline verifier output is independent enough to disagree with live runtime
  discovery and report that disagreement
- physical layout reports identify authoritative and derived record placement
  without making placement semantic authority
- no certification lane relies on logs, same-run self-comparison, or successful
  completion as proof
- `physical_logical_decode_after_invalid_header_count` remains zero
- `physical_whole_store_materialization_attempt_count` remains zero in required
  platform-grade lanes
- `physical_legacy_backend_platform_claim_rejection_count` matches hostile
  legacy-claim lanes exactly

S.1 is not closed by "records survived restart" tests if survival is proven
through a heap map, serde-loaded full object graph, SQLite row lookup without
the S.1 physical substrate, or backend-private file residue.

## Architectural Notes

- The smart abstraction is not a generic `StorageRecord`. The smart
  abstraction is a physical proof chain: root manifest to segment or extent
  manifest to page/frame header to slot or extent record to framed payload.
- S.1 should use names that teach the physical model directly. Prefer
  `PhysicalRootManifest`, `SegmentMembershipRecord`, and `StalePhysicalReference`
  over broad terms such as `StorageMetadata` or `RecordInfo`.
- Physical ids should be boring and strict. If they become convenient aliases
  for semantic artifact identity, the authority boundary has already leaked.
- A page header checksum slot is not a checksum program. S.1 may reserve and
  serialize integrity fields so S.3 can enforce them, but it must not claim
  corruption localization yet.
- The platform-grade physical facade should be narrow. Later sequences should
  harden it rather than every subsystem inventing a private byte path.
- Existing backends should not be shamed out of existence. They are useful for
  bootstrap, compatibility, and semantic certification, but those are different
  claims from platform-grade physical database posture.

## Sequencing Notes

S.1 belongs immediately after S.0 because the system must first stop
overclaiming existing persistence, then define the physical substrate that can
eventually earn stronger claims.

- S.1 consumes S.0 capability-tier vocabulary and deferred physical-guarantee
  mapping.
- S.1 must close before S.2 can honestly bound memory residency, because S.2's
  buffer pool needs page identities, frame boundaries, and manifests to lease.
- S.1 must close before S.3 can honestly localize corruption, because S.3 needs
  physical artifact boundaries to attack and quarantine.
- S.1 must close before S.4 can honestly define pageLSN, checkpoint, and WAL
  recovery physics, because recovery needs stable physical roots and references.
- S.1 may be designed while late 13.x documentation closes, but platform-grade
  implementation belongs inside the Roadmap 2 gate before Milestone 14.

## Required Self-Check

- Does the sequence solve a real structural problem or just package work
  cosmetically?
  Yes. It creates the physical byte-addressed substrate without which Store is
  still a semantic persistence harness rather than a database.
- Is the adversarial constraint precise and load-bearing?
  Yes. Every phase prevents full-store materialization, stale physical
  references, backend residue guessing, or legacy backend overclaiming.
- Does the sequence preserve crate authority boundaries?
  Yes. It gives Store physical byte authority while preserving canonical commit
  envelopes and `forge-relational` semantics as truth authority.
- Does the sequence define proof obligations, not just implementation tasks?
  Yes. It names the required suite, evidence outputs, matrix rows, exact
  counters, shortcut rejections, offline verifier behavior, and zero-count
  assertions.
- Could a competent engineer map this spec into honest types, modules, and
  tests?
  Yes. The spec names physical ids, headers, slot directories, manifests,
  allocation classes, backend tiers, verifier paths, counters, phases, and
  certification lanes.
- Does the sequence belong in this roadmap sequence, or is it out of order?
  Yes. S.1 follows S.0 claim reclassification and must precede memory,
  integrity, recovery, isolation, I/O, blob, layout, operations, security, and
  certification work.
