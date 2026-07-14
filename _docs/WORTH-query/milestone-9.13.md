# Milestone 9.13: Declarative Query Experience And Phase-Surface Cutover

## Goal

Make ordinary `worth-query` usage capability-oriented, declarative, and
discoverable: consumers describe the query outcome they need, Query owns the
proof-preserving phase progression, and lower-level canonicalization,
admission, planning, lowering, execution, maintenance, and inspection
mechanisms cannot become a second application framework.

## Why This Milestone Exists

Milestone 9.12 closes raw authority escape hatches, but authority safety alone
does not make a coherent product API. The ordinary facade still reflects much
of Query's internal phase topology. A consumer can be forced to understand
which request to canonicalize, which binding function to call, which planning
context to seed, which lowerer to select, and which executor or receipt builder
finishes the operation.

That ceremony creates two risks. Consumers rebuild local orchestration above
Query, and later store-backed work duplicates the same public journey for a
second substrate. This milestone closes both risks before Milestone 10.

## Governing Summaries

- `MENTALITY.md` protects production-grade foundations: solve the hostile
  composition problem first, make incorrect usage mechanically unavailable,
  and do not preserve ordinary completion work as compatibility debt.
- `arch_laws.md` protects proof and authority continuity: semantic intent must
  compile into orchestration, lowered plans are the executor's only input, and
  every phase consumes the proof emitted by its predecessor.
- `composition_laws.md` protects legible responsibilities: facade modules may
  aggregate but not implement, orchestration must read as named semantic steps,
  and public phase mechanics must not form an accidental second framework.
- `domain_structure_laws.md` protects structural ownership: public capability
  surfaces must be narrower than internal topology, and authoring, authority,
  execution, diagnostics, compatibility, and certification must remain
  distinguishable in both the tree and import graph.
- `perf_laws.md` protects pre-resolved and cost-honest execution: policy and
  topology decisions happen before the hot path, proof is carried instead of
  rediscovered, and ergonomic APIs may not hide broad scans or allocation.
- `WORTH_query_roadmap.md` protects one declared meaning lowered once and
  executed against canonical truth. It positions this cutover after authority
  closure and before runtime/store execution parity.

## Adversarial Constraint

An ordinary consumer must be able to express each supported read, live,
historical, comparison, preview, workflow, and inspection outcome without
choosing or reconstructing Query's internal phase order, backend, policy
lowerer, execution route, or receipt assembly.

Equivalent declarations must converge on the same canonical artifact,
admitted authority, plan, result meaning, and diagnostic identity. Skipped,
reordered, cross-capability, cross-basis, stale, or backend-shaped assembly
must be unrepresentable or deny before lower-runtime contact. The ergonomic
surface must add no unbounded work and must not become a generic builder that
erases capability-specific denials.

## Product Decision Lock

- Ordinary APIs express desired Query capabilities, not pipeline mechanics.
- Query owns canonicalization, binding, validation, admission, planning,
  lowering, execution routing, receipt production, and derived diagnostics.
- Proof-bearing phase artifacts may be returned for observation or advanced
  integration, but ordinary consumers cannot construct them or independently
  advance them.
- Capability families retain distinct declarations and typed outcomes where
  their cost, failure, lifecycle, or correctness semantics differ. This is not
  a single universal query builder.
- Safe defaults may remove ceremony only when Query can derive them from
  admitted context. Authority, basis, policy, tenant, bounds, and lifecycle
  choices are never guessed.
- Runtime-backed and future store-backed execution implement the same admitted
  capability contract. Backend selection is Query-owned and observable, not
  application-authored.
- Advanced domain contribution remains possible through explicit extension
  contracts; it does not require public access to Query's internal pipeline.
- Compatibility aliases, dual constructors, and deprecated phase entrypoints
  do not survive closeout.
- Certification, support discovery, inventories, and golden artifacts remain
  outside the ordinary product surface.

## Phase Plan

### Phase 1: Freeze Consumer Journeys And The Public Phase Graph

Inventory what ordinary consumers actually do and map each journey from public
entrypoint to canonical result, including every public function that exposes or
advances an internal Query phase. This freezes the refactor boundary before API
design begins.

**Relevant subsystems**

- `facade` and crate-root exports
- authoring, canonicalization, binding, validation, planning, and execution
- policy, live, historical, preview, workflow, and inspection surfaces
- Worth UI and at least one serious domain consumer

**Relevant APIs**

- public constructors and free functions in ordinary facade namespaces
- public phase artifacts and their transition functions
- consumer-local Query wrappers, coordinators, and receipt assembly
- public API snapshots, residue registries, and import audits

**Warnings**

- Export count alone cannot distinguish a rich product API from leaked
  machinery; inventory entries must record who is expected to call them.
- Existing consumer wrappers may encode missing Query capabilities rather than
  mere cleanup debt. Their semantic responsibility must be classified before
  deletion.
- Tests and examples are consumers and can perpetuate the wrong mental model.

**Test requirements**

- A seeded inventory test adds an unclassified phase transition to an ordinary
  facade and proves the audit fails with its exact source location.
- A consumer-residue test detects local canonicalize/bind/plan/execute
  orchestration even when it is renamed or split across helper functions.
- A journey-equivalence test proves facade aliases and deep imports resolve to
  distinct rows rather than being collapsed by name similarity.

**Engineering decisions**

- Each public surface is classified as ordinary declaration, ordinary outcome,
  sealed returned proof, extension contract, terminal representation,
  diagnostics, certification, compatibility, or internal mechanism.
- Each consumer journey records declared intent, required context, admitted
  capability, Query-owned phase chain, result, receipts, diagnostics, cost
  counters, and local ceremony.
- The inventory is source-backed and fails when newly public constructors or
  transitions are not classified.

**Open questions**

- None.

### Phase 2: Freeze The Capability-Oriented Facade And DX Grammar

Define the ordinary vocabulary consumers discover first. The surface is
organized by what the consumer is trying to accomplish, with progressive
disclosure for optional policy, live, history, workflow, and inspection
capabilities.

**Relevant subsystems**

- facade namespaces and re-export files
- authoring builders and typed domain declarations
- ordinary outcome and typed stop families
- API documentation and IDE-visible examples

**Relevant APIs**

- query/read declarations
- live subscription declarations
- history, diff, and correspondence declarations
- preview, mutation, and workflow declarations
- result, receipt, and inspection entrypoints

**Warnings**

- A single prelude containing every public type is not discoverability.
- Fluent syntax must not permit invalid phase order or conceal expensive
  traversal, unbounded collections, or implicit basis selection.
- Parallel convenience methods with slightly different defaults recreate
  semantic ambiguity.

**Test requirements**

- A compiler-backed grammar contract freezes one complete ordinary journey
  signature per capability family: facade namespace, declaration/refinement/
  terminal vocabulary, outcome, typed stop, next action, explicit context,
  cost disclosure, ceremony budget, and the later phase that owns its first
  executable transcript. Missing fields or an unowned transcript fail this
  phase.
- Compile-pass DX transcripts prove every capability implemented through
  Phase 5 uses only its frozen ordinary facade imports. Phases 6 through 9
  must add the executable transcript for each family they implement, and
  Phase 12 must reject any frozen grammar without a compiling final journey.
- Compile-fail transcripts prove capability-specific options cannot be applied
  to the wrong declaration family or after admission.
- Compile-fail tests prove a declaration cannot run without an explicit context
  handoff. Hostile context tests prove missing or mismatched basis, policy,
  tenant, or relationship authority inside that handoff produces a typed
  next-action stop instead of guessing.

**Engineering decisions**

- The public grammar follows `declare -> refine -> admit/run -> outcome`; Query
  may fuse steps internally without exposing its internal phase count.
- Phase 2 freezes the final grammar and transcript owner; it does not create
  placeholder declarations, synthetic success outcomes, or non-constructible
  context shells for capabilities whose operational semantics belong to later
  phases. Each owning phase must implement the already-frozen signature rather
  than redesigning it.
- Common refinement vocabulary is shared only where meaning, cost, denial, and
  lifecycle are genuinely identical.
- Typed stops report the missing decision and the valid next action. They do
  not expose internal stage names as consumer instructions.
- The facade has a measurable ceremony budget: reference journeys count
  imports, consumer-authored intermediate values, explicit transitions, and
  local adapters before and after cutover.

**Open questions**

- Exact namespace and method names remain implementation decisions, provided
  the frozen journey grammar and capability boundaries are preserved.

### Phase 3: Converge Ordinary Authoring On One Canonical Declaration Entry

Ensure all ordinary query shapes begin as typed declarative intent and lower
through one Query-owned canonicalization path. Eliminate parallel raw request,
artifact-first, and payload-shaped authoring lanes.

**Relevant subsystems**

- `authoring`
- `canonicalization`
- composition, scopes, templates, predicates, and result shapes
- domain graph operation declarations

**Relevant APIs**

- detail and collection authoring builders
- typed predicate, traversal, ordering, and result-shape selectors
- scope/template composition and parameter binding
- canonical query bundle construction

**Warnings**

- Ergonomic strings are acceptable only as authoring input that immediately
  lowers into native typed carriers; they cannot survive as internal authority.
- Direct canonical artifact construction is a parallel authoring lane even if
  the resulting artifact is well formed.
- A generic expression escape hatch can silently bypass family-specific
  legality and cost bounds.

**Test requirements**

- Equivalent direct, composed-scope, and template-instantiated declarations
  converge on the same canonical query and result-shape identity.
- Compile-fail tests prove ordinary consumers cannot construct canonical
  bundles, canonical entries, or validated selector carriers directly.
- Invalid field, traversal, predicate, and result-shape combinations deny with
  typed authoring context before binding or lower-runtime contact.

**Engineering decisions**

- Canonical artifacts remain inspectable, serializable where appropriate, and
  stable for replay, but are minted only by Query-owned lowering.
- Domain extensions contribute typed vocabulary through declared extension
  contracts rather than custom canonicalization forks.
- Authoring sugar is tested for canonical equivalence, not as a separate
  semantic path.

**Open questions**

- None.

### Phase 4: Bind Context, Basis, Tenant, And Policy Declaratively

Replace consumer-authored sequencing across binding, basis, tenant, policy,
and admission APIs with one declarative context handoff that produces the
sealed capability required by the requested journey.

**Relevant subsystems**

- binding and query context
- basis lifecycle
- policy basis, narrowing, planning, and delivery
- tenant and relationship-proof admission

**Relevant APIs**

- binding requirements and resolutions
- scoped basis capabilities
- policy/tenant admission and narrowing
- relationship-proof requirements and typed denials

**Warnings**

- Context ergonomics must not flatten distinct basis, tenant, policy, and
  relationship authorities into an untyped bag.
- Reusing a context is legal only under an explicit equivalence contract.
- Policy or tenant defaults cannot be inferred from ambient process state.

**Test requirements**

- Equivalent declarations with equivalent admitted context converge on the
  same scoped capability and narrowed canonical identity regardless of input
  ordering.
- Cross-basis, stale-policy, cross-tenant, and mismatched relationship proof
  combinations deny before planning or execution and produce no partial
  successor.
- Reuse tests prove changed policy epoch or basis generation invalidates only
  the affected context and cannot reuse a prior admission.

**Engineering decisions**

- Consumers supply declarations and authority-bearing context; Query derives
  binding requirements, resolves supported bindings, admits basis, narrows
  policy, and produces one sealed handoff.
- Missing context returns a typed, capability-specific stop that identifies
  what authority must supply next.
- Internal policy phase artifacts are carried through the proof chain, not
  exposed as ordinary assembly pieces.

**Open questions**

- None.

### Phase 5: Collapse Read Planning And Execution Behind Admitted Queries

Make one-shot detail, collection, graph, aggregate, and composed reads execute
from an admitted query capability. Consumers may inspect planning and execution
evidence, but do not select planners, seed plan contexts, choose execution
routes, or assemble result bundles.

**Relevant subsystems**

- validation and planning
- frontier planning and execution
- collection and graph composition
- runtime-backed read routing and result construction

**Relevant APIs**

- planning request contexts and plan seed functions
- collection planning and post-read shaping
- serial and parallel execution routes
- runtime execution and result-bundle builders

**Warnings**

- Hiding the planner does not permit hiding traversal breadth, fallback, or
  execution cost from the outcome.
- Collection, detail, graph, and aggregate families must not be forced through
  scalar or shape-erasing execution.
- Backend choice cannot leak into canonical query meaning or result shape.

**Test requirements**

- Direct ordinary execution and an instrumented internal phase-chain fixture
  produce identical canonical plan identity, result, receipt, and counters.
- Compile-fail tests prove ordinary consumers cannot seed plans, choose serial
  or parallel routes, call executors with partially proved artifacts, or build
  successful result bundles.
- Unsupported family, excessive traversal, and invalid result-shape requests
  deny before execution with exact zero lower-runtime work.

**Engineering decisions**

- The admitted query is the only ordinary input to execution.
- Planning reports, route posture, fallbacks, and counters are derived evidence
  attached to outcomes or inspection, not prerequisites callers assemble.
- Internal execution remains phase-typed and backend-extensible so Milestone 10
  can add store-backed implementations without changing the ordinary journey.

**Open questions**

- None.

### Phase 6: Make Live Queries Framework-Owned Resources

Turn live usage into a declarative lifecycle owned by Query. Consumers declare
desired live behavior and receive a managed handle; they do not independently
assemble subscription admission, activation, maintenance, continuation,
suppression, or teardown phases.

**Relevant subsystems**

- declarative live
- subscription declaration and activation
- live planning, maintenance, delivery, and continuation
- temporal, async, preview-isolation, and policy-live paths

**Relevant APIs**

- live query declarations and view-shape refinements
- subscription family selection and managed lifecycle handles
- delivery batches, continuation receipts, and teardown outcomes
- policy drift and live rebind outcomes

**Warnings**

- A managed handle must not become a mutable bag that lets consumers assert
  activation or maintenance posture.
- Delivery callbacks, streams, and UI adapters are projections of the same
  lifecycle, not separate subscription authorities.
- Resource ownership includes deterministic teardown and orphan prevention.

**Test requirements**

- A compile-pass `facade::live` journey implements the Phase 2 grammar contract
  exactly and proves declaration, context handoff, open, delivery observation,
  and deterministic close without importing lifecycle phase machinery.
- One-shot and live-promoted evaluation converge for the same admitted query,
  basis, policy, and view shape across update, suppression, and replay.
- Compile-fail tests prove consumers cannot activate an unadmitted declaration,
  author maintenance posture, fabricate continuation, or revive a disposed
  handle.
- Crash/resume, policy drift, stale basis, and preview isolation tests prove a
  handle cannot cross its admitted lifecycle or leak authoritative residue.

**Engineering decisions**

- Query registers, tracks, advances, and disposes every live resource.
- The ordinary surface exposes declaration refinements, a managed handle,
  typed delivery, observable lifecycle posture, and explicit typed stops.
- Temporal and async causes remain distinct in evidence even when the consumer
  receives them through one managed delivery experience.

**Open questions**

- None.

### Phase 7: Unify Historical, Diff, And Correspondence Journeys

Make historical reads, branch comparison, diffs, lineage, and correspondence
begin from the same declarative query vocabulary while preserving their
distinct basis, ambiguity, materialization, and cost semantics.

**Relevant subsystems**

- historical and query context
- correspondence and correspondence history
- identity evolution and lineage
- diff shaping and comparison result bundles

**Relevant APIs**

- historical basis and materialization declarations
- comparison-basis declarations
- diff, lineage, and correspondence refinements
- ambiguity, disagreement, denial, and success outcomes

**Warnings**

- Historical and current reads may share authoring grammar but not pretend to
  have identical availability or materialization cost.
- Correspondence advisory evidence must never silently promote to continuity.
- Diff and comparison must bind both bases structurally; a convenient tuple is
  not sufficient proof.

**Test requirements**

- Compile-pass `facade::history` and `facade::comparison` journeys implement
  their Phase 2 grammar contracts exactly using only capability-specific
  declaration, context, outcome, stop, and inspection vocabulary.
- Current, historical, and restored internal fixtures yield equivalent result
  meaning when they observe the same canonical truth basis.
- Cross-basis pairing, missing history, ambiguous correspondence, and stale
  lineage deny or remain advisory without partial success artifacts.
- Compile-fail tests prove consumers cannot build comparison contexts, promote
  ambiguity, or assemble historical success envelopes directly.

**Engineering decisions**

- Shared query shape and projection vocabulary remain common; basis pair,
  materialization path, correspondence posture, and typed outcome remain
  capability-specific.
- Query owns historical planning and comparison assembly.
- Durable restore remains Milestone 10/11 scope, but it must implement the same
  admitted historical capability rather than add a new public journey.

**Open questions**

- None.

### Phase 8: Converge Preview, Mutation, And Workflow Orchestration

Make preview, writeback, effect, merge-adjacent, and domain workflow usage
declarative while preserving lower-runtime mutation and branch authority.
Consumers describe the workflow and inspect its outcomes; Query owns the
admission and orchestration chain.

**Relevant subsystems**

- preview and preview-live
- workflow and query-authored mutation declarations
- effect lifecycle and writeback
- domain capability contributions and lower-runtime routing

**Relevant APIs**

- preview workflow declarations and scoped preview bindings
- mutation/effect declarations
- writeback triggers and branch workflow declarations
- execution, promotion, denial, closeout, and aftermath outcomes

**Warnings**

- Declarative mutation does not transfer mutation, merge, or preview-session
  authority into Query.
- Workflow convenience must not collapse admission, execution, promotion, and
  aftermath into an uninspectable boolean result.
- Read-only and promotion-eligible preview states require distinct proof types.

**Test requirements**

- Compile-pass `facade::preview`, `facade::mutation`, `facade::workflow`, and
  `facade::domain` journeys implement their Phase 2 grammar contracts exactly;
  each transcript must exercise its real authority handoff and terminal rather
  than a placeholder or compile-only shell.
- Equivalent explicit internal orchestration and ordinary workflow declaration
  produce identical lower-runtime request, receipt, aftermath, and inspection
  identity.
- Unauthorized mutation, stale preview, cross-session promotion, and
  unsupported writeback deny before lower-runtime execution with no partial
  authority residue.
- Compile-fail tests prove consumers cannot construct admitted effects, lowered
  plans, promotion eligibility, or successful workflow envelopes.

**Engineering decisions**

- Ordinary workflows return structured outcomes preserving advisory,
  violation, execution, and aftermath distinctions.
- Domain-defined workflow vocabulary enters through contribution contracts and
  lowers through Query-owned orchestration.
- Lower runtimes remain the only executors of the authorities they own.

**Open questions**

- None.

### Phase 9: Make Results, Receipts, And Inspection Cohesive

Give consumers one coherent way to use successful results, typed stops,
operational receipts, and optional rich inspection without requiring them to
know which internal subsystem produced each artifact.

**Relevant subsystems**

- ordinary outcome
- result bundles and projection consumption
- causal inspection and diagnostics
- support posture and execution evidence

**Relevant APIs**

- ordinary success, advisory, violation, deferred, and unavailable outcomes
- projection facts and consumed authority
- operational receipts and causal inspection requests
- planning, execution, live, history, and policy diagnostics

**Warnings**

- A unified outcome cannot erase capability-specific facts or failure
  topologies.
- Receipts are evidence of completed work, not authority for new work.
- Rich diagnostics must remain policy-controlled and must not burden the
  ordinary hot path when not requested.

**Test requirements**

- A compile-pass `facade::inspection` journey implements its Phase 2 grammar
  contract exactly, begins from authentic outcome evidence plus scoped
  inspection basis, and never imports source phase artifacts.
- Every capability family exposes a common outcome navigation contract while
  preserving its family-specific success and denial payloads.
- Receipt-only, digest-only, and diagnostic-only attempts to authorize new
  execution or inspection fail at compile time or typed admission.
- Minimal and rich diagnostic policies produce identical operational outcomes
  and receipts while differing only in derived inspection materialization.

**Engineering decisions**

- Common outcome traits or navigation vocabulary unify only shared structural
  fate; concrete payloads preserve capability semantics.
- Projection consumption is the ordinary typed fact extraction lane.
- Inspection begins from an authentic outcome/receipt plus scoped inspection
  basis; it does not reopen source authority.

**Open questions**

- None.

### Phase 10: Quarantine Phase APIs And Contract The Ordinary Facade

Move phase constructors, transition functions, route selectors, report
builders, support inventories, and certification machinery out of ordinary
consumer reach. Preserve only capability declarations, sealed returned proofs,
typed outcomes, operational receipts, and explicit extension contracts.

**Relevant subsystems**

- facade export groups and crate-root exports
- internal phase modules
- certification and Consumer Kit namespaces
- support, inventory, migration, and compatibility surfaces

**Relevant APIs**

- canonicalize, bind, validate, admit, plan, lower, execute, and envelope
  transition functions
- public artifact constructors and success-envelope builders
- support-discovery, matrix, audit, and certification exports
- deprecated aliases and compatibility adapters

**Warnings**

- Moving an API to a deeper public module is not quarantine.
- Making constructors private while retaining public free-function transitions
  may still expose the same assembly framework.
- Tests cannot justify product-public visibility; certification gets an
  explicit non-ordinary namespace or crate-local access.

**Test requirements**

- Golden public API snapshots prove ordinary namespaces contain only the
  classified product surface and fail on any newly exposed phase transition.
- Compile-fail suites prove downstream crates cannot import or call internal
  phase machinery through crate root, deep modules, re-exports, aliases, or
  generic conversion traits.
- Internal parity tests prove facade contraction does not change canonical
  artifacts, outcomes, receipts, diagnostics, or cost counters.

**Engineering decisions**

- Public visibility is minimized at the defining module, not only hidden from
  re-export lists.
- Certification and Consumer Kit tooling remain visibly separate from product
  use and cannot mint operational authority.
- Removed ordinary phase APIs receive permanent prohibition and residue rows;
  no deprecated compatibility period survives milestone closure.

**Open questions**

- None.

### Phase 11: Cut Over Reference Consumers And Rewrite Discovery

Migrate Worth UI and a serious domain consumer to the capability-oriented
surface, delete their local Query orchestration, and make documentation teach
only the resulting present-tense mental model.

**Relevant subsystems**

- Worth UI runtime/query integration
- serious domain entry and workflow integration
- Query AI discovery and feature documentation
- examples, tests, fixtures, and consumer residue audits

**Relevant APIs**

- ordinary declaration and managed lifecycle journeys
- domain contribution contracts
- result/projection consumption and inspection
- consumer-local query coordinators and wrappers

**Warnings**

- A consumer wrapper that merely renames the old phase chain is not adoption.
- Discovery docs must not teach historical methods as alternatives or as
  banned folklore; obsolete methods should simply be absent.
- Test fixtures must model the ordinary API unless their explicit purpose is
  internal certification.

**Test requirements**

- Consumer-shaped end-to-end tests exercise representative read, live,
  historical/comparison, workflow, and inspection journeys using only ordinary
  facade imports.
- Residue tests inject local canonicalization/planning/execution and local
  subscription lifecycle assembly into consumer sources and prove exact
  detection.
- Documentation examples compile, and discovery-link tests prove every
  advertised capability resolves to a current ordinary API.

**Engineering decisions**

- Adoption evidence records deleted local types, helpers, transitions, deep
  imports, and backend decisions, not only new call sites.
- DX transcripts measure before/after ceremony and local decision count.
- `AI_README.md` remains a discovery document: it explains what exists, where
  to start, and which capability family to choose without historical API
  instruction.

**Open questions**

- None.

### Phase 12: Certify The Declarative Product Boundary

Run hostile certification over the complete ordinary experience and close only
when every supported capability follows one declarative entry, one
proof-preserving internal chain, and one coherent outcome contract.

**Relevant subsystems**

- all ordinary capability facade groups
- public API, prohibition, and residue certification
- reference consumers and DX transcripts
- runtime-backed hostile certification matrix

**Relevant APIs**

- final ordinary facade
- final extension contracts
- certification bundles and support matrix
- prohibition and consumer-residue registries

**Warnings**

- Happy-path brevity is insufficient; closure requires hostile composition,
  stale proof, denial localization, lifecycle, and cost evidence.
- A phase API with no current consumer remains a competing public framework and
  blocks closure.
- Store-backed absence is allowed; a runtime-specific ordinary contract is not.

**Test requirements**

- The final compiler-backed transcript matrix proves every Phase 2 grammar row
  resolves to an executable ordinary facade journey with the same namespace,
  ordered vocabulary, typed outcome/stop/next-action contract, context
  requirement, and ceremony ceiling; a frozen-but-unimplemented or drifted
  family blocks closure.
- A hostile matrix covers equivalent declaration convergence, cross-capability
  option rejection, cross-basis and stale-context denial, one-shot/live parity,
  historical ambiguity, preview/workflow denial, receipt non-promotion, and
  diagnostic-policy equivalence.
- Sabotage tests add a public phase constructor, deep transition, backend
  selector, success-envelope builder, compatibility alias, and consumer-local
  coordinator; each must fail a named enforcement layer.
- Exact counter tests prove invalid declarations deny before planning or
  lower-runtime contact and ordinary ergonomic lowering adds no work dependent
  on unrelated workspace, history, subscriber, or consumer size.
- Reference-consumer certification proves canonical result, receipt, authority
  identity, and diagnostics match the internal instrumented phase-chain oracle.

**Engineering decisions**

- Closure evidence composes facade snapshot, prohibition, residue, DX,
  reference-consumer, semantic parity, lifecycle, and bounded-work digests.
- Every shipped capability has one declared ordinary journey and one owner for
  each internal transition.
- Any missing ordinary capability discovered during cutover expands this
  milestone unless it requires a genuinely store-dependent implementation.

**Open questions**

- None.

## Must Ship

- capability-oriented ordinary facade grammar
- one typed declarative authoring entry into canonical query meaning
- declarative basis, tenant, policy, and relationship-context handoff
- admitted-query execution for read and collection families
- framework-owned live resource lifecycle
- coherent historical, diff, correspondence, preview, mutation, workflow, and
  inspection journeys
- cohesive outcomes, receipts, projection consumption, and optional diagnostics
- internalized phase transitions and permanently prohibited legacy phase APIs
- Worth UI and serious-domain adoption with measured DX evidence
- hostile facade, residue, parity, lifecycle, and bounded-work certification

## Must Preserve

- Query owns expression, canonicalization, admission, planning, orchestration,
  result shaping, and public developer experience
- lower runtimes retain truth, mutation, merge, reactive scheduling,
  persistence, and transport authorities
- basis, tenant, policy, branch, preview, and relationship authority remain
  explicit and proof-bearing
- family-specific cost, failure, correctness, and lifecycle distinctions remain
  visible
- diagnostics and support posture remain derived and non-authoritative
- advanced domain contribution remains possible without internal pipeline access
- canonical semantics, receipts, replay identity, and exact counters do not
  change merely because the ordinary facade becomes simpler

## Acceptance Evidence

- ordinary consumers perform no manual canonicalize/bind/validate/plan/lower/
  execute/envelope progression
- ordinary consumers cannot construct or advance internal phase artifacts
- equivalent declarations converge across ergonomic forms and reference
  consumers
- invalid combinations deny before expensive construction or lower-runtime
  contact with typed next-action context
- live resources are registered, tracked, resumable where supported, and
  deterministically disposable
- historical, comparison, preview, workflow, and inspection journeys preserve
  their distinct proof and denial semantics
- public API snapshots, compile-fail tests, permanent prohibitions, residue
  audits, docs, and support matrices agree on the same surface
- reference journeys show materially lower ceremony without semantic defaults,
  backend choices, or authority decisions moving into consumer code
- no compatibility aliases, callable phase entrypoints, consumer-local Query
  coordinators, or backend-shaped ordinary APIs remain

## Sequencing Notes

This milestone belongs after 9.12 because authority must be sealed before the
ordinary experience can safely hide internal phase transitions. It belongs
before Milestone 10 because store-backed execution must implement an existing
admitted capability contract, not create a second set of consumer-visible
planning and execution journeys.

Milestone 10 may add backend-specific internal plans, pushdown evidence, and
fallback diagnostics. It may not change the declaration grammar, context
handoff, managed lifecycle, or outcome contract frozen here.

## Store Dependency

This milestone is not blocked on `worth-store`. Runtime-backed execution is
sufficient to freeze and certify the product boundary. Store-backed parity,
durable restore, saved artifact survival, and durable continuations remain
Milestones 10 and 11, but they must enter through the contracts established
here.
