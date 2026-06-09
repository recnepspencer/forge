# Milestone 9.9 Engineering Spec: Graph Touch Obligation Authority

> **Status:** Draft
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Primary predecessors:** [milestone-9.8.md](./milestone-9.8.md), [milestone-9.7.md](./milestone-9.7.md), [milestone-9.6.md](./milestone-9.6.md)
>
> **Purpose:** establish graph touch obligation dispatch as a complete Query
> authority boundary — every obligation kind executable on every real covered
> read/mutation lane, typed verdicts, canonical dispatch artifacts on receipts and decision
> traces, index-backed selection, relational execution bridge, duplicate-rule
> elimination, and mechanical consumer anti-folklore — certified
> architecturally and proven by reference-consumer deletion of parallel
> legality.

## Goal

Make touch shape and operating world mechanically determine which obligations
execute, with typed verdicts and canonical dispatch artifacts on every covered
Query path that already carries equivalent read/mutation meaning. Host-local
legality graphs, manual invariant-pack closures, and caller-side "remember to
run this check" code must become unrepresentable on covered surfaces.

Investigation shows the real seams are not where the first draft assumed:

- **`write_graph_batch` already flows through authoritative write-batch intent
  admission** with graph program and breadth in
  `ForgeQueryAuthoritativeMutationBatchIntentSeed` — obligation dispatch belongs
  there, not as a side callback before intent review.
- **Forge Query already freezes its covered intent-admission floor in tests and
  docs** — authoritative mutation includes scalar write, batch write, and
  effect-triggered write-intent execution; read execution includes family,
  basis-context, and live-read paths. This spec must follow that shipped
  surface inventory rather than a narrower mental model.
- **`ComposeGraph` is declared for primitive construction but never executed**
  — kernel phase-chain legality is offline-only; worth-topo construction builds
  handoffs without `compose_graph`.
- **worth-topo operators already enter through configured domain handles and
  contribution orchestration** before `compose_graph` or `batch` — kernel
  construction does not.

This milestone builds the complete authority pipeline on those real seams.

## Why This Milestone Exists

Query exposes graph composition, relational invariant registration, policy
narrowing on read, configured domain handles, contribution orchestration,
intent admission for authoritative mutations, and commit-boundary execution —
as separate folklore lanes downstream code re-stitches by hand.

`worth-topo` calls `compose_graph` and `batch` yet enforces loop wiring in
three places. `worth-kernel` runs a full offline phase-chain legality pipeline
while declaring `ComposeGraph`, without operating context and with motion
sequencing guarded by `unreachable!`.

Per `MENTALITY.md` §2 and §14: build the foundation that survives the
adversarial constraint now; expand scope across crates until the blocker is
gone.

## Governing Summaries

- `MENTALITY.md`: enforce mechanically; scope expands until blockers are gone;
  debt is not a substitute for building the real path.
- `arch_laws.md`: Law 4, 5, 12, 17, 33 — shape determines checks; three-state
  verdicts; applicability pre-solved at entry; one truth, derived views.
- `composition_laws.md`: registration, indexing, dispatch, envelope, and denial
  projection remain distinct named surfaces.
- `domain_structure_laws.md`: relational executes invariant semantics; Query
  owns routing, touch vocabulary, and dispatch artifacts.
- `perf_laws.md`: dispatch selection is index lookup with named complexity
  contracts and proof tests.
- `forge_query_roadmap.md`: graph authoring through Query is dishonest while
  legality lives in consumer callbacks.

## Adversarial Constraint

For any registered obligation set, any admitted graph composition or
touch-bearing read/mutation artifact, and any operating world on the handle,
obligation dispatch is a pure function of
`(touch_descriptor, operating_world_descriptor, obligation_index)` on every
covered lane that already exists in Forge Query: authoritative scalar write,
authoritative batch write, effect-triggered write-intent execution,
declaration-entry orchestration where it gates mutation, read-family
execution, live-read execution, preview-local direct mutation, and preview/
branch intent execution where applicable.

Certification proves exact-zero tolerance on covered surfaces for: false
negatives, false positives, duplicate rule implementations, manual pre-check
residue, dispatch-plan drift, obligation kinds without executable paths, and
mutation lanes that bypass dispatch.

## Product Decision Lock

- All obligation kinds built executable:
  `BlockingInvariant`, `SchemaContractValidator`, `AdvisoryObligation`,
  `PreflightSequencingObligation`, `CapabilityGapScreen`,
  `OperatingContextGate`.
- Dispatch integrates with the **authoritative mutation intent-admission
  family**. Graph composition anchors on authoritative write-batch review,
  while scalar-write and effect-triggered fronts must reuse the same touch
  vocabulary instead of becoming parallel obligation paths.
- Relational gains explicit graph-composition execution point; existing
  milestone-one registrations migrate from commit-only posture.
- Policy-aware graph mutation execution built so operating-context gates run
  on write authority.
- Primitive construction birth **executes** `compose_graph` from admitted
  handoffs — handoff-only receipts are not closure.
- Every dispatch materializes a canonical envelope; receipts and intent
  decision traces carry obligation evidence.
- Closure = architectural certification + full reference adoption in
  `worth-topo` and `worth-kernel` — not operator samples without birth
  execution.

## Phase Plan

### Phase 1: Obligation Authority Model And Dispatch Envelope

Freeze sealed obligation kinds, three-state verdicts (`Allow` / `Advise` /
`Block`), dispatch plan rows, and the canonical dispatch envelope — including
multi-obligation recordings when several rules fire on one touch.

**Relevant subsystems**
- `crates/forge-query/src/runtime/mutation/graph_composition/`
- `crates/forge-query/src/intent_admission/trace/`

**Relevant APIs**
- new: `ForgeQueryGraphObligationKind`, `ForgeQueryGraphObligationVerdict`,
  `ForgeQueryGraphObligationDispatchPlan`,
  `ForgeQueryGraphObligationDispatchEnvelope`
- `ForgeQueryGraphCompositionDomainInvariantDenial`
- `ForgeQueryIntentDecisionTraceEnvelope`

**Test requirements**
- Adversarial equivalence: envelope digests stable under replay.
- Adversarial localization: host-forged envelopes uncompilable or sealed.
- Adversarial completeness: multi-obligation touches record every fired rule.

**Engineering decisions**
- Block outcomes lower into graph-composition domain-invariant denials where
  compatible; advise outcomes carry structured context.
- Envelope scheme version explicit; digests lower through Milestone `9.6`
  evidence identity where applicable.

**Open questions**
- None.

### Phase 2: Graph Touch Descriptor Vocabulary

Ship sealed `ForgeQueryGraphTouchDescriptor` derived from graph composition
programs, batch mutation shape, command breadth, write commands, and touched
aspect keys (including `.touches(...)` where present).

**Relevant subsystems**
- `crates/forge-query/src/runtime/mutation/graph_composition/`
- `crates/forge-query/src/runtime/surface/graph_composition_domain_invariant_summary.rs`
- `crates/forge-query/src/runtime/mutation/` (batch lowering)

**Relevant APIs**
- `ForgeQueryGraphCompositionProgram`, `ForgeQueryGraphCompositionProgramStepKind`
- `ForgeQueryAuthoritativeMutationBatchIntentSeed` (graph artifacts already carried)
- new: `ForgeQueryGraphTouchDescriptor`

**Test requirements**
- Adversarial parity: semantic-equivalent programs → equal descriptors.
- Adversarial rejection: relation kind / lifecycle / aspect changes alter digest.

**Engineering decisions**
- Read touch descriptors use the same vocabulary rules as mutation descriptors.

**Open questions**
- None.

### Phase 3: Registration, Touch Selectors, And Operating World Binding

Ship touch selectors, operating-world selectors, and registration for every
obligation kind on the ordinary Query runtime builder. Auto-index native
relational schema contracts from lowered contract descriptors.

**Relevant subsystems**
- `crates/forge-query/src/runtime/builder.rs`
- `crates/forge-relational/src/validation/data/`, `schema/`

**Test requirements**
- Adversarial equivalence: selectors do not cross-match unrelated lanes.
- Adversarial rejection: conflicting registrations fail at assembly.

**Engineering decisions**
- Custom invariant scope planners normalize into selector vocabulary.

**Open questions**
- None.

### Phase 4: Assembly Index And Complexity Contract

Build `ForgeQueryGraphObligationIndex` at assembly with inspectable support rows
and named complexity contracts for index build and dispatch selection.

**Test requirements**
- Adversarial parity: identical registration → identical index digests.
- Adversarial budget: dispatch selection counters prove O(matched obligations).

**Engineering decisions**
- Lookup: `(touch_descriptor, operating_world_descriptor) -> dispatch plan`.
- Support rows report Verified for every obligation kind and lane this
  milestone ships.

**Open questions**
- None.

### Phase 5: Graph Composition Execution Point And Relational Rule Migration

Add explicit relational graph-composition invariant execution point. Migrate
covered milestone-one custom invariant registrations from commit-only execution
to graph-composition + commit backstop with single rule identity.

**Relevant subsystems**
- `crates/forge-relational/src/validation/data/execution.rs`
- `crates/forge-relational/src/validation/engine/`
- `crates/worth-topo/src/validation/reference_integrity/`

**Relevant APIs**
- `InvariantExecutionPoint` (new graph-composition variant)
- `milestone_one_invariant_registrations()`

**Test requirements**
- Adversarial parity: `.m1.topology.loop_wiring` and siblings share one rule
  identity across graph-composition and commit-boundary execution.
- Adversarial duplication audit: exact-zero second implementations on covered
  rule identities.

**Engineering decisions**
- Document relationship to `MutationSensitive` and `CommitBoundary`; covered
  topo rules migrate to graph-composition execution point for compose-time
  dispatch while retaining commit backstop.

**Open questions**
- None.

### Phase 6: Authoritative Mutation Intent Admission Integration

Integrate obligation dispatch into the canonical authoritative mutation
intent-admission family. `compose_graph` anchors on
`review_authoritative_runtime_write_batch_with_graph_artifacts` → admit →
execute, while scalar-write and effect-triggered write-intent fronts lower
through the same touch vocabulary and evidence model rather than separate
folklore lanes.

**Relevant subsystems**
- `crates/forge-query/src/runtime/runtime_batch_write_intents.rs`
- `crates/forge-query/src/runtime/runtime_batch_write_entrypoints.rs`
- `crates/forge-query/src/runtime/runtime_intents.rs`
- `crates/forge-query/src/runtime/workspace.rs`
- `crates/forge-query/src/intent_admission/plans/mutation.rs`
- `crates/forge-query/src/runtime/workspace_graph.rs`

**Relevant APIs**
- `ForgeQueryAuthoritativeMutationBatchIntentSeed`
- `review_authoritative_runtime_write_batch_with_graph_artifacts`
- `resolve_reviewed_admitted_authoritative_write_batch_handoff`
- `execute_authoritative_mutation_batch_execution_binding`
- `runtime.write`, `runtime.write_intent`, `runtime.write_batch`,
  `runtime.write_batch_intent`
- `workspace.write`, `workspace.write_intent`, `workspace.write_batch_intent`,
  `workspace.batch`
- `workspace.insert`, `workspace.update`, `workspace.update_existing`,
  `workspace.assert_existing`, `workspace.verify_existing`,
  `workspace.update_existing_verified`, `workspace.delete`,
  `workspace.delete_with`, `workspace.delete_existing`,
  `workspace.delete_existing_with`, `workspace.delete_existing_verified`
- `runtime.next_effect_write_intent`,
  `admit_next_effect_write_intent_for_execution`
- `ForgeQueryGraphObligationDispatchEnvelope`

**Target shape — before**

```rust
// workspace_graph.rs — manual pack pre-hook, then intent admission underneath
invariant_gate(&invariant_context)?;
self.runtime.write_graph_batch(commands, breadth, program)?;
```

**Target shape — after**

```rust
// obligation dispatch runs inside authoritative write-batch review/admit
// graph program + breadth already in ForgeQueryAuthoritativeMutationBatchIntentSeed
// compose_graph and batch both reach the same dispatch authority
self.runtime.write_graph_batch(commands, breadth, program)?;
// review/admit/execute materializes obligation dispatch envelope on the handoff path
```

**Test requirements**
- Adversarial rejection: loop-wiring violation blocks during write-batch review
  with typed denial + envelope — not via manual pre-hook only.
- Adversarial equivalence: scalar write, batch write, and `compose_graph`
  paths sharing touch shape produce compatible dispatch evidence through the
  authoritative mutation intent family.
- Adversarial delegation: `runtime.next_effect_write_intent(...)` cannot become
  an obligation bypass around the same touch descriptors the authoritative
  mutation family would enforce directly.

**Engineering decisions**
- Remove manual `invariant_pack` as the ordinary compose path; mechanically
  contain it as thin projection helper only.
- Scalar topo `finalize_batch_write_closeout` participates through the same
  authoritative mutation intent integration.
- Do not treat scalar write, batch write, and effect-triggered write intent as
  separate obligation authorities; they are distinct front doors over the same
  canonical family.

**Open questions**
- None.

### Phase 7: Declaration-Entry And Contribution-Orchestration Dispatch

Wire obligation dispatch at declaration-entry and contribution-composed
orchestration boundaries — where worth-topo already stops mutations before
the runner reaches `compose_graph` or `batch`.

**Relevant subsystems**
- `crates/forge-query/src/grouped_authoring/contributions.rs`
- `crates/worth-topo/src/topology_operators/application/declaration_entry/orchestration_boundary.rs`
- `crates/worth-topo/src/topology_operators/query_workflow/`

**Test requirements**
- Adversarial rejection: contribution-denied orchestration produces obligation
  envelope evidence without reaching workspace mutation.
- Adversarial equivalence: operating context on declaration entry matches
  dispatch operating-world descriptor on subsequent mutation.

**Engineering decisions**
- Preflight and advisory obligations may fire at orchestration boundary when
  touch shape is known from declaration payload; sequencing obligations order
  orchestration before workspace execution.

**Open questions**
- None.

### Phase 8: Read Execution, Read Composition, And Live Read Obligation Dispatch

Extend obligation dispatch to all read-composition product entry points:
`compose_read`, `define_read_family`, `execute_read_family`, live-read
execution, and the intent-admission front doors those surfaces already expose.

**Relevant subsystems**
- `crates/forge-query/src/runtime/workspace_queries.rs`
- `crates/forge-query/src/runtime/runtime_read_intents.rs`
- `crates/forge-query/docs/authoring/read-composition.md`
- `crates/forge-query/docs/execution/intent-admission.md`

**Relevant APIs**
- `compose_read`, `compose_read_with_invariant_pack`
- `define_read_family`, `define_read_family_with_invariant_pack`
- `execute_read_family`, `execute_read_family_in_basis_context`
- `read_family_intent`, `read_family_in_basis_context_intent`
- `read`, `read_live_intent`, `read_live_by_name`

**Test requirements**
- Adversarial parity: read-family obligation registered for a collection kind
  fires on matching read touch without manual pack closure.
- Adversarial rejection: unrelated read bundles do not invoke non-matching
  obligations.
- Adversarial equivalence: `execute_read_family`, `read_family_intent(...).execute()`,
  `read_live_intent(...).execute()`, and helper fronts such as `read(&view)`
  retain the same obligation evidence when they lower the same admitted read
  meaning.

**Engineering decisions**
- Read touch descriptors from Phase 2 drive read-side index lookup.
- `compose_read`, `execute_read_family`, `read_family_intent`, and
  `read_live_intent` are one public family story for this milestone; do not
  certify only the helper with the shortest name.

**Open questions**
- None.

### Phase 9: Preview Direct Mutation And Branch/Preview Intent Parity

Extend obligation dispatch to the real non-authoritative surfaces that exist
today: preview-local direct writes/batches and preview/branch intent execution.
Do not pretend Forge Query already has a branch batch-write API when the branch
surface is intent-first.

**Relevant subsystems**
- `crates/forge-query/src/runtime/preview/mutation_ops.rs`
- `crates/forge-query/src/runtime/preview/workflow_ops.rs`
- `crates/forge-query/src/runtime/branch.rs`
- `crates/forge-query/src/runtime/workspace.rs` (preview/branch entry)
- `crates/worth-kernel/src/construction/runtime_proof/motion/branch_runtime.rs`

**Relevant APIs**
- `ForgeQueryPreviewSession::write`, `ForgeQueryPreviewSession::batch`
- `ForgeQueryPreviewSession::execute_intent`, `ForgeQueryPreviewSession::promote`
- `ForgeQueryBranchSession::execute_intent`

**Test requirements**
- Adversarial parity: preview-local direct writes and batches carrying graph
  touch shape receive obligation dispatch evidence equivalent to authoritative
  touch selection where policy allows execution.
- Adversarial parity: preview/branch intent execution cannot bypass obligation
  dispatch for touch-bearing intent payloads routed through those lanes.
- Adversarial localization: preview lane cannot bypass obligations that
  authoritative lane would enforce for the same touch descriptor.

**Engineering decisions**
- Kernel branch-preview motion surfaces certify under this phase.
- `preview.promote()` re-enters the authoritative `runtime.write(...)` path, so
  preview-local evidence and promoted authoritative evidence must remain
  auditable as one continuous story.
- Branch parity in this milestone is about `execute_intent`, not an invented
  branch-local batch mutation API.

**Open questions**
- None.

### Phase 10: Policy-Aware Graph Mutation And Operating Context Gate Execution

Build policy-aware graph mutation execution and operating-context gate
dispatch on write authority — extending policy narrowing beyond read plans.

**Relevant subsystems**
- `crates/forge-query/src/policy_narrowing/`
- `crates/forge-query/src/policy_basis/`, `tenant_basis/`
- configured domain handle operating context surfaces

**Test requirements**
- Adversarial rejection: collaborative vs restricted operating contexts produce
  different gate verdicts on the same touch when rules differ.
- Adversarial parity: read-side policy basis and write-side gate evidence cite
  compatible basis identity.

**Engineering decisions**
- `PolicyAwareExecution` support row moves to Verified for graph mutation
  obligation dispatch in this milestone.

**Open questions**
- None.

### Phase 11: Advisory, Capability Gap, And Preflight Sequencing Executors

Wire remaining obligation kinds to Query authority surfaces: admission
contributions / intent admission lattice, capability gap and support matrix
truth, and ordered prerequisite obligations (motion witness before finish).

**Relevant subsystems**
- `crates/forge-query/docs/domain-capabilities/admission/advisory-and-violation-contributions.md`
- `crates/forge-query/docs/domain-capabilities/invariants/capability-gaps-and-invariant-denials.md`
- `crates/worth-kernel/src/construction/runtime_proof/motion/`

**Test requirements**
- Adversarial rejection: finish-before-witness sequencing blocks with typed
  preflight denial — not `unreachable!`.
- Adversarial equivalence: advisory obligations produce `Advise` verdicts in
  envelope.

**Open questions**
- None.

### Phase 12: Envelope Attachment To Receipts, Results, Decision Traces, And Mutation Evidence

Wire obligation dispatch envelopes onto write receipts, read results, live-read
results, intent decision traces, execution handoffs, and downstream mutation
evidence surfaces.

**Relevant subsystems**
- `crates/forge-query/src/intent_admission/trace/envelope.rs`
- `crates/forge-query/src/runtime/surface/` (batch write receipt)
- `crates/forge-query/src/runtime/runtime_read_intents.rs`
- `crates/worth-topo/src/topology_operators/application/declared_mutation_artifact/`

**Test requirements**
- Adversarial equivalence: receipt inspection and intent trace reproduce the
  same dispatch envelope digest for one mutation.
- Adversarial equivalence: read-result and live-read-result evidence retain the
  same obligation envelope digest emitted at the covered read execution seam.
- Adversarial completeness: topo `TopologyDeclaredMutationArtifact` exposes
  obligation evidence without host-local recomputation.

**Engineering decisions**
- `TopologyMutationApplicationEvidence` extends to carry obligation dispatch
  summary alongside existing verified-operation counts.

**Open questions**
- None.

### Phase 13: Derived Read Validation Re-Homed

Re-home materialized-view and interpreted-topology validators as derived read
diagnostics from registered rule identity on covered mutation paths.

**Relevant subsystems**
- `crates/worth-topo/src/validation/`
- `crates/worth-topo/src/projection/runtime_boundary/read_stage.rs`

**Test requirements**
- Adversarial parity: derived read reports agree with compose-time envelope on
  same state.
- Adversarial duplication audit: exact-zero second implementations on covered paths.

**Open questions**
- None.

### Phase 14: Consumer Obligation Bypass Audit

Ship mechanical enforcement against obligation folklore using Milestone `9.8`
audit machinery extended for legality duplication patterns.

**Relevant subsystems**
- Milestone `9.8` prohibition registry and audit artifact
- `crates/worth-topo/src/topology_operators/`
- `crates/worth-kernel/src/construction/phase_chain/`

**Test requirements**
- Seeded manual guards, legality graphs, and ordinary-path invariant-pack usage
  fail audit with zero false positives on literals/comments.

**Open questions**
- None.

### Phase 15: Kernel Construction Platform Operating Context Wiring

Wire worth-kernel construction through configured domain handles and
operating context — matching binding's platform entry pattern. worth-topo
operator declaration entry already carries operating context; this phase
targets kernel construction's naked `ForgeQueryWorkspace` authoring session.

**Relevant subsystems**
- `crates/worth-kernel/src/construction/authoring.rs`
- `crates/worth-kernel/src/binding/authoring/query_domain.rs` (reference)
- `crates/forge-query/docs/domain-capabilities/configured-domain-handles.md`

**Test requirements**
- Adversarial equivalence: construction operating context yields stable
  `operating_context_identity_digest` on obligation dispatch.
- Adversarial rejection: covered construction mutation without operating context
  fails platform entry contract or audit.

**Open questions**
- None.

### Phase 16: Primitive Construction Birth Compose Execution

Build and execute primitive construction birth `compose_graph` programs from
admitted handoffs — closing the gap where `ComposeGraph` is declared but never
executed.

**Relevant subsystems**
- `crates/worth-topo/src/construction/query_native_boundary/`
- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/`
- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/` (7 families)

**Relevant APIs**
- `prepare_primitive_construction_query_admitted_handoff_from_synopsis`
- `TopologyConstructionQueryMutationSurface::ComposeGraph`
- birth compose program authoring per family

**Test requirements**
- Adversarial rejection: shell-with-hole layout violation blocks at compose
  with typed obligation denial — not offline string-mapped `InvalidRequest`.
- Adversarial equivalence: handoff synopsis → compose program → obligation
  dispatch envelope → committed topology state for representative families.

**Engineering decisions**
- worth-topo construction lane executes compose; kernel consumes obligation
  denials from runtime execution instead of pre-checking offline only.
- All seven birth families in `family_birth_input/families/` reach compose
  execution or explicit certification exclusion with residue audit — no silent
  skip.

**Open questions**
- None.

### Phase 17: Reference Adoption — worth-topo Operator Catalog

Migrate worth-topo topology operators: all `milestone_one_invariant_registrations()`
with touch selectors, all `compose_graph` and covered `batch` paths, delete
parallel enforcement.

**Relevant subsystems**
- `topology_operators/local_rewrites/`
- `validation/reference_integrity/`
- `runtime_support.rs`

**Adoption targets**
- `RewireLoopSuccessor`, wire rehome, shell membership, face inner loop, scalar
  batch operators
- Delete manual `ExistingEntityIncomingRelationCountMismatch` guards
- Delete compose-bypass loop-wiring folklore and duplicate validator implementations

**Test requirements**
- Hostile operator matrices with envelope inspectability.
- Adoption manifest residue: exact-zero on listed files/patterns.

**Open questions**
- None.

### Phase 18: Reference Adoption — worth-kernel Construction Surfaces

Migrate worth-kernel construction: full phase-chain legality through registered
obligations at compose boundary, motion sequencing through preflight obligations,
delete parallel offline enforcement on covered paths.

**Relevant subsystems**
- `construction/phase_chain/admitted_scaffold/`
- `construction/runtime_proof/motion/branch_runtime.rs`
- `construction/result_surface/result.rs`

**Adoption targets**
- Delete host-local layout legality on covered shell-with-hole and family paths
- Delete motion dual-pass `unreachable!` sequencing folklore
- Verify binding workflows remain clean; add operating-context gate obligations
  where binding kinds require world-sensitive checks

**Test requirements**
- Construction hostile matrices pass with typed obligation denials.
- Adoption manifest residue on covered kernel files.

**Open questions**
- None.

### Phase 19: Public Docs, Support Rows, And AI_README Category

Ship graph touch obligation authority as first-class product docs; re-home
graph-composition-authoring.md away from manual invariant-pack as primary path;
add AI_README category and update the existing docs that already define these
front doors so the milestone does not silently under-document real surfaces.

**Relevant subsystems**
- `crates/forge-query/docs/AI_README.md`
- `crates/forge-query/docs/authoring/graph-composition-authoring.md`
- `crates/forge-query/docs/execution/intent-admission.md`
- `crates/forge-query/docs/execution/writes-and-intents.md`
- `crates/forge-query/docs/foundations/branches-and-previews.md`
- `crates/forge-query/docs/domain-capabilities/configured-domain-handles.md`
- `crates/forge-query/docs/domain-capabilities/declaration-entry-orchestration.md`
- new: `crates/forge-query/docs/authoring/graph-touch-obligation-authority.md`

**Test requirements**
- Adversarial agreement: docs, support rows, and certification name the same
  obligation kinds and covered lanes.

**Open questions**
- None.

### Phase 20: Architectural Certification Closeout

Property-based hostile matrices, complexity proofs, bypass audit closure,
full kind × lane × touch certification matrix, adoption residue.

**Relevant subsystems**
- `crates/forge-query/tests/`
- `_docs/forge-query/test-requirements.md`
- `crates/worth-topo`, `crates/worth-kernel` certification tests

**Test requirements**
- Add `Milestone 9.9 Graph Touch Obligation Authority Hostile Certification
  Matrix` to [test-requirements.md](./test-requirements.md).
- Property tests, false-fire/false-miss matrices, replay equivalence, every
  obligation kind × representative touch × covered lane, adoption residue.

**Open questions**
- None.

## Must Ship

- complete obligation authority model with multi-obligation envelopes
- graph touch descriptors for mutation and read lanes
- registration, auto-indexing, assembly index with complexity contracts
- relational graph-composition execution point and rule migration
- authoritative mutation intent admission integration for batch, scalar, and
  effect-triggered write fronts
- declaration-entry and contribution-orchestration dispatch
- read execution, read composition, and live-read dispatch across existing
  helper and intent front doors
- preview direct mutation and branch/preview intent parity
- policy-aware graph mutation and operating-context gate execution
- advisory, capability-gap, and preflight-sequencing executors
- envelope attachment to receipts, decision traces, and mutation evidence
- derived read validation re-homed
- consumer obligation bypass audit
- kernel construction operating context wiring
- primitive construction birth compose execution (all covered families)
- full worth-topo operator catalog adoption
- full worth-kernel construction surface adoption
- public docs and AI_README category
- architectural certification matrix closure

## Must Preserve

- relational invariant execution authority
- typed graph-composition domain-invariant denials on block paths
- declaration legality and support admission as upstream lanes obligations consume
- reference-consumer semantics through migration

## Acceptance Evidence

- property-test certification of pure-function dispatch on every authoritative
  mutation lane plus covered read/live/preview-intent lanes
- authoritative mutation intent admission carries obligation dispatch for
  batch/scalar/effect fronts — manual invariant-pack pre-hook eliminated on
  covered compose paths
- primitive construction birth executes compose_graph with obligation routing
  for covered families
- every obligation kind executes in certification matrix across representative
  touches and lanes
- policy-aware mutation gates and preflight sequencing certified
- exact-zero duplicate implementations and manual pre-check residue on adoption
  manifests
- full milestone-one topo registrations indexed; kernel phase-chain covered
  surfaces migrated
- envelopes inspectable on receipts and intent decision traces
- bypass audit and certification matrix pass

## Sequencing Notes

- After Milestone `9.8` (consumer backend + bypass audit machinery).
- Before Milestone `10` (store-backed execution inherits complete obligation
  authority).
- Phases 1–5: vocabulary, index, relational execution point.
- Phase 6: canonical dispatch seam (authoritative mutation intent admission)
  before surface-specific wiring (7–9).
- Phases 10–12: remaining executors and envelope attachment.
- Phases 13–14: re-homing and bypass audit.
- Phase 15: kernel operating context before birth compose (16).
- Phases 17–18: adoption after execution surfaces exist (16) and platform
  context (15).
- Phases 19–20: docs then certification close strictly last.
