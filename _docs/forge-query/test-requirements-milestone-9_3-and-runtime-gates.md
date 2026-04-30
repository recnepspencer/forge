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

