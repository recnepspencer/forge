## Milestone 9.3 Named Certification Suites

### 9.3. Query Subscription Bridge Parity And Diagnostic Sufficiency Test

Purpose

Prove that every admitted automatic query subscription family is
bridge-explainable, support-reportable, offline-diagnosable, and
runtime-certified without inventing hidden semantics beyond canonical query,
bridge, signal, and runtime lifecycle artifacts.

Scenario

- use the concrete `EmployeeRecord` fixture from Milestones 9.1 and 9.2
- derive admitted automatic subscription families for:
  - detail exact
  - inspector detail exact
  - ordered collection membership
  - grouped collection membership
  - bounded materialization where admitted
- assemble for each admitted family:
  - support report for explicit declaration/lifecycle/preview support subjects
  - support lookup receipt
  - manual bridge witness
  - bridge parity explanation
  - bridge parity receipt
  - lifecycle certification bundle
  - offline admitted diagnostic bundle
  - diagnostic assembly receipt
  - runtime family certification bundle
  - certification coverage receipt
- exercise:
  - declaration-family-preserving lifecycle closure
  - grouped and inspector families that share lower bridge classes while
    remaining query-side distinct
  - continuation and preview evidence carried into diagnostic bundles
  - hostile support, bridge-parity, and certification-coverage denials
- vary:
  - masked and unmasked policy digests
  - tenant truth/schema basis digests
  - relationship-proof digests
  - view-shape digests
  - current, branch-head, runtime snapshot, and preview-scoped bases
  - admitted and denied family coverage sets

Required concrete lanes

- detail support lane where admitted declaration, bridge lowering, lifecycle
  certification, and diagnostic bundle all bind the same declaration and bridge
  digests
- grouped family lane where grouped query-side meaning remains distinct from an
  ordered collection family even if the bridge family is shared underneath
- preview certification lane where preview discard or promotion evidence is
  surfaced in the diagnostic bundle and family certification scope
- support-overclaim denial lane where runtime-backed support is requested for a
  family lacking required hostile coverage
- denied diagnostic bundle lane where declaration or bridge denial is still
  emitted as one offline-readable denied bundle without fake lifecycle slots
- bridge-parity mismatch lane where a foreign declaration or signal strategy
  source is supplied and parity fails before a certification bundle exists

Must verify

- support reporting is query-family-aware and does not collapse distinct query
  subscription families into shared bridge-family claims
- runtime-backed support cannot be claimed for uncertified families
- support reporting is phase-typed and distinguishes declaration support,
  active lifecycle support, continuation support, preview-closeout support, and
  deferred durable/store-backed support
- support reporting exposes exact lookup posture and lookup width through a
  receipt; indexed lookup is admitted, while broader scans are explicit debt or
  denial
- store-backed restart, durable replay, and persisted continuation claims
  remain explicit deferred or denied support surfaces
- every admitted automatic family has one bridge-facing explanation binding
  query family, declaration, bridge family, bridge slices, basis posture, and
  signal strategy
- every bridge-facing explanation is bound to one tangible manual bridge
  witness describing the host-equivalent bridge request that Query claims to
  automate
- bridge parity is comparison over pre-lowered or canonically composed witness
  artifacts; witness rebuild or semantic rediscovery after witness construction
  is explicit debt or denial
- grouped and inspector families remain mechanically distinct in support,
  diagnostics, and certification artifacts even when their bridge lowerings
  share infrastructure below Query
- diagnostic bundles are sufficient for offline explanation of admitted and
  denied paths without re-running Query or consulting hidden host state
- admitted and denied diagnostic bundles are different proof types rather than
  one optional-hole envelope
- admitted and denied bundle assembly emit receipts proving stage evidence and
  semantic labels were composed rather than re-derived
- diagnostic traces localize whether failure occurred during family selection,
  declaration, bridge lowering, support reporting, lifecycle certification,
  bridge parity, or coverage closure
- declaration-family drift remains distinct from lifecycle-instance churn such
  as lane handle, attachment, delivery sequence, continuation epoch, or preview
  closeout changes
- runtime family certification requires at least one admitted row and one
  hostile row for each supported family
- runtime family certification also requires representative basis, policy,
  tenant, relationship-proof, view-shape, and lifecycle-class variation rows
  for each supported family
- runtime family certification consumes family-scoped indexed coverage handles
  or explicit matrix-scan debt/denial posture; raw row iteration is not an
  invisible implementation choice
- support, bridge parity, lifecycle certification, and runtime family
  certification bind the same canonical query/declaration/bridge digests
- compile-fail boundaries prove external callers cannot mint support reports,
  bridge parity explanations, diagnostic bundles, or runtime family
  certification bundles directly
- small/medium/larger fixture runs prove support and bundle assembly cost
  slopes are bounded by family coverage width, diagnostic bundle width, and
  indexed coverage width rather than unrelated row count

Required verification output

- `query_digest`
- `subscription_family_digest`
- `subscription_declaration_digest`
- `subscription_equivalence_digest`
- `bridge_declaration_digest`
- `bridge_basis_digest`
- `signal_strategy_digest`
- `support_report_digest`
- `support_matrix_digest`
- `support_lookup_receipt_digest`
- `manual_bridge_witness_digest`
- `bridge_parity_digest`
- `bridge_parity_receipt_digest`
- `diagnostic_trace_digest`
- `admitted_diagnostic_bundle_digest`
- `denied_diagnostic_bundle_digest`
- `diagnostic_assembly_receipt_digest`
- `lifecycle_certification_digest`
- `runtime_certification_bundle_digest`
- `certification_coverage_receipt_digest`
- `continuation_digest`
- `preview_isolation_digest`
- `failure_digest`
- `counter_snapshot`
- `subscription_support_scale_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Automatic subscription family selection remains bridge-honest, support claims
remain synchronized with certified runtime behavior, diagnostic bundles remain
offline-sufficient, and unsupported or uncertified family claims fail-closed
before store-backed or durable milestones build on top of them.

## Milestone 9.3.1 Named Certification Suites

### 9.3.1. Cross-Runtime Causal Explanation Envelope Test

Purpose

Prove that Query inspection can expose one bridge-owned cross-runtime causal
explanation envelope for query-observed outcomes, anchored to the Query
operational artifact that produced the observation, without requiring
downstream domains to stitch runtime bridge diagnostics, relational runtime
evidence, and signal graph internals directly.

Scenario

- use a concrete fixture with:
  - a relational truth mutation that changes a query-observed result
  - a relational truth mutation that is causally relevant but suppressed by
    query shape, policy, tolerance, or result equivalence
  - a branch or preview mutation whose basis must remain explicit
  - a historical or replayed observation whose materialization path must remain
    explicit
- ask Query inspection for causal explanations of changed, suppressed, denied,
  branch/preview, and replayed observations
- derive a causal observation anchor and evidence-reference set from each Query
  operational artifact before inspection admission
- prove the ordered progression with proof-bearing phase artifacts so request,
  reference, admission, bridge-envelope, and materialization phases cannot be
  skipped
- assemble one bridge-owned causal explanation envelope for each admitted
  or advisory inspection request
- materialize one Query-owned causal inspection artifact from each admitted or
  advisory bridge envelope
- exercise hostile missing-evidence lanes for missing bridge route evidence,
  missing signal evidence, incompatible relational authority, policy-redacted
  detail, and unsupported explanation families
- include a Worth-style consumer lane that is allowed to consume only Query
  inspection artifacts, not runtime bridge diagnostics facade handles,
  relational runtime internals, or signal graph internals

Required concrete lanes

- changed-result lane where relational authority, bridge route/evaluation,
  bridge source/structural/preview/writeback evidence where applicable, signal
  invalidation/evaluation, signal forensic availability, lineage, provenance,
  replay posture, and query observation digests all bind the same query
  observation anchor
- no-op/suppression lane where the causal chain remains referenceable even
  though the query-shaped public result does not change
- advisory-redaction lane where the causal explanation is valid but detail is
  narrowed with a success/advisory/violation admission trace rather than
  collapsed into a binary admitted/denied result
- branch/preview lane where basis identity and preview closeout posture remain
  visible in the causal envelope
- replay lane where materialization path and replay posture are explicit and
  distinguished from live invalidation
- missing-signal-evidence denial lane where the bridge route exists but signal
  invalidation or evaluation evidence cannot be admitted
- missing-bridge-route denial lane where signal or relational evidence alone is
  insufficient to mint a public causal explanation
- policy-redaction lane where richer narrative detail is denied or narrowed
  without changing causal digests or query meaning
- public-boundary lane where domain code that tries to construct the ordinary
  explanation through lower-runtime imports fails at the certified boundary

Must verify

- Query owns the public causal inspection request, admission record, redaction
  posture, result-shape context, causal observation anchor, evidence-reference
  set, and materialized inspection artifact
- runtime bridge owns the cross-runtime causal explanation envelope and names
  which lower runtime supplied each evidence family
- relational remains the authority for truth, commits, snapshots, and
  relational decision evidence
- signal remains the authority for observation, invalidation, scheduling,
  signal lineage, and signal provenance evidence
- bridge, relational, and signal evidence digests agree with the lower-runtime
  records they summarize
- Query evidence references resolve through existing lower-runtime records,
  Query inspection evidence, signal forensic availability, and relational
  authority identities rather than a new Query-owned diagnostics store
- changed, suppressed, denied, branch/preview, and replayed observations all
  receive typed artifacts rather than optional holes in one loose structure
- success, advisory, and violation outcomes are represented as typed admission
  decisions with decision-trace indexes
- forge-proof or an equivalent proof-bearing substrate enforces phase ordering,
  authority witnesses, fixed-shape/canonical evidence-reference collections,
  and trust-boundary readmission between Query and runtime bridge
- missing evidence fails as typed diagnostic denial and redacted/narrowed
  evidence becomes typed advisory detail rather than best-effort prose
- diagnostic richness changes only cold-path materialization detail, not query
  meaning, bridge route meaning, signal invalidation meaning, or relational
  authority
- hot-path query execution and signal invalidation emit only bounded evidence
  references, counters, and digests needed for later cold-path explanation
- ordinary downstream domains can explain query observations through Query
  inspection without direct imports from runtime bridge diagnostics,
  relational runtime internals, or signal graph internals
- small/medium/larger fixture runs prove causal inspection cost slopes are
  bounded by evidence-reference width, requested richness, and admitted
  materialization width rather than unrelated runtime graph size or total
  bridge diagnostics retention width
- performance counters are reported separately for observation anchor
  derivation, evidence-reference resolution, inspection admission, bridge
  envelope assembly, redaction, materialization, and artifact serialization

Required verification output

- `query_digest`
- `query_observation_receipt_digest`
- `causal_observation_anchor_digest`
- `inspection_target_digest`
- `inspection_request_digest`
- `admitted_inspection_digest`
- `advisory_inspection_digest`
- `denied_inspection_digest`
- `causal_envelope_digest`
- `causal_evidence_reference_digest`
- `decision_trace_index_digest`
- `relational_authority_digest`
- `bridge_route_digest`
- `bridge_evaluation_digest`
- `bridge_source_materialization_digest`
- `bridge_structural_digest`
- `bridge_stream_digest`
- `bridge_preview_digest`
- `bridge_writeback_digest`
- `signal_invalidation_digest`
- `signal_evaluation_digest`
- `signal_forensic_availability_digest`
- `signal_lineage_digest`
- `signal_provenance_digest`
- `query_observation_digest`
- `materialization_policy_digest`
- `replay_posture_digest`
- `redaction_policy_digest`
- `admitted_causal_inspection_artifact_digest`
- `advisory_causal_inspection_artifact_digest`
- `denied_causal_inspection_artifact_digest`
- `causal_materialization_receipt_digest`
- `causal_inspection_proof_shape_digest`
- `causal_inspection_phase_progression_digest`
- `causal_inspection_witness_authority_digest`
- `causal_evidence_reference_collection_proof_digest`
- `causal_bridge_readmission_proof_digest`
- `failure_digest`
- `counter_snapshot`
- `causal_inspection_scale_slope_digest`
- `causal_anchor_derivation_slope_digest`
- `causal_reference_resolution_slope_digest`
- `causal_admission_slope_digest`
- `causal_bridge_envelope_slope_digest`
- `causal_materialization_slope_digest`
- `causal_artifact_serialization_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Query inspection exposes one authority-preserving cross-runtime causal
explanation surface, changed and non-changed outcomes are equally explainable,
missing evidence fails closed, redacted evidence is represented as typed
advisory/narrowed detail, and domain consumers no longer need direct
lower-runtime stitching to explain why query-observed outcomes happened.

## Milestone 9.3.2 Named Certification Suites

### 9.3.2. Query Basis Capability Lifecycle Test

Purpose

Prove that Query basis is a phase-typed capability lifecycle rather than a raw
branch, preview, tenant, policy, snapshot, historical, or runtime identifier.
Every observation, mutation-preparation, replay, inspection, materialization,
subscription, and certification surface must consume admitted basis capability
proof or fail typed before constructing operational artifacts.

Scenario

- use concrete runtime-backed fixtures with current-head, branch-head, explicit
  branch snapshot, preview, preview-derived, runtime snapshot, historical,
  tenant-scoped, and policy-scoped basis intents
- construct equivalent basis intents through at least two public or
  compatibility paths and prove they normalize to the same capability envelope
- construct intentionally different basis meanings and prove their declared
  digest fields diverge
- consume admitted basis capabilities through observation, mutation
  preparation, replay, inspection, materialization, subscription declaration,
  subscription activation, preview closeout, and certification lanes where
  those operation families are admitted
- compile golden DX transcripts for current-head observation, branch-head
  mutation preparation, inspection with lower-runtime evidence materialization,
  support discovery, and typed denial handling
- prove common-path callers can express basis intent, choose an operation lane,
  execute the admitted operation, inspect support posture, and read the basis
  envelope without manually constructing proof internals
- exercise the lifecycle as a linear typestate chain where each transition
  consumes the prior proof type and returns the next proof type or a typed
  denial
- prove operation lanes are unforgeable lane witnesses or operation-specific
  wrappers rather than booleans, strings, or loose enum flags on one universal
  token
- bind basis capabilities to retained relational, runtime-bridge, and signal
  evidence without letting Query mint lower-runtime authority records
- prove the lower-runtime binding path reuses bridge subscription, truth-view,
  continuity, preview, writeback, and causal-envelope facade artifacts;
  relational branch/head/snapshot/history facade artifacts and
  `RuntimeBridgeRelationalSource`; and signal snapshot, replay, lineage, and
  diagnostic facade artifacts
- emit an API reuse matrix with one row for each basis-adjacent lower-runtime
  surface naming owning crate, owning facade type/function/trait, existing
  authority artifact, Query wrapper type, allowed carried fields, forbidden
  duplicate fields, consuming operation lanes, denial/deferred posture, and
  the test or compile-fail proof that enforces the row
- exercise hostile lanes for stale preview, inaccessible branch, policy mask,
  tenant/schema mismatch, historical replay unsupported, lower-runtime binding
  mismatch, missing signal observation basis, durable reload overclaim,
  temporal deferred, and async/resource deferred requests

Required concrete lanes

- current-head observation lane where canonical current-head intent becomes an
  admitted observation capability and emits a basis use receipt
- branch-head mutation-preparation lane where mutation setup consumes scoped
  basis capability proof rather than a raw branch identifier
- preview closeout lane where stale preview basis drift denies before authority
  execution or closeout artifacts exist
- historical/replay lane where admitted historical posture is distinct from
  durable replay or store-restored snapshot-plus-tail claims
- tenant/policy lane where equivalent scoped intents normalize while policy or
  tenant/schema mismatch denies typed and early
- causal-inspection lane where the 9.3.1 causal observation anchor consumes
  basis capability proof and preserves lower-runtime authority names
- bridge-readmission mismatch lane where returned bridge or relational basis
  evidence conflicts with the admitted Query capability and fails before a use
  receipt exists
- future-neighbor lane where temporal, async/resource, store-backed, and
  durable basis requests report deferred or unsupported posture with zero
  operational residue

Must verify

- raw basis identifiers are not capabilities and cannot reach read, mutation,
  replay, inspection, materialization, or subscription entrypoints directly
- target DX APIs expose ordinary basis authoring through narrow, memorable
  facade methods while keeping expensive or boundary-crossing work visible
  through explicit calls such as admission, execution, lower-runtime evidence
  inclusion, materialization, and certification
- golden DX transcripts remain synchronized with executable admission behavior,
  support metadata, receipts, and self-describing envelopes
- common-path inspection/envelope APIs expose lower-runtime authority bindings
  without requiring caller-side bridge, signal, or relational stitching
- equivalent basis intents normalize to the same `NormalizedBasisIntent` and
  self-describing basis envelope
- intentionally different basis authority, scope, visibility, lifecycle,
  tenant/schema, policy, operation lane, or lower-runtime binding changes the
  relevant digest
- eligibility precedes operational artifact construction for every operation
  lane
- observation, mutation-preparation, replay, inspection, materialization,
  subscription, preview closeout, and certification lanes are distinct
  capability proofs rather than booleans on one raw token
- advisory, success, and violation eligibility outcomes cannot be substituted
  for one another in downstream phase signatures
- plain digests cannot be passed to executable APIs where proof-bearing
  lifecycle types are required
- lifecycle typestate tests prove production APIs cannot skip directly from
  `RawBasisIntent` or `NormalizedBasisIntent` to scoped use, lower-runtime
  binding, receipt, envelope, or certification artifacts without consuming the
  required proof-bearing transition types
- compatibility debt is registered with owner, entrypoint, target lifecycle
  phase, blocking reason, and denial/adapter posture; no unowned compatibility
  bucket may satisfy certification
- denied basis capability artifacts are different proof types from admitted
  basis capabilities
- lower-runtime relational, bridge, and signal evidence is readmitted by digest
  and authority name rather than re-minted as Query-owned facts
- Query does not create twins of bridge `ValidatedSubscriptionBasisBinding`,
  bridge truth-view authority, bridge continuity authority, bridge writeback
  bases, relational `BranchHead`/`SnapshotHandle`/`CanonicalCommitEnvelope`,
  or signal `SignalSnapshotV1`/`SignalCheckpointImage`/`LineageRecord`/
  `ReplayCursor`
- bridge-bound relational truth flows use `RuntimeBridgeRelationalSource`
  instead of Query-side commit/snapshot/history loaders
- any lower-runtime API gap is reported as deferred or unsupported support
  posture rather than patched with a private-module import or parallel Query
  authority model
- every lower-runtime adapter carries only facade-returned identity, digest,
  receipt, denial, counter, support-posture, authority-label, and Query
  lifecycle proof fields; no adapter carries reconstructive lower-runtime
  fields sufficient to replay, restore, re-resolve, or reissue authority-owned
  records
- the API reuse matrix is executable evidence: if a row lists a forbidden
  duplicate field, compile-fail or structural tests prove Query cannot expose
  or construct that duplicate
- support metadata, executable admission, and certification coverage agree for
  admitted, advisory, denied, deferred, and unsupported basis families
- support metadata is derived from executable admission facts or certification
  fails when admission and support rows drift
- temporal, async/resource, store-backed parity, durable reload, and
  restart-stable basis envelopes remain typed deferred or unsupported until
  their owning milestones close
- compile-fail boundaries prove external callers cannot mint normalized basis
  intents, admitted capabilities, scoped bases, receipts, envelopes, support
  rows, certification bundles, or lower-runtime authority witnesses
- small/medium/larger fixture runs prove normalization, eligibility,
  lower-runtime binding, scoped-use construction, receipt emission, envelope
  materialization, support lookup, and certification costs are bounded by basis
  evidence width, operation-lane width, and lower-runtime binding width rather
  than unrelated runtime graph size or retained diagnostics width

Required verification output

- `query_digest`
- `raw_basis_intent_digest`
- `normalized_basis_intent_digest`
- `basis_family_digest`
- `basis_authority_digest`
- `basis_scope_digest`
- `basis_visibility_digest`
- `basis_lifecycle_digest`
- `basis_policy_digest`
- `basis_tenant_schema_digest`
- `basis_operation_lane_digest`
- `basis_eligibility_digest`
- `admitted_basis_capability_digest`
- `denied_basis_capability_digest`
- `scoped_basis_digest`
- `basis_use_receipt_digest`
- `basis_envelope_digest`
- `relational_basis_authority_digest`
- `bridge_basis_authority_digest`
- `signal_basis_authority_digest`
- `lower_runtime_basis_binding_digest`
- `basis_readmission_proof_digest`
- `basis_target_dx_digest`
- `basis_golden_transcript_digest`
- `lower_runtime_api_reuse_matrix_digest`
- `adapter_shape_contract_digest`
- `typestate_transition_digest`
- `lane_witness_digest`
- `phase_artifact_manifest_digest`
- `compatibility_debt_registry_digest`
- `basis_transition_digest`
- `basis_support_matrix_digest`
- `basis_future_neighbor_denial_digest`
- `basis_proof_shape_digest`
- `basis_phase_progression_digest`
- `failure_digest`
- `counter_snapshot`
- `basis_normalization_slope_digest`
- `basis_eligibility_slope_digest`
- `basis_lower_runtime_binding_slope_digest`
- `basis_scoped_use_slope_digest`
- `basis_receipt_slope_digest`
- `basis_envelope_materialization_slope_digest`
- `basis_support_lookup_slope_digest`
- `compile_fail_boundary_digest`

Pass condition

Every runtime-backed Query basis consumed by ordinary public surfaces is
admitted, denied, or deferred through one phase-typed capability lifecycle;
raw lower-runtime identifiers cannot act as capability tokens; lower-runtime
authority remains relational/bridge/signal-owned; and later temporal,
async/resource, store-backed, and durable milestones inherit this lifecycle
instead of adding parallel basis APIs.

## Runtime API Public Stabilization Gate Named Certification Suites

### Runtime API Golden DX And Async-Safe Facade Test

Purpose

Prove that the ordinary public runtime API is beautiful enough for serious
domain runtimes and structurally honest enough for later temporal/async
extension, without relying on lower-runtime plumbing or sync-only assumptions.

Scenario

- execute golden public-facade transcripts for:
  - workflow/editor surfaces with live sections, nested computeds, conditional
    effects, branch preview, branch-local intent, and inspection
  - geometry/kernel surfaces with topology neighborhood live views, expensive
    derived outputs, fallback posture, invariant-preserving intent, and branch
    experiment inspection
  - table/spreadsheet surfaces with visible rows, formula/dropdown/layout
    computeds, ordered/grouped delivery, batched edit intent, and async-ready
    state vocabulary
  - one composed adversarial surface spanning live subscription evidence,
    nested computeds, effect pending intents, authoritative/effect/branch
    intents, preview isolation, inspection, and feedback phase graph evidence
- exercise unsupported-neighbor requests for temporal basis, async/resource
  state, mixed-cause delivery, store-backed parity, and durable restart before
  those milestones are admitted

Must verify

- golden transcripts use the final public facade vocabulary only
- no transcript manually installs subscription declarations, bridge
  lowerings, signal observers, grouped baselines, active lanes, or CDC filters
- every transcript asserts meaningful proof artifacts: receipts, support
  posture, authority lanes, basis lanes, aspect contracts, dependency handles,
  delivery batches, residue counters, and inspection sections
- state/read APIs do not imply always-synchronous values for surfaces that may
  later admit temporal or async/resource state
- unsupported temporal/async/store/durable neighbors fail typed and early with
  zero forbidden delivery or authority residue
- inspection explains every advertised handle/receipt family from retained
  artifacts rather than debug strings or re-lowering
- final public names do not hide query execution, subscription activation,
  branch/preview binding, intent commit, temporal/async, or diagnostic
  boundary crossings
- support metadata and executable admission agree for stable, deferred, and
  unsupported public API families
- compile-fail boundaries prove ordinary callers cannot bypass the facade or
  synthesize proof-bearing handle/inspection/support artifacts externally

Required verification output

- `public_api_surface_digest`
- `golden_transcript_digest`
- `handle_contract_digest`
- `state_contract_digest`
- `aspect_contract_digest`
- `authority_lane_digest`
- `inspection_contract_digest`
- `support_matrix_digest`
- `deferred_temporal_async_gate_digest`
- `failure_digest`
- `counter_snapshot`
- `compile_fail_boundary_digest`

Pass condition

The public runtime API is stable enough for domain runtimes to build on now,
while temporal, async/resource, mixed-cause, store-backed, and durable claims
remain explicit deferred or unsupported surfaces until their owning milestones
close.

## Runtime Authoritative Mutation Evidence Gate Named Certification Suites

### Runtime Authoritative Mutation Evidence And Existing-Truth Binding Test

Purpose

Prove that the public mutation facade preserves enough authored and resolved
authority meaning that write-heavy domains do not need local target recovery,
existing-truth rebinding, causality/provenance reconstruction, or
naming/continuity explanation glue.

Scenario

- execute direct authoritative and preview-local mutation sessions covering:
  - insert of new truth
  - update of existing truth through admitted existing-target binding
  - delete of existing truth with explicit touched-aspect meaning
  - retained existing-truth assertion through `assert_existing(...)`
  - backend-verified existing-truth assertion through `verify_existing(...)`
  - backend-verified existing-truth probe through `probe_existing(...)`
  - backend-verified existing-target update through `update_existing_verified(...)`
  - backend-verified existing-target delete through `delete_existing_verified(...)`
  - ordered batch mixing same-batch symbolic references and existing-truth
    references
  - hostile mixed authority sessions that combine retained assertion,
    backend-verified assertion, backend-verified update/delete, same-batch
    symbolic naming, continuity-aware mutation evidence, and probe-before or
    probe-after truth checks
  - authoritative import/session aggregation where the session cannot honestly
    be summarized as one scalar write
  - admitted naming-aware mutation evidence
  - admitted continuity-aware mutation evidence
- exercise unsupported-neighbor requests for:
  - unresolved existing-truth binding
  - collection-mismatched or missing existing-truth binding
  - backend verification unsupported for assertion, probe, and verified
    mutation lanes
  - unsupported naming-writeback family
  - unsupported continuity-sensitive family

Must verify

- receipts preserve declared-versus-resolved target evidence together with
  touched-aspect fallout evidence
- retained assertions, backend-verified assertions, backend-verified probes,
  verified updates, and verified deletes remain semantically distinguishable
  across receipts, scalar inspection, batch inspection, and aggregate session
  evidence
- receipts preserve first-class causality and provenance bundles rather than
  flattening source lineage into incidental metadata
- preview and authoritative receipts use the same target-evidence model
- batch/session inspection exposes per-component and aggregate authority
  evidence honestly, including causality/provenance summaries
- same-batch symbolic targets, existing authoritative targets, and denied
  bindings are distinguished explicitly
- impossible mutation-family or assertion-mode combinations fail loudly rather
  than producing misleading aggregate summaries
- naming-aware and continuity-aware families either preserve explicit outcome
  evidence or deny typed and early
- support metadata and executable admission behavior agree for all admitted and
  denied mutation-evidence neighbors

Required verification output

- `mutation_receipt_digest`
- `mutation_target_evidence_digest`
- `batch_authority_evidence_digest`
- `existing_truth_mode_digest`
- `existing_truth_probe_digest`
- `mutation_causality_digest`
- `mutation_provenance_digest`
- `existing_truth_binding_digest`
- `naming_mutation_evidence_digest`
- `continuity_mutation_evidence_digest`
- `support_matrix_digest`
- `failure_digest`
- `counter_snapshot`

Pass condition

The public mutation surface preserves target, authority, and admitted
naming/continuity evidence strongly enough that downstream domains can rely on
Query receipts and inspection instead of rebuilding the same explanation layer
locally, and the bridge-side replay/provenance artifacts remain compatible with
that same public evidence story.

### Runtime Mixed-Shape Graph Authoring And Identity-Preserving Mutation Test

Purpose

Prove that the public runtime mutation facade can serve as a real graph-shaped
authoring runtime for serious downstream domains without forcing them to
reintroduce local relation-rewrite semantics, symbolic graph folklore, or
bridge-backed verification glue.

Scenario

- execute public runtime authoring sessions covering:
  - identity-preserving existing-target relation update
  - identity-preserving existing-target relation verified update
  - same-batch graph composition with:
    - created entity -> created relation
    - existing entity -> created relation
    - mixed existing-target and same-batch symbolic references
    - symbolic entity follow-up mutation
    - symbolic relation follow-up mutation
    - symbolic relation retirement
    - mixed create/update/delete composition in one canonical program
  - geometry-inspired hostile generic graph programs covering:
    - `EdgeSplit`
      - existing edge relation retired
      - two new edges created
      - one new vertex created
      - adjacency relations rewired
      - receipt proves lineage and identity outcomes
    - `LoopSuccessorRewire`
      - existing successor relation retargets old successor to new successor
      - relation identity is preserved
      - verification asserts old source/target assumptions before mutation
    - `FaceInnerLoopInsertion`
      - create loop entity
      - create relation from existing face to symbolic loop
      - create symbolic edges/vertices
      - resolution map exposes all symbolic-to-resolved identities
    - `FailedNonManifoldAdmission`
      - substrate can express the graph shape
      - domain invariant hook denies it
      - denial remains distinct from runtime support denial
  - bridge-backed backend-verified existing-truth assertion/probe/verified
    update/delete on admitted entity and relation families
- exercise hostile neighbors for:
  - replacement-shaped relation rewrite masquerading as update
  - unresolved symbolic graph references
  - reversed or illegal composition order
  - incomplete invariant-bearing graph subgraphs
  - bridge-backed runtime families lacking verification support
  - domain-invalid but substrate-expressible graph programs
  - target-collection and target-shape mismatch

Must verify

- existing-target relation update preserves authoritative relation identity
  across receipt, inspection, and batch/session aggregate evidence
- update denial fires typed and early whenever the lower runtime cannot preserve
  identity honestly
- graph composition is one public authoring family rather than caller-owned
  scalar batch folklore
- graph composition receipts preserve symbolic-to-resolved target mapping,
  component ordering, target evidence, lifecycle-step meaning, lifecycle
  outcome meaning, and affected live/computed breadth
- graph composition preserves and exposes assumption/read-set evidence
  distinctly from target binding and mutation result evidence
- graph composition denial distinguishes unresolved symbolic references,
  illegal ordering, incomplete subgraph workflows, unsupported mixed-shape
  capability families, lower-runtime identity-preservation gaps, verification
  substrate unavailability, domain invariant denial, and unsupported runtime
  families
- denied paths expose an admission trace showing the classification stage where
  execution stopped, and that trace is not the same artifact as a receipt
- bridge-backed backend-verified existing-truth surfaces report admitted versus
  unsupported posture honestly through support metadata and executable behavior
- memory or compatibility-backed support does not overclaim bridge-backed
  production support
- public docs/examples match the admitted and denied runtime story literally
- compile-fail boundaries prove external callers cannot mint proof-bearing
  graph-composition or support/closeout artifacts directly

Required verification output

- `relation_update_identity_digest`
- `relation_update_target_evidence_digest`
- `relation_update_denial_digest`
- `graph_composition_digest`
- `graph_symbolic_resolution_digest`
- `graph_composition_program_digest`
- `graph_composition_denial_digest`
- `graph_composition_lifecycle_digest`
- `verified_assumption_set_digest`
- `assumption_snapshot_digest`
- `verified_precondition_digest`
- `verification_read_set_breadth_digest`
- `admission_trace_digest`
- `verified_existing_bridge_support_digest`
- `verified_existing_runtime_receipt_digest`
- `support_matrix_digest`
- `failure_digest`
- `counter_snapshot`
- `compile_fail_boundary_digest`

Pass condition

The public runtime authoring facade is strong enough that downstream graph
domains can use Query as the ordinary mutation runtime for admitted relation
updates, same-batch graph composition, and bridge-backed verified existing-
truth work, while unsupported neighbors fail closed before local substitute
semantics can grow around them.

