# Milestone 3.8.1: Runtime Authority Closure and Honest Composition Gate

**Status:** Planned. Blocking prerequisite for Milestone 3.9.

## Goal

Close the remaining composition seams in the shipped 3.8 substrate so one
prepared application authority, one active runtime generation, one inspection
truth, and one admitted host session cannot drift apart.

## Why This Milestone Exists

Milestone 3.8 shipped real declaration, graph, Query-binding, allocation, and
runtime machinery, but a code-level end-to-end audit found five structural
gaps: candidate and graph truth can be separated, configured host posture is
not operationally enforced, public freeze converts admission denial into
panic, the ordinary framework-turn owner contains too much policy execution,
and dormant GPU/theme files exist outside the compiled module graph.

Milestone 3.9 would make these seams more expensive by lowering additional
execution authority onto them. This milestone therefore closes them before
execution-plan work continues.

## Governing Summaries

- `MENTALITY.md` protects foundation-first, adversarial, mechanically enforced
  correctness. The strongest effect here is that split application truth must
  become unrepresentable before dependent features continue.
- `arch_laws.md` protects contractual facades, proof-bearing phase progression,
  typed denials, and pre-resolved execution. Launch, freeze, host admission, and
  framework-turn transitions must consume types carrying the exact proof each
  preceding phase established.
- `composition_laws.md` protects named semantic steps and reviewable
  responsibilities. The framework-turn owner may orchestrate broadly, but it
  may not inline classification, selection, mutation, receipt construction,
  and completion mapping as one private runtime.
- `domain_structure_laws.md` protects physical boundaries that preserve
  authority, truth source, lifecycle, and failure meaning. Candidate
  preparation, application authority, host session, transition planning,
  transition execution, and visual-host mechanisms require distinct,
  discoverable homes.
- `perf_laws.md` protects delta-bounded execution and policy resolution before
  the hot path. Framework-turn policy must lower once into typed dispositions,
  and execution evidence must expose the breadth actually touched.
- The Worth UI roadmap protects one canonical artifact pipeline,
  runtime-owned meaning, host adapters as native-mechanics translators, and
  steady-state execution free of semantic rediscovery. It requires this closure
  gate immediately after 3.8 and before 3.9.

## Adversarial Constraint

Under arbitrary valid and invalid source replacements, arbitrary framework-turn
source pressure, and arbitrary attempts to mix otherwise-valid application
parts, it must be mechanically impossible for the active artifact, declaration
authority, committed graph, Query binding, inspection surfaces, capability
snapshot, or host session to describe different application generations.

Every denial must preserve the last complete admitted authority without panic,
partial publication, adapter substitution, or derived-state residue.

## Product Decision Lock

- One sealed, move-only prepared application authority is the only ordinary
  input to runtime launch and replacement staging.
- A digest may identify or compare that authority, but a raw digest is never
  authority and cannot open a launch, replacement, host, or inspection door.
- File-authored candidate truth and its source-backed declaration/graph truth
  remain inseparable through preparation and cutover.
- Active inspection always describes the same committed generation the runtime
  executes. Candidate inspection is explicitly candidate-scoped and cannot
  masquerade as active truth.
- Host selection is a real lifecycle decision. Per-turn measurement access is
  borrowed from the admitted host session; callers cannot substitute an
  unrelated adapter at the source boundary.
- Publicly constructible invalid input returns typed denial. Panic is reserved
  for unreachable internal invariant corruption, not ordinary declaration or
  topology rejection.
- The framework-turn owner remains the single clock and close/pump authority,
  while policy classification and policy-family execution move into named,
  typed transitions.
- Dormant rendering code is not preserved as speculative inventory. Uncompiled
  GPU/theme implementations are deleted; future rendering work must re-enter
  through the roadmap-owned host and appearance milestones.
- No compatibility facade, deprecated split lane, optional legacy constructor,
  or feature-gated bypass may preserve the old composition model.
- Milestone 3.9 remains blocked until every phase and acceptance gate here is
  closed.

## Phase Plan

### Phase 1: Canonical Prepared-Application Authority

Define the single authority object from which an active Worth UI application
may be launched. This phase freezes ownership and identity before changing
ingress, freeze, or runtime behavior.

**Relevant subsystems**

- Worth UI public facade and application lifecycle
- capability snapshot authority
- canonical artifact and declaration authority
- committed graph authority
- Query binding plan
- host contract/session plan
- inspection authority

**Relevant APIs**

- `WorthUiApp`
- `WorthUiRuntimeLaunch`
- `WorthUiSourceBackedDslPackage`
- `CapabilitySnapshot`
- `UiGraphSnapshot`
- `WorthUiQueryBindingPlan`
- `WorthUiHostContract`
- new sealed prepared-application authority and identity types

**Required design**

- Introduce one sealed prepared-application authority that owns the canonical
  artifact, admitted declaration artifacts, committed graph snapshot,
  capability snapshot, Query binding plan, host-session plan, and inspection
  indexes for exactly one application generation.
- Give the authority a typed generation identity derived during preparation.
  Expose comparison-safe identity only; do not expose constructors that accept
  raw component digests.
- Make the authority move-only. Components may expose read-only projections,
  but callers cannot extract independently launchable artifact, graph, Query,
  or host parts.
- Separate candidate-scoped projections from active projections in the type
  system. A candidate identity must never satisfy an active-authority API.
- Keep derived indexes rebuildable from the authority-owned canonical inputs.

**Warnings**

- Do not solve shared observation by making the entire authority freely
  cloneable. Observation handles must borrow or carry a generation-scoped
  witness without becoming a second owner of truth.
- Do not use an aggregate digest as the only enforcement. The sealed authority
  type, private fields, and constructor visibility are the enforcement; its
  identity is evidence and comparison material.
- Do not place the new object in a generic `state`, `bundle`, or `context`
  module. Its physical home must name prepared application authority.

**Test requirements**

- Compile-fail: external callers cannot construct prepared authority, mint its
  identity, replace one constituent part, or promote candidate identity into
  active identity.
- Adversarial drift: two preparations sharing a capability snapshot but
  differing in declarations, graph, Query binding, or host plan produce
  distinct authority identities and cannot exchange launch witnesses.
- Rebuild honesty: destroying every derived inspection/index projection and
  rebuilding it from prepared authority yields equivalent projections without
  consulting runtime-local residue.

**Engineering decisions**

- The prepared application authority is the milestone's canonical application
  artifact; all launch and replacement surfaces narrow around it.
- Public inspection may borrow generation-scoped projections, but it does not
  own or reconstruct active authority.

**Open questions**

- Final public type names may follow the existing facade vocabulary, but the
  prepared-versus-active distinction and sealed ownership are not negotiable.

### Phase 2: Inseparable Candidate Composition at Source Ingress

Make source ingress produce one composition input whose runtime artifact and
declaration/graph source cannot be separated or silently discarded.

**Relevant subsystems**

- watched source ingress
- file-authored lowering
- Rust-authored composition
- canonical artifact assembly
- source-backed declaration witness
- replacement candidate admission

**Relevant APIs**

- `WorthUiWatchedCandidateSubmission`
- `WorthUiReplacementCandidate`
- `WorthUiSourceBackedDslPackage`
- `lower_to_candidate_submission`
- `into_candidate`
- file/Rust replacement parity surfaces

**Required design**

- Replace the split `candidate` plus optional source-backed package posture
  with one sealed candidate-composition type whose variants carry every input
  required to prepare coherent application authority.
- Remove `into_candidate` and any equivalent accessor that can consume or copy
  the runtime candidate while dropping declaration/graph material.
- File-authored and Rust-authored paths must both name their declaration source
  explicitly and converge before application preparation. An `Option` must not
  hide a missing semantic lane.
- Preserve source revision, ordering receipt, provenance, and ingress counters
  on the inseparable composition object.
- Make candidate artifact identity and declaration-source identity part of one
  typed preparation basis, while preserving their distinct semantic types.

**Warnings**

- Do not repair only the currently observed reload-storm caller. The old split
  must become unavailable everywhere, including certification and support
  fixtures.
- Do not fabricate source-backed declarations from a finished canonical
  artifact after the boundary. Both must descend from the admitted structured
  input that actually established their equivalence.
- Do not preserve a certification-only constructor that can recreate the
  production bypass under another feature flag.

**Test requirements**

- File/Rust convergence: semantically equivalent file-authored and
  Rust-authored inputs produce equivalent candidate-composition authority and
  prepare equivalent application generations.
- Compile-fail: callers cannot extract an independently launchable replacement
  candidate or discard the declaration source from a watched submission.
- Adversarial mismatch: an artifact from source revision A and declaration
  material from revision B cannot be represented as one admitted composition,
  even when capability snapshots match.
- Reload-storm preservation: rapid valid and invalid ingress sequences retain
  revision/order evidence without ever publishing artifact-only candidate
  truth.

**Engineering decisions**

- Candidate composition remains pre-authority input. It cannot answer active
  inspection or runtime APIs until preparation succeeds.
- Production and certification consume the same sealed ingress object; test
  support may originate inputs but cannot mint the admitted result.

**Open questions**

- None.

### Phase 3: Fallible Application Preparation and Typed Freeze Denials

Replace panic-based freeze with a typed preparation pipeline that denies before
publishing any application authority.

**Relevant subsystems**

- `WorthUiBuilder`
- capability registration freeze
- declaration lowering
- graph-handoff lowering and admission
- graph generation commit
- Query binding preparation
- host-session planning
- inspection-index assembly

**Relevant APIs**

- `WorthUiBuilder::freeze`
- `WorthUiCapabilityRegistrationFreezeCore::freeze_from_registration`
- `lower_graph_handoffs`
- `admit_graph_handoffs`
- `commit_initial_generation`
- new application-preparation denial taxonomy

**Required design**

- Make the public freeze/preparation surface return a typed result whose success
  contains prepared application authority and whose denial identifies the
  exact failed phase and structured local evidence.
- Replace every `expect` reachable from publicly constructible declaration,
  topology, Query, capability, or host input with explicit typed propagation.
- Order preparation so cheap eligibility and structural denials precede graph
  commit, derived-index construction, Query activation, host activation, or
  other expensive work.
- Commit the graph and seal prepared authority only after all prerequisite
  admissions succeed. No denial path may publish a partial graph generation or
  retain partially activated Query/host state.
- Keep internal invariant-corruption panics distinct from ordinary input
  denial, and make that distinction visible in module ownership and tests.

**Warnings**

- Do not flatten graph local denials into a generic freeze failure or string.
- Do not catch panics and translate them after mutation; ordinary invalid input
  must never panic in the first place.
- Do not keep an infallible convenience `freeze` beside the fallible ordinary
  API. There is one production preparation lane.

**Test requirements**

- Adversarial topology denial: duplicate or missing product-root topology,
  contradictory graph basis, and illegal structural handoffs return the
  correct typed preparation denial without unwinding.
- Residue rejection: failure at each preparation phase leaves no committed
  graph generation, activated Query binding, host session, inspection index,
  or launchable witness.
- Deterministic denial: replaying identical invalid input produces equivalent
  phase-local denial evidence and never changes the previously active
  application.
- Compile-fail: callers cannot bypass fallible preparation to construct
  prepared authority directly from a capability snapshot or graph snapshot.

**Engineering decisions**

- Registration reporting may remain separately inspectable, but successful
  registration alone is not prepared application authority.
- The denial topology mirrors the actual preparation phases rather than
  preserving old facade convenience.

**Open questions**

- None.

### Phase 4: Active Application Session and Launch Admission

Consume prepared authority into one active application session that owns both
runtime execution and active inspection for the same generation.

**Relevant subsystems**

- Worth UI application facade
- runtime launch and active state
- active inspection bridge
- retained allocation evidence
- Query binding activation
- generation identity and last-valid runtime state

**Relevant APIs**

- `WorthUiApp::launch_runtime`
- `WorthUiRuntimeLaunch`
- `WorthUiRuntime::launch`
- `WorthUiActiveArtifact`
- `WorthUiRuntimeQueryBinding`
- `WorthUiLastValidRuntimeState`
- new active-application session facade

**Required design**

- Replace independent `&app -> runtime` launch with a consuming transition from
  prepared application authority to one active application session.
- Make the active session the only ordinary owner of runtime execution,
  active-generation inspection, retained allocation evidence, Query runtime
  binding, and host session.
- Require a sealed launch admission witness produced from the entire prepared
  authority. Capability or artifact digests alone cannot satisfy launch.
- Remove ordinary runtime constructors that accept a canonical artifact or
  replacement candidate without the matching prepared authority.
- Make active inspection receipts carry the active generation identity so
  consumers can prove they describe the runtime being executed.
- Preserve last-valid state inside the active session, not as a substitute
  authority callers can independently activate.

**Warnings**

- Do not solve the ownership problem by cloning app truth into runtime truth.
- Do not leave `WorthUiRuntimeLaunch::from_candidate` public if it remains a
  path around prepared application authority.
- Certification-only launch construction must stay support-authority scoped and
  must not be type-compatible with the production launch witness.

**Test requirements**

- Adversarial cross-launch: a candidate prepared against application A cannot
  launch or replace application B, even when both share the same capability
  snapshot and artifact digest collision is simulated at the comparison seam.
- Active inspection parity: every active inspection receipt and runtime frame
  outcome reports the same generation identity through launch and ordinary
  frame execution.
- Compile-fail: a prepared app cannot be launched twice, inspected as active
  before launch, or split into independently owned runtime and inspection
  objects.
- Query-free/installed parity: both postures use the same active-session
  lifecycle without imposing Query ceremony on Query-free applications.

**Engineering decisions**

- Prepared and active application sessions are distinct lifecycle types.
- Runtime internals remain private; the active application facade exposes only
  admitted execution, source, inspection, and replacement capabilities.

**Open questions**

- None.

### Phase 5: Atomic Replacement and Inspection Continuity

Make replacement cutover publish artifact, declarations, graph, Query binding,
derived indexes, and active inspection as one generation transition.

**Relevant subsystems**

- replacement candidate admission
- impact narrowing and reconciliation
- activation staging and gate
- declaration/graph preparation
- active inspection authority
- last-valid preservation
- reload-storm certification

**Relevant APIs**

- `WorthUiAdmittedReplacementCandidate`
- activation staging bundle and gate
- replacement impact/narrowing surfaces
- durable-state reconciliation plan
- reload-storm scenario
- new prepared-replacement and application-cutover artifacts

**Required design**

- Lower admitted candidate composition into a sealed prepared replacement that
  carries the candidate generation's artifact, declarations, committed graph,
  Query rebind outcome, derived inspection/index projections, and cutover
  evidence.
- Keep candidate inspection explicitly candidate-scoped until cutover. Active
  inspection continues to describe the prior generation while staging occurs.
- Publish the prepared replacement through one atomic cutover owned by the
  active application session. No constituent authority may become visible
  earlier than another.
- On denial, discard all candidate-derived authority and preserve the prior
  active generation, host session, runtime state, and inspection truth.
- Reconciliation and impact narrowing consume the prepared replacement basis;
  they must not independently reconstruct graph or declaration equivalence.

**Warnings**

- Do not mutate the active graph and then attempt to roll back from candidate
  source if a later Query or activation phase denies.
- Do not allow inspection generation to lead or lag runtime generation during
  cutover.
- Do not use shared raw IDs to bridge old and candidate generations. Identity
  correspondence must remain an admitted reconciliation artifact.

**Test requirements**

- Hostile cutover sequence: thousands of alternating valid, invalid, no-op,
  and structurally different replacements never expose mixed generation
  identities across runtime, graph lookup, Query binding, or inspection.
- Failure localization: denial injected at every staging/cutover phase leaves
  the prior generation byte-for-byte equivalent at its authoritative
  projections and leaves no candidate residue.
- No-op convergence: equivalent replacements retain the active generation and
  produce typed no-op evidence rather than republishing derived state.
- Observer boundary: candidate inspection can explain a denied candidate while
  active inspection continues to explain only the still-running generation.

**Engineering decisions**

- Replacement is application-authority replacement, not artifact-only swap.
- Candidate diagnostics are derived evidence and cannot affect cutover
  eligibility or active operational truth.

**Open questions**

- None.

### Phase 6: Operational Host-Session Authority

Turn configured host posture into the sole admitted source of native
observation and measurement capabilities for an active application.

**Relevant subsystems**

- `worth-ui-host-contract`
- `worth-ui-host-egui`
- Worth UI builder host configuration
- runtime host observation and measurement collection
- framework-turn source capabilities
- host capability reports and generations

**Relevant APIs**

- `WorthUiHostAdapter`
- `WorthUiMeasurementHostAdapter`
- `WorthUiHostContract`
- `WorthUiBuilder::with_host`
- `WorthUiHostMeasurementTurnSource::collect_and_submit`
- host measurement collector and evidence boundary
- new sealed host-session authority and measurement capability

**Required design**

- Prepare and activate one host session from the configured adapter/contract,
  host kind, capability report, and observation generation.
- Make the active application session own host-session authority. Framework
  turns borrow a generation-scoped measurement/observation capability from
  that session rather than accepting arbitrary adapters.
- Bind every host-produced observation and measurement receipt to host-session
  identity and observation generation before it enters allocation admission.
- Deny missing, stale, foreign, or capability-incompatible host evidence before
  measurement construction or frame submission.
- Keep native mechanics inside host adapters and semantic admission inside
  Worth UI runtime boundaries.
- Remove the unused retained `_host_contract` posture and any alternate path
  where host configuration is stored but not consumed.

**Warnings**

- Do not turn `WorthUiHostContract` into a semantic layout authority. It grants
  access to native observations; declarations and runtime plans still decide UI
  meaning.
- Do not compare only `WorthUiHostKind`. Session identity, capability posture,
  and observation generation are distinct facts.
- Do not require Query-free or headless applications to instantiate egui
  machinery. Host-session variants must preserve their actual capability
  boundaries.

**Test requirements**

- Adapter substitution denial: an adapter not admitted into the active host
  session cannot submit measurements, even when it implements the same public
  trait and reports the same host kind.
- Host-generation freshness: stale observations from the previous host
  capability generation deny before allocation ingress and cannot advance
  frame source order.
- Headless/egui contract parity: both hosts use the same session lifecycle and
  receipt authority while producing only capabilities they genuinely support.
- Compile-fail: external callers cannot mint host-session authority or obtain a
  turn-source measurement capability without borrowing the active session.

**Engineering decisions**

- `with_host` remains only if it performs this real lifecycle role; decorative
  configuration is forbidden.
- Host replacement, if supported, is an explicit application transition with
  typed invalidation. Ambient adapter swapping is not supported.

**Open questions**

- Whether host replacement ships now depends on existing product callers. If
  retained, it must use the same atomic application-authority transition; it
  may not weaken the ordinary host session.

### Phase 7: Framework-Turn Transition Planning

Extract policy classification, narrowing, and selection from the close/pump
owner into a proof-bearing transition plan that is complete before mutation.

**Relevant subsystems**

- allocation frame scheduler and dispatcher
- stream policy resolution
- invalidation narrowing
- viewport and resize policy classification
- allocation-neighborhood selection
- allocation receipt planning
- framework-turn counters

**Relevant APIs**

- `WorthUiRuntime::execute_framework_turn`
- `close_allocation_ingress_at_framework_boundary`
- `UiAllocationFrameTurnOutcome`
- `UiAllocationFrameConsumptionDisposition`
- `UiAllocationInvalidationNarrowingDisposition`
- `UiResolvedAllocationCommitPlan`
- `select_replan_neighborhoods`
- new proof-bearing framework-transition plan family

**Required design**

- Preserve one owner for framework-turn clocking, ingress closure, and exactly
  one dispatcher pump.
- Move stream consumption, invalidation narrowing, policy-family
  classification, and neighborhood selection into named planning transitions
  that borrow immutable authority and produce a sealed plan or typed denial.
- Define exhaustive plan variants for no-ingress execution, ordinary
  allocation, viewport resize, resize preview, durable resize, drag-resize,
  and every existing backpressure or denial posture.
- Carry every proof execution needs: sealed frame identity, source order,
  narrowed invalidations, selected neighborhoods, resize identity/extent,
  expected receipt basis, and active application generation.
- Attach structural counters for admitted ingress width, invalidation breadth,
  selected neighborhood breadth, and policy classification.

**Warnings**

- Do not merely move the existing nested match into a differently named file.
  Planning must become a real phase with a sealed output type and no commit
  authority.
- Do not unify policy variants whose costs, failure modes, or correctness
  requirements differ.
- Do not let the executor rediscover neighborhood selection, durable extent,
  or policy family from raw frame facts.

**Test requirements**

- Replay convergence: the same sealed frame and active authority produce an
  equivalent transition plan and exact counters regardless of diagnostic
  richness.
- Mutation isolation: every planning denial leaves receipt ledgers,
  invalidation authority, durable resize state, and active frame generation
  unchanged.
- Exhaustive family matrix: each admitted source combination selects exactly
  one plan variant; ambiguous or contradictory combinations deny before
  execution.
- Breadth certification: local invalidations produce exact neighborhood and
  counter breadth without unrelated graph scans.

**Engineering decisions**

- Planning owns policy choice; execution owns only the mutation named by the
  chosen plan.
- Existing correct classifier and selector types should be retained where they
  already carry the required proof, but raw or partial outputs may not cross
  into execution.

**Open questions**

- None.

### Phase 8: Policy-Family Execution and Thin Framework Ownership

Split allocation mutation into policy-family executors and reduce the
framework-turn owner to lifecycle orchestration visible as a short semantic
sequence.

**Relevant subsystems**

- framework-turn owner and completion facade
- ordinary allocation transaction
- viewport resize commit
- resize preview publication
- durable resize commit
- drag-resize preview/terminal commit
- allocation receipt ledger and invalidation authority

**Relevant APIs**

- `close_allocation_ingress_at_framework_boundary`
- allocation transaction commit surfaces
- `UiViewportResizeOutcome`
- `UiResizePreviewOutcome`
- `UiDurableResizeCommitOutcome`
- `WorthUiFrameworkTurnCompletion`
- new policy-family execution transitions

**Required design**

- Give each policy family one named execution responsibility with a typed plan
  input, typed commit/denial output, and the minimum mutable authority it needs.
- Keep receipt publication and authority mutation transactionally aligned for
  every family. A denial cannot publish a receipt or partially mutate durable
  state.
- Make the framework-turn owner read as: collect sources, close ingress, plan
  transition, execute transition, publish completion. It must not inline
  family-specific selection or commit mechanics.
- Keep backpressure, panic-safe ingress closeout, and exactly-once pump behavior
  at the owner boundary.
- Place execution modules by policy responsibility, not under generic
  `helpers`, `handlers`, `operations`, or milestone-named folders.

**Warnings**

- Do not introduce trait abstraction merely because executors share method
  shape. Share lifecycle only where authority, cost, and failure topology are
  actually equivalent.
- Do not widen mutable borrows from family executors back to the whole runtime.
- Do not preserve the old owner body as a fallback or certification reference.

**Test requirements**

- Family parity: ordinary, viewport, preview, durable, and drag-resize scenarios
  retain their admitted outcomes, receipts, and denials after decomposition.
- Transactional hostility: injected denial at every family mutation boundary
  produces no partial ledger entry, invalidation update, durable-state update,
  or completion claiming success.
- Exactly-once lifecycle: source callback success, source callback panic,
  downstream backpressure, and no-ingress turns each close/pump exactly once
  where permitted and never double-publish completion.
- Borrow-boundary enforcement: structural/compile checks prevent a family
  executor from receiving unrelated runtime subsystems or independently
  clocking the dispatcher.

**Engineering decisions**

- Single framework authority does not require single-function implementation.
- Completion mapping is presentation over typed transition outcomes and cannot
  decide policy or mutate allocation truth.

**Open questions**

- None.

### Phase 9: Dormant Theme and Rendering Residue Removal

Remove source files that claim GPU/theme functionality but are not part of the
compiled crate graph, and add mechanical detection so orphan production Rust
cannot recur.

**Relevant subsystems**

- `worth-ui-theme`
- `worth-ui-components`
- `worth-ui-host-egui`
- workspace source-reachability enforcement
- theme/component documentation and facade claims

**Relevant APIs and source surfaces**

- `worth-ui-theme/src/lib.rs`
- `worth-ui-theme/src/backdrop.rs`
- `worth-ui-theme/src/metal.rs`
- `worth-ui-theme/src/shader_pipeline.rs`
- `worth-ui-theme/Cargo.toml`
- workspace structural certification scripts/tests

**Required design**

- Delete the unreachable backdrop, metal-paint, and shader-pipeline files and
  any dead references, claims, fixtures, or manifest residue associated with
  them.
- Do not activate their `wgpu`/`egui-wgpu` implementation in this milestone.
  GPU-backed mounted rendering belongs after the roadmap's mounted-host and
  appearance authority are available.
- Audit theme and component public documentation for claims stronger than the
  compiled implementation and correct those claims without adding historical
  legacy narrative.
- Add a workspace check that fails when a production Rust source file under a
  crate's `src` tree is absent from every compiled module graph, except for a
  narrowly named and reviewed generated-source exemption.
- Ensure the check understands ordinary `mod`, explicit `#[path]`, feature
  combinations, and platform-gated production modules so it does not reward
  false deletion or false reachability.

**Warnings**

- Do not keep the files as examples, future reference, or an uncompiled
  feature. Version control is the historical store.
- Do not make every optional target compile in the ordinary binary; the check
  must reason over declared supported feature/platform graphs.
- Do not broaden this into Milestone 3.16 appearance semantics or Milestone 10
  real-time rendering product work.

**Test requirements**

- Structural rejection: an intentionally orphaned production `.rs` fixture
  causes the source-reachability gate to fail with the exact crate and path.
- Feature/platform reachability: legitimate feature-gated and `#[path]`
  modules are recognized when their declared compilation graph is included.
- Residue audit: no backdrop, metal, shader-pipeline, `wgpu`, or `egui-wgpu`
  production reference remains in the current theme/component crates after
  deletion.
- Theme honesty: every documented public theme capability is reachable through
  the compiled facade and exercised by at least one focused test.

**Engineering decisions**

- The current semantic-token/theme surface remains; only uncompiled claimed
  functionality is removed.
- Future GPU rendering must be reintroduced from the correct host/appearance
  authority, not resurrected by copying these files back.

**Open questions**

- None.

### Phase 10: Hostile End-to-End Closure and Anti-Bypass Certification

Certify that the repaired workspace has one application authority from ingress
through active execution, inspection, host observation, replacement, and
framework-turn closeout.

**Relevant subsystems**

- `worth-ui-certification`
- public facade compile-fail suites
- source ingress and reload-storm scenarios
- active inspection harness
- host adapter certification
- allocation framework-turn scenarios
- workspace boundary and source-reachability gates

**Relevant APIs and evidence**

- prepared and active application facades
- candidate composition and replacement cutover receipts
- application/host generation identities
- typed preparation, launch, replacement, host, and frame denials
- allocation breadth counters
- boundary-check, agent-context, line-cap, lint, and reachability gates

**Required design**

- Add one production-path certification scenario that starts from both
  file-authored and Rust-authored composition, prepares an application,
  activates a real host session, executes representative framework turns,
  performs valid/no-op/invalid replacements, and inspects the active result.
- Assert one generation identity across runtime execution, declaration lookup,
  graph lookup, Query binding, host evidence, allocation receipts, and active
  inspection at every published boundary.
- Add anti-bypass compile-fail and structural checks for every removed split
  constructor, raw-digest promotion, arbitrary turn adapter, infallible freeze,
  independent runtime launch, and dormant source lane.
- Require clean workspace warnings, strict Clippy, dead-code posture, Rust file
  line caps, boundary check, agent-context check, and documentation truth before
  closure.
- Update public docs and `AI_README.md` discovery surfaces to teach only the
  final ordinary path and current authority vocabulary.

**Warnings**

- Certification must consume production facades. Support authority may provide
  source/host inputs and fault injection, but it cannot mint prepared, active,
  host-session, or cutover authority.
- A green behavioral suite is not sufficient if the old split APIs still
  compile or orphan production files remain invisible to the compiler.
- Do not retain historical migration instructions in discovery documentation.

**Test requirements**

- Mixed-generation attack matrix: systematically attempt every artifact,
  declaration, graph, Query, host, inspection, and launch cross-product and
  prove all foreign combinations are unrepresentable or deny before mutation.
- End-to-end convergence: equivalent file/Rust composition and replayed source
  sequences converge to the same active authority, transition outcomes,
  receipts, inspection evidence, and exact structural counters.
- Failure preservation: fault injection at every preparation, launch,
  replacement, host-observation, planning, and execution boundary preserves the
  last complete active authority without panic or residue.
- Anti-bypass topology: production source and public facade scans prove there
  is exactly one preparation lane, one launch lane, one host-session lane, one
  replacement cutover, and no orphan production Rust files.

**Engineering decisions**

- This phase closes discovered architecture, not only tests changed during the
  milestone. Substrate failures exposed by certification are in scope.
- Milestone 3.9 may begin only after the hostile scenario and all mechanical
  gates are green with no named debt.

**Open questions**

- None.

## Must Ship

- one sealed prepared application authority and distinct active application
  session
- inseparable source candidate composition for file- and Rust-authored paths
- fallible application preparation with typed phase-local denials
- launch and replacement cutover bound to complete application authority
- operational host-session authority with generation-bound observation access
- proof-bearing framework-turn transition planning
- policy-family allocation executors under one thin framework clock owner
- deletion of unreachable GPU/theme source and mechanical orphan-source
  detection
- production-path hostile certification and anti-bypass compile/structure gates
- current discovery and feature documentation for the final ordinary path

## Must Preserve

- the canonical UI artifact remains the source of runtime UI meaning; the new
  application authority binds it to related truth without replacing it with an
  aggregate hash or facade-owned shadow model
- Worth UI remains above host adapters and does not give native code authority
  over layout, operability, Query meaning, or visible semantic state
- Query binding remains Query-owned and installed-authority-backed; this
  milestone binds its lifecycle to the application generation without
  reimplementing Query locally
- invalid reloads preserve the last admitted active generation and typed
  candidate diagnostics remain observational
- allocation work stays delta-bounded, counter-visible, and free of per-frame
  semantic rediscovery
- Query-free and headless applications remain ceremony-free within the same
  honest lifecycle
- support authority remains distinct from production authority
- all production and test Rust files remain within the workspace line-cap rule
  unless explicitly and narrowly exempted

## Acceptance Evidence

- external callers cannot compile any path that launches an artifact,
  replacement candidate, graph, Query binding, or host adapter independently of
  the matching prepared application authority
- invalid public declaration/topology input returns typed preparation denial
  without unwind or partial publication
- active runtime, graph, declarations, Query binding, host observations,
  allocation receipts, and inspection receipts expose one matching application
  generation across launch and replacement
- foreign/stale host adapters and observations deny before measurement or frame
  ingress
- framework-turn planning is mutation-free, family execution is transactional,
  and the owner performs exactly one permitted close/pump per turn
- local allocation changes retain exact bounded breadth counters after
  decomposition
- equivalent file- and Rust-authored applications converge through the same
  prepared and active lifecycle
- every production Rust source file participates in a declared compiled module
  graph and the dormant theme/GPU files and claims are absent
- workspace tests, strict Clippy, warnings/dead-code checks, line-cap checks,
  boundary check, agent-context check, compile-fail suites, and hostile
  certification are green

## Sequencing Notes

- Phase order is mandatory. Candidate ingress cannot be narrowed honestly
  before canonical prepared authority exists; launch and replacement cannot be
  repaired before fallible preparation exists; frame execution cannot be split
  before a complete transition plan exists.
- This milestone is a closure gate for 3.8, not an early implementation of 3.9,
  3.10, or 3.16. It does not lower the future execution plan, mount host render
  receipts, or introduce runtime appearance semantics.
- Milestone 3.9 is blocked until Phase 10 closes with no compatibility lane or
  named architectural debt.
