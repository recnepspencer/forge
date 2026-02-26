# Foundation Phase 2 Contracts Spec

Status: Draft (implementation planning)

Purpose: define the next contract layer on top of the completed audit/policy
base layers (sections 1-7) so future implementations (policy protocol, region
merge audit records, persistent naming integration, replay bridge) are built on
production-grade interfaces instead of ad hoc glue.

This spec is intentionally contract-first. It locks semantics, lifecycle, and
trace/audit requirements before code changes.

References:
- `/Users/spenstar/Documents/programming/Forge/Vision.md`
- `/Users/spenstar/Documents/programming/Forge/ARCHITECTURE_COMPONENT_INVENTORY.md`
- `/Users/spenstar/Documents/programming/Forge/docs/engineering/AUDIT_POLICY_BASE_LAYERS_SPEC.md`
- `/Users/spenstar/Documents/programming/Forge/docs/engineering/FOUNDATION_HARDENING_QA_CHECKLIST.md`

## Scope

This spec covers six Foundation Phase 2 contracts:
1. Trace adjunct/versioning strategy (typed policy/provenance payload attachment)
2. Operation finalization contract (context drain + envelope merge + audit emit)
3. Policy registry/config source model
4. Persistent-name resolution result contract (typed + traced)
5. Persistent re-identification substrate (lineage linkage + delta/audit integration)
6. Replay/audit bridge contract

Non-goals:
- Full curved/NURBS implementation
- UI for policy overrides
- Full persistent naming rollout for every feature
- Database-backed audit storage

## Difficulty Summary (planning signal)

- `P2-1 Trace adjunct/versioning strategy`: **High**
  - Blast radius: `forge-core` tracing schema + readers/writers + `forge-view`
  - Risk: trace compatibility drift, duplicate semantics between side-channel and decision records
- `P2-2 Operation finalization contract`: **Medium-High**
  - Blast radius: top-level kernel operations + `ModelingContext` lifecycle + envelope handling
  - Risk: double counting, lost traces/metadata, inconsistent error-path finalization
- `P2-3 Policy registry/config source model`: **Medium**
  - Blast radius: `ModelingContext`, policy resolution callsites
  - Risk: signature churn if precedence/source semantics are not fixed first
- `P2-4 Persistent-name resolution result contract`: **Medium-High**
  - Blast radius: naming resolver APIs, region merge intent/result schemas, tracing/audit outputs
  - Risk: ambiguous resolution semantics / snapshot-handle leakage into persistent outputs
- `P2-4A Persistent re-identification substrate`: **High**
  - Blast radius: `forge-topo` lineage persistence/event schema, `forge-kernel` audit/finalization, persistent naming fallback
  - Risk: fake lineage fallback, non-replayable identity claims, incomplete provenance for audit/reidentification
- `P2-5 Replay/audit bridge contract`: **High**
  - Blast radius: audit artifact schema, replay analysis modules, witness formats
  - Risk: building rich audit artifacts that are not actually replayable

Implementation order (recommended):
1. `P2-1`
2. `P2-2`
3. `P2-3`
4. `P2-4`
5. `P2-4A`
6. `P2-5`

## Global Production-Grade Requirements (applies to all five)

1. Explicit semantics, no “helper magic”
- If behavior is policy-driven, the policy source and selected outcome must be explicit and typed.

2. Deterministic behavior
- Same inputs and policy config must produce the same trace IDs/order, audit payloads, and summaries.

3. Fail-closed ambiguity handling
- No first-match or silent fallback for ambiguous resolution.
- All ambiguity outcomes must be typed and traced.

4. Transactional metadata handling
- Operation finalization must be single-pass and idempotent-safe (no mixed drain/borrow semantics).

5. Machine-readable critical semantics
- No reliance on `Display` strings for policy decisions, provenance, error summaries, or replay mapping.

6. Snapshot vs persistent identity labeling
- Every schema carrying entity identities must declare scope (`snapshot`, `persistent`, `hash`) and enforce naming conventions.

7. Provenance completeness (or explicit absence)
- Trace/audit artifacts for policy/geometry decisions must include sufficient
  typed provenance to reconstruct causal inputs, or explicitly declare what
  provenance is unavailable and why.

## P2-1. Trace Adjunct / Versioning Strategy

### Goal

Promote typed policy/provenance payloads from ad hoc side channels into a
versioned, first-class trace adjunct model without breaking determinism or
causal trace readability.

### Current state

- `TracedDecision` carries generic `DecisionKind` + `DecisionContext`
- `PolicyDecisionTracePayload` exists as a typed side-channel keyed by `DecisionId`
- `BoundarySegmentProvenance` / `MergeStepProvenance` exist as serializable payloads
- No canonical attachment mechanism/versioning contract for typed adjuncts

### Contract

Trace data must support typed adjunct payloads associated with a decision/event:
- typed payload kind
- payload schema version
- decision/event linkage key (`DecisionId`, and optionally `SpanId`)
- deterministic serialization

The adjunct system must support multiple payload families over time:
- policy resolution payloads
- provenance payloads
- (future) precision escalation payloads, replay witness payloads

### Design requirements

1. No duplication ambiguity
- If both `TracedDecision` and adjunct payload carry overlapping fields, define
  which is authoritative and require consistency validation.

2. Versioning is per adjunct family
- A single global trace schema version is not enough once adjunct payloads evolve.

3. Reader behavior must be fail-soft, semantics fail-closed
- Unknown adjunct payload versions may be retained/forwarded as opaque bytes/json,
  but kernel logic must not silently act on unknown semantics.

4. Deterministic ordering
- Adjunct payloads attached to a decision must have stable ordering by payload type key.

5. `forge-view` compatibility is a contract, not a best-effort add-on
- Trace readers/viewers must preserve unknown adjunct kinds/versions without loss.
- Known adjunct renderers may degrade gracefully, but must not rewrite or drop
  adjunct payloads on load/save/export paths.

### Proposed API shape (contract-level)

- `TraceAdjunctRecord`
  - `decision_id: DecisionId`
  - `payload_kind: String` (stable snake_case)
  - `payload_version: u32`
  - `payload_json: serde_json::Value` (transport)

- Typed wrappers/builders in `forge-core::tracing` for known families
  - `PolicyDecisionTracePayload`
  - `DecisionProvenancePayload` (future aggregator around provenance structs)

- Consistency validators
  - `validate_policy_adjunct_vs_decision(...)`
  - `validate_provenance_adjunct_vs_decision(...)` (where applicable)

### Must-have tests

- adjunct serialization deterministic for same payload
- adjunct ordering deterministic when multiple adjuncts attach to same decision
- unknown adjunct version preserved round-trip without semantic interpretation
- typed adjunct validator catches contradiction with `TracedDecision`
- `forge-view`/trace-reader path preserves unknown adjuncts on parse + re-emit (or equivalent store round-trip)

### Acceptance criteria

- There is a canonical, versioned way to attach typed payloads to decisions
- `forge-view` / trace readers can ingest adjuncts without losing unknown versions
- No production path emits policy/provenance side data outside the adjunct contract

## P2-2. Operation Finalization Contract

### Goal

Define one production-grade operation-boundary finalization path for:
- `ModelingContext` trace + sub-op metadata sinks
- `OperationResult` envelope metadata
- trace adjunct payload attachment/finalization
- audit record emission/storage
- error-path vs success-path behavior

### Current state

- `ModelingContext` has `take_decision_log()` and `take_sub_metadata()`
- `OperationResult` has `absorb_metadata(...)`
- `ModelingContext::absorb_sub_result(...)` now true-drains child metadata
- Top-level operations still finalize inconsistently

### Contract

Every top-level operation must finalize through a single explicit finalization
protocol (collector + emitter, or equivalent) that:
1. drains context decision log
2. drains context sub-op metadata
3. attaches typed trace adjunct payloads in deterministic order
4. merges drained metadata into parent envelope
5. sets or validates operation boundary state hashes (`before` / `after`) with explicit error-path semantics
6. emits/stores audit artifact(s) if configured
7. preserves error-path traceability (including partial traces)

### Design requirements

1. Single-pass drain semantics
- Finalization may be called at most once for a given context/operation pair.
- Repeated calls must be rejected with a typed error or produce a provably empty result.

2. Error-path parity
- Errors must still finalize trace/audit outputs (unless explicitly disabled).
- Success and error finalization must be separate explicit API paths (not string status flags).

3. No hidden persistence
- Finalization should explicitly call trace/audit emit paths; `Drop` remains fallback only.

4. Collect vs emit separation
- Finalization must define a deterministic "collect/merge" phase that can be
  tested without I/O, and a separate "emit" phase for storage side effects.
- I/O failures must not retroactively invalidate already-collected deterministic summaries.

5. Configurable sinks, deterministic payloads
- Storage side effects may be configurable; generated artifacts must be deterministic before write.

6. State-hash semantics must be explicit
- The finalization contract must specify whether state hashes are:
  - computed during finalization,
  - provided by the caller and validated, or
  - optional on error paths.
- For this phase, hash semantics must be labeled explicitly as topology-state hashes
  unless a composite `KernelState` hash/fingerprint contract is implemented.
- Error-path behavior must be typed and deterministic (e.g., `after` absent vs sentinel is not implicit).

7. Adjunct/finalizer coherence
- Finalization must be the canonical point where trace adjunct payloads are
  attached to the finalized trace/audit outputs for migrated paths.
- No migrated path may finalize `DecisionLog` and adjunct payloads in separate,
  unsynchronized code paths.

### Proposed API shape (contract-level)

- `OperationFinalizer` (small service in `forge-kernel::core`)
  - single-use finalization boundary object for one operation/context pair
  - rejects re-finalization with typed error (`AlreadyFinalized` or equivalent)

- `collect_success(...)` / `collect_error(...)`
  - drains context + merges envelope + attaches adjuncts + computes hashes/fingerprint
  - no I/O required
  - returns deterministic collected artifact bundle / summary

- `emit(...)`
  - writes trace/audit outputs from collected artifacts according to sink options

- Convenience wrappers (optional)
  - `finalize_success(...) = collect_success + emit`
  - `finalize_error(...) = collect_error + emit`

- `FinalizationSummary`
  - `trace_fingerprint`
  - `adjunct_count`
  - `audit_record_emitted: bool`
  - `topology_state_hash_before: Option<u128>`
  - `topology_state_hash_after: Option<u128>`
  - `drained_metadata_counts`

### Must-have tests

- finalization drains context once and merges metadata exactly once
- double-finalization cannot double-count metadata
- error-path finalization emits trace/audit summary
- success-path and error-path preserve deterministic fingerprints for same causal trace
- adjunct attachments are preserved and deterministically ordered in finalized outputs
- topology-state hash fields are set/validated consistently for success and explicitly handled on error paths
- collect phase is deterministic and testable without I/O; emit failures do not corrupt collected summary

### Acceptance criteria

- Top-level region merge/boolean ops use the finalization contract
- No mixed drain/borrow metadata patterns remain in operation-boundary code
- Migrated paths do not finalize `DecisionLog` and adjunct payloads separately

## P2-3. Policy Registry / Config Source Model

### Goal

Define where policy values come from and how precedence is resolved before full
`PolicyQuery` / `PolicyResult` integration is rolled out.

### Current state

- `ModelingContext` stores policy/tolerance config
- policy traces can label source (`DefaultPolicy`, `UserOverride`, etc.)
- no canonical registry/precedence source model

### Contract

Policy resolution source precedence must be explicit and typed:
1. Non-overridable rule (hard invariant)
2. Per-operation override (if allowed)
3. Per-feature override (if allowed)
4. Model/spec override
5. Session/user override
6. Default policy

The selected source and effective value must be traceable.

### Design requirements

1. Precedence is deterministic and documented
2. Missing policy handling is explicit (`fail-closed` by default)
3. Overrides may be scoped (operation, feature, model/spec, session/user) and must not silently leak across boundaries
4. Trace payload must capture both source and chosen resolution
5. Policy source scope identifiers should be serializable for audit/replay (`feature_id`, `operation_id`, model policy key/path) where available

### Proposed API shape (contract-level)

- `PolicyRegistrySnapshot` (immutable effective view during an operation)
- `PolicyOverrideSource` (typed scope identifier)
- `ResolvedPolicyValue<T>` with source metadata
- `ModelingContext::resolve_policy_query(...) -> Result<ResolvedPolicyValue<_>, KernelError>`

### Must-have tests

- precedence order deterministic across overlapping overrides
- per-op override does not leak to next operation after reset/finalization
- missing policy for ambiguous query fails closed and traces source attempt

### Acceptance criteria

- No policy application path relies on undocumented precedence
- Policy source in traces is backed by actual resolution, not labels only

## P2-4. Persistent-Name Resolution Result Contract (Typed + Traced)

### Goal

Create a reusable, typed, traceable result contract for resolving persistent
names/selectors to snapshot handles so region merge and future features do not
invent incompatible ambiguity handling.

### Current state

- `forge-topo` has persistent naming subsystem
- `forge-topo` also has lineage/replay infrastructure (`Lineage`, `LineageEvent`, `LineageStore`, replay logs)
- region merge APIs/results are still snapshot-scoped
- no reusable kernel-level typed resolution result contract with trace semantics

### Difficulty / estimated implementation size

- **Difficulty**: Medium-High (contract-heavy, lineage coupling, trace/audit integration)
- **Spec clarification + checklist updates**: ~80-160 LOC docs/tests-list
- **Core contract types (result/candidate/trace payload)**: ~220-420 LOC
- **`ModelingContext` / resolver wiring (typed + traced)**: ~180-350 LOC
- **Region-merge first integration (persistent -> snapshot)**: ~220-500 LOC
- **Adversarial tests (determinism/ambiguity/leakage/lineage)**: ~250-600 LOC
- **Expected total for first production slice**: ~900-2,000 LOC

This estimate assumes a production-grade first slice (typed results, typed traces,
deterministic ordering, fail-closed semantics, and one real region-merge entrypoint),
not a minimal helper wrapper.

### Contract

Name/selector resolution returns a typed result:
- `Resolved(T)`
- `Ambiguous { query, candidates, evidence }`
- `Missing { query, evidence }`
- `Incompatible { query, incompatibility }` (typed; e.g. version/scope/lineage incompatibility)

All non-`Resolved` outcomes are traced and fail-closed by default.

Resolution is a **two-phase contract**:
1. **Resolution analysis** (deterministic candidate generation + typed result)
2. **Kernel policy/operation handling** (accept/reject/escalate based on feature semantics)

No API may collapse analysis outcomes into `Option<T>` or string-only errors.

### Design requirements

1. No first-match behavior
2. Candidate ordering deterministic
3. Snapshot handles only in resolution internals or explicitly labeled snapshot debug fields
4. Persistent output identity separated from snapshot debug identity
5. Resolution outputs must be compatible with lineage-based re-identification workflows (not naming-only islands)
6. Resolver path provenance must be explicit (`direct-name`, `lineage-reidentified`, `hybrid`, `none`)
7. Candidate set semantics must be stable under deterministic topology traversal (ordering must not depend on hash map iteration)
8. Ambiguity evidence must be machine-readable (not prose-only) so audit/replay tooling can inspect why multiple candidates survived
9. Missing/incompatible outcomes must carry enough typed query data to support exact trace/audit correlation without parsing strings
10. Region-merge integration must not leak `FaceId`/snapshot handles into persistent API outputs except in explicitly labeled debug sections

### Clarified typed result model (required)

The contract must define a reusable resolver result family (names illustrative):

- `ResolutionResult<T>`
  - `Resolved { value: T, route, evidence }`
  - `Ambiguous { query, candidates, evidence }`
  - `Missing { query, evidence }`
  - `Incompatible { query, incompatibility }`
- `ResolutionRoute`
  - `DirectPersistentName`
  - `LineageReidentified`
  - `Hybrid` (must enumerate substeps in evidence)
- `ResolutionEvidence`
  - typed summary of resolver passes attempted, filters applied, and surviving candidate counts
- `ResolutionIncompatibility`
  - typed enum for version/scope/schema/lineage-store incompatibilities

This result family must be crate-reusable (not region-merge-specific).

### Candidate payload contract (required)

`ResolutionCandidate` must be typed + serializable and include, at minimum:

- `persistent_ref` (the persistent identifier that matched or was derived)
- `snapshot_ref` (typed snapshot handle ref, explicitly labeled snapshot-scoped)
- `entity_kind` (e.g. Face)
- `route` (`DirectPersistentName`, `LineageReidentified`, etc.)
- `match_kind` (exact-name, lineage-descendant, alias, etc.; typed enum)
- `rank_key` / deterministic ordering fields (or enough fields to derive them deterministically)
- `provenance` (typed provenance/evidence payload summary sufficient for audit)

Candidate payloads must not require `Display` parsing to understand why they were included.

### Deterministic ordering contract (required)

Candidate ordering must be deterministic and specified. At minimum, define:

1. primary key: `entity_kind`
2. secondary key: canonical persistent identifier bytes/string
3. tertiary key: snapshot `(index, generation)` (explicitly snapshot-scoped)
4. quaternary key: route/match-kind discriminator

If a different ordering is chosen, it must be documented and tested. Hash-map iteration order
or arena insertion coincidence is never acceptable as ordering semantics.

### Lineage fallback contract (required)

The resolver must define an explicit fallback pipeline (feature may opt out, but cannot improvise):

1. direct persistent-name resolution
2. lineage-backed re-identification (using current lineage store / replay metadata)
3. hybrid reconciliation (if supported)
4. fail-closed typed `Missing` / `Ambiguous` / `Incompatible`

Each attempted phase must be reflected in typed `ResolutionEvidence` and trace payloads.

### Typed trace adjunct contract (required)

Add a `ResolutionTracePayload` adjunct family (via `P2-1` trace adjunct strategy) keyed by `DecisionId`.
It must include:

- query identity (typed, serializable)
- resolver route(s) attempted
- candidate count and (for ambiguity) ordered candidate summaries
- chosen outcome (`Resolved`, `Ambiguous`, `Missing`, `Incompatible`)
- source scope / operation scope identifiers when available
- deterministic fingerprint/hash of the candidate set (optional but recommended)

This avoids repeating the prior "trace-label cosplay" problem for persistent-name resolution.

### Selector-based persistent resolution contract (required for `P2-4` completion)

Selector resolution must not be a separate ad hoc contract. It must reuse the same
`ResolutionResult<T>` family and `ResolutionTracePayload` adjunct family as
`PersistentName` resolution.

Required clarifications:

1. **Selector query normalization**
   - The resolver must canonicalize selector queries (or produce a canonical summary)
     before hashing/tracing so semantically equivalent selector forms do not produce
     nondeterministic trace identities.
   - `ResolutionQuerySummary::Selector` must capture:
     - selector kind
     - target entity kind (if constrained)
     - canonical selector fingerprint/hash (recommended)

2. **Selector result-kind enforcement**
   - If a feature expects faces, selector resolution must reject mixed-kind result sets
     with typed `Incompatible` (not silently filter by first matching kind).
   - Any feature-side narrowing (e.g. face-only) must be explicit and traced.

3. **Selector ambiguity semantics**
   - Ambiguity is determined after deterministic dedup + ordering.
   - The trace payload must include ordered candidate summaries and candidate count
     exactly as returned to the caller (no hidden post-trace filtering).

4. **Selector composition evidence**
   - `ResolutionEvidence` for selectors must include typed filter/application steps
     (at minimum: selector subclauses applied and post-filter candidate counts).

### Lineage fallback pipeline runtime contract (required for `P2-4` completion)

The fallback pipeline (`Direct -> LineageReidentified -> Hybrid`) must be specified as
runtime behavior, not just route labels.

Required runtime semantics:

1. **DirectPersistentName phase**
   - Resolve via current naming subsystem only (`PersistentName` / `Selector`)
   - Produce deterministic candidate set
   - If `Resolved` or `Ambiguous`, stop (no implicit lineage fallback on ambiguity unless
     feature explicitly opts into a refinement strategy and traces it)

2. **LineageReidentified phase**
   - Entered only on `Missing` (default) or explicit feature opt-in path
   - Must consult lineage store / replay metadata using a typed compatibility gate
   - Must produce `ResolutionEvidence` describing:
     - lineage source version(s)
     - re-identification pass used
     - candidate counts pre/post lineage filter
   - If lineage metadata is unavailable/incompatible, return typed `Incompatible`

3. **Hybrid phase**
   - Must be explicitly defined (e.g., intersect direct selector candidates with lineage-derived candidates)
   - If unsupported for a query/entity kind, return typed `Incompatible` instead of silently skipping
   - Must emit route/evidence showing hybrid substeps

4. **Determinism**
   - Every phase must produce deterministically ordered candidates using the shared ordering contract.
   - Candidate ordering may not depend on lineage store internal map iteration.

5. **Fail-closed default**
   - If direct resolution is missing and lineage fallback is disabled/unavailable, return typed `Missing`/`Incompatible`
     and trace both the attempted direct route and the absence of fallback.

### Merge error taxonomy contract for persistent resolution failures (required)

`P2-4` production integration must not collapse persistent resolution failures into
generic `KernelError::InvalidInput` / `KernelError::AmbiguousResult` once merge execution
adopts persistent entrypoints.

Required contract:

1. Add typed `MergeError` variants for persistent-resolution failures, e.g.:
   - `PersistentResolutionMissing { role, query_summary }`
   - `PersistentResolutionAmbiguous { role, candidate_count, query_summary }`
   - `PersistentResolutionIncompatible { role, incompatibility }`

2. Mapping rule
   - Resolver returns `ResolutionResult<T>` + traced decision + typed adjunct payload
   - Region-merge adapter maps non-`Resolved` to `KernelError::MergeFailure(MergeError::...)`
   - No string-only merge failure mapping for these outcomes

3. Trace-before-error invariant
   - The typed resolution decision + adjunct must be recorded before returning the merge error.
   - Error paths must preserve `DecisionId` correlation with the adjunct payload.

4. Role semantics
   - `role` must be typed or constrained string enum (`surviving_face`, `selected_face`, `protected_face`, ...)
   - Role is part of deterministic resolution decision identity (or explicitly excluded and documented)

5. Batch semantics
   - When resolving lists (`selected_faces`, `protected_faces`), failure behavior must be explicit:
     - fail on first error (default) or
     - aggregate multiple failures in deterministic order (if implemented later)
   - Current phase should document and test fail-on-first deterministic behavior.

### Proposed API shape (contract-level)

- `ResolutionResult<T>`
- `ResolutionCandidate` (typed + serializable, with snapshot and/or persistent labels)
- `ResolutionTracePayload` (adjunct family for `P2-1`)
- `ResolutionEvidence`
- `ResolutionIncompatibility`
- `ResolutionRoute`
- `ResolutionMatchKind`

### Region-merge integration requirements (first consumer)

1. Introduce explicit API split:
   - `MergeRegionSelectionPersistent`
   - `MergeRegionSelection` (snapshot; existing internal execution form)
2. Add a deterministic, traced resolver:
   - `MergeRegionSelectionPersistent -> Result<MergeRegionSelection, KernelError>`
3. Region merge must fail closed on any unresolved ambiguity/missing/incompatibility unless explicitly handled by a feature policy contract
4. Persistent result outputs must separate:
   - persistent identity output (user/agent-facing)
   - snapshot debug output (optional, labeled)
5. Lineage delta emitted by merge must be sufficient to support future re-identification of survivor/killed faces
6. Persistent-resolution failures must surface as typed `MergeError` variants (not generic input/ambiguity errors) once the persistent entrypoint is declared production-ready

### Must-have tests

- ambiguous selector returns deterministic candidate order
- missing selector emits typed trace payload and fails closed
- generation reuse does not cause persistent selector success via stale snapshot handle leakage
- lineage-backed re-identification remains possible after resolution + merge (or typed incompatibility is reported)
- direct-name and lineage fallback produce distinct typed `ResolutionRoute` outcomes
- resolver trace payload preserves ordered candidate summaries deterministically
- region-merge persistent entrypoint never exposes raw snapshot handles in persistent output fields
- incompatible lineage/schema versions return typed `Incompatible` (not generic string error)
- selector-based persistent resolution uses the same typed result + adjunct family as persistent-name resolution
- lineage fallback disabled/unavailable yields traced fail-closed `Missing`/`Incompatible` (documented default)
- persistent region-merge adapter maps missing/ambiguous/incompatible to typed `MergeError` variants with role

### Acceptance criteria

- Region merge can reuse one typed resolution contract (not custom result enums)
- Ambiguity and missing outcomes are machine-readable and traced
- Candidate ordering is deterministic and documented
- Lineage-backed fallback behavior is explicit, typed, and traced
- Persistent-facing outputs remain free of unlabeled snapshot handles
- Persistent-resolution failure modes are surfaced via typed merge errors, not generic input/string errors

## P2-4A. Persistent Re-identification Substrate (Lineage Linkage + Delta / Audit Integration)

### Goal

Provide the persisted lineage/provenance substrate required for real
`LineageReidentified` fallback in `P2-4`, instead of route labels or typed
"unsupported" placeholders.

This section exists because the current architecture stores enough lineage to
support persistent naming and causal audit, but not enough structured linkage to
re-identify a *missing* persistent name across topology evolution in a
deterministic, audit-grade way.

### Current state (source-derived)

- `PersistentName` stores `{ ancestry_hash, kind, ordinal }` (`forge-topo::topology::naming`)
- `resolve_name` / `resolve_selector` query the *current* arena only
- `TopologyState` persists `lineage_events: Arc<Vec<LineageEvent>>`
- `LineageStore` exists, but is draft-local and not persisted as a queryable
  committed-state index
- `LineageEvent::{EntityCreated, EntityDeleted, EntityModified}` preserve current
  lineage values but do not encode parent-lineage linkage in a form sufficient
  for deterministic descendant re-identification from a missing ancestry hash
- `LineageDelta` in `OperationResult` is count-only summary (good for accounting,
  insufficient for re-identification)

### Why this contract is required (explicit limitation)

`P2-4` can produce typed, traced `Incompatible` outcomes when lineage fallback is
unavailable, but it cannot implement real `LineageReidentified` behavior until a
persisted re-identification substrate exists.

Without this section, the project risks:
- fake lineage fallback claims
- audit artifacts that *look* rich but cannot support replay/re-identification
- persistent naming APIs that silently degrade into direct-name-only behavior

### Contract

Forge must provide a deterministic, persisted re-identification substrate that:
1. maps persistent identity queries to candidate descendants/ancestors across
   topology evolution using structured lineage linkage
2. exposes typed evidence sufficient for audit and trace adjunct payloads
3. integrates with operation envelopes/finalization so re-identification data is
   preserved at operation boundaries
4. distinguishes unsupported vs unavailable vs ambiguous re-identification with
   typed outcomes

This substrate is a prerequisite for runtime `ResolutionRoute::LineageReidentified`
in `P2-4`.

### Design requirements

1. **Structured lineage linkage, not just event chronology**
- Persist enough linkage to answer re-identification queries deterministically:
  - predecessor/successor relationships and/or stable ancestry references
  - operation boundary association (`OpSignature`, replay index, or equivalent)
- Scanning `LineageEvent` logs by operation name alone is not sufficient.

2. **Snapshot identity and persistent identity are both explicit**
- Re-identification data must clearly distinguish:
  - persistent query identity (e.g. ancestry hash + ordinal + kind)
  - snapshot candidate identity (typed snapshot refs with generation)
- No unlabeled raw indices in APIs/audit payloads.

3. **Deterministic candidate enumeration**
- Candidate ordering must be deterministic and compatible with `P2-4` ordering rules.
- Internal map iteration order may not affect outputs.

4. **Typed availability / compatibility gates**
- The substrate must report typed incompatibilities such as:
  - lineage linkage unavailable for this topology epoch
  - unsupported entity kind / resolver mode
  - schema/version mismatch for persisted lineage linkage records
- No generic string errors.

5. **LineageDelta integration is explicit**
- `LineageDelta` remains a count summary for envelope/accounting, but cannot be
  treated as the re-identification substrate.
- If re-identification-relevant lineage details are needed in operation outputs,
  they must be emitted in a separate typed payload (audit artifact and/or adjunct),
  not squeezed into count-only `LineageDelta`.

6. **Finalization integration**
- `P2-2` finalization must be the canonical point that attaches/persists any
  re-identification metadata emitted by migrated operations.
- No split path where lineage-reidentification evidence is logged outside the
  finalized trace/audit bundle.

7. **Replay compatibility**
- The substrate must be designed so `P2-5` can map re-identification evidence to:
  - exact replay witnesses (when sufficient)
  - counterfactual-only evidence (when exact replay is impossible)
- Replay compatibility status must be typed, not inferred from missing fields.

### Proposed architecture shape (contract-level)

The exact data structures may differ, but Phase 2 must provide equivalents to:

- `ReidentificationLinkStore` (persisted/queryable per committed topology state or audit artifact)
  - deterministic index for lineage-derived candidate lookup
  - versioned schema

- `ReidentificationQuery`
  - persistent identity target (`PersistentName` / canonical selector summary)
  - target entity kind
  - mode (`descendants`, `ancestors`, `hybrid`)

- `ReidentificationCandidate`
  - snapshot candidate ref
  - derived persistent summary (if available)
  - linkage evidence summary (typed)
  - route/match kind compatible with `P2-4`

- `ReidentificationEvidence`
  - linkage sources consulted
  - operation/replay ranges considered
  - candidate counts before/after filters
  - compatibility/version metadata

- `ReidentificationCompatibility`
  - `Available`
  - `Unavailable`
  - `SchemaVersionMismatch`
  - `MissingLinkage`
  - `UnsupportedMode`
  - etc. (typed)

### Integration requirements (with existing components)

1. **`forge-topo` lineage / replay**
- Must define how linkage records are derived from:
  - `Lineage`
  - `LineageEvent`
  - `ReplayLog`
- If current event schema is insufficient, this section authorizes a schema
  upgrade (new event payload fields or parallel linkage records).

2. **`forge-core::envelope::LineageDelta`**
- Keep `LineageDelta` as count summary.
- Add separate typed re-identification metadata path (audit record field and/or
  trace adjunct family); do not overload `LineageDelta`.

3. **`P2-4` resolver contract**
- `LineageReidentified` route may only be emitted when backed by this substrate.
- Otherwise resolver must emit typed `Incompatible`/`Missing` and trace that fact.

4. **`P2-5` replay/audit bridge**
- Re-identification linkage/evidence schema must expose versioning and witness
  references so replay bridge can classify exact vs counterfactual compatibility.

### Must-have tests

- lineage linkage persistence is deterministic for identical operation sequences
- re-identification substrate distinguishes unavailable linkage vs missing entity
- candidate ordering from linkage-derived results is deterministic
- generation reuse/topology reorder does not alias stale snapshot identities into
  re-identified candidates
- `LineageDelta` remains count-only while re-identification details are preserved
  in the typed metadata path (no silent data loss)
- replay/counterfactual bridge can classify re-identification evidence as exact,
  counterfactual-only, or incompatible (typed)

### Acceptance criteria

- `P2-4` lineage fallback can be implemented against a real persisted substrate
  (not route labels / typed placeholders)
- Re-identification evidence is typed, deterministic, and audit-compatible
- `LineageDelta` accounting remains intact while detailed lineage/reidentification
  semantics are carried in a separate typed channel
- Replay/audit bridge has the data it needs to classify re-identification
  compatibility without parsing human-readable logs

## P2-5. Replay / Audit Bridge Contract

### Goal

Define how stored audit records and trace adjuncts map back into replay and
counterfactual tooling so audit artifacts are operationally useful, not just logs.

### Current state

- audit storage substrate exists (`forge-io::audit`)
- trace fingerprints exist
- analysis/counterfactual replay tooling exists in kernel (`analysis/counterfactual/*`)
- topology replay logs exist in `forge-topo` (`topology/history/replay.rs`)
- no canonical bridge contract from audit artifacts to replay inputs/witnesses

### Contract

Every replayable audit artifact must declare:
- snapshot requirements (or references)
- required witness/provenance payloads
- policy decisions/overrides used
- replay compatibility versions

Replay tooling must report typed incompatibility reasons when an audit record
cannot be replayed exactly.

The bridge contract must explicitly support two replay targets:
- exact replay (where sufficient snapshots/witnesses exist)
- counterfactual replay (decision/policy perturbation workflows)

### Design requirements

1. Explicit replay compatibility versioning
2. Typed witness mapping (not free-form strings)
3. Deterministic replay input reconstruction when sufficient data exists
4. Exact replay and counterfactual replay compatibility must be distinguished (typed)
5. Graceful degradation: “inspectable but not replayable” is allowed, but must be explicit

### Proposed API shape (contract-level)

- `ReplayBridgeRecord` (serializable summary extracted from feature audit record)
- `ReplayCompatibility`
  - `Compatible`
  - `CounterfactualOnly`
  - `RequiresSnapshot`
  - `RequiresWitness`
  - `SchemaVersionMismatch`
  - `UnsupportedOperationVersion`
- `ReplayWitnessRef` (typed provenance/policy witness references)

### Must-have tests

- replay bridge identifies missing witness vs schema mismatch distinctly
- deterministic bridge output for identical audit record input
- typed error summary preserved in replay bridge failure path
- bridge distinguishes exact-replay-compatible vs counterfactual-only cases

### Acceptance criteria

- Audit artifacts can be programmatically mapped to replay/counterfactual inputs or explicit incompatibility reasons
- No replay bridge logic depends on parsing human log strings

## Deferred Hardening Note (carry forward)

- `BoundarySegmentProvenance` currently exposes public fields, so invariants are
  enforced by `validate()` and constructors/tests rather than the type system.
  This is acceptable for the current phase because invariant validation is
  implemented and covered by adversarial tests. A future refactor should make
  invalid states unrepresentable (private fields + validated constructors/builders)
  once more subsystems depend on this payload.

## Definition of Done for Phase 2 Contracts (spec stage)

- [ ] Each `P2-*` section has finalized contract language (not implementation notes only)
- [ ] Difficulty/blast-radius notes reviewed and accepted
- [ ] Cross-dependencies between `P2-1..P2-5` are explicit
- [ ] Acceptance tests listed for each item
- [ ] Deferred hardening notes documented (including provenance invalid-state note)
