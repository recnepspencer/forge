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

This spec covers five Foundation Phase 2 contracts:
1. Trace adjunct/versioning strategy (typed policy/provenance payload attachment)
2. Operation finalization contract (context drain + envelope merge + audit emit)
3. Policy registry/config source model
4. Persistent-name resolution result contract (typed + traced)
5. Replay/audit bridge contract

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
- `P2-5 Replay/audit bridge contract`: **High**
  - Blast radius: audit artifact schema, replay analysis modules, witness formats
  - Risk: building rich audit artifacts that are not actually replayable

Implementation order (recommended):
1. `P2-1`
2. `P2-2`
3. `P2-3`
4. `P2-4`
5. `P2-5`

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

### Contract

Name/selector resolution returns a typed result:
- `Resolved(T)`
- `Ambiguous { candidates, reason }`
- `Missing { queried }`

All non-`Resolved` outcomes are traced and fail-closed by default.

### Design requirements

1. No first-match behavior
2. Candidate ordering deterministic
3. Snapshot handles only in resolution internals or explicitly labeled snapshot debug fields
4. Persistent output identity separated from snapshot debug identity
5. Resolution outputs must be compatible with lineage-based re-identification workflows (not naming-only islands)

### Proposed API shape (contract-level)

- `ResolutionResult<T>`
- `ResolutionCandidate` (typed + serializable, with snapshot and/or persistent labels)
- `ResolutionTracePayload` (adjunct family for `P2-1`)

### Must-have tests

- ambiguous selector returns deterministic candidate order
- missing selector emits typed trace payload and fails closed
- generation reuse does not cause persistent selector success via stale snapshot handle leakage
- lineage-backed re-identification remains possible after resolution + merge (or typed incompatibility is reported)

### Acceptance criteria

- Region merge can reuse one typed resolution contract (not custom result enums)
- Ambiguity and missing outcomes are machine-readable and traced

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
