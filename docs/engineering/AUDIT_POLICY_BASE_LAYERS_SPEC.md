# Audit / Policy Base Layers Spec (Foundation for Region Merge Follow-On)

Status: Draft for implementation

Last updated: 2026-02-25

## Purpose

This spec defines the shared base-layer contracts required before implementing:

1. Full policy protocol integration (`PolicyQuery` / `PolicyResult`)
2. Precision / `SurfaceRelation::Undetermined` semantics
3. Persistent naming + lineage integration for region merge
4. Rich serializable region-merge audit artifacts beyond `MergeResultSummary`

These are cross-cutting contracts. Implementing them ad hoc inside region merge will create drift in:
- typed error encoding
- policy trace payloads
- provenance representation
- envelope/context metadata lifecycle
- audit schema versioning/naming

This spec exists to prevent that.

## Source Alignment (Architecture Inventory Cross-Check)

This spec is grounded in the existing components already in code (see `ARCHITECTURE_COMPONENT_INVENTORY.md`):

- `forge-core`
  - `PolicyKind`, `PolicyQuery`, `PolicyResult<T>`
  - `TracedDecision`, `DecisionKind`, `DecisionTier`, `DecisionContext`, `DecisionLog`
  - `OperationResult<T>`, `OperationMetrics`, `KernelWarning`, `LineageDelta`
- `forge-kernel`
  - `ModelingContext`
  - `ToleranceConfig` and policy structs
- `forge-topo`
  - `PersistentName`, selector resolution, lineage (`Lineage`, `LineageEvent`, `OpSignature`)
- `forge-io`
  - Versioned JSON persistence
  - New audit storage substrate (`forge-io::audit`)

This spec does **not** replace those contracts. It defines the glue-layer types and lifecycle rules needed to use them consistently.

## Scope

### In scope

- Typed serializable error-summary layer for audit/policy/provenance outputs
- Policy-resolution trace payload schema (typed payload carried by traces/audit)
- Structured provenance payload schema (serializable, stable, reusable)
- `ModelingContext` metadata sink/read/reset contract
- Cross-kernel audit schema conventions (versioning, naming, snapshot/persistent labeling)
- Deterministic trace fingerprint helper
- Reusable test fixture builders for certifier/gate/policy trace tests

### Out of scope (this spec)

- Full curved geometry implementation
- Full policy-protocol rollout across all kernel features
- Persistent naming integration for all kernel operations
- UI/CLI policy override authoring
- Database storage backend

### Explicit milestone note (curves)

Curved modules may remain placeholders in the current milestone. This spec still defines the **foundation contracts** those future curved implementations must use (policy traces, typed errors, provenance, audit schemas).

## Non-Negotiable Invariants

1. **No stringly critical semantics**
- Audit-critical outcomes (merge/cert/policy/provenance failures) must be represented by typed serializable enums/structs, not only `Display` strings.

2. **Fail-closed ambiguity**
- Any ambiguous result (policy, naming resolution, precision classification) must be explicitly represented and traced.
- No implicit default acceptance without a typed policy resolution path.

3. **Deterministic serialization**
- For deterministic inputs and deterministic execution, emitted audit artifacts and fingerprints must be deterministic.

4. **Snapshot vs persistent identity must be explicit**
- Every serialized identity field must be labeled as snapshot-scoped or persistent.
- Snapshot handles/indices may appear in audit/debug records, but they must not be mislabeled as persistent identity.

5. **Envelope metadata must not be silently dropped**
- Sub-operation metadata absorption must have explicit lifecycle semantics (merge vs drain) and tests.

## Implementation Order (Required)

Implement in this order:

1. Typed serializable error summaries
2. Structured provenance payloads
3. Policy-resolution trace payload schema
4. `ModelingContext` metadata sink lifecycle contract/API
5. Audit schema conventions (shared versioned types + naming rules)
6. Deterministic trace fingerprint helper
7. Reusable fixture builders

Then implement `RegionMergeAuditRecord` and the four follow-on epics against these contracts.

## 1) Typed Serializable Error-Summary Layer

### Goal

Provide stable, machine-readable error summaries for audit artifacts and traces without requiring `KernelState` or string parsing.

### Design

Introduce a serializable error-summary module (recommended crate: `forge-core`, because it is shared and serialization-safe).

Proposed file:
- `crates/forge-core/src/errors/summary.rs`

### Core types

#### `ErrorSummary`

A top-level serializable envelope for operation errors.

Required fields:
- `category: ErrorCategory`
- `kernel: Option<KernelErrorSummary>`
- `source_chain: Vec<SourceErrorSummary>` (optional future-proofing; may start empty)
- `human_message: Option<String>` (non-authoritative display convenience only)

#### `KernelErrorSummary`

Serializable typed summary of `KernelError` variants needed by audit artifacts.

Requirements:
- preserve typed structure for:
  - merge failures (`MergeError`)
  - topology failures (`TopologyError`) where practical
  - ambiguous results / policy-required failures
  - invalid input
  - internal error (with constrained summary)
- allow lossy fallback for currently unsummarized variants via explicit `UnsupportedVariant { variant_name }`
  - this fallback is allowed temporarily but must be visible and test-covered

#### `MergeErrorSummary`

Serializable typed summary for region-merge failures.

Must include structured fields for:
- `BoundaryCertificationFailed { reason, witness }`
- `ProtectedUseConflict { face_index, edge_index }`
- `AmbiguousRadialSelection { edge_index, valence }`
- `PartialMergePlanRejected { step_index, reason }`
- other existing `MergeError` variants

### Rules

- `human_message` is optional and non-authoritative
- audit code must not parse `Display` strings to recover semantics
- conversion functions must be explicit:
  - `impl From<&KernelError> for KernelErrorSummary`
  - `impl From<&MergeError> for MergeErrorSummary`

### Acceptance tests (must-have)

- `merge_error_summary_preserves_boundary_reject_witness_reason`
- `kernel_error_summary_preserves_typed_merge_variant`
- `error_summary_round_trips_json`
- `unsummarized_kernel_variant_maps_to_explicit_unsupported_variant` (if fallback exists)

## 2) Structured Provenance Payload Schema

### Goal

Define a reusable, serializable provenance payload shape for boundary certification, merge planning, and audit artifacts.

This prevents each subsystem from inventing its own provenance structure.

### Design

Recommended crate placement:
- `forge-core` (shared trace/audit semantics), or
- `forge-kernel` if it requires kernel-specific concepts immediately

Preferred: `forge-core` for reuse.

Proposed file:
- `crates/forge-core/src/provenance/schema.rs`

### Core types

#### `SnapshotHandleRef`

Serializable snapshot-scoped handle identity:
- `kind: EntityKind`
- `index: u32`
- `generation: u32`

Notes:
- This is snapshot-scoped by definition.
- It must not be called `Persistent*`.

#### `BoundarySegmentProvenance`

Serializable provenance for a boundary segment used in certifier flows.

Required fields:
- `transport_hash: u64` (deterministic ID used in certifier transport / joins)
- `start_vertex: SnapshotHandleRef`
- `end_vertex: SnapshotHandleRef`
- `source_halfedge: Option<SnapshotHandleRef>`
- `source_edge: Option<SnapshotHandleRef>`
- `source_face: Option<SnapshotHandleRef>`
- `directed: bool` (explicitly record direction semantics)

#### `MergeStepProvenance`

Serializable provenance for a region-merge step.

Required fields:
- `step_index: u32`
- `edge: SnapshotHandleRef` (if known at runtime)
- `survive_face: SnapshotHandleRef`
- `kill_face: SnapshotHandleRef`
- `selector_origin: SelectorOrigin` (enum)

`SelectorOrigin` variants:
- `AutoDerived`
- `UserSelector`
- `PolicyResolved`

### Rules

- Transport hashes are convenience identifiers, not the full provenance.
- Provenance payloads must include generational handle identity where available.
- Provenance payloads must be usable in audit artifacts without access to `KernelState`.

### Acceptance tests

- `boundary_segment_provenance_json_round_trip`
- `transport_hash_changes_on_generation_change`
- `transport_hash_depends_on_both_endpoints`
- `snapshot_handle_ref_labels_kind_index_generation`

## 3) Policy-Resolution Trace Payload Schema

### Goal

Define a typed payload for policy resolution events so traces/audit records carry enough data for replay/debug without relying on generic `DecisionKind` + ad hoc strings.

### Why now

`DecisionKind::PolicyApplied` exists, but by itself it does not encode:
- query details
- candidate value semantics
- chosen resolution
- policy source

Without this layer, “policy integration” will still be under-specified in the trace.

### Design

Recommended crate placement:
- `forge-core` (`tracing` or `policy` module), because it bridges policy + trace and must be reusable by `forge-geom` and `forge-kernel`.

Proposed file:
- `crates/forge-core/src/tracing/policy_trace.rs`

### Core types

#### `PolicyResolutionSource`

Serializable enum:
- `DefaultPolicy`
- `UserOverride`
- `ForcedSafeFallback`
- `NonOverridableRule`

#### `PolicyResolutionOutcome`

Serializable enum:
- `AcceptedPotentialValue`
- `RejectedPotentialValue`
- `EscalatedError`

#### `PolicyDecisionTracePayload`

Serializable typed payload attached to trace/audit records (not necessarily embedded inside `TracedDecision` immediately; can be side-channel until tracing schema evolves).

Required fields:
- `decision_id: DecisionId`
- `policy_kind: PolicyKind`
- `query_location: [f64; 3]`
- `measured_margin: f64`
- `threshold: Option<f64>` (some queries do not have a single threshold yet)
- `overridable: bool`
- `candidate_summary: CandidateValueSummary`
- `outcome: PolicyResolutionOutcome`
- `source: PolicyResolutionSource`
- `default_used: bool` (kept explicit even if derivable for audit readability)

#### `CandidateValueSummary`

Serializable summary of the potential value/result (typed, not raw generic `T`).

For v1, use a constrained enum sufficient for region-merge/cert/surface classification:
- `WeakSimpleCertificateKind { kind: WeakSimpleCertificateKindSummary }`
- `SurfaceRelation { relation: SurfaceRelationSummary }`
- `BooleanFlag { value: bool }`
- `Opaque { type_name: String }` (explicit temporary escape hatch; test-covered)

### Integration rule (important)

This spec does **not** require changing `TracedDecision` immediately.

Acceptable staged integration:
- continue recording `TracedDecision`
- also record `PolicyDecisionTracePayload` in:
  - `OperationResult.extra_summaries` temporarily (not ideal), or preferably
  - a new typed side-channel on `OperationResult` in a follow-up change

Final target:
- trace/audit artifact includes both generic decision log and typed policy payloads keyed by `DecisionId`.

### Acceptance tests

- `policy_trace_payload_round_trips_json`
- `policy_trace_payload_captures_default_vs_user_source`
- `policy_trace_payload_outcome_matches_decision_kind_and_tier`

## 4) `ModelingContext` Envelope Metadata Sink Contract

### Goal

Define explicit lifecycle semantics for sub-operation metadata accumulated in `ModelingContext` so audit record generation does not depend on ad hoc reads/reset behavior.

### Current state (as of this spec)

`ModelingContext` aggregates:
- `sub_warnings`
- `sub_metrics`
- `sub_lineage_delta`
- `sub_accumulated_error_budget`

via `absorb_sub_result(...)`, but lifecycle semantics are not yet fully formalized.

### Required contract

`ModelingContext` must support **three explicit use modes**:

1. **Accumulate-only (default during operation)**
- sub-op metadata accumulates as nested ops run

2. **Snapshot-read (non-destructive)**
- caller can inspect current aggregates for diagnostics

3. **Take-and-reset (operation boundary)**
- top-level operation finalization can drain aggregated metadata into:
  - returned `OperationResult`
  - audit artifacts
  - persisted traces

### Required API additions

Recommended names (exact naming may vary, but semantics must match):

- `get_sub_warnings() -> &[KernelWarning]` (exists)
- `get_sub_metrics() -> &OperationMetrics` (exists)
- `get_sub_lineage_delta() -> &LineageDelta` (exists)
- `get_sub_accumulated_error_budget() -> f64` (exists)
- `take_sub_warnings() -> Vec<KernelWarning>`
- `take_sub_metrics() -> OperationMetrics`
- `take_sub_lineage_delta() -> LineageDelta`
- `take_sub_accumulated_error_budget() -> f64`
- or a single:
  - `take_sub_metadata() -> SubOperationMetadata`

Preferred: single typed drain:
- `SubOperationMetadata { warnings, metrics, lineage_delta, accumulated_error_budget }`

### Rules

- Drain methods must reset the internal aggregates to defaults
- repeated drains without new absorbed sub-ops must return empty/zero values
- docs must state whether top-level `decision_log` and sub-op aggregates are independent (they currently are)

### Acceptance tests

- `absorb_sub_result_accumulates_then_take_sub_metadata_drains`
- `take_sub_metadata_is_idempotent_after_reset`
- `sub_metadata_can_be_folded_into_operation_result_without_double_counting`

## 5) Versioned Audit Schema Conventions (Cross-Kernel)

### Goal

Define shared naming and versioning conventions so feature-specific audit artifacts (starting with region merge) do not drift.

### Applies to

- `forge-io::audit::VersionedAuditRecord<T>`
- feature audit record schemas (`RegionMergeAuditRecord`, future others)
- typed error/provenance/policy payloads stored in audit artifacts

### Conventions (required)

#### Versioning

Every feature audit record must include:
- `schema_version` (serialization schema version for the artifact shape)
- `operation_type` (stable string, e.g. `"region_merge"`)
- `operation_version` (semantic version of operation behavior/encoding)

Rule:
- Increment `schema_version` when JSON field schema changes
- Increment `operation_version` when semantics change without shape change (e.g., policy resolution behavior)

#### Identity field naming

Required suffix/prefix conventions:
- snapshot-scoped fields: include `_snapshot` or `snapshot_` in name
- persistent identity fields: include `_persistent` or `persistent_`
- hashed fingerprints: include `_hash`

Examples:
- `intent_snapshot`
- `topology_effects_snapshot`
- `surviving_faces_persistent`
- `trace_hash`

#### Typed error encoding

- use `ErrorSummary` / `KernelErrorSummary` (from section 1)
- never encode critical failure semantics only as `error_message: String`
- optional human text may be included as secondary field

#### Determinism

- arrays representing ordered execution must use deterministic order
- maps in serialized artifacts should be avoided unless stable ordering is guaranteed (`BTreeMap`)
- artifact generation must not depend on hash-map iteration order

### Acceptance tests

- `versioned_audit_record_requires_schema_and_operation_versions`
- `audit_schema_fields_label_snapshot_vs_persistent_identity`
- `audit_record_serialization_is_deterministic_for_same_input`

## 6) Deterministic Trace Fingerprint Helper

### Goal

Provide a reusable helper to compute stable trace fingerprints for tests and audit artifacts.

### Design

Recommended crate: `forge-core::tracing`

Proposed API:
- `compute_trace_fingerprint(log: &DecisionLog) -> u64`
- optional richer:
  - `TraceFingerprint { trace_hash: u64, decision_ids: Vec<u64>, summary_hash: u64 }`

### Rules

- fingerprint algorithm must be deterministic across runs/platforms for identical serialized decision content
- do not hash wall-clock durations if determinism is required (or gate behind mode)
- document what is included/excluded:
  - include decisions and semantic payloads
  - exclude timing fields by default

### Acceptance tests

- `trace_fingerprint_is_stable_for_identical_decision_logs`
- `trace_fingerprint_changes_when_decision_semantics_change`
- `trace_fingerprint_ignores_span_timing_when_configured_default`

## 7) Reusable Test Fixture Builders (Policy/Cert/Gate)

### Goal

Reduce duplicated, fragile setup in adversarial tests for certifier gating, policy resolution, provenance, and trace payloads.

### Design

Recommended crate: `forge-test` (shared infra), with feature-specific adapters in `forge-kernel` tests if needed.

Initial fixture builders (must-have for region merge follow-on):
- rejected boundary cert fixture (crossing/self-intersecting boundary)
- weakly-simple cert fixture (touching but certifiable)
- ambiguous selector / radial valence fixture
- deterministic group-selection fixture for repeatable policy decision IDs

### Rules

- fixtures must document which invariants they intentionally violate
- fixtures used to test post-gate planning must be labeled “pre-gate” if they are not certifiable
- builders should return typed handles + summaries (not force test code to rediscover everything)

### Acceptance tests

- `fixture_rejected_boundary_produces_expected_cert_outcome`
- `fixture_weakly_simple_boundary_produces_touch_count`
- `fixture_group_hash_is_deterministic`

## Mapping to the Four Follow-On Architecture Items

This base-layer spec is a foundation for the follow-on work (already defined separately):

1. **Full Policy Protocol Integration**
- depends on sections 1, 3, 4, 5, 6, 7

2. **Precision / Undetermined Surface Semantics**
- depends on sections 1, 3, 4, 5, 6

3. **Persistent Naming + Lineage for Region Merge**
- depends on sections 1, 2, 4, 5
- plus future `PersistentName` resolver result spec

4. **Rich Serializable Region Merge Audit Artifacts**
- depends on **all** sections in this spec

## Delivery Guard / CI Requirements for These Base Layers

These implementations must satisfy the delivery guards and checklist discipline:

- no non-curve placeholders in production paths
- no `todo!()` / `unimplemented!()` in production paths
- checklist-backed completion evidence for each implemented base layer

Suggested checklist file for this work:
- `docs/engineering/AUDIT_POLICY_BASE_LAYERS_CHECKLIST.md`

## Open Decisions (Must Be Resolved Before Coding the Corresponding Item)

1. **Crate placement for provenance schema**
- `forge-core` (preferred for shared reuse) vs `forge-kernel` (if kernel-specific fields dominate)

2. **Where typed policy payloads live before `TracedDecision` can carry them**
- side-channel in audit record only
- side-channel in `OperationResult`
- tracing schema extension

3. **Error-summary coverage strategy**
- strict all-variants coverage immediately
- explicit unsupported-variant fallback for staged rollout

4. **`ModelingContext` drain API shape**
- one `SubOperationMetadata` drain (preferred)
- multiple granular drain methods

## Definition of Done (for this spec’s implementation)

This base-layer foundation is considered complete when:

- sections 1–7 have concrete code + tests
- each section’s acceptance tests pass
- delivery guard passes
- a checklist for this spec is fully evidence-backed
- region-merge audit artifact implementation can proceed without inventing new ad hoc error/policy/provenance/context conventions
