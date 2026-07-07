# Storage Foundation S.8 Engineering Spec: Index, Layout, And Access-Path Discipline

## Goal

Make Forge Store's physical access structures explicit, typed, costed,
rebuildable, versioned, and auditable per durable artifact family.

`S.8` is not a blob-only milestone. It is the storage-wide layout discipline
layer that prevents pages, WAL, recovery records, blob indexes, reclaim maps,
security/custody lookup structures, and future physical artifacts from
accidentally sharing a generic index story that hides cost or authority.

## Why This Milestone Exists

After `S.7`, Forge Store has native blob storage, lifecycle evidence, recovery
surfaces, streaming paths, reachability, reclaim, and physical placement
pressure. Those systems cannot scale safely if the next lookup path is chosen
by convenience.

This milestone turns physical layout into an explicit proof flow:

1. A durable artifact family declares what it is.
2. Store admits the layout strategy for that family.
3. Store lowers an access request into a typed access plan.
4. Store executes the plan through the declared path.
5. Store records exact access-path and amplification counters.
6. Store can rebuild, migrate, or deny the structure without losing authority.

The goal is not to add more proof nouns. The goal is proof grammar: visible
states, named transitions, exact denial cases, and public APIs that teach the
physical lifecycle.

## Non-Goals

- Do not build the application-facing query/indexing layer.
- Do not guess industry-specific semantic indexes.
- Do not expose a generic plugin callback that can smuggle an unproven index
  into the engine.
- Do not make certification the source of layout law.
- Do not treat Foundational layout or performance claims as executed Store
  proof.
- Do not make JSON, serde, logs, diagnostics, or report rows admission
  authority.

## Governing Constraint

No durable artifact family may be readable, searchable, streamed, mutated,
migrated, rebuilt, repaired, or certified through an implicit access path whose
key domain, comparator law, structure invariants, authority role, derivation
status, materialization state, coverage basis, absence proof, pre-execution
budget, observed cost, rebuild source, corruption behavior, mutation protocol,
version policy, or trust-boundary posture is undeclared.

Broad scans are allowed only when the scan itself is the declared access shape,
or when running a typed verifier, rebuild, migration, or diagnostic lane with
exact counters and no authority upgrade from the scan result alone.

An explicit degraded exact scan is allowed only as its own caller-visible,
budgeted, counter-backed, non-indexed outcome class. It must not masquerade as
point, prefix, range, streaming, locality-bounded, or cheap foreground access.

## Architectural Frame

### Store Owns Layout Law

Forge Store owns the physical artifact vocabulary, layout admission rules,
access-path lowering rules, and runtime counter semantics. Lower crates must
carry the law. Certification may prove the law was followed, but certification
must not define the law.

### Foundational Usage

Forge Foundational is used where Store crosses shared platform boundaries:

- `FoundationalLayoutIntentClaim` may describe the intended layout shape or
  performance posture, but it is not proof of Store execution.
- `FoundationalPolicyAdmissionReceipt` may record that a layout policy was
  admitted, but it is not enough to prove the access path executed correctly.
- `FoundationalCounterBackedPerformanceReceipt` is produced only from executed
  Store counters.
- `FoundationalPerformanceCounterRow` is used for boundary/reporting rows, not
  as internal Store authority.
- `FoundationalPerformanceReportPlan` and `MaterializedPerformanceReport` are
  terminal/reporting surfaces unless readmitted into a Store-owned witness.
- Certified or readmitted performance bundles may support trust-boundary
  movement, but Store must still preserve native authority internally.

The ladder is:

`layout intent claim -> Store layout admission -> Store executed counters ->
Foundational counter-backed receipt -> report/certification/readmission`.

The reverse direction is forbidden.

### Forge Proof Usage

Forge Proof is used to encode transition grammar:

- unresolved layout declaration
- resolved artifact-family layout declaration
- admitted layout strategy
- lowered access plan
- execution-ready access path
- executed access evidence
- counter-backed performance evidence
- rebuild, migration, rollback, or denial outcome

Use `TransitionOutcome` faithfully. Denied, deferred, stale, rebind-required,
and failed cases must remain distinct. A broad scan, stale layout version, or
incompatible authority source must not be collapsed into ordinary success.

### Authority And Projection

Physical indexes and layout projections are derived unless a Store-owned type
explicitly classifies a durable artifact as authoritative. Derived indexes may
accelerate access, but they must not become source truth through convenience.

### Physical Mechanics Lock

S.8 is a physical database foundation milestone, not only a proof-vocabulary
milestone. If Store admits B-tree, LSM, range, prefix, hash, chunk-tree,
manifest-walk, cache-assisted, or streaming access, the admitted strategy must
have enough concrete mechanics to make the claim mechanically testable.

S.8 must therefore introduce or consume Store-owned law for:

- physical key domains, canonical key encodings, comparators, prefix bounds,
  range bounds, composite-key ordering, hash collision behavior, and tenant/key
  scope partitioning
- baseline B-tree and LSM strategy families with real lookup, publication,
  mutation, integrity, and recovery invariants
- strategy invariant suites that make corruption, rebuild, migration, and cost
  expectations testable before a strategy is admitted
- materialization state, coverage basis, freshness watermarks, absence proofs,
  and derived accuracy class propagation
- deterministic physical plan selection, plan fingerprints, pre-execution cost
  budgets, planned counter envelopes, and planned-versus-observed receipts
- live index maintenance modes, mutation protocols, and publication states
- bootstrap catalog discovery that is tiny, fixed, versioned, checksummed, and
  incapable of answering ordinary access
- legacy access-path disposition and bypass proof for old Store surfaces
- layout/access-path hazard inventory for S.12 certification

## Known API Surface Map

This milestone must not leave API discovery to implementation time. The exact
S.8 type names may evolve during implementation, but the implementation must
start from these known surfaces and either consume them directly, adapt them
through narrow Store-owned wrappers, or introduce the planned S.8 surfaces named
below.

### Forge Proof Surfaces

Use `forge_proof` for progression law, not storage execution:

- `use forge_proof::prelude::*`
- `recipe(payload)`
- `.resolve_with(authority, basis)`
- `.lower_with(capability)`
- `.admit_with(authority)`
- `.ready_with(authority, runtime)`
- `.execute()`
- checked progression: `.try_resolve_ready(...)`, `.try_lower_ready(...)`,
  `.try_admit_ready(...)`, `.try_ready_now(...)`, `.try_execute()`
- trust-boundary/freshness progression:
  `.bridge_trust_boundary()`, `.rebind_with(...)`, `.readmit_with(...)`
- raw lane when the phase needs explicit transition nouns:
  `forge_proof::raw::*`, `Recipe<Unresolved, T>`, `Recipe<Resolved, T>`,
  `Recipe<Lowered, T>`, `ExecutionReadyRecipe<T, A>`, and
  `ExecutedRecipe<T, A>`
- non-success topology:
  `TransitionOutcome`, `SuccessfulTransitionOutcome`,
  `DenialTransitionOutcome`, `DeferredTransitionOutcome`,
  `FreshnessTransitionOutcome`, `ProofOutcome`, and `ProofOutcomeKind`
- deterministic shape helpers:
  `Pair<T>`, `NonEmpty<T>`, `CanonicalVec<T>`, `UniqueVec<T>`,
  `join_ready(...)`, `compose_ready(...)`, `family_pair(...)`, `create(...)`,
  `rewrite(...)`, `supersede(...)`, `retire(...)`, `sym(...)`, and
  `member(...)`

S.8 must use these surfaces for:

- artifact-family declaration progression
- layout strategy admission
- access-plan lowering
- execution readiness
- executed access evidence
- stale/rebind/readmission handling
- deterministic family composition when one Store operation touches a fixed set
  of layout families

### Forge Foundational Surfaces

Use `forge_foundational` for shared boundary vocabulary, not Store-owned layout
law:

- `forge_foundational::performance_api::common_path`
- `forge_foundational::performance_api::common_path::performance()`
- `FoundationalPerformanceClaimAuthoringFrontDoor`
- `FoundationalLayoutIntentClaim`
- `forge_foundational::performance_api::lower_lane::policy`
- `FoundationalPolicyAdmissionReceipt`
- `forge_foundational::performance_api::lower_lane::basis`
- `forge_foundational::performance_api::lower_lane::receipts`
- `counter_backed_performance_receipt(bundle)`
- `FoundationalCounterBackedPerformanceReceipt`
- `FoundationalPerformanceCounterRow`
- `forge_foundational::performance_api::lower_lane::reports`
- `FoundationalPerformanceReportPlan`
- `FoundationalMaterializedPerformanceReport`
- `forge_foundational::performance_api::stronger_lane::certified`
- `forge_foundational::performance_api::stronger_lane::readiness`
- `forge_foundational::performance_api::performance_public_surface_inventory()`
- `forge_foundational::canonicalization_api::{common_path, lower_lane,
  stronger_lane}`
- `forge_foundational::boundary_evidence_api::{common_path, lower_lane,
  stronger_lane}`
- `forge_foundational::aspects()`, `forge_foundational::compatibility()`, and
  native aspect carriers such as `AspectValue`, `StructAspectValue`,
  `AspectKey`, `ContractValidatedAspectValue`, and admitted authoritative
  aspect-state surfaces when S.8 evidence crosses aspect-native boundaries

The Foundational strength ladder for S.8 is fixed:

```text
Store layout declaration
-> Store layout admission
-> Store lowered access plan
-> Store executed access counters
-> FoundationalCounterBackedPerformanceReceipt
-> FoundationalMaterializedPerformanceReport or certified/readmitted bundle
```

The reverse direction is not legal.

### Existing Forge Store Surfaces To Consume Or Refactor

S.8 must inspect and either consume, wrap, migrate, or explicitly supersede
these current Store surfaces:

- public facade: `ForgeStore`, `ForgeStoreBuilder`, and the
  `crates/forge-store/src/facade` and `backend/facade` families
- current layout/chunk model:
  `AspectLayoutReadRequest`, `AdmittedAspectLayoutReadPlan`,
  `RejectedAspectLayoutReadPlan`, `AspectLayoutReadExecutionDecision`,
  `AspectLayoutReadExecutionResult`, `AspectReadRegime`,
  `AspectLayoutFallbackClass`, `AspectLayoutPerformanceEnvelope`,
  `Milestone6LayoutMaterialization`, `Milestone6PreparedLayoutSupport`,
  `Milestone6ResolvedLayoutSupportLane`, `ChunkModelFrozenPhysicalLayout`,
  `ChunkDeterminismWitness`, `PhysicalChunkId`, `StructuralBlockId`,
  `StructuralBlockLookup`, `DedupAdmittedBlockReuse`,
  `DedupBackedReadResult`, `Milestone7IndependentLayoutReference`, and
  `Milestone9PhysicalChunkReference`
- current media/WAL surfaces:
  `DurabilityBarrierClass`, `DurableBackendFamily`, `DurableMediaReport`,
  `WalRecord`, `WalRecordFamily`, `WalRecordPayload`, `DurableMutationId`,
  `DurablePublicationPhase`, `RecoveryDecisionClass`, and
  `CURRENT_WAL_VERSION`
- current compatibility/versioning surfaces:
  `ArtifactFamilyId`, `ArtifactFormatVersion`, `ArtifactSemanticVersion`,
  `ArtifactCompatibilityWindow`, `CompatibilityFamilyDeclaration`,
  `CompatibilityRegistry`, `CompatibilityReadIntent`,
  `CompatibilityWriteIntent`, `ReadCompatibilityReceipt`,
  `WriteCompatibilityReceipt`, `CompatibilityReadAdmissionOutcome`,
  `CompatibilityWriteAdmissionOutcome`, `CompatibilityRejection`,
  `CompatibilityRejectionKind`, `CompatibilityDerivedRebuildRequest`,
  `CompatibilityDerivedRebuildOutcome`, `RollingUpgradePolicy`,
  `RollingUpgradeAdmissionPlan`, `RestoreCompatibilityPlan`, and
  `MixedVersionStorePosture`
- current maintenance/retention/compaction surfaces:
  `MaintenanceDeclaration`, `AdmittedMaintenanceDeclaration`,
  `LoweredRetentionMaintenanceBatch`, `LoweredCompactionDeclaration`,
  `LoweredRebuildDeclaration`, `LoweredReclaimDeclaration`,
  `CompactionPlan`, `CompactionCutoverWitness`, `PublishedCompactionProduct`,
  `ReclaimEligibilityWitness`, `ReclaimExecutionReport`,
  `RetentionPlanningReport`, `RetentionClosureWitness`,
  `RetentionMaintenanceVerification`, `DerivedFamilyRetentionPolicy`, and
  `LayoutFamilyCompactionUnit`
- current recovery/snapshot/live-query/tiering surfaces:
  `RecoveryStatusReport`, `RecoverySourceReport`, `RecoveryQuarantineScope`,
  `DurableRecoveryOutcome`, `SnapshotImageBundle`, `SnapshotReadRequest`,
  `SnapshotReadResult`, `StableBasisReadRequest`, `StableBasisReadPlan`,
  `ContinuationBatchResult`, `ContinuationRetentionStatus`,
  `PlacementBoundArtifactRef`, `PlacementResolvedReadHandle`,
  `ResidentReadLease`, `ColdRecallLease`, `TierPlacementEvidence`, and
  `RecallAmplificationBudget`
- current counters and certification evidence:
  `StoreCounterSnapshot`, `Milestone6CounterContract`,
  `Milestone7CounterContract`, `Milestone10CounterContract`,
  `Milestone11CounterContract`, `Milestone12CounterContract`,
  `Milestone6AccessStructureVerification`,
  `Milestone7AccessStructureVerification`, and the S.4.5 harness/certification
  modules under `crates/forge-store/src/tests/harness`
- S.0 handoff and Roadmap 2 source-boundary surfaces:
  `StorageFoundationS1Handoff`, `EvidenceBundleReadiness`,
  `RoadmapSequenceStatusMatrix`, `S0ValidatedStorageFoundationS1HandoffArtifact`,
  `S0CounterSnapshot`, `S0ComplexityContract`, and `S0StableDigest`

### Planned Store Surfaces From S.1 Through S.7

S.8 is allowed to depend on the planned closed outputs of S.1 through S.7, and
must preserve their authority distinctions:

- S.1 physical substrate:
  `PhysicalPageId`, `PhysicalSegmentId`, `PhysicalExtentId`,
  `PhysicalFrameId`, `PhysicalRecordSlot`, `PhysicalGeneration`,
  `PhysicalEpoch`, `PhysicalReference`, `PhysicalRootReference`,
  `PhysicalPageHeader`, `PhysicalFrameHeader`, `PhysicalRootManifest`,
  `PhysicalManifestIndex`, `PhysicalFreeSpaceSearchPolicy`,
  `PhysicalFragmentationPressureReport`, `PlatformPhysicalFacade`,
  `OfflinePhysicalVerifier`, and `S2PhysicalSubstrateReadiness`
- S.2 buffer/memory surfaces:
  page leases, pin/unpin handles, resident-byte counters, pinned-page counters,
  dirty-page counters, allocation-scope counters, zero-copy/bounded-copy record
  views, read-ahead/write-behind admission, and OOM/admission denials
- S.3 integrity surfaces:
  page/frame/chunk checksums, pre-decode integrity validation, scrub evidence,
  corruption localization, quarantine witnesses, and readmission evidence
- S.4 WAL/recovery surfaces:
  WAL segments, page-LSN and checkpoint manifests, recovery-source precedence,
  replay receipts, recovery source reports, and bounded-recovery counters
- S.5 read-stability surfaces:
  stable physical read plans, physical leases, COW publication, generation
  validation, and reclaim barriers
- S.5.1 security-scope surfaces:
  key scope, key version, tenant scope, authenticity class, custody posture,
  repair blast-radius readiness, and import/readmission scope witnesses
- S.6 I/O isolation surfaces:
  foreground latency guard, I/O scheduler admission, direct-I/O posture,
  background work reservations, physical backend evidence, and
  `S6IoQosIsolationReadiness`
- S.7 blob surfaces:
  `BlobObjectId`, `BlobGeneration`, authoritative/derived blob
  classification, chunk identity, `LogicalContentDigest`, `StoredChunkDigest`,
  `AuthenticatedFrameDigest`, chunk-tree root, canonical chunking rule,
  `BlobGenerationPublished`, resumable ingest session states, dedupe admission,
  reachability edges, retention holds, reclaim receipts, placement movement,
  blob compaction, export/import bundle surfaces, heavy-blob fixture evidence,
  and sealed `S7NativeBlobStoreCloseout`
- S.7.1 proof-flow cleanup:
  `S7_1ProofFlowCleanupCloseout` or equivalent closeout evidence proving named
  transitions, phase-shaped topology, facade-only public access, and
  certification-as-courtroom boundaries before S.8 implementation begins

### S.8 Surfaces To Introduce

The implementation should introduce Store-owned surfaces equivalent to:

- `PhysicalArtifactFamily`
- `PhysicalArtifactFamilyDeclaration`
- `ArtifactFamilyAuthorityClass`
- `AuthorityRole`
- `ArtifactFamilyLifecycleClass`
- `ArtifactFamilyAccessLane`
- `DerivedAccuracyClass`
- `PhysicalKeyDomain`
- `CanonicalKeyEncoding`
- `ComparatorLaw`
- `PrefixLaw`
- `RangeBoundLaw`
- `HashCollisionLaw`
- `CompositeKeyOrderingLaw`
- `TenantScopedKeyDomain`
- `LayoutStrategyFamily`
- `LayoutStrategyDeclaration`
- `LayoutStrategyCapability`
- `LayoutStrategyInvariantSuite`
- `BTreeLayoutStrategy`
- `BTreeNodeFormatLaw`
- `BTreeSeparatorLaw`
- `BTreeSplitMergeLaw`
- `BTreeRootPublicationLaw`
- `LsmLayoutStrategy`
- `LsmMemtableWalLaw`
- `LsmRunPublicationLaw`
- `LsmTombstoneLaw`
- `LsmCompactionOrderingLaw`
- `LayoutVersion`
- `LayoutCompatibilityWindow`
- `LayoutMigrationPlan`
- `LayoutRollbackPlan`
- `LayoutMaterializationState`
- `PhysicalCoverageBasis`
- `LayoutCoverageWitness`
- `IndexWatermark`
- `CoverageGapWitness`
- `PhysicalAbsenceProof`
- `RangeCompletenessWitness`
- `PrefixCompletenessWitness`
- `LayoutAdmissionRequest`
- `AdmittedLayoutStrategy`
- `LayoutAdmissionDenial`
- `AccessShape`
- `AccessShapeContract`
- `PhysicalMutationShape`
- `AccessLoweringRequest`
- `AdmittedLayoutStrategySet`
- `AccessPathAlternative`
- `DeterministicPlanSelectionPolicy`
- `PlanSelectionReceipt`
- `PlanFingerprint`
- `AccessPlanCostEstimate`
- `AccessPlanBudget`
- `PlannedCounterEnvelope`
- `PlannedVsObservedCounterReceipt`
- `CostEnvelopeViolationOutcome`
- `LoweredAccessPlan`
- `ExecutionReadyAccessPlan`
- `ExecutedAccessPathEvidence`
- `AccessPathCounterSnapshot`
- `AccessPathAmplificationReceipt`
- `DerivedIndexRebuildPlan`
- `DerivedIndexParityWitness`
- `LayoutMutationPlan`
- `IndexMaintenanceMode`
- `IndexPublicationProtocol`
- `IndexLagWitness`
- `IndexMaintenanceFailureOutcome`
- `LayoutCorruptionClassification`
- `LayoutQuarantineWitness`
- `LayoutReadmissionWitness`
- `BootstrapLayoutCatalog`
- `MinimalRootDiscoveryLayout`
- `CatalogReadAdmission`
- `BootstrapOnlyAccessPath`
- `LegacyAccessPathBypassInventory`
- `LegacySurfaceDisposition`
- `ExplicitDegradedExactScan`
- `EphemeralAccessAid`
- `AdvisoryAccessAid`
- `CacheHitEvidence`
- `NeverAuthorityCacheClass`
- `S8LayoutHazardInventory`
- `StoreLayoutPerformanceReceipt`
- `StorageFoundationS9LayoutHandoff`

Names may change, but the conceptual surfaces may not disappear into raw
fields, untyped enums, or certification-only rows.

## Phase 1: Durable Artifact Family Inventory

### Purpose

Create the Store-owned vocabulary for durable artifact families before any
layout is admitted.

### Relevant APIs

- Existing Store inventory inputs: `ForgeStore`, `StoreCounterSnapshot`,
  `StorageFoundationS1Handoff`, `EvidenceBundleReadiness`,
  `RoadmapSequenceStatusMatrix`, `S0StableDigest`
- Existing artifact families to classify: `WalRecordFamily`,
  `CompatibilityFamilyKind`, `MaintenanceArtifactFamily`,
  `SupportArtifactFamily`, `PlacementArtifactFamily`,
  `PublicationFamily`, `DerivedFamilyRetentionPolicy`,
  `LayoutFamilyCompactionUnit`
- Planned S.1/S.7 family nouns:
  `PhysicalPageId`, `PhysicalSegmentId`, `PhysicalExtentId`,
  `PhysicalRootManifest`, `BlobObjectId`, `BlobGeneration`, chunk-tree root,
  reachability edge, retention hold, and reclaim receipt families
- S.8 surfaces to introduce: `PhysicalArtifactFamily`,
  `PhysicalArtifactFamilyDeclaration`, `ArtifactFamilyAuthorityClass`,
  `ArtifactFamilyLifecycleClass`, `ArtifactFamilyAccessLane`
- Proof surfaces: `recipe(...)`, `.resolve_with(...)`, `TransitionOutcome`

### Required Work

Define typed inventory surfaces for physical artifact families, including at
least:

- page, segment, extent, and root manifest artifacts
- WAL, checkpoint, replay, and recovery record artifacts
- blob chunk, blob manifest, blob stream, and chunk-tree artifacts
- dedupe, reachability, retention, reclaim, placement, and residency artifacts
- corruption, quarantine, repair, and readmission artifacts
- security/custody lookup artifacts when physical access depends on them
- export, import, capsule, and offline verification artifacts when durable

Each family must declare:

- whether it is authoritative, derived, diagnostic, terminal, or certification
  evidence
- whether it is hot-path, maintenance-path, verifier-path, or terminal-path
- whether it can be rebuilt, partially rebuilt, migrated, rolled back, or only
  quarantined
- which lower-crate boundary owns the production contract

### Implementation Shape

The expected shape is a small Store-owned vocabulary module, not a broad bag of
layout names:

```rust
let family = declare_physical_artifact_family(input);
let authority = classify_artifact_family_authority(&family);
let lifecycle = classify_artifact_family_lifecycle(&family);
let declaration = build_artifact_family_declaration(family, authority, lifecycle);
```

### Tests

- Every durable family currently admitted by Store has an explicit declaration.
- Derived families cannot be used as authority in compile-fail tests.
- Diagnostic, terminal, and certification artifacts cannot satisfy production
  layout admission.
- Missing family declarations produce a typed denial before access-path lowering.

### Closeout Gate

This phase is done when reviewers can list Store's durable physical families
without reading implementation predicates or certification harness code.

## Phase 2: Artifact Authority And Lifecycle Classification

### Purpose

Separate inventory from authority classification. Phase 1 names durable
families; this phase proves what each family is allowed to mean before any
layout strategy can be admitted.

### Relevant APIs

- Existing authority and projection inputs:
  `AuthoritativeExportBundle`, `VerifiedAuthoritativeAppend`,
  `PersistedAuthoritativeCommit`, `Milestone6LayoutMaterialization`,
  `CompatibilityDerivedRebuildOutcome`, `PublishedCompactionProduct`,
  `SubscriptionSupportAccessStructure`, `PlacementNonAuthorityWitness`
- Existing classification surfaces:
  `CompatibilityAuthorityClassification`, `CompatibilityFamilyKind`,
  `DerivedCompatibilityLaneKind`, `PublicationFamily`,
  `MaintenanceDeclarationClass`, `SupportTrustClass`
- Planned S.1/S.7 authority nouns:
  `PhysicalRootManifest`, segment/extent manifests, `BlobGenerationPublished`,
  authoritative blob classification, derived blob classification
- S.8 surfaces to introduce:
  `ArtifactFamilyAuthorityClass`, `ArtifactFamilyLifecycleClass`,
  `ArtifactFamilyAccessLane`, `ArtifactFamilyAuthorityWitness`
- Proof surfaces:
  `AuthorityWitness`, `Proof<...>`, `TransitionOutcome::denied(...)`

### Required Work

For every `PhysicalArtifactFamilyDeclaration`, classify:

- source truth versus derived projection
- diagnostic or terminal evidence versus production authority
- foreground, background, verifier, repair, export, or certification lane
- rebuildable, partially rebuildable, migrated, readmitted, quarantined, or
  non-rebuildable lifecycle posture
- whether a stale version can be read, must rebind, or must deny

This phase must produce the proof-bearing authority classification that later
strategy admission consumes. Later phases must not reclassify authority from
raw family names.

### Tests

- Authority parity: destroying all derived families and rebuilding them from
  declared authoritative sources yields the same derived access answers,
  physical coverage witnesses, declared accuracy classes, and parity
  witnesses. Family classification remains stable because it is
  declaration-derived, not projection-derived.
- Authority denial: a derived, diagnostic, terminal, or certification family
  cannot satisfy APIs that require production authority.
- Lifecycle denial: a stale or quarantined family cannot enter layout strategy
  admission without the required rebind or readmission witness.

### Closeout Gate

This phase is done when every durable family has one Store-owned authority and
lifecycle classification, and later phases consume that classification rather
than reconstructing it from names, counters, or records.

## Phase 3: Authority Roles, Accuracy Classes, And Scope Partitioning

### Purpose

Refine "authoritative versus derived" into the physical roles Store actually
needs before any layout strategy is declared.

### Relevant APIs

- Existing authority-adjacent surfaces:
  `PhysicalRootManifest`, `WalRecord`, `RecoverySourceReport`,
  `SnapshotImageBundle`, `TierPlacementEvidence`,
  `FoundationalCounterBackedPerformanceReceipt`,
  `AuthoritativeExportBundle`, `SupportTrustAccessPath`
- Planned S.5.1 metadata:
  key scope, key version, tenant scope, authenticity class, custody posture,
  repair blast-radius readiness, and import/readmission scope witnesses
- S.8 surfaces to introduce:
  `AuthorityRole`, `DerivedAccuracyClass`, `TenantScopedKeyDomain`,
  `ArtifactFamilyAuthorityWitness`, and scope-partitioned family declarations
- Proof surfaces:
  law 41 proof-bearing wrappers, `AuthorityWitness`,
  `TransitionOutcome::denied(...)`

### Required Work

Every physical family and strategy must declare its authority role, not only
whether it is broadly authoritative or derived. Roles include:

- semantic authority consumer, but not semantic authority owner
- physical discovery authority
- allocation authority
- recovery authority
- custody evidence authority
- performance evidence authority
- terminal transport evidence
- certification evidence

Every derived family must also declare its accuracy class:

- exact
- conservative
- approximate
- heuristic
- advisory

Accuracy class must flow into family declarations, strategy capabilities,
access shape contracts, access lowering, parity witnesses, and absence proofs.
Approximate, heuristic, or advisory structures may guide planning only under
explicit rules; they may not produce exact absence, exact reachability, delete
authority, reclaim authority, custody authority, or production read authority.

S.5.1 scope metadata must not be isolated to security lookup families. Every
ordinary physical layout strategy must declare whether it is single-tenant,
tenant-partitioned, cross-tenant with admitted equivalence, single-key-scope,
cross-key-scope denied, or cross-key-scope admitted under a named policy.

### Tests

- Accuracy denial: approximate/advisory structures cannot produce exact
  absence, exact reachability, reclaim authority, custody authority, or
  production read authority.
- Scope denial: a shared B-tree, hash index, chunk lookup, range map, or dedupe
  map cannot mix tenant/key scopes without an admitted scope-partition policy.
- Authority-role parity: root manifest physical discovery authority does not
  become semantic truth, and WAL recovery authority does not become final
  semantic authority.

### Closeout Gate

This phase is done when authority role, derived accuracy class, and tenant/key
scope posture are carried by every later layout and access proof type.

## Phase 4: Physical Key-Domain Law

### Purpose

Define how Store physical keys compare, encode, prefix-match, hash, and bound
ranges before any ordered or hashed strategy can be admitted.

### Relevant APIs

- Planned S.1/S.5.1/S.7 inputs:
  page ids, segment ids, extent ids, physical references, tenant scope, key
  scope, blob object ids, blob generations, chunk identity, root manifest keys
- Existing key-like surfaces:
  `ArtifactFamilyId`, `ArtifactFormatVersion`, `PhysicalChunkId`,
  `StructuralBlockId`, `DurableMutationId`, `WalRecordFamily`
- S.8 surfaces to introduce:
  `PhysicalKeyDomain`, `CanonicalKeyEncoding`, `ComparatorLaw`, `PrefixLaw`,
  `RangeBoundLaw`, `HashCollisionLaw`, `CompositeKeyOrderingLaw`,
  `TenantScopedKeyDomain`
- Foundational/proof surfaces:
  canonicalization readiness for boundary movement only; Store-owned key law
  remains the production authority

### Required Work

Define physical key-domain law for point, range, prefix, hash/equality,
composite, and tenant/key-scoped access. The law must specify:

- byte encoding and version bytes
- field order for composite keys
- null/sentinel/end-bound representation where relevant
- comparator behavior
- inclusive and exclusive range-bound behavior
- prefix boundary behavior
- hash collision behavior and collision verification
- tenant/key-scope prefixes or partitioning
- canonical ordering stability across restart, migration, and import

Range and prefix access are not legal claims until the relevant key-domain law
is admitted. Hash/equality access is not legal until collision behavior is
declared and tested.

### Tests

- Comparator parity: canonical key ordering is identical across restart,
  migration replay, and certification replay for the same key domain.
- Boundary denial: a range or prefix lookup cannot lower without admitted
  range-bound and prefix law.
- Collision denial: hash/equality lookup cannot claim exact identity without
  collision verification or a declared impossible-collision basis.
- Scope denial: tenant/key-scope prefixes must prevent cross-scope range,
  prefix, and hash reuse unless an admitted equivalence policy exists.

### Closeout Gate

This phase is done when range, prefix, hash, and composite access have physical
key law instead of relying on ad hoc byte or string comparisons.

## Phase 5: Strategy Invariant Suites And Baseline B-Tree/LSM Algorithms

### Purpose

Make B-tree and LSM claims real physical algorithms instead of vague strategy
labels.

### Relevant APIs

- Planned S.1/S.2/S.3/S.4/S.5 inputs:
  page/frame formats, buffer leases, checksums, WAL/page-LSN records,
  checkpoint manifests, stable physical read plans, and crash-replay evidence
- S.8 surfaces to introduce:
  `LayoutStrategyInvariantSuite`, `BTreeLayoutStrategy`,
  `BTreeNodeFormatLaw`, `BTreeSeparatorLaw`, `BTreeSplitMergeLaw`,
  `BTreeRootPublicationLaw`, `LsmLayoutStrategy`, `LsmMemtableWalLaw`,
  `LsmRunPublicationLaw`, `LsmTombstoneLaw`, `LsmCompactionOrderingLaw`
- Proof/counter surfaces:
  `TransitionOutcome`, `AccessPathCounterSnapshot`,
  `PlannedCounterEnvelope`, `PlannedVsObservedCounterReceipt`

### Required Work

S.8 must implement minimal but real baseline B-tree and LSM strategy families.
The milestone does not need every future optimization, but it must establish
the concrete algorithmic core.

The B-tree baseline must define:

- page/node format
- separator key law
- search and insertion path
- split behavior
- root publication behavior
- sibling-link posture, including explicit absence if not used
- occupancy constraints
- tombstone posture, if any
- stable-read behavior
- checksum scope and corruption localization
- rebuild and migration behavior

The LSM baseline must define:

- memtable/WAL relationship
- immutable sorted-run publication
- run lookup and merge order
- tombstone behavior
- manifest update protocol
- compaction ordering
- filter/bloom/advisory authority class if present
- stale run cleanup
- write-amplification accounting
- crash-safe publication and replay behavior

No strategy may be admitted because it merely advertises range or write
optimization. It must present an invariant suite that makes lookup, mutation,
corruption, rebuild, migration, and counter expectations mechanically testable.

### Tests

- B-tree invariant tests cover separator ordering, split/root publication,
  checksum localization, stable read behavior, and replay after interruption.
- LSM invariant tests cover WAL/memtable recovery, sorted-run lookup,
  tombstone preservation, manifest publication, compaction ordering, and stale
  run cleanup.
- Strategy denial: B-tree and LSM declarations without concrete baseline
  algorithms and invariant suites cannot enter layout admission.
- Counter parity: baseline B-tree and LSM lookups produce planned and observed
  counter receipts tied to the declared strategy.

### Closeout Gate

This phase is done when B-tree and LSM are concrete Store-owned baseline
strategies with tested invariants, not names over future work.

## Phase 6: Materialization, Coverage, Freshness, And Absence Proofs

### Purpose

Model the state and coverage of a specific layout instance before access plans
can claim exact answers.

### Relevant APIs

- Existing and planned state surfaces:
  `Milestone6LayoutMaterialization`, `SnapshotImageBundle`,
  `StableBasisReadPlan`, `RecoverySourceReport`, `BlobGenerationPublished`,
  checkpoint manifests, root epochs, WAL LSNs, and branch/snapshot bases
- S.8 surfaces to introduce:
  `LayoutMaterializationState`, `PhysicalCoverageBasis`,
  `LayoutCoverageWitness`, `IndexWatermark`, `CoverageGapWitness`,
  `PhysicalAbsenceProof`, `RangeCompletenessWitness`,
  `PrefixCompletenessWitness`
- Proof surfaces:
  freshness outcomes, rebind/readmission transitions, and exact/non-exact
  transition variants

### Required Work

Every specific layout instance must expose materialization state:

- declared only
- absent
- empty initialized
- building
- partially covered
- exact
- exact through physical basis
- lagged
- stale
- rebuild required
- migrating
- quarantined
- retired

Coverage must be physical and explicit. Examples include exact-through-LSN,
exact-through-root-epoch, exact-through-blob-generation, exact-through-checkpoint,
and partially covered range. Store does not decide semantic visibility here,
but it must honestly report the physical basis.

Absence must be proof-bearing. "Not found" from an exact index, "not found"
from a partially built projection, "not found" after an allowed bounded scan,
and "not found while the relevant range is quarantined" are different outcome
classes.

### Tests

- Absence denial: partially covered, quarantined, or stale layouts cannot
  return exact absence.
- Coverage parity: exact-through-LSN/root-epoch/blob-generation witnesses
  survive reopen and certification replay.
- Freshness denial: a correct format version with stale physical coverage must
  return stale/rebind/lagged evidence, not success.
- Gap localization: a coverage gap identifies the physical range, basis, and
  family that prevent exact access.

### Closeout Gate

This phase is done when exact read, exact absence, and exact parity are
impossible without materialization and coverage witnesses.

## Phase 7: Layout Strategy Vocabulary

### Purpose

Define the set of physical layout strategies Store can admit.

### Relevant APIs

- Existing layout surfaces:
  `AspectLayoutReadRequest`, `AdmittedAspectLayoutReadPlan`,
  `RejectedAspectLayoutReadPlan`, `AspectReadRegime`,
  `AspectLayoutFallbackClass`, `AspectLayoutPerformanceEnvelope`,
  `ChunkModelFrozenPhysicalLayout`, `StructuralBlockLookup`,
  `DedupAdmittedBlockReuse`
- Existing compatibility surfaces:
  `CompatibilityFamilyDeclaration`, `CompatibilityRegistry`,
  `ArtifactFormatVersion`, `ArtifactCompatibilityWindow`,
  `MixedVersionStorePosture`
- Foundational surfaces:
  `forge_foundational::performance_api::common_path::performance()`,
  `FoundationalLayoutIntentClaim`
- S.8 surfaces to introduce: `LayoutStrategyFamily`,
  `LayoutStrategyDeclaration`, `LayoutStrategyCapability`,
  `PhysicalKeyDomain`, `CanonicalKeyEncoding`, `ComparatorLaw`, `PrefixLaw`,
  `RangeBoundLaw`, `LayoutStrategyInvariantSuite`, `BTreeNodeFormatLaw`,
  `BTreeSeparatorLaw`, `BTreeSplitMergeLaw`, `BTreeRootPublicationLaw`,
  `LsmMemtableWalLaw`, `LsmRunPublicationLaw`, `LsmTombstoneLaw`,
  `LsmCompactionOrderingLaw`, `LayoutVersion`, `LayoutMaterializationState`,
  `LayoutAdmissionRequest`, `AdmittedLayoutStrategy`,
  `LayoutAdmissionDenial`, `PlannedCounterEnvelope`
- Proof surfaces: `Recipe<Resolved, T>`, `.lower_with(...)`,
  `.admit_with(...)`, `ProofOutcomeKind`

### Required Work

Introduce explicit strategy families such as:

- append log
- heap or slotted-page file
- page table
- baseline B-tree range structure
- baseline LSM write-optimized structure
- sparse index
- chunk tree
- manifest table
- bitmap, free-space map, or allocation map
- hash/equality index
- range map
- quarantine or corruption map
- streaming cursor index

Each strategy must declare:

- supported access shapes
- required physical key domains
- canonical key encoding and comparator law for any ordered or equality access
- prefix and range-bound law for prefix/range access
- required invariant suite
- B-tree laws when the strategy is B-tree based:
  node format, separator law, split/merge law, and root publication law
- LSM laws when the strategy is LSM based:
  memtable/WAL law, immutable run publication law, tombstone law, and
  compaction ordering law
- expected locality properties
- expected read and write amplification dimensions
- pre-execution counter envelope dimensions
- materialization-state compatibility
- rebuild source requirements
- corruption isolation behavior
- versioning and migration posture
- whether it is allowed on foreground, background, verifier, or terminal lanes

### Foundational And Proof Usage

Store may emit a `FoundationalLayoutIntentClaim` for shared boundary language,
but the Store-owned `LayoutStrategyDeclaration` is the production contract.
Forge Proof should encode declaration resolution and admission as separate
states.

### Tests

- A family cannot select an unsupported strategy.
- A strategy cannot advertise point, range, prefix, scan, or streaming support
  unless it declares the required key law, invariant suite, and counters.
- A terminal/report-only strategy cannot become hot-path execution authority.
- A strategy declaration without planned counter envelope fields cannot enter
  admission even before the counter phase deepens executed counter evidence.
- Compile-fail tests prove external crates cannot construct admitted layout
  strategies from raw fields.

### Closeout Gate

This phase is done when every allowed strategy has explicit access, key-domain,
invariant, cost, rebuild, corruption, and migration semantics.

## Phase 8: Layout Strategy Admission Registry

### Purpose

Turn strategy vocabulary into an admission registry that can reject unsupported
family/strategy combinations before access lowering exists.

### Relevant APIs

- Existing layout and compatibility inputs:
  `CompatibilityRegistry`, `CompatibilityFamilyDeclaration`,
  `CompatibilityAdmissionPlan`, `CompatibilityAdmissionReceipt`,
  `AspectLayoutPerformanceEnvelope`, `Milestone6AccessStructureContract`,
  `Milestone7AccessStructureContract`
- Foundational performance inputs:
  `FoundationalLayoutIntentClaim`,
  `FoundationalPolicyAdmissionReceipt`,
  `forge_foundational::performance_api::lower_lane::policy`
- S.8 surfaces to introduce:
  `LayoutAdmissionRequest`, `AdmittedLayoutStrategy`,
  `LayoutAdmissionDenial`, `LayoutStrategyCapability`,
  `LayoutStrategyRegistrySnapshot`, `LayoutMaterializationState`,
  `LayoutCoverageWitness`, `PhysicalAbsenceProof`, `AccessPlanBudget`,
  `IndexMaintenanceMode`, `PhysicalMutationShape`
- Proof surfaces:
  `.admit_with(...)`, `.try_admit_ready(...)`,
  `TransitionOutcome::{Success, Denied, Deferred}`

### Required Work

Build a registry that admits strategy declarations only after it can prove:

- the artifact family authority class allows the strategy
- the lifecycle class allows the strategy
- the strategy supports each requested access shape
- the required physical key-domain law exists
- the required invariant suite exists
- canonical key encoding, comparator, prefix, range-bound, and hash-collision
  law are compatible with the requested access shapes
- the requested strategy can produce the required materialization and coverage
  evidence for its authority role
- the strategy can produce the required physical absence proof class for any
  exact-not-found claim it advertises
- required counters and rebuild sources are declared
- pre-execution budget fields exist for planned access
- tenant/key/custody scope partitioning is compatible with the family and
  admitted access lane
- mutation and maintenance modes are compatible with the strategy's invariant
  suite and publication protocol
- version and migration posture are compatible with the family
- foreground/background/verifier/repair/terminal lane restrictions are honored

### Tests

- Admission parity: the same family declaration and strategy declaration admit
  deterministically across restart and certification replay.
- Admission denial: copied strategy rows, layout intent claims, policy receipts,
  or report rows cannot mint `AdmittedLayoutStrategy`.
- Capability denial: a strategy without streaming support cannot be admitted
  for blob streaming, and a strategy without range support cannot be admitted
  for ordered traversal.
- Key-law denial: a range, prefix, hash/equality, or composite-key strategy
  cannot admit unless the corresponding key-domain law is compatible with the
  family and requested lane.
- Materialization denial: a strategy that cannot produce the required
  materialization, coverage, or absence proof cannot advertise exact access.
- Scope denial: tenant/key/custody partitioning failures deny admission before
  lowering, even for ordinary B-tree, LSM, hash, manifest, chunk, or dedupe
  indexes.

### Closeout Gate

This phase is done when no later access phase can choose a strategy directly;
it must consume `AdmittedLayoutStrategy` or a stronger Store-owned proof.

## Phase 9: Customization Boundary For Future Higher Layers

### Purpose

Create a narrow customization boundary without guessing user or industry
indexes.

### Relevant APIs

- Existing higher-layer-adjacent surfaces:
  `AspectLayoutTarget`, `AspectProjectionSet`, `AspectScopeClass`,
  `StableBasisReadRequest`, `StableBasisReadPlan`,
  `SubscriptionSupportAccessStructure`, `SupportTrustAccessPath`,
  `SupportTrustAccessStructurePlan`
- Foundational aspect surfaces:
  `forge_foundational::aspects()`, `AspectKey`, `StructAspectValue`,
  `ContractValidatedAspectValue`, projection/mutation/diagnostic mask
  admission surfaces
- S.8 surfaces to introduce: `AccessShape`, `AccessShapeContract`,
  `LayoutStrategyCapability`, `LayoutAdmissionDenial`
- Proof surfaces: `CapabilityWitness`, `TransitionOutcome::denied(...)`,
  `TransitionOutcome::deferred(...)`

### Required Work

Define typed request vocabulary for future higher layers to ask for physical
capabilities without directly choosing unsafe implementation details.

Examples:

- point lookup over a declared key domain
- ordered range traversal over a declared key domain
- prefix traversal over a declared prefix domain
- streaming over a declared chunk or segment domain
- rebuildable secondary projection over a declared authority source
- verifier-only scan over a declared corpus

The boundary must make clear that higher layers may request access shapes and
workload envelopes, but Store admits the actual physical strategy.

### Tests

- A higher-layer request cannot inject a custom comparator, serializer, or scan
  callback as authority.
- A request for range access cannot be silently served by a whole-family scan
  unless the verifier/diagnostic lane is explicit.
- Unsupported capability requests return typed denials with the missing layout
  fact visible.

### Closeout Gate

This phase is done when S.8 gives future query/security/application layers an
extension point without turning physical layout into an untyped plugin system.

## Phase 10: Access Shape Contracts

### Purpose

Name access shapes before any lowering or execution path exists.

### Relevant APIs

- Existing Store access surfaces:
  `AspectLayoutReadRequest`, `AdmittedAspectLayoutReadPlan`,
  `AspectLayoutReadExecutionDecision`, `AspectLayoutReadExecutionResult`,
  `BranchDeltaReadRequest`, `BranchDeltaReadPlan`, `SnapshotReadRequest`,
  `SnapshotReadResult`, `PlacementResolvedReadHandle`, `ResidentReadLease`,
  `ColdRecallLease`
- Planned S.1/S.6/S.7 inputs:
  `PlatformPhysicalFacade`, physical read leases, stable physical read plans,
  `S6IoQosIsolationReadiness`, blob streaming cursor/resume indexes, chunk-tree
  root
- S.8 surfaces to introduce: `AccessShape`, `AccessShapeContract`,
  `PhysicalMutationShape`, `AccessShapeUnsupportedDenial`,
  `AccessLaneClassification`, `ExplicitDegradedExactScan`
- Proof surfaces: `CapabilityWitness`, `TransitionOutcome::denied(...)`,
  `TransitionOutcome::deferred(...)`

### Required Work

Define access shape contracts for:

- point lookup
- batch point lookup
- sorted batch lookup
- range lookup
- multi-range lookup
- prefix lookup
- grouped prefix lookup
- coalesced page read
- chunk-tree walk
- manifest graph walk
- bounded scan
- full declared scan
- streaming read
- streaming continuation read
- append
- compaction read
- rebuild read
- verifier read
- repair/quarantine read

Mutation shapes are not ordinary read access shapes. Update-in-place must live
under `PhysicalMutationShape` and requires WAL-before-data proof, page latch or
stable-read exclusion proof, page-LSN update behavior, checksum rewrite
behavior, crash replay behavior, and torn-write handling before a family may
admit it.

Explicit degraded exact scans are legal only as caller-visible, budgeted,
counter-backed outcomes. They cannot be selected as silent fallback for point,
range, prefix, streaming, or locality-bounded APIs.

Lowering must bind:

- artifact family
- admitted layout strategy
- layout version
- authority or derivation posture
- access shape
- foreground/background/verifier/terminal lane
- expected counter dimensions
- stale/rebind behavior

### Tests

- An unsupported access shape returns `Denied`, not a fallback scan.
- A verifier, rebuild, repair, or terminal access shape cannot be called from a
  foreground API without an explicit lane contract.
- Batch/coalescing denial: many legal point lookups cannot be lowered as a
  loop of scalar accesses when a batch, sorted-batch, multi-range, grouped
  prefix, coalesced page, chunk-tree walk, or manifest walk shape is required.
- Mutation denial: update-in-place cannot be admitted without the stronger
  physical mutation proof requirements.
- Shape parity: equivalent point/range/prefix/scan/streaming requests classify
  to the same access shape across replay.

### Closeout Gate

This phase is done when every access request first becomes a named shape with
declared lane, cost dimensions, and unsupported-denial behavior.

## Phase 11: Deterministic Plan Selection And Pre-Execution Budget Admission

### Purpose

Choose between admitted physical alternatives before execution and deny unsafe
plans before they run.

### Relevant APIs

- Existing planning/cost surfaces:
  `AspectLayoutPerformanceEnvelope`, `RecallAmplificationBudget`,
  `Milestone6CounterContract`, `Milestone7CounterContract`,
  `StoreCounterSnapshot`, `S6IoQosIsolationReadiness`
- S.8 surfaces to introduce:
  `AdmittedLayoutStrategySet`, `AccessPathAlternative`,
  `DeterministicPlanSelectionPolicy`, `PlanSelectionReceipt`,
  `PlanFingerprint`, `AccessPlanCostEstimate`, `AccessPlanBudget`,
  `PlannedCounterEnvelope`, `CostEnvelopeViolationOutcome`,
  `EphemeralAccessAid`, `AdvisoryAccessAid`, `CacheHitEvidence`,
  `NeverAuthorityCacheClass`
- Foundational/proof surfaces:
  performance readiness and counter-backed receipts after Store execution,
  not before; `TransitionOutcome::Deferred` and denied/degraded outcomes for
  budget or envelope failures

### Required Work

When more than one admitted strategy can answer a physical access shape, S.8
must produce a deterministic plan-selection receipt before lowering. Selection
must name:

- candidate strategies and their authority roles
- why each candidate is eligible or rejected
- deterministic tie-break or priority rule
- selected strategy
- plan fingerprint
- planned counter envelope
- memory, page, chunk, range, and byte budget
- allowed degraded exact scan posture, if any
- ephemeral or advisory access aids, if any, including why they are never
  authority

The executor must consume the selected, budget-admitted plan. It must not
re-run planner logic or silently broaden when observed counters exceed the
planned envelope. Exceeding the envelope must produce a distinct
`CostEnvelopeViolationOutcome`, `Deferred`, or typed denial.

Buffer-pool state, read-ahead state, hotness maps, bloom filters, plan caches,
and ephemeral cursors may participate only as declared access aids. A cache hit
must carry `NeverAuthorityCacheClass` or equivalent proof that it accelerates a
declared path without replacing the path's authority, coverage, or absence
proof.

### Tests

- Selection parity: the same strategy set, request, and physical basis produce
  the same plan fingerprint across restart, migration replay, and
  certification replay.
- Budget denial: a plan whose estimated page/chunk/range/byte budget exceeds
  the admitted envelope denies or defers before execution.
- Broadening denial: a selected point/range/prefix/streaming plan cannot widen
  into an undeclared scan when its primary path is unavailable.
- Cache denial: cache hits, bloom filters, hotness maps, and plan caches cannot
  answer as production authority or exact absence without the underlying
  admitted path proof.
- Regression guard: plan fingerprints detect unexpected strategy-selection
  drift after format migration or index rebuild.

### Closeout Gate

This phase is done when execution receives one deterministic, budget-admitted
plan instead of a set of tempting alternatives.

## Phase 12: Access Lowering And Proof Progression

### Purpose

Make every physical access path pass through a typed lowering and readiness
chain after shape classification.

### Relevant APIs

- Existing Store access surfaces:
  `AspectLayoutReadRequest`, `AdmittedAspectLayoutReadPlan`,
  `AspectLayoutReadExecutionDecision`, `AspectLayoutReadExecutionResult`,
  `BranchDeltaReadRequest`, `BranchDeltaReadPlan`, `SnapshotReadRequest`,
  `SnapshotReadResult`, `PlacementResolvedReadHandle`, `ResidentReadLease`,
  `ColdRecallLease`
- Planned S.1/S.6/S.7 inputs:
  `PlatformPhysicalFacade`, physical read leases, stable physical read plans,
  `S6IoQosIsolationReadiness`, blob streaming cursor/resume indexes, chunk-tree
  root
- S.8 surfaces to introduce: `AccessLoweringRequest`, `PlanSelectionReceipt`,
  `AccessPlanBudget`, `PlannedCounterEnvelope`, `LayoutCoverageWitness`,
  `LoweredAccessPlan`, `ExecutionReadyAccessPlan`,
  `ExecutedAccessPathEvidence`
- Proof surfaces:
  `Recipe<Unresolved, T>`, `Recipe<Resolved, T>`, `Recipe<Lowered, T>`,
  `ExecutionReadyRecipe<T, A>`, `ExecutedRecipe<T, A>`,
  `.ready_with(...)`, `.execute()`, `FreshnessTransitionOutcome`

### Required Work

Implement the proof progression:

```rust
let declaration = resolve_layout_declaration(family)?;
let admitted = admit_layout_strategy(declaration)?;
let access_shape = classify_access_shape(admitted, request)?;
let alternatives = enumerate_admitted_access_path_alternatives(access_shape)?;
let selected = select_access_path_deterministically(alternatives, policy)?;
let budgeted = admit_access_plan_budget(selected, workload_envelope)?;
let coverage = prove_layout_coverage(budgeted)?;
let access_plan = lower_access_shape(coverage)?;
let ready = prepare_access_execution(access_plan)?;
let executed = execute_access_path(ready)?;
let receipt = compare_planned_vs_observed(budgeted, executed)?;
```

The execution API may consume only `ExecutionReadyAccessPlan` or a stronger
type. It must not re-decide layout strategy, lane, broad-scan fallback,
version posture, materialization state, coverage basis, budget envelope, or
counter dimensions. Planned-versus-observed comparison is part of the proof
chain, not a diagnostics afterthought.

### Tests

- Stage denial: unresolved, resolved, admitted, or lowered access artifacts
  cannot be passed to execution APIs.
- Freshness denial: stale layout bindings return `RebindRequired`, not success
  or generic failure.
- Coverage denial: lowered plans without exact-enough materialization and
  coverage for their claimed outcome cannot become execution-ready.
- Cost denial: a plan that has no admitted budget or planned counter envelope
  cannot become execution-ready.
- Replay honesty: executing the same admitted access plan against the same
  authority basis yields the same executed evidence and counter dimensions.

### Closeout Gate

This phase is done when access execution is impossible without a lowered,
execution-ready access plan.

## Phase 13: Rebuild Source And Derived Index Consistency

### Purpose

Make derived structures honest about where they come from and how they recover.

### Relevant APIs

- Existing derived/rebuild surfaces:
  `Milestone6DerivedArtifactRebuildReport`,
  `CompatibilityDerivedRebuildRequest`,
  `CompatibilityDerivedRebuildOutcome`, `DerivedRebuildRequirement`,
  `LoweredRebuildDeclaration`, `RebuildDebtSummary`,
  `SupportRebuildAdmissionWitness`, `SupportRebuildEquivalenceWitness`
- Existing authority inputs:
  `AuthoritativeExportBundle`, `VerifiedAuthoritativeAppend`,
  `PhysicalRootManifest`, `WalRecord`, `SnapshotImageBundle`,
  `BlobGenerationPublished`
- S.8 surfaces to introduce: `DerivedIndexRebuildPlan`,
  `DerivedIndexParityWitness`, `LayoutCorruptionClassification`
- Proof surfaces: `Proof<...>`, `AuthorityWitness`, `UniqueVec<T>` for parity
  sets where order/uniqueness matters

### Required Work

For every derived index or layout projection, define:

- canonical authority source
- optional physical authority source if the family is explicitly authoritative
- rebuild scope
- partial rebuild key space
- parity check strategy
- corruption classification
- result identity after rebuild

Derived rebuild must preserve the authority/projection distinction. A rebuilt
projection may prove parity with authority, but it does not become authority
unless the family declaration already allows that.

### Tests

- Corrupt derived indexes rebuild to parity with authority.
- Corrupt authoritative artifacts quarantine or deny rather than silently
  rebuilding from projection data.
- Rebuild from certification rows, reports, logs, or JSON is rejected.
- Rebuild parity checks verify key identity, value identity, ordering identity,
  coverage identity, and declared cost-envelope compliance. Exact counter
  identity is required only for strategy families that explicitly claim
  deterministic physical shape.

### Closeout Gate

This phase is done when every derived access structure has a visible rebuild
source and a visible parity proof.

## Phase 14: Live Index Maintenance And Publication Protocols

### Purpose

Define how physical projections remain honest while writes, compaction,
migration, and background maintenance are happening.

### Relevant APIs

- Existing maintenance inputs:
  `LoweredRetentionMaintenanceBatch`, `LoweredCompactionDeclaration`,
  `LoweredRebuildDeclaration`, `CompactionPlan`, `CompactionCutoverWitness`,
  `PublishedCompactionProduct`, `ReclaimExecutionReport`
- Planned S.4/S.5/S.6/S.7 inputs:
  WAL/page-LSN evidence, checkpoint manifests, stable physical read plans,
  reclaim barriers, I/O scheduler readiness, blob generation publication,
  reachability edges, retention holds, and compaction evidence
- S.8 surfaces to introduce:
  `LayoutMutationPlan`, `IndexMaintenanceMode`,
  `IndexPublicationProtocol`, `IndexLagWitness`,
  `IndexMaintenanceFailureOutcome`, `PhysicalMutationShape`
- Proof surfaces:
  transition outcomes for synchronous update, async lag, rebuild-only,
  advisory-only, verifier-only, migration-only, cutover, denial, and deferral

### Required Work

Every derived physical layout must declare a maintenance mode:

- synchronous exact maintenance
- asynchronous exact but lagged maintenance
- rebuild-only projection
- lazy/materialized-on-demand projection
- advisory maintenance
- verifier-only projection
- migration-only projection

Every maintained layout must also declare its publication protocol and failure
outcomes. A live index may not be treated as exact unless its maintenance mode,
lag witness, publication protocol, and coverage basis justify that exactness.

Mutation support must be explicit. Update-in-place requires
`PhysicalMutationShape` admission with WAL-before-data proof, latch or stable
read/COW proof, page-LSN behavior, checksum rewrite behavior, torn-write
handling, and crash replay behavior.

### Tests

- Maintenance honesty: async-lagged, rebuild-only, lazy, advisory,
  verifier-only, and migration-only layouts cannot answer as synchronous exact
  indexes.
- Publication replay: index publication, compaction cutover, and migration
  cutover converge after crash/restart to the declared materialization state.
- Mutation denial: update-in-place without WAL/page-LSN/checksum/stable-read
  proof cannot be admitted as an execution-ready mutation.
- Lag localization: an index lag witness names the family, range, basis, and
  maintenance mode responsible for non-exact access.

### Closeout Gate

This phase is done when live index maintenance is a typed protocol rather than
an optimistic rebuild story.

## Phase 15: Corruption, Quarantine, And Readmission Semantics

### Purpose

Separate corruption handling from ordinary miss, stale, and unsupported cases.

### Relevant APIs

- Existing corruption/recovery surfaces:
  `RecoveryQuarantineScope`, `RecoveryStatusReport`, `RecoverySourceReport`,
  `DurableRecoveryOutcome`, `QuarantinedDecodedArtifact`,
  `CompatibilityRejection`, `CompatibilityRejectionKind`,
  `ManifestDigestMismatch`, `StaleDerivedVersionRejection`
- Planned S.3/S.5.1/S.7 surfaces:
  pre-decode integrity validation, scrub evidence, quarantine witnesses,
  readmission evidence, security-scope witnesses, blob corruption localization
- Foundational readmission surfaces:
  `forge_foundational::boundary_evidence_api::stronger_lane::readmission`,
  `forge_foundational::canonicalization_api::stronger_lane`
- S.8 surfaces to introduce: `LayoutCorruptionClassification`,
  `LayoutQuarantineWitness`, `LayoutReadmissionWitness`
- Proof surfaces:
  `.bridge_trust_boundary()`, `.rebind_with(...)`, `.readmit_with(...)`,
  `TransitionOutcome::stale(...)`, `TransitionOutcome::rebind_required(...)`

### Required Work

Define typed outcomes for:

- clean access
- not found
- unsupported access shape
- stale layout binding
- corrupted derived projection
- corrupted authoritative artifact
- quarantine required
- readmission required
- rebuild required
- migration required

Corruption must be localized to the family, layout strategy, version, and access
shape that observed it.

### Foundational Usage

Use Foundational boundary evidence only when corruption, quarantine, or
readmission crosses a trust boundary. Internally, Store keeps native typed
evidence.

### Tests

- Not-found cannot mask corruption.
- Corruption cannot be converted into an empty result.
- Quarantined physical artifacts cannot be read through ordinary access APIs.
- Readmission after backup/import or trust-boundary movement requires native
  Store witnesses before access resumes.

### Closeout Gate

This phase is done when the corruption path is a first-class state machine, not
an error string attached to an index lookup.

## Phase 16: Versioning, Migration, And Rollback

### Purpose

Make physical format evolution explicit before the engine depends on many
layout families.

### Relevant APIs

- Existing compatibility/version surfaces:
  `ArtifactFormatVersion`, `ArtifactSemanticVersion`,
  `ArtifactCompatibilityWindow`, `CompatibilityManifestIndex`,
  `CompatibilityRegistrySnapshot`, `RollingUpgradePolicy`,
  `RollingUpgradeAdmissionPlan`, `RollingWindowCompatibilityReceipt`,
  `RestoreCompatibilityPlan`, `RestoreCompatibilityReceipt`,
  `BackwardReadCompatibilityWitness`, `ForwardReadCompatibilityWitness`
- Planned S.1/S.4/S.7 version inputs:
  physical format version, root manifest version, WAL/checkpoint version,
  blob chunk-tree version, blob generation publication version
- S.8 surfaces to introduce: `LayoutVersion`,
  `LayoutCompatibilityWindow`, `LayoutMigrationPlan`, `LayoutRollbackPlan`
- Proof surfaces: `.try_resolve_ready(...)`, `.try_lower_ready(...)`,
  `.try_ready_now(...)`, `TransitionOutcome::RebindRequired`

### Required Work

Each admitted layout must declare:

- format version
- compatibility window
- read-old/write-new behavior, if supported
- dual-read or dual-write posture, if supported
- migration source and target
- rollback source and target
- interruption behavior
- stale binding detection

### Forge Proof Usage

Migration and rollback are proof transitions. They must not be plain helper
functions that mutate a layout descriptor in place.

### Tests

- Old compatible layouts can be read through an explicit compatibility lane.
- Incompatible layouts deny with a typed reason.
- Interrupted migration resumes or rolls back according to declaration.
- Rollback preserves authority and does not use derived projections as truth.

### Closeout Gate

This phase is done when layout evolution can be reasoned about without reading
ad hoc migration code.

## Phase 17: Amplification And Exact Counter Evidence

### Purpose

Attach real cost to every physical access path.

### Relevant APIs

- Existing counter surfaces:
  `StoreCounterSnapshot`, `Milestone6CounterContract`,
  `Milestone7CounterContract`, `Milestone10CounterContract`,
  `Milestone11CounterContract`, `Milestone12CounterContract`,
  `AspectLayoutPerformanceEnvelope`, `RecallAmplificationBudget`,
  `RetentionPlanningReport`, `ReclaimExecutionReport`
- Foundational performance surfaces:
  `FoundationalPerformanceCounterRow`,
  `FoundationalCounterBackedPerformanceReceipt`,
  `forge_foundational::performance_api::lower_lane::basis`,
  `forge_foundational::performance_api::lower_lane::receipts`,
  `counter_backed_performance_receipt(bundle)`,
  `FoundationalPerformanceReportPlan`,
  `FoundationalMaterializedPerformanceReport`
- S.8 surfaces to introduce: `AccessPlanCostEstimate`, `AccessPlanBudget`,
  `PlannedCounterEnvelope`, `AccessPathCounterSnapshot`,
  `AccessPathAmplificationReceipt`, `PlannedVsObservedCounterReceipt`,
  `CostEnvelopeViolationOutcome`, `StoreLayoutPerformanceReceipt`
- Proof surfaces: `ExecutedRecipe<T, A>` and `SuccessfulTransitionOutcome`
  only after Store execution has produced the counter rows

### Required Work

Define exact counters for at least:

- page touches
- index probes
- key comparisons
- range steps
- prefix steps
- chunk-tree node reads
- manifest reads
- bytes read
- bytes written
- write fanout
- read amplification
- write amplification
- compaction debt created or retired
- rebuild bytes read and written
- verifier bytes read
- cache or buffer residency effects where Store already models them

This phase deepens counter evidence; it does not introduce cost discipline for
the first time. Strategy declaration, strategy admission, plan selection,
access lowering, and execution readiness must already carry the planned
counter dimensions required for their stage.

Counters must be attached to transition outcomes, not incidental branches.
Successful execution must produce a planned-versus-observed receipt that names
the admitted plan fingerprint, planned envelope, observed counters, and any
deviation outcome.

### Foundational Usage

Only executed Store counters may produce
`FoundationalCounterBackedPerformanceReceipt`. Layout intent claims and policy
admission receipts are not counter proof.

### Tests

- Each admitted access family has counter assertions.
- A hidden broad scan fails the counter contract.
- A plan whose observed counters exceed the admitted envelope produces a typed
  violation outcome rather than ordinary success.
- Counter rows cannot be copied into readiness without executed Store evidence.
- Denied, stale, deferred, and failed outcomes still report the cost already
  spent where applicable.

### Closeout Gate

This phase is done when access-path cost is mechanically visible and cannot be
hand-waved by a claimed layout.

## Phase 18: Bootstrap Catalog And Minimal Root Discovery

### Purpose

Define the tiny fixed access path that discovers the layout catalog without
becoming an ordinary generic fallback.

### Relevant APIs

- Planned S.1 inputs:
  `PhysicalRootReference`, `PhysicalRootManifest`, `PhysicalManifestIndex`,
  `PhysicalPageHeader`, `PhysicalFrameHeader`, physical format version,
  checksums, and root-open evidence
- S.8 surfaces to introduce:
  `BootstrapLayoutCatalog`, `MinimalRootDiscoveryLayout`,
  `CatalogReadAdmission`, `BootstrapOnlyAccessPath`,
  `LayoutMaterializationState`
- Foundational/proof surfaces:
  boundary readiness only after Store has produced native root/catalog evidence

### Required Work

The bootstrap path must be:

- tiny
- fixed
- versioned
- checksummed
- readmission-gated after import/restore
- incapable of answering ordinary family access
- capable only of discovering the root manifest and layout catalog state

The bootstrap catalog must produce typed catalog read admission before ordinary
layout declarations, strategies, materialization states, or access plans can be
resolved. It must not become an alternate page table, manifest walker, or
fallback scan lane.

### Tests

- Bootstrap denial: bootstrap-only access cannot answer page, WAL, blob,
  security, export, maintenance, or recovery access directly.
- Catalog replay: the minimal root discovery path yields the same catalog state
  across reopen, crash replay, and certification replay.
- Corruption denial: corrupt bootstrap/root/catalog bytes quarantine or deny
  before ordinary layout resolution.
- Readmission denial: imported or restored catalog/root evidence cannot enter
  ordinary access without explicit readmission.

### Closeout Gate

This phase is done when Store can discover the layout catalog through a
mechanically bounded bootstrap path and no broader access path can hide inside
that exception.

## Phase 19: Page, Frame, Segment, And Extent Layout Families

### Purpose

Apply the new layout law to core physical payload containers.

### Relevant APIs

- Planned S.1 physical APIs:
  `PhysicalPageId`, `PhysicalSegmentId`, `PhysicalExtentId`,
  `PhysicalFrameId`, `PhysicalRecordSlot`, `PhysicalGeneration`,
  `PhysicalReference`, `PhysicalRootReference`, `PhysicalPageHeader`,
  `PhysicalFrameHeader`, `PhysicalRootManifest`, `PhysicalManifestIndex`,
  `PhysicalFreeSpaceSearchPolicy`, `PlatformPhysicalFacade`,
  `OfflinePhysicalVerifier`, `S2PhysicalSubstrateReadiness`
- Existing Store physical-adjacent APIs:
  `DurableBackendFamily`, `DurableMediaReport`, `DurabilityBarrierClass`,
  `PlacementBoundArtifactRef`, `PlacementResolvedReadHandle`,
  `TierPlacementEvidence`, `FamilyLocalPlacementPlan`
- S.8 surfaces to introduce: `PhysicalArtifactFamilyDeclaration`,
  `LayoutStrategyDeclaration`, `AccessShapeContract`,
  `AccessPathCounterSnapshot`
- Foundational surfaces:
  `FoundationalLayoutIntentClaim` for boundary description only and
  `FoundationalCounterBackedPerformanceReceipt` after executed counters

### Required Work

Admit layouts for:

- pages
- segments
- extents

Each family must have declared point/range/scan behavior where relevant,
declared authority posture, rebuild behavior, version posture, and counters.

### Tests

- Page and segment access cannot bypass layout admission.
- Frame and slot access cannot bypass layout admission.
- Page/segment/extent access has exact probe and byte counters.

### Closeout Gate

This phase is done when page, frame, segment, and extent access no longer use
implicit lookup helpers.

## Phase 20: Root Manifest, Allocation, Free-Space, And Placement Layout Families

### Purpose

Apply layout law to the discovery and allocation structures that tell Store
where physical artifacts live.

### Relevant APIs

- Planned S.1 physical APIs:
  `PhysicalRootManifest`, `PhysicalRootReference`, `PhysicalManifestIndex`,
  `PhysicalFreeSpaceSearchPolicy`, `PhysicalFragmentationPressureReport`,
  segment manifests, extent manifests, free-space maps, allocation classes
- Existing placement/tiering APIs:
  `PlacementBoundArtifactRef`, `PlacementResolvedReadHandle`,
  `TierPlacementEvidence`, `FamilyLocalPlacementPlan`,
  `CanonicalResidencyManifest`, `ResidentReadLease`, `ColdRecallLease`
- S.8 surfaces to introduce:
  manifest `PhysicalArtifactFamilyDeclaration`,
  allocation `LayoutStrategyDeclaration`, placement `AccessShapeContract`,
  placement `AccessPathCounterSnapshot`

### Required Work

Admit layouts for:

- root manifests
- segment and extent manifest indexes
- allocation classes
- free-space maps
- fragmentation reports
- placement and residency maps

Root manifest layout is discovery authority. Allocation, free-space, and
placement structures may accelerate reads and writes, but they must not become
data authority.

### Tests

- Discovery parity: reopen and offline verification discover the same admitted
  manifest graph from `PhysicalRootManifest`.
- Authority denial: free-space, placement, or residency projections cannot
  satisfy data-authority APIs.
- Broad-scan denial: append and locate paths cannot scan all pages, all
  segments, or all placement rows unless using a typed verifier/rebuild lane.

### Closeout Gate

This phase is done when physical discovery, allocation, free-space, and
placement layouts are declared separately from page/segment/extent payload
layouts.

## Phase 21: WAL And Checkpoint Layout Families

### Purpose

Apply S.8 to write-ahead and checkpoint structures before recovery and replay
indexes are admitted.

### Relevant APIs

- Existing WAL/media APIs:
  `WalRecord`, `WalRecordFamily`, `WalRecordPayload`, `DurableMutationId`,
  `DurablePublicationPhase`, `RecoveryDecisionClass`, `CURRENT_WAL_VERSION`,
  `DurabilityBarrierClass`, `DurableBackendFamily`
- Planned S.4 APIs:
  WAL segment declarations, checkpoint manifests, page-LSN replay law,
  durable publication records, checkpoint compatibility witnesses
- S.8 surfaces to introduce:
  WAL `PhysicalArtifactFamilyDeclaration`, checkpoint
  `LayoutStrategyDeclaration`, append `AccessShapeContract`
- Proof surfaces:
  `TransitionOutcome::{Denied, Stale, RebindRequired, Failed}` must remain
  distinct through replay and recovery lowering

### Required Work

Admit layouts for:

- WAL records
- checkpoints
- recovery manifests
- crash-boundary access structures

The implementation must distinguish WAL/checkpoint authority from later replay
indexes and must preserve crash/recovery ordering semantics.

### Tests

- Checkpoint indexes cannot replace WAL authority.
- WAL append and checkpoint lookup expose exact range, byte, and barrier
  counters.
- Corrupt checkpoint projections quarantine or rebuild without masking WAL
  authority corruption.

### Closeout Gate

This phase is done when WAL and checkpoint layouts have explicit authority,
access, version, corruption, and counter contracts before replay indexes are
introduced.

## Phase 22: Recovery, Replay, And Crash-Boundary Index Families

### Purpose

Apply S.8 to recovery and replay projections after WAL/checkpoint authority is
declared.

### Relevant APIs

- Existing recovery APIs:
  `RecoveryStatusReport`, `RecoverySourceReport`, `RecoverySourceKind`,
  `RecoveryQuarantineScope`, `DurableRecoveryOutcome`,
  `DurableRecoverySourceSummary`, `BackupRestoreCompatibilityReport`
- Existing snapshot APIs:
  `SnapshotImageBundle`, `SnapshotReadRequest`, `SnapshotRestorePlan`
- Planned S.4 APIs:
  recovery-source precedence witnesses, replay receipts, bounded-recovery
  counters, replay source reports
- S.8 surfaces to introduce:
  replay `PhysicalArtifactFamilyDeclaration`, replay `AccessShapeContract`,
  `DerivedIndexRebuildPlan`, `LayoutQuarantineWitness`
- Proof surfaces:
  `TransitionOutcome::{Denied, Stale, RebindRequired, Failed}` must remain
  distinct through replay and recovery lowering

### Required Work

Admit layouts for:

- replay indexes
- recovery records
- recovery source reports
- crash-boundary verifier indexes
- bounded WAL-tail lookup structures

Replay indexes are derived unless a lower Store type explicitly classifies a
recovery artifact as authoritative. Recovery scans are verifier/rebuild lanes,
not foreground fallbacks.

### Tests

- Replay parity: derived replay indexes rebuild from WAL/checkpoint authority
  and produce the same replay frontier.
- Authority denial: replay index rows cannot replace WAL/checkpoint authority.
- Crash-boundary denial: backend residue, leftover files, and successful
  filesystem enumeration cannot satisfy recovery layout admission.

### Closeout Gate

This phase is done when crash/recovery access has the same layout discipline as
ordinary foreground access without letting replay projections become authority.

## Phase 23: Snapshot, Branch, And Continuation Layout Families

### Purpose

Apply S.8 to durable read-support structures that are easy to forget because
they are not pages, WAL, or blobs.

### Relevant APIs

- Existing snapshot APIs:
  `SnapshotImageBundle`, `SnapshotReadRequest`, `SnapshotReadResult`,
  `SnapshotRestorePlan`, `PublishedSnapshotHandle`
- Existing branch/delta APIs:
  `BranchDeltaReadRequest`, `BranchDeltaReadPlan`, `BranchDeltaReadResult`,
  `BranchDeltaRewritePlan`, `BranchDeltaRebuildReceipt`,
  `SameBranchDescendantWitness`
- Existing live-query/continuation APIs:
  `StableBasisReadRequest`, `StableBasisReadPlan`, `ContinuationBatchResult`,
  `ContinuationRetentionStatus`, `CursorContinuationPlan`,
  `BroadenedBatchReceipt`, `AdmittedNarrowBatchReceipt`
- S.8 surfaces to introduce:
  snapshot, branch-delta, and continuation `PhysicalArtifactFamilyDeclaration`
  values; read-support `AccessShapeContract`; support `AccessPathCounterSnapshot`

### Required Work

Admit layouts for:

- snapshot images and snapshot indexes
- branch delta layers and delta rewrite products
- live-query stable-basis indexes
- cursor continuation support structures
- subscription-support access structures that act as durable read support

These families often look like semantic support, but S.8 owns their physical
access paths and derived-layout honesty.

### Tests

- Replay/convergence: branch, snapshot, and continuation read-support layouts
  rebuild from their declared authority and converge to the same read frontier.
- Authority denial: continuation, support, or snapshot projections cannot
  become canonical commit authority.
- Counter denial: broadened continuation reads cannot hide whole-family scans
  behind support-row counters.

### Closeout Gate

This phase is done when durable read-support structures have first-class
physical layout declarations instead of inheriting whatever layout their
feature originally used.

## Phase 24: Blob Object, Chunk-Tree, And Streaming Layout Families

### Purpose

Apply S.8 to the S.7 blob object and streaming structures before admitting
maintenance indexes that derive from them.

### Relevant APIs

- Planned S.7 blob APIs:
  `BlobObjectId`, `BlobGeneration`, authoritative/derived blob
  classification, chunk identity, `LogicalContentDigest`, `StoredChunkDigest`,
  `AuthenticatedFrameDigest`, chunk-tree root, canonical chunking rule,
  `BlobGenerationPublished`, resumable ingest states,
  `HeavyBlobFixturePlan`, and sealed `S7NativeBlobStoreCloseout`
- Existing Store chunk/layout APIs:
  `PhysicalChunkId`, `ChunkModelFrozenPhysicalLayout`,
  `ChunkDeterminismWitness`, `Milestone6ChunkModelExport`,
  `StructuralBlockLookup`, `DedupAdmittedBlockReuse`,
  `DedupBackedReadResult`, `Milestone7IndependentLayoutReference`,
  `Milestone9PhysicalChunkReference`
- S.8 surfaces to introduce: chunk-tree `LayoutStrategyDeclaration`,
  streaming `AccessShapeContract`, blob `AccessPathCounterSnapshot`,
  `DerivedIndexParityWitness`
- Foundational/certification surfaces:
  `FoundationalCounterBackedPerformanceReceipt` for executed blob counters and
  S.4.5 simulation replay bundles for heavy-blob evidence

### Required Work

Admit layouts for:

- blob chunk trees
- blob manifests
- streaming cursors or resume indexes
- blob generation publication records
- stored chunk lookup structures

Large blob tests must include a real multi-gigabyte locally generated corpus
through the S.7 harness plan, not a tiny synthetic fixture pretending to cover
large-blob behavior.

### Tests

- Streaming reads use declared streaming access paths.
- Blob identity is preserved across chunk-tree, streaming, and generation
  publication structures.
- Multi-gigabyte blob access records exact counters and does not silently route
  through whole-store scans.

### Closeout Gate

This phase is done when blob object, chunk-tree, and streaming layouts are
declared separately from blob maintenance projections.

## Phase 25: Blob Dedupe, Reachability, Retention, Reclaim, And Compaction Layout Families

### Purpose

Apply S.8 to blob maintenance projections only after the authoritative blob
object and chunk-tree families are explicit.

### Relevant APIs

- Planned S.7 blob maintenance APIs:
  dedupe admission, reachability edges, retention holds, reclaim receipts,
  placement movement, blob compaction, corruption quarantine, export/import
  bundle surfaces, and sealed `S7NativeBlobStoreCloseout`
- Existing maintenance/retention APIs:
  `RetentionPlanningReport`, `RetentionClosureWitness`,
  `ReclaimEligibilityWitness`, `ReclaimExecutionReport`,
  `CompactionPlan`, `CompactionCutoverWitness`, `PublishedCompactionProduct`,
  `LoweredCompactionDeclaration`, `LoweredReclaimDeclaration`
- S.8 surfaces to introduce:
  maintenance-family `PhysicalArtifactFamilyDeclaration`,
  `DerivedIndexParityWitness`, `LayoutQuarantineWitness`,
  `AccessPathAmplificationReceipt`, and maintenance counter snapshots
- Foundational/certification surfaces:
  counter-backed performance receipts for executed maintenance paths and S.4.5
  replay bundles for corruption, compaction, retention, and reclaim evidence

### Required Work

Admit layouts for:

- dedupe indexes
- reachability maps
- retention and hold maps
- reclaim queues
- compaction support structures
- corruption/quarantine maps

Dedupe, reachability, retention, reclaim, compaction, and quarantine structures
are derived maintenance evidence. They may guide work, but they do not become
blob identity or blob authority.

### Tests

- Dedupe indexes cannot become blob authority.
- Reachability and reclaim maps cannot delete or retain without the required
  authority witness.
- Compaction support structures cannot mutate the blob generation they derive
  from without an admitted cutover transition.
- Reclaim queues cannot convert missing reachability evidence into delete
  authority.

### Closeout Gate

This phase is done when every blob maintenance layout is visibly derived from
the blob authority graph and cannot masquerade as source truth.

## Phase 26: Maintenance, Tiering, And I/O Scheduler Layout Families

### Purpose

Apply S.8 to background work, tier movement, and I/O scheduling structures
after the concrete artifact families they operate on are explicit.

### Relevant APIs

- Existing maintenance and tiering APIs:
  `CompactionPlan`, `PublishedCompactionProduct`,
  `LoweredCompactionDeclaration`, `LoweredReclaimDeclaration`,
  `TierPlacementEvidence`, `FamilyLocalPlacementPlan`,
  `CanonicalResidencyManifest`, `ResidentReadLease`, `ColdRecallLease`
- Planned S.6 I/O QoS APIs:
  foreground/background admission, direct-I/O posture, compaction interference
  counters, scheduler readiness, and published I/O admission evidence
- S.8 surfaces to introduce:
  maintenance queue declarations, scheduler reservation indexes, tier placement
  layout declarations, recall queue access shapes, and interference counter
  snapshots
- Foundational/proof surfaces:
  performance readiness, counter-backed receipts, non-success scheduler
  outcomes, and replay evidence for background work admission

### Required Work

Admit layouts for:

- maintenance queues
- scheduler reservation indexes
- tier placement manifests
- cold recall queues
- working-set and recall amplification indexes
- foreground interference accounting structures

Foreground admission remains a scheduler decision backed by executed evidence.
Maintenance, tiering, and recall indexes may accelerate decisions, but they do
not publish I/O readiness by themselves.

### Tests

- Scheduler readiness consumes executed Store-published evidence, not copied
  queue rows.
- Tier placement and residency projections cannot replace artifact authority.
- Foreground interference counters are exact for the declared access shape and
  cannot hide background scans.

### Closeout Gate

This phase is done when maintenance, tiering, recall, and I/O scheduler layouts
are explicit accelerators with bounded authority.

## Phase 27: Security And Custody Lookup Layout Families

### Purpose

Apply layout discipline to physical structures that touch security and custody
without making Store an identity provider.

### Relevant APIs

- Planned S.5.1 security-scope APIs:
  key scope, key version, tenant scope, authenticity class, custody posture,
  repair blast-radius readiness, import/readmission scope witnesses
- Existing support/security-adjacent APIs:
  `RawSupportTrustRequest`, `SupportTrustRequestAdmitted`,
  `SupportTrustAccessPath`, `SupportTrustAccessStructurePlan`,
  `SupportTrustClassificationWitness`, `SupportTrustFreshnessWitness`,
  `SupportTrustReadmissionStatus`, `SupportImportAdmissionWitness`,
  `SupportManifestAdmissionWitness`, `SupportRoadmapPhysicalReadinessPosture`
- S.8 surfaces to introduce: security/custody
  `PhysicalArtifactFamilyDeclaration`, `LayoutReadmissionWitness`,
  `AccessShapeContract`, `LayoutQuarantineWitness`

### Required Work

Admit layouts for physical access structures tied to:

- tenant scope lookup
- key scope lookup
- custody posture lookup
- repair blast-radius lookup

These structures may accelerate security-boundary decisions, but they must not
turn identity-provider claims, JWT subjects, app org IDs, IAM roles, KMS key
IDs, or operator identities into Store authority.

This phase does not localize tenant/key/custody scope to security lookup
families. Phase 3 and Phase 8 require every ordinary physical layout strategy,
including shared B-trees, LSM runs, hash indexes, dedupe indexes, chunk
lookups, manifest indexes, and range maps, to declare whether it is
single-tenant, tenant-partitioned, cross-tenant-denied,
cross-tenant-admitted, single-key-scope, or cross-key-scope admitted under an
explicit policy. This phase only admits the lookup families that help inspect
or accelerate those scope decisions.

### Tests

- Security/custody projections cannot satisfy Store authority directly.
- JWT subject, application org ID, KMS key ID, IAM role, and operator identity
  examples are denied as Store authority.
- Repair blast-radius readiness proves where repair may physically observe or
  read; it does not prove who may initiate repair.

### Closeout Gate

This phase is done when security-adjacent layout structures are fast without
becoming an accidental auth system.

## Phase 28: Export, Import, Capsule, And Offline Verifier Layout Families

### Purpose

Apply S.8 to terminal and offline structures with explicit readmission at every
trust boundary.

### Relevant APIs

- Existing export/import/compatibility APIs:
  `AuthoritativeExportBundle`, `AuthoritativeExportRestoreRequest`,
  `BackupCompatibilityManifest`, `RestoreCompatibilityPlan`,
  `RestoreCompatibilityReceipt`, `RestoreBackupScope`,
  `CompatibilityRestoreExecutionOutcome`
- Planned S.7/S.10 export/import APIs:
  blob export/import bundle surfaces, capsule manifests, compatibility
  manifests, backup/restore evidence, and offline verifier evidence
- Foundational APIs:
  `forge_foundational::canonicalization_api::lower_lane::export`,
  `forge_foundational::canonicalization_api::stronger_lane`,
  `forge_foundational::boundary_evidence_api::stronger_lane::readmission`,
  identity and boundary-artifact surfaces for bridged/readmitted evidence
- S.8 surfaces to introduce:
  export/import `PhysicalArtifactFamilyDeclaration`,
  `LayoutReadmissionWitness`, terminal declaration parsing, offline verifier
  access shapes, and readmission counter snapshots

### Required Work

Admit layouts for:

- export/import bundle indexes
- capsule manifests
- offline verifier indexes
- backup/restore layout evidence
- imported layout readmission evidence

Serde and JSON representations are terminal declarations only. Deserialization
may produce raw declarations, but every deserialized security or layout value
must be readmitted before it can become a witness. Trust boundaries include a
different deployment, different Store instance, different key-scope generation,
different tenant-scope authority, different custody domain, offline
export/import, and backup restoration after key rotation.

### Tests

- Imported or restored layout evidence crossing a trust boundary requires
  explicit readmission.
- Offline verifier indexes cannot be used as foreground authority.
- Terminal serde/JSON declarations cannot become admission authority without
  readmission.
- Backup after key rotation, different Store instance, different tenant
  authority, and different custody domain are all denied without readmission.

### Closeout Gate

This phase is done when terminal/offline layout evidence is useful for
transport and verification but never enters Store authority without
readmission.

## Phase 29: Legacy Surface Disposition And Dedicated Workspace Boundary

### Purpose

Classify every old access-path surface before shaping the final S.8 public
facades.

### Relevant APIs

- Existing Store surfaces to disposition:
  `ForgeStore`, `ForgeStoreBuilder`, `AspectLayoutReadRequest`,
  `AdmittedAspectLayoutReadPlan`, `AspectLayoutReadExecutionDecision`,
  `Milestone6LayoutMaterialization`, `Milestone6PreparedLayoutSupport`,
  `ChunkModelFrozenPhysicalLayout`, `StructuralBlockLookup`,
  `DedupAdmittedBlockReuse`, `Milestone7IndependentLayoutReference`,
  `Milestone9PhysicalChunkReference`, compatibility registries, maintenance
  declarations, support-trust access structures, and certification helpers
- Roadmap 2 boundary:
  the dedicated Store workspace/crate family is the architectural foundation;
  old topology is migration input, not precedent
- S.8 surfaces to introduce:
  `LegacyAccessPathBypassInventory`, `LegacySurfaceDisposition`,
  `LegacySurfaceDispositionAndDedicatedWorkspaceBoundary`

### Required Work

Every legacy surface must receive one explicit disposition:

- consumed as production authority
- consumed as input only
- wrapped behind S.8 facade
- superseded and forbidden
- certification only
- terminal only
- deprecated debt, only if the debt rules are actually satisfied
- forbidden as authority

The inventory must identify old helpers, fallback classes, broad public
exports, certification fixtures, compatibility bridges, and support structures
that could skip declaration, admission, lowering, readiness, execution, or
readmission.

S.8 production authority must live in the dedicated Store workspace/crate
family. Existing `crates/forge-store/src/...` paths may be consumed or migrated,
but they may not define topology precedent.

### Tests

- Legacy bypass denial: old access structures, fallback classes, certification
  helpers, and broad facades cannot skip the S.8 declaration -> admission ->
  selection -> budget -> lowering -> readiness -> execution chain.
- Disposition parity: each legacy surface named in the inventory has a test or
  compile-fail proof matching its disposition.
- Dedicated-boundary denial: old topology paths cannot be deep-imported as
  production S.8 authority.

### Closeout Gate

This phase is done when there is no ambiguous old surface left that a future
caller could treat as a valid physical access-path shortcut.

## Phase 30: Public Facades And Directory Shape

### Purpose

Make the module topology teach the lifecycle.

### Relevant APIs

- Existing facades:
  `ForgeStore`, `ForgeStoreBuilder`, `crates/forge-store/src/facade`,
  `crates/forge-store/src/backend/facade`, `layout::mod`,
  `media::mod`, `wal::mod`, `maintenance::mod`, `retention::mod`,
  `tiering::mod`, `compatibility::mod`
- Existing public export pressure:
  the broad `crates/forge-store/src/lib.rs` re-export surface must be split or
  curated so S.8 public imports reveal lifecycle order
- S.8 public facade surfaces to introduce:
  `layout_families`, `layout_strategy_admission`, `access_lowering`,
  `access_execution`, `layout_rebuild`, `layout_migration`,
  `layout_counters`, `layout_readmission`, and `layout_certification`
  facades or equivalent lifecycle-shaped modules
- Compile-fail support:
  the S.8 UI suite must prove raw fields cannot construct admitted strategies,
  ready access plans, executed counter receipts, or readmitted witnesses

### Required Work

The public surface should be phase-shaped and authority-shaped. It should make
these questions obvious:

- Am I declaring a family, admitting a layout, lowering access, executing
  access, rebuilding, migrating, reporting counters, or certifying?
- Is this source truth, derived projection, diagnostic evidence, terminal
  export, or certification harness vocabulary?
- Is this foreground, background, verifier, repair, or terminal access?

Avoid broad `mod.rs` business logic. Avoid helper buckets. Keep classifiers,
transition functions, receipts, counters, and facade exports in directories that
match their semantic responsibility.

### Tests

- Compile-fail coverage proves external callers cannot construct admitted
  layout, ready access, executed counter receipts, or readmitted witnesses from
  raw fields.
- Public imports reveal lifecycle order rather than one flat ontology.
- Code-quality QA passes with composition, topology, and god-function findings
  cleared.

### Closeout Gate

This phase is done when an implementer can find the correct access-path phase
from the directory tree before reading function bodies.

## Phase 31: Certification Harness Vocabulary And Scenario Skeleton

### Purpose

Teach the harness vocabulary to describe layout law without becoming layout
law.

### Relevant APIs

- Existing harness/certification surfaces:
  `crates/forge-store/src/tests/harness`, corruption local-file helpers,
  `Milestone6AccessStructureVerification`,
  `Milestone7AccessStructureVerification`, `Milestone6CertificationBundle`,
  `Milestone7CertificationBundle`, `Milestone12CertificationRunner`,
  `Milestone12CertificationLaneInput`, `Milestone12CertificationLaneOutcome`
- S.4.5 harness concepts:
  scenario plans, production actors, fault injectors, observers, oracles,
  transcripts, replay bundles, coverage rows, shortcut denials
- Foundational readiness surfaces:
  `forge_foundational::performance_api::stronger_lane::readiness`,
  `forge_foundational::boundary_evidence_api::stronger_lane::readiness`,
  `forge_foundational::canonicalization_api::stronger_lane::readiness`
- S.8 surfaces to introduce:
  layout certification scenario fixtures, scenario skeleton vocabulary, layout
  performance evidence bundle, and `StorageFoundationS9LayoutHandoff`
- Proof surfaces:
  `ProofOutcomeKind`, `TransitionOutcome`, `ExecutedRecipe<T, A>` in harness
  assertions without letting harness code mint production authority

### Required Work

Extend the S.4.5 harness vocabulary so it can describe:

- layout declaration inventory tests
- access-shape denial tests
- broad-scan rejection tests
- exact counter tests
- corruption/rebuild parity tests
- migration/rollback interruption tests
- trust-boundary readmission tests
- multi-artifact integration simulations

Certification should consume production witnesses and executed evidence. It
must not define production constructors that lower crates depend on.

### Tests

- Harness helpers cannot mint production layout authority.
- Scenario skeletons identify production APIs, actors, fault injectors,
  oracles, counters, and closeout evidence before runtime simulations exist.
- Certification fixtures declare which production transition states ordinary
  Store code must pass through.

### Closeout Gate

This phase is done when the harness can describe S.8 evidence while remaining
the courtroom, not the law.

## Phase 32: Compile-Fail Authority And Facade Proofs

### Purpose

Prove the public S.8 surface cannot be bypassed from external crates.

### Relevant APIs

- Public facade APIs introduced in Phase 30
- Sealed S.8 authority surfaces:
  `AdmittedLayoutStrategy`, `LoweredAccessPlan`,
  `ExecutionReadyAccessPlan`, `ExecutedAccessPathEvidence`,
  `AccessPathCounterSnapshot`, `AccessPathAmplificationReceipt`,
  `DerivedIndexParityWitness`, and `LayoutReadmissionWitness`
- Existing compile-fail style from the S.4.5/S.5/S.6/S.7 UI suites
- Forge Proof surfaces:
  `TransitionOutcome`, non-success evidence, and executed recipe/certificate
  progression where applicable

### Required Work

Add compile-fail coverage proving external crates cannot:

- construct admitted layouts from raw fields
- execute weak artifact declarations
- mint counter receipts from copied rows
- readmit terminal/export/offline values without the readmission transition
- use certification fixtures as production authority
- deep-import internal family modules to skip facade order

### Tests

- Compile-fail tests cover every forbidden constructor class above.
- Public imports reveal lifecycle order rather than one flat ontology.
- Facade bypass attempts fail at compile time, not through runtime denial
  wrappers.

### Closeout Gate

This phase is done when external callers can only move through the phase graph
with the returned proof object from the previous valid transition.

## Phase 33: Runtime Simulation And Integration Certification

### Purpose

Run adversarial S.8 simulations only after the vocabulary, facades, and
compile-fail boundary are shaped.

### Relevant APIs

- S.4.5 harness runtime APIs:
  scenario plans, production actors, fault injectors, observers, oracles,
  transcripts, replay bundles, coverage rows, and shortcut denials
- Executed S.8 surfaces:
  layout declarations, admitted strategies, lowered plans, execution-ready
  plans, executed evidence, counter receipts, parity witnesses, readmission
  witnesses, and S.9 handoff
- Foundational surfaces:
  performance readiness, boundary-evidence readiness, canonicalization
  readiness, counter-backed performance receipts, and materialized reports
- S.7 heavy fixture surface:
  `HeavyBlobFixturePlan` or equivalent real multi-gigabyte local corpus plan

### Required Work

Run runtime simulations for:

- page, frame, segment, extent, root manifest, allocation, and placement
- WAL, checkpoint, recovery, replay, crash-boundary, snapshot, branch, and
  continuation
- blob object, chunk-tree, streaming, dedupe, reachability, reclaim, retention,
  compaction, and quarantine
- maintenance, tiering, recall, and I/O scheduler structures
- security, custody, export, import, capsule, and offline verifier boundaries

Each simulation must identify production APIs, exact counters, fault injection,
replay/rebuild/readmission expectations, and broad-scan rejection evidence.

### Tests

- Every admitted `LayoutStrategyFamily` has coverage for success,
  unsupported-shape denial, stale/rebind, corrupt-derived, corrupt-authority,
  rebuild, migration/rollback, hidden-scan denial, readmission, and
  cost-envelope violation unless the spec names an explicit equivalence class
  and proves why that family is covered by it.
- Integration simulations cover core physical families, recovery families, blob
  families, maintenance/I/O families, and security/custody/export boundary
  families through the coverage matrix rather than a single representative
  happy path.
- Fault simulations prove corrupted derived projections quarantine or rebuild
  without masking authoritative corruption.
- Scale simulations use the S.7 heavy fixture plan for large-blob access and
  exact counter evidence.

### Closeout Gate

This phase is done when S.8 has executed evidence across the major lifecycle
families, not just compile-time shape.

## Phase 34: Layout Hazard Inventory And S.9 State-Machine Handoff

### Purpose

Name the S.8 hazards and formal-model handoff state machines before milestone
closeout.

### Relevant APIs

- S.8 state machines:
  artifact declaration, key-domain admission, strategy invariant admission,
  layout admission, plan selection, budget admission, access lowering,
  execution readiness, executed evidence, materialization, coverage, absence,
  rebuild/parity, mutation maintenance, migration/rollback,
  stale/rebind/readmission, corruption/quarantine, bootstrap catalog, legacy
  disposition, and hidden-scan denial
- S.9 roadmap targets:
  WAL/checkpoint/page flush ordering, recovery source precedence, compaction
  cutover, physical leases, repair/quarantine, replication/import admission
- S.8 surfaces to introduce:
  `S8LayoutHazardInventory`, `StorageFoundationS9LayoutHandoff`,
  hazard detection/containment/recovery/proof-lane/residual-risk rows

### Required Work

Produce the S.8 hazard inventory that S.12 will certify later and S.9 can
model next. It must include at least:

- hidden broad scan accepted as bounded access
- stale index accepted as exact
- partial index returns false absence
- derived projection used as authority
- wrong tenant/key-scope index shared
- corruption converted into empty result
- migration reads derived data as rollback authority
- B-tree separator corruption misroutes lookup
- LSM tombstone lost during compaction
- cache hit bypasses layout admission
- legacy helper constructs ready plan
- copied counter rows become evidence without execution

The S.9 handoff must explicitly name the state machines S.8 created:

- layout admission
- access selection, budget, lowering, readiness, and execution
- derived rebuild and parity
- live index maintenance and publication
- migration and rollback
- stale, rebind, and readmission
- corruption and quarantine
- bootstrap catalog discovery
- hidden-scan denial and explicit degraded exact scan

### Tests

- Hazard coverage: every listed hazard maps to at least one compile-fail,
  runtime, simulation, or formal-model handoff lane.
- Handoff denial: S.9 cannot consume a generic layout summary that omits the
  state machines S.8 introduced.
- Residual-risk honesty: any residual risk must name detection, containment,
  recovery action, proof lane, and why it is not ordinary completion work.

### Closeout Gate

This phase is done when S.8 has a concrete hazard inventory and S.9 receives
the layout/access state machines it must model.

## Phase 35: Workspace-Wide Closeout And S.9 Handoff

### Purpose

Prove the milestone is complete and does not leave a hidden generic layout
fallback behind.

### Relevant APIs

- S.8 closeout must consume:
  `PhysicalArtifactFamilyDeclaration`, `AuthorityRole`,
  `DerivedAccuracyClass`, `PhysicalKeyDomain`, `CanonicalKeyEncoding`,
  `ComparatorLaw`, `PrefixLaw`, `RangeBoundLaw`, `HashCollisionLaw`,
  `CompositeKeyOrderingLaw`, `LayoutStrategyInvariantSuite`,
  `BTreeLayoutStrategy`, `LsmLayoutStrategy`, `LayoutMaterializationState`,
  `LayoutCoverageWitness`, `PhysicalAbsenceProof`,
  `AdmittedLayoutStrategy`, `PlanSelectionReceipt`,
  `AccessPlanBudget`, `PlannedCounterEnvelope`, `LoweredAccessPlan`,
  `ExecutionReadyAccessPlan`, `ExecutedAccessPathEvidence`,
  `AccessPathCounterSnapshot`, `AccessPathAmplificationReceipt`,
  `PlannedVsObservedCounterReceipt`, `DerivedIndexParityWitness`,
  `LayoutMutationPlan`, `IndexMaintenanceMode`,
  `LayoutReadmissionWitness`, `BootstrapLayoutCatalog`,
  `LegacyAccessPathBypassInventory`, `S8LayoutHazardInventory`, and
  `StorageFoundationS9LayoutHandoff`
- Foundational closeout must consume:
  `FoundationalCounterBackedPerformanceReceipt`,
  `FoundationalPerformanceReportPlan`,
  `FoundationalMaterializedPerformanceReport`, certified/readmitted
  performance bundles, boundary-evidence readiness, and canonical readiness
- Proof closeout must consume:
  executed proof progression, non-success outcome evidence, stale/rebind
  evidence, and compile-fail proof that weaker stages cannot enter stronger
  APIs
- Existing Store closeout must cover:
  page/frame/segment/extent/root-manifest/allocation/placement families,
  WAL/checkpoint/replay/recovery/crash-boundary families,
  snapshot/branch/continuation families, blob/chunk/streaming/reachability/
  retention/reclaim/compaction families, maintenance/tiering/I/O scheduler
  families, security/custody/export/import/offline families, and
  compatibility/version families

### Required Work

Run focused closeout across:

- production layout declarations
- lower-crate visibility boundaries
- compile-fail authority tests
- runtime integration tests
- code-quality QA
- test-quality QA
- performance-counter assertions
- planned-versus-observed budget assertions
- baseline B-tree and LSM invariant assertions
- materialization, coverage, and absence-proof assertions
- legacy surface disposition proof
- S.8 hazard inventory and S.9 state-machine handoff
- docs and roadmap consistency

### Required Verification

The exact command list may evolve with crate names, but closeout must include
focused checks equivalent to:

```text
cargo check -p forge-store-physical-isolation --features certification-authority
cargo check -p forge-store-certification
cargo test -p forge-store-certification --test s8_layout_access_path_harness
cargo test -p forge-store-certification --test s8_layout_corruption_rebuild
the relevant compile-fail UI suite for S.8 layout authority
the workspace Rust line-cap and code-quality guards
```

### Closeout Gate

`S.8` is not closed until each admitted durable artifact family has:

- declared layout strategy
- admitted physical key-domain law
- tested strategy invariant suite, including baseline B-tree/LSM where claimed
- declared access shapes
- declared authority or derivation posture
- declared authority role, accuracy class, and tenant/key-scope partition
- materialization state, coverage basis, freshness, and absence-proof behavior
- deterministic plan-selection receipt and plan fingerprint
- pre-execution cost budget and planned counter envelope
- bounded access-path counters
- planned-versus-observed counter receipt
- live maintenance/publication protocol where derived state is maintained
- corruption and rebuild behavior
- version and migration posture
- trust-boundary behavior where relevant
- bootstrap/catalog behavior where relevant
- legacy-surface disposition where relevant
- S.8 hazard inventory coverage
- compile-fail protection against raw construction
- runtime tests proving the honest path and adversarial denials

No required family may fall back to an implicit whole-store scan where the
roadmap claims indexed, locality-bounded, streaming, or strategy-bound access.
