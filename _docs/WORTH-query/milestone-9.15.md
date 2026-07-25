# Milestone 9.15: Execution-Grade Domain Computation, Transactional Attempts, And Managed Artifacts

## Goal

Make the installed operation model from Milestone 9.14 operationally complete
for large, long-running, data-dependent domain computation without teaching
Query any domain's algorithms or vocabulary.

An installed operation must be able to carry large typed intermediate products,
execute against an admitted provider plan, observe positive and negative graph
facts, retain an exact realized dependency footprint, build and inspect a
provisional post-state, run real domain invariants over that post-state, and
atomically commit or leave no authoritative residue. The same lifecycle must
admit bounded cancellation, backpressure, resumable progress, managed access
products, set-oriented execution, structural cost evidence, and policy-governed
domain decision records.

## Why This Milestone Exists

Milestone 9.14 closes Query's semantic control plane: installed operation
meaning, one operating-world root, graph participation, workflow progression,
publication, lineage, replay fences, dependency impact, sharing, invalidation,
and managed consumer lifecycle are Query-owned and proof-bearing.

That is not yet enough for a geometry-kernel-grade data plane. A naive adoption
can still:

- squeeze domain products through primitive workflow values, whole projections,
  strings, serialized blobs, or side-channel stores
- call a selected obligation an executed invariant without evaluating domain
  state
- read a neighborhood, decide an edit, and commit later without proving that
  every positive, negative, and structural decision fact is still current
- call provider commit admission "atomic" while effects occur through
  unbound sequential callbacks
- report progress, streaming, or resource bounds after complete eager
  materialization has already occurred
- retain spatial or other domain indexes without a completeness, membership,
  invalidation, disposal, or memory contract
- invalidate from a declared upper bound because the exact realized footprint
  was discarded
- scalarize bulk domain work or demand quadratic pairwise independence proof
- rebuild queryable domain decision logs and structural counters in every
  downstream crate

Those failures would force geometry, topology, simulation, routing, compiler,
and other high-volume domains to regrow local runtimes above Query. Milestone
9.15 closes the generic execution substrate before Milestone 13 freezes
provider-independent certification.

## Governing Summaries

### Engineering Mentality

The hard production constraint must determine the first implementation. The
milestone therefore begins with managed artifact and transactional-attempt
authority, not convenience builders or one happy-path provider.

### Architectural Laws

Authority, derivation, resource lifecycle, phase progression, cancellation,
schema evolution, evidence, and failure topology must be explicit and typed.
Selection cannot masquerade as execution, cancellation cannot masquerade as
rollback, and no representation may reconstruct operational authority.

### Composition Laws

Read binding, decision-footprint capture, provisional mutation, invariant
evaluation, prepare, commit, abort, and recovery are separate semantic phases.
Provider callbacks, helper modules, or facade methods may not hide those
transitions or their effects.

### Domain Structure Laws

Artifacts, access products, managed runs, provider sessions, transaction
attempts, footprints, evidence, and publication have distinct authority and
lifecycle. Their destination topology must preserve those distinctions before
the first implementation and must admit future domain families without
reclassification.

### Performance Laws

Execution breadth must be bounded by semantic delta and the smallest honest
physical granule. Bulk semantics stay bulk; actual work, allocation, retained
state, membership maintenance, coordination, and physical amplification are
separate named counter domains. Synthetic progress and post-hoc cost estimates
are not execution evidence.

### Query Roadmap

Milestone 9.14 freezes installed semantic authority. Milestone 13 certifies the
complete runtime-backed framework and exports provider-independent oracles.
Milestone 9.15 belongs between them because provider certification is dishonest
until the installed model can carry domain products, prove real post-state
invariants, govern actual provider sessions, and survive resource and
concurrency pressure.

## Adversarial Constraint

> A long-running installed domain operation computes over a large graph while
> carrying multi-gigabyte-class intermediate products, using a derived access
> product whose result depends on both present rows and negative membership
> space. It discovers a narrow realized footprint inside a wider declared
> authority closure, yields and resumes under bounded memory and backpressure,
> races a relevant and an unrelated concurrent mutation, constructs a
> provisional post-state, and runs domain invariants over that exact post-state.
> Cancellation may arrive at every safe point and provider failure may occur
> before prepare, after prepare, during effect staging, or after an uncertain
> commit response. The operation must either publish one canonical committed
> result bound to the exact read basis, invariant receipts, effects, artifacts,
> footprint, and actual cost, or return a typed stale, yielded, cancelled,
> exhausted, degraded, aborted, partial-effect, or indeterminate outcome with no
> fabricated rollback, no widened authority, no stale promotion, no leaked
> resource, and no unexplained authoritative residue.

Equivalent serial, partitioned, resumed, replay-certified, and alternate
provider executions must converge on the same declared semantic result and
canonical evidence according to the installed reproducibility class, without
collapsing distinct required execution/acquisition occurrences or fabricating
independent verification from reuse. Irrelevant concurrent mutation must not
cause false conflict. A heuristic or incomplete candidate search must not claim
uniqueness or optimality; a merely stable or feasible iteration must not claim
convergence; and transformation, correspondence, loss, repair, candidate, or
advisory evidence must not become admission or durable resolution authority.
Relevant mutation, missing negative-space coverage, widened provider scope,
failed invariant execution, stale attempt generation, and incomplete commit
knowledge must fail at the exact responsible phase.

## Product Decision Lock

The following decisions are frozen for this milestone:

1. Query owns installed contracts, admission, authority-bearing handles,
   lifecycle, phase progression, dependency carriage, public evidence, and
   certification. Domains own artifact payload meaning, algorithms, validators,
   counter semantics, decision-record payloads, replanning policy, and
   equivalence comparators. Lower providers own physical construction,
   allocation, transaction mechanics, and lookup execution.
2. Query will not gain geometry, topology, spatial, solver, routing, compiler,
   or other domain vocabulary. No public type may mention B-rep entities,
   predicates, tolerances, BVH/R-tree/AABB strategy, manifold rules, repair
   policy, or persistent-name selection policy.
3. A workflow artifact is an installed semantic contract plus a runtime-minted
   managed handle. `Any`, `dyn Any`, arbitrary JSON, opaque serialized blobs,
   caller-chosen type strings, and caller-provided digests are not artifact
   authority.
4. Artifact payloads remain domain/provider-owned. Query carries identity,
   schema/protocol version, basis, provenance, lifecycle, ownership, cost,
   disclosure, equivalence, invalidation, and reconstruction posture.
5. Artifacts move by default. Cloning requires a declared observer, isolation,
   retry, temporal, or provider-transfer boundary and must expose byte and
   ownership cost. Semantic replay records a canonical semantic projection or
   retained handle identity, not a clone of the entire payload.
6. Obligation selection, support registration, a successful generic write, or
   an installed invariant name is not invariant execution evidence. A blocking
   invariant must return a provider/domain-minted verdict over the exact
   proposed post-state and basis.
7. An atomic operation is one transactionally bound attempt. Commit admission
   without a session token, sequential unbound touch callbacks, compensating
   after partial effects, or several independent mutation receipts may not be
   called atomic.
8. Every fact used to choose an effect belongs to the decision read-set. This
   includes positive rows, absence, membership, predicate results, cardinality,
   ordering, traversal frontier, closure, and structural proof. Restating only
   copied target values is insufficient.
9. Runtime-discovered footprints may narrow a predeclared closure but can never
   widen it or mint authority. Widening is a typed denial before the widened
   read or effect.
10. Cancellation is an outcome, not rollback. A cancelled run states completed
    work, effects, retained artifacts, cleanup, resumability, and recovery
    posture. No cancellation path may imply reversal of an escaped effect.
11. Progress and streaming evidence come from actual provider work and bounded
    chunks. Deriving page, frontier, or checkpoint counters from a completed
    eager result is prohibited on the execution-grade lane.
12. A domain access product is derived, disposable, and rebuildable. Query may
    govern its identity and lifecycle but cannot promote it to truth or choose
    its physical algorithm.
13. Spatial-like membership requires a coverage witness for negative space.
    Dependencies on returned rows alone cannot justify reuse or incremental
    maintenance when new or moved entities could enter the result.
14. Bulk domain work crosses Query as bulk work. External scalar loops and
    quadratic all-pairs independence enumeration are prohibited when the domain
    cardinality is set-oriented.
15. Mandatory correctness, authority, recovery, and declared-cost evidence stay
    in the canonical boundary core. High-cardinality domain decisions, traces,
    and optional counters are policy-materialized sidecars with explicit
    omission, redaction, retention, and disclosure posture.
16. Durable artifact payloads, access products, checkpoints, continuations,
    journals, transaction recovery, and restart reconciliation remain Store
    responsibilities. Query defines the semantic and provider contracts Store
    must implement without claiming durability locally.
17. The existing one-operating-world root, installed operation identity, basis,
    publication, lineage, compatibility, dependency-impact, lease, window,
    patch, and lifecycle authorities from 9.14 are extended, never duplicated.
18. No fallback may weaken correctness, authority, security, or durability.
    Degradation may reduce freshness, richness, throughput, retained progress,
    or physical strategy only when the retained and weakened guarantees are
    named.
19. Semantic artifact identity, production or acquisition occurrence identity,
    and independent certification or evidence identity are distinct. Equal
    payloads or semantic projections may justify computational reuse only when
    the installed consumer contract permits substitution; they cannot erase a
    required fresh occurrence or satisfy an independent-verification
    obligation.
20. Every executable artifact-producing operation declares one installed
    reproducibility class: exact deterministic, seeded deterministic,
    deterministic under canonical reduction, domain-comparator equivalent,
    interval/error-bound equivalent, distributionally equivalent, or
    observational and non-replayable. Query carries and enforces that contract;
    domains own comparator, bound, distribution, and evidence meaning.
21. A candidate-producing computation declares search-universe, termination,
    completeness, feasibility, comparison, and optimality posture. `exhaustive`,
    `proven_top_k`, `bounded`, `sampled`, `heuristic`, and `incomplete` remain
    distinct; so do `proven_optimal`, `bounded_gap`, `pareto_for_declared_set`,
    `feasible_only`, and `unknown`. Query carries the installed posture and
    evidence while domains own candidate and objective meaning.
22. Query may manage bounded single-basis convergence epochs with installed
    progress, convergence, incumbent, iteration, oscillation, cancellation, and
    exhaustion contracts. It does not decide domain convergence or turn a
    derived candidate, repair proposal, or advisory into resolution authority.
23. Transformation occurrences and loss evidence are derived artifact evidence.
    They may bind immutable source occurrence, versioned transformation,
    source/output correspondence, disposition, and error posture, but they
    cannot admit foreign input, repair truth, preserve identity, or authorize
    publication by themselves.
24. Durable conflict identity, typed resolution alternatives, admitted/rejected/
    superseded/applied decision state, participant roles, approval, deferral,
    carry-forward, journaling, and resolution-session recovery belong to
    `_docs/cross-runtime/merging-and-branching-roadmap.md` Milestones 10-11.
    Milestone 9.15 must not create a Query-local substitute.

## Canonical Capability Progression

```text
PortableDomainComputationContracts
  -> InstalledDomainComputationContracts
  -> DomainComputationAdmission
  -> BoundArtifactIdentityAndReproducibilityPolicy
  -> BoundSearchConvergenceAndTransformationPolicy
  -> BoundExecutionResourcePlan
  -> ManagedDomainRun
  -> PreparedProviderSession
  -> BasisPinnedDecisionReadSet
  -> DeclaredDomainStageExecutionEvidence
     | CandidateSearchEvidence
     | ConvergenceEpochEvidence
     | TransformationEvidence
  -> ProvisionalEffectProgram
  -> ProposedPostState
  -> DomainInvariantExecutionReceipts
  -> PreparedCommitAttempt
  -> CommittedDomainAttempt
     | StaleAttempt
     | YieldedRun
     | CancelledRun
     | ExhaustedRun
     | DegradedRun
     | AbortedAttempt
     | PartialEffectAttempt
     | IndeterminateAttempt
  -> DomainComputationBoundaryEnvelope
  -> DomainComputationCertificationBundle
```

The declared stage-evidence step is present only when the installed operation
claims one of those evidence families; its absence cannot be interpreted as an
empty exhaustive search, immediate convergence, or lossless transformation.

No later state accepts raw components from an earlier phase. In particular:

- a provider session cannot be reconstructed from provider identity and basis
- a proposed post-state cannot be reconstructed from an effect list and a
  snapshot label
- an invariant receipt cannot be reconstructed from an invariant identity and
  successful mutation receipt
- a prepared commit cannot be reconstructed from independent read, effect, and
  invariant receipts
- a committed attempt cannot be reconstructed from a provider success string,
  commit-admission receipt, or matching digest
- a resolution command or durable decision cannot be reconstructed from
  candidate, convergence, transformation, correspondence, loss, repair, or
  advisory evidence
- native admission or identity cannot be reconstructed from source occurrence,
  transformation disposition, or matching correspondence fields

## Authority And Destination Topology

Milestone 9.15 uses the authority package graph frozen by Milestone 9.13.2. It
does not create a new aggregate Query package or restore the deleted monolith.
The smallest populated destination topology is:

```text
worth-query-declaration/
  domain_computation/
    correlated_path/
    resource_request/
    artifact_request/

worth-query-installation/
  domain_computation/
    artifact_contract/
    artifact_occurrence_contract/
    reproducibility_contract/
    candidate_search_contract/
    convergence_contract/
    transformation_evidence_contract/
    access_path_contract/
    invariant_executor_contract/
    structural_counter_contract/
    decision_record_contract/

worth-query-admission/
  domain_computation/
    artifact_admission/
    occurrence_substitution_admission/
    reproducibility_admission/
    candidate_search_admission/
    convergence_epoch_admission/
    access_product_admission/
    execution_resource_admission/
    provider_session_admission/
    attempt_admission/
    conflict_partition_admission/

worth-query-execution/
  domain_computation/
    artifact_owner/
    artifact_occurrence/
    reproducibility_comparison/
    candidate_search/
    convergence_epoch/
    access_product_owner/
    managed_run/
    provider_session/
    decision_read_set/
    provisional_attempt/
    invariant_execution/
    commit_attempt/
    realized_footprint/
    correlated_path_execution/
    partitioned_execution/

worth-query-publication/
  domain_computation/
    artifact_publication/
    occurrence_evidence/
    reproducibility_evidence/
    candidate_search_evidence/
    transformation_evidence/
    attempt_outcome/
    structural_cost/
    decision_attachment/
    boundary_envelope/

worth-query-certification/
  oracle/
    domain_computation/
      hostile_provider/
      hostile_domain/
      transactional_attempt/
      resource_lifecycle/
      access_product/
      partitioned_execution/
```

The exact leaf split may become finer when one listed responsibility contains
multiple independently owned mechanisms. It may not become flatter. Generic
`artifact.rs`, `provider.rs`, `session.rs`, `helpers.rs`, `common.rs`,
`shared.rs`, or `domain_computation.rs` catch-all implementations are forbidden.
The stable public facade remains the appropriate Query audience facade; internal
authority packages are not new consumer entry points.

## Phase Plan

### Phase 1: Installed Domain Artifact Contract

Freeze the portable declaration and installed semantic contract for values that
cross installed workflow stages without becoming Query projections or Query-
owned domain payloads.

**Relevant subsystems**

- `worth-query-installation` domain computation contracts
- Foundational schema, canonical identity, provenance, classification, and
  performance vocabulary
- `worth-proof` installation, basis, freshness, and runtime-affinity witnesses

**Relevant APIs**

- installed workflow value contract
- artifact occurrence and evidentiary-substitution contract
- installed reproducibility and comparison-authority contract
- installed candidate-search and convergence contract
- installed transformation-occurrence and loss-evidence contract
- installed operation workflow stage semantics
- installed package conflict and equivalence admission
- semantic replay comparator registration

**Required contract**

Every artifact family declares:

- stable semantic family identity and schema/protocol version
- semantic content identity, occurrence identity policy, and the purposes for
  which one occurrence may or may not substitute for another
- producer and permitted consumer stage roles
- payload owner and provider family
- canonical semantic projection used for comparison and inspection
- basis, provenance, dependency, invalidation, and equivalence requirements
- reproducibility class, determinism posture, comparison authority, and
  causally relevant environment or entropy dependencies
- candidate-universe, search-termination, completeness, feasibility,
  comparison, and optimality posture where the family presents alternatives
- progress measure, convergence comparator, incumbent posture, iteration bound,
  and oscillation classification where the family iterates
- immutable source occurrence, versioned transformation identity, source/output
  correspondence cardinality, disposition, and error/loss posture where the
  family transforms external or prior artifacts
- move, borrow, clone, provider-transfer, and serialization posture
- transient, arena-scoped, retained, reconstructible, or externally durable
  lifecycle
- required byte, element, and structural-work counters
- audience, classification, redaction, retention, deletion, and legal-hold
  posture
- supported compatibility window, migration owner, retirement rule, and
  downgrade posture

Installation rejects an artifact contract whose identity is caller-digest-
defined, whose schema is unversioned, whose comparator is absent for a reusable
family, whose ownership is ambiguous, or whose reconstruction posture would
make a derived payload authoritative.

**Warnings**

- Do not add a generic opaque payload variant without an installed contract.
- Do not serialize arbitrary domain products into `Text` or a projection row.
- Do not treat equal Rust type identity, equal bytes, or equal reporting digest
  as semantic artifact equivalence.
- Do not derive occurrence identity from content identity, provider output
  bytes, or retry identity.
- Do not let a cache hit, shared computation, or prior certificate satisfy a
  contract that requires a fresh occurrence or independent verification.
- Do not use bitwise equality as the universal reproducibility contract or
  accept an uninstalled tolerance, statistical test, or host comparator.
- Do not call a candidate unique, best, optimal, or Pareto-qualified without the
  installed search-universe, completeness, and comparison evidence required for
  that exact claim.
- Do not let a transformation record, similarity match, source locator, loss
  row, repair proposal, or selected candidate mint admission, identity,
  resolution, or publication authority.
- Portable contract declarations remain non-operational until installed by one
  runtime.

**Test requirements**

- Adversarial convergence test: equivalent artifact contracts authored in
  different order install to the same canonical meaning and semantic identity.
- Adversarial conflict test: same family/version with different payload owner,
  comparator, lifecycle, disclosure, or basis requirements fails installation
  atomically with no partial registry residue.
- Authority test: an artifact contract reconstructed from serialized fields,
  strings, or a copied digest cannot mint a runtime artifact handle.
- Evolution test: unsupported version, retired version, and ambiguous migration
  deny before provider contact with distinct typed outcomes.
- Occurrence-independence test: two equal semantic products from distinct
  admitted executions retain distinct occurrence evidence, while a consumer
  permitting computational substitution may reuse either through a new lease.
- Independent-verification denial test: cached or shared output cannot satisfy
  a fresh-execution or independent-checker requirement and denies before reuse.
- Reproducibility-class test: exact, canonical-reduction, domain-comparator,
  distributional, and non-replayable families admit only their installed
  comparison authority; copied labels and caller comparators mint no proof.
- Search-posture test: exhaustive, proven-top-k, bounded, sampled, heuristic,
  and incomplete searches can express only the installed optimality claims;
  changing universe, termination, comparator, or bound changes contract meaning.
- Transformation-authority denial test: copied source anchors, correspondence
  rows, loss evidence, or a domain repair proposal cannot mint native identity,
  authoritative admission, or a durable resolution decision.

**Engineering decisions**

- Artifact contract identity is owner-minted and canonicalized from semantic
  fields; it is never an external authorization token.
- Domain payload schema belongs to the domain. Query retains only the installed
  contract needed to govern carriage, lifecycle, comparison, and evidence.
- Query owns occurrence binding, substitution admission, and reproducibility-
  contract carriage. Domains own what counts as an independent occurrence and
  the semantic comparator or evidence relation.
- Query owns search/convergence posture carriage and transformation-evidence
  attachment. Domains own candidate universes, objectives, convergence meaning,
  repair meaning, loss classification, and source/native correspondence claims.
- Artifact contracts extend the 9.14 installed operation closure rather than
  creating a second domain registry.

**Open questions**

- None. Provider-specific zero-copy layout negotiation is Phase 3 and does not
  weaken this semantic contract.

### Phase 2: Managed Artifact Ownership And Workflow Carriage

Replace the closed primitive-only workflow edge with a proof-bearing,
runtime-affine managed artifact handle while preserving the existing primitive
and projection families for their honest use cases.

**Relevant subsystems**

- `worth-query-execution` artifact owner
- installed workflow progression and semantic trace
- managed lifecycle, replacement, rebind, disposal, and lease authorities from
  9.14

**Relevant APIs**

- workflow value and workflow value contract
- stage executor input/output material
- ordinary re-execution and cert-only replay semantic value
- workflow stage workspace and execution context

**Required lifecycle**

```text
InstalledArtifactContract
  -> ArtifactProductionAdmission
  -> RuntimeArtifactOwner
  -> MoveOnlyArtifactHandle
  -> BorrowedArtifactView | TransferredArtifactHandle
  -> RetainedArtifactLease | DisposedArtifact
```

The handle binds runtime, installation generation, operation/run/stage,
artifact family/version, payload owner, basis, provenance, dependency identity,
resource owner, retained bytes, and lifecycle generation. It exposes no public
payload downcast and cannot be copied, serialized as authority, restamped, or
rebound by matching fields.

Stage transition consumes the predecessor handle unless the installed contract
admits a named borrow or lease. Semantic traces carry handle identity plus the
canonical semantic projection and disposition, never the full payload by
default.

**Warnings**

- `Arc<dyn Any>`, `Box<dyn Any>`, caller-selected generic tags, and a global
  artifact map are forbidden.
- A clone is not a harmless ergonomic operation. It is an observer and retained-
  memory boundary with explicit counters and lifecycle.
- Dropping a consumer wrapper cannot silently leak or orphan the framework-owned
  payload.
- Artifact handles cannot be used as mutation, invariant, or commit authority.

**Test requirements**

- Adversarial mix test: handles from different runtime, generation, run, stage,
  basis, version, or owner deny before payload access or workflow progression.
- Lifecycle test: move, borrow, lease, replacement, cancellation, and disposal
  schedules leave exactly the declared owner/lease count and dispose the payload
  exactly once.
- Replay-honesty test: ordinary re-execution and cert replay compare canonical
  semantic projections without cloning payload bytes or accepting the original
  operational handle.
- Leak test: panic/failure at every stage handoff releases or retains the
  artifact exactly according to declared recovery posture.

**Engineering decisions**

- Query owns resource registration and handle minting; the provider owns payload
  allocation and destruction mechanics.
- Artifact identity remains distinct from publication identity, consumer lease
  identity, reporting identity, and semantic equivalence.
- The existing workflow value enum may gain a managed-handle family only through
  the installed contract and cannot become an open payload bag.

**Open questions**

- None.

### Phase 3: Bulk And Chunked Native Artifact Access

Provide execution-grade native consumption without scalar per-field affinity
checks, whole-projection cloning, or unconstrained pointer escape.

**Relevant subsystems**

- declaration-indexed native access from 9.14
- managed artifact owner and provider session
- result shaping and projection consumption

**Relevant APIs**

- native layout/access contracts
- artifact borrow admission
- stage workspace artifact reader
- chunk cursor and bounded sink contracts

**Required access families**

- typed borrowed row batch
- typed borrowed column or field slice where the provider admits it
- bounded chunk iterator with opaque continuation
- provider-native bulk projection into a declared destination layout
- explicit scalar fallback only when its complexity and call amplification are
  admitted

One access admission binds artifact handle, layout contract, basis, affinity,
borrow generation, requested fields, chunk bounds, alignment, lifetime, and
provider session. Returned views cannot outlive that admission or cross runtime,
thread, provider, or session boundaries unless a separately admitted transfer
contract permits it.

**Warnings**

- Do not expose raw stable pointers, unconstrained slices, or provider arenas
  through the public facade.
- Do not run one runtime-affinity lookup per scalar when one admitted batch can
  carry the proof.
- Do not call an eager full materialization "chunked" because the caller later
  iterates pages.
- Native layout compatibility is not semantic artifact equivalence.

**Test requirements**

- Bulk/scalar parity test: admitted bulk and scalar lanes return identical
  semantic values and basis evidence, while counters distinguish their physical
  work.
- Lifetime sabotage test: stale borrow generation, disposed owner, foreign
  thread/session, wrong layout, and escaped chunk view deny before access.
- Allocation test: chunk width controls actual peak scratch/result memory;
  reducing the width must reduce the measured resident peak rather than only the
  reported counter.
- Projection test: opaque/reference/content families either use a declared
  provider-native contract or deny typed; they never silently widen to cloned
  generic rows.

**Engineering decisions**

- The public API returns proof-bound views and chunks, not provider containers.
- Query verifies admission and accounting; provider certification proves the
  physical access path obeys the declared layout and lifetime.
- Bulk access belongs on the execution lane; consumer delivery still uses
  projection/publication contracts where those are the correct boundary.

**Open questions**

- None.

### Phase 4: Installed Structural Counter And Decision-Evidence Schemas

Let a domain declare the structural work and decision evidence necessary to
interpret its operation without letting counters or logs become authority.

**Relevant subsystems**

- installation contribution contracts
- canonical boundary envelope core and diagnostic sidecars
- decision-log and performance vocabulary

**Relevant APIs**

- stage and operation material
- execution counters and consumption-cost snapshots
- causal inspection and certification bundles

**Required schema split**

- required structural counter rows needed to substantiate declared cost
- optional high-cardinality counter rows
- required decision summary fields needed to interpret outcome/recovery
- policy-materialized decision-record sidecars
- required candidate-search summary fields needed to interpret completeness,
  feasibility, optimality, termination, and incumbent posture
- policy-materialized transformation and loss-evidence sidecars with a
  mandatory typed disposition summary

Counter schemas declare unit, aggregation law, monotonicity, scope, reset
boundary, required/optional status, and replay comparison posture. Decision
schemas declare kind, semantic reason family, affected artifact-key family,
causal-parent shape, payload version, classification, and retention posture.
Candidate-search schemas declare universe identity, candidates considered,
termination class, completeness class, feasibility class, comparison authority,
optimality class, rejected-candidate count, and incumbent disposition.
Transformation-evidence schemas declare source and output occurrence identity,
transformation contract/version, correspondence cardinality, preserved,
normalized, approximated, repaired, omitted, unsupported, or quarantined
disposition, error posture, and admission-authority absence.

**Warnings**

- Counter values do not prove authority or correctness.
- Free-form names, maps, labels, or JSON rows cannot substitute for an installed
  schema.
- Required cost rows cannot be dropped by diagnostic policy.
- High-cardinality decision payloads cannot be forced onto the ordinary hot
  path when policy omits them.
- Candidate lists and per-source transformation rows are sidecars; their
  mandatory summaries cannot overstate search completeness or erase loss.

**Test requirements**

- Schema convergence test: equivalent schemas install identically; unit,
  aggregation, requiredness, replay, or retention drift conflicts atomically.
- Counter honesty test: a provider omitting a required row, changing a
  monotonic counter backward, or reporting impossible aggregate relations fails
  receipt admission.
- Sidecar policy test: omitted optional decisions leave typed omission and
  preserve the mandatory interpretive/cost core.
- Security test: redaction, expiry, deletion, and legal-hold posture propagate
  through derived inspection and certification copies without leaking payload.
- Search-summary honesty test: omitted candidate sidecars preserve exact
  universe, termination, completeness, feasibility, optimality, and incumbent
  posture in the canonical core.
- Loss-summary denial test: redacted or omitted transformation details retain
  typed loss/disposition posture while neither full nor summarized evidence can
  authorize admission or repair application.

**Engineering decisions**

- Domains name meaningful work such as candidate comparisons or invariant
  neighborhoods; Query binds the installed schema to run/stage/basis and checks
  structural consistency.
- Search and loss summaries explain what was examined and transformed. They do
  not prove that an alternative is exhaustive, authorize one candidate, or
  admit transformed output as truth without the owning authority.
- Counter verification remains provider/domain certification, not Query
  reinterpretation of domain algorithms.
- Decision records explain why. Checkpoints and journals remain the state-
  reconstruction mechanism.

**Open questions**

- None.

### Phase 5: Execution Resource Request And Admission

Make resource posture part of the lowered installed operation plan before
provider allocation or domain computation begins.

**Relevant subsystems**

- Query admission and installed operation cost contracts
- graph-read access planning
- provider capability/support registry
- managed resource lifecycle

**Relevant APIs**

- operation and stage resource declarations
- admission decision lattice
- lowered workflow stage plan
- support and capability snapshots

**Required resource dimensions**

- independent semantic scale axes and admitted ceilings
- source rows/edges, visited/frontier state, candidate/work-item count
- scratch, transient, peak resident, retained, reclaimable, and output bytes
- allocation count and allocator/arena family
- provider contacts, messages, retries, barriers, and synchronization budget
- queue depth, concurrency width, chunk width, and fan-out
- deadline, cancellation polling interval, safe-point family, and cleanup budget
- partial-effect, yielded-state, degraded-state, and retained-progress posture

Admission chooses one named strategy and resource envelope. It may reject,
require a different provider/access product, require asynchronous execution, or
admit a named degradation. The executor cannot increase limits, change strategy,
or reinterpret the scale axes.

**Warnings**

- Categorical `small`/`medium`/`large` cost alone is insufficient.
- A single aggregate byte limit cannot hide transient, peak, retained, queued,
  and output memory.
- Deadline and cancellation configuration without named safe points is not a
  cancellation contract.
- Background execution cannot launder unbounded queue or retained-state cost.

**Test requirements**

- Rejection-order test: over-budget requests deny before artifact allocation,
  provider session preparation, graph traversal, or domain compute counters
  increment.
- Independent-axis test: vary model size, touched region, valence, candidate
  density, output width, and batch width independently and prove the declared
  counters respond only to causally relevant axes.
- Saturation test: arrival pressure reaches typed reject, backpressure, or named
  degradation before queue bounds are exceeded.
- Provider mismatch test: a provider lacking one required resource or safe-point
  capability cannot be selected through a generic fallback.

**Engineering decisions**

- Resource admission extends installed operation planning and support posture;
  it is not a host scheduler callback.
- Query owns policy and the bound envelope. Providers own physical capacity
  reporting and allocation mechanics.
- The admitted envelope is immutable for one attempt. A changed envelope
  requires a new attempt identity and admission.

**Open questions**

- None.

### Phase 6: Managed Run Lifecycle, Cancellation, Yield, And Resume

Introduce a framework-owned run resource whose states reflect actual execution
and retained progress rather than live-query lifecycle labels reused by analogy.

**Relevant subsystems**

- `worth-query-execution` managed run owner
- temporal/async result-state and continuation contracts
- artifact owner and resource envelope
- Store handoff for durable checkpoints

**Relevant APIs**

- stage executor context
- cancellation token/probe
- bounded chunk sink
- run continuation and cleanup receipts
- convergence-epoch admission and progress evidence

**Required state machine**

```text
AdmittedRun
  -> Running
  -> Completed
     | Yielded
     | Cancelled
     | Exhausted
     | Degraded
     | Failed

Yielded
  -> ResumptionAdmission
  -> ResumedRun

Cancelled | Exhausted | Failed
  -> CleanupComplete
     | CleanupPending
     | RecoveryRequired
```

Every transition records actual completed work, effects, owned/leased artifacts,
scratch and retained bytes, provider session state, continuation/checkpoint
identity where present, cleanup disposition, and recovery authority. Only a
runtime-minted continuation bound to the same installed contract and admitted
basis can resume.

An iterative domain operation may additionally advance:

```text
AdmittedConvergenceEpoch
  -> Iterating
  -> Converged
     | StableWithoutProof
     | FeasibleIncumbent
     | Oscillating
     | Exhausted
     | Cancelled
     | Indeterminate
```

The epoch binds one semantic-world basis, installed convergence comparator,
progress measure, candidate/incumbent family, iteration and resource budgets,
checkpoint posture, and repeated-state/oscillation evidence. Each iteration is
a bounded execution step; the domain supplies convergence meaning and Query
governs only progression, accounting, cancellation, and retained evidence.

**Warnings**

- Cancellation does not imply transaction abort unless the attempt phase proves
  abort completion.
- A yielded checkpoint is not durable merely because it is serializable.
- Polling a token only before and after one unbounded domain callback does not
  satisfy cooperative cancellation.
- Partial result delivery must not become authoritative publication.
- A caller loop over repeated resolver invocations is not a managed convergence
  epoch.
- `converged`, `stable`, `feasible`, `optimal`, and `exhausted` are distinct
  claims. Query may not infer one from another.
- This phase creates no durable conflict, approval, participant, or resolution-
  session lifecycle.

**Test requirements**

- Safe-point matrix: cancellation at every declared safe point produces the
  exact state, cleanup, retained-artifact, and effect posture required there.
- Resume parity test: uninterrupted and repeatedly yielded/resumed runs converge
  on equivalent semantic result, footprint, invariant, and structural-counter
  evidence.
- Stale continuation test: foreign runtime, provider, generation, basis,
  resource envelope, artifact version, or completed attempt denies before work.
- Leak/backpressure test: stalled consumers and cancelled producers leave no
  unbounded queue, orphan arena, provider session, artifact owner, or promotable
  partial result.
- Convergence-schedule parity test: admitted chunk widths, yields, resumes, and
  provider scheduling converge on equivalent domain result and epoch evidence
  when the installed comparator says they should.
- Oscillation/exhaustion test: repeated-state cycles, stalled progress,
  iteration exhaustion, cancellation, and indeterminate comparison remain
  distinct and preserve the exact incumbent and resource posture.
- Resolution-authority denial test: a converged or feasible candidate cannot
  become an admitted durable decision, approval, or publication merely through
  run or continuation authority.

**Engineering decisions**

- Query owns run/continuation authority and state transitions. Domains define
  checkpoint semantic payloads; Store owns durable survival and reload.
- Providers report actual safe-point work and retained resource state.
- Resumption is a new execution attempt joined to the original logical run, not
  identity reuse.
- Convergence is single-basis execution in this milestone. Rebase, participant
  roles, deferral, supersession, and durable decision recovery belong to the
  cross-runtime governed-resolution substrate.

**Open questions**

- None.

### Phase 7: Provider Execution Plan And Prepared Session

Turn graph/domain provider contact into a sealed execution protocol rather than
independent synchronous callbacks receiving weak call descriptions.

**Relevant subsystems**

- lower-runtime capability routing and boundary envelopes
- installed graph participation and commit authority
- execution resource admission
- Relational/runtime-bridge/provider facades

**Relevant APIs**

- graph provider and graph commit provider contracts
- lower-runtime bound basis
- bound graph invocation plan
- provider support/capability registration

**Required progression**

```text
AdmittedProviderExecutionPlan
  -> ProviderPlanReadmission
  -> PreparedProviderSession
  -> SessionBoundReadsAndEffects
  -> SessionPrepareOutcome
  -> SessionCommitOrAbortOutcome
```

The plan binds exact installed operation/stage, provider family/version,
runtime, graph authority, basis/snapshot, declared read/touch/effect closure,
resource envelope, artifact contracts, invariant contracts, expected transaction
posture, compensation/reconciliation posture, and canonical plan identity.

The provider returns a non-forgeable session token. Every staged read, artifact,
effect, invariant overlay, prepare, commit, abort, and receipt carries that
token's identity and generation. A provider receiving only a canonical-query
digest or free-form scope label cannot satisfy this lane.

**Warnings**

- Commit admission is not prepare, commit, or atomicity proof.
- Query must not invent a distributed transaction where authorities cannot
  supply one.
- A provider receipt string that claims to bind a call is not sufficient
  session authority.
- Provider sessions cannot be hidden behind a property-like synchronous API.

**Test requirements**

- Phase-order test: effect, invariant, prepare, commit, and abort calls without
  the exact predecessor session state are uncallable or deny before provider
  work.
- Token substitution test: copied fields, colliding digests, foreign provider
  tokens, stale generation, or cross-attempt session mixing cannot progress.
- Plan-honesty test: provider execution observes the sealed plan; any attempt to
  use a different basis, strategy, resource envelope, or closure is localized.
- Failure matrix: failure before preparation, after session preparation, during
  staged work, during prepare, during commit, and during abort returns distinct
  typed state and recovery posture.

**Engineering decisions**

- Query owns the protocol and phase authority; provider implementations own
  physical sessions and transaction mechanics.
- Same-authority atomicity is admitted only from a provider capability that
  proves the required session lifecycle.
- Multi-authority work remains compensated/reconciled unless one genuine shared
  commit authority supplies the session.

**Open questions**

- None. Physical two-phase or provider-specific protocols remain provider/Store
  implementation choices behind this contract.

### Phase 8: Basis-Complete Decision Read-Set

Capture every fact that influenced domain planning so compare-and-commit can
reject relevant drift without conflicting on unrelated truth.

**Relevant subsystems**

- graph read declarations and access plans
- scoped basis authority
- dependency-impact compilation
- provider session reads

**Relevant APIs**

- installed read family execution
- graph-read access-plan consumption
- mutation verified-target/read-set preconditions
- structural fact and evidence locators

**Required dependency families**

- observed entity, relation, aspect, field, and value facts
- absence and non-membership facts
- predicate and comparison outcomes
- ordering and selected-extremum facts
- cardinality, uniqueness, and ownership facts
- traversal frontier, visited closure, bounded exhaustion, and path witnesses
- access-product coverage/membership basis
- artifact semantic projections consumed in planning
- domain-declared structural proofs used to choose effects

The complete set is canonical and order-independent. Each fact binds the exact
basis/snapshot and provider session. The domain may identify which facts
influenced its decision only through installed fact families that Query can
bind, compare, and include in the attempt; it may not return arbitrary invalidation
callbacks or raw hashes.

**Warnings**

- Existing-target equality preconditions alone are not a complete decision
  read-set.
- Recording rows returned by a query omits absence and negative-space
  membership.
- A broad snapshot-generation conflict is correct but not scalable when exact
  decision dependencies are available.
- Runtime discovery may narrow declared read authority but cannot authorize an
  undeclared read.

**Test requirements**

- Relevant-drift test: mutate each positive, negative, predicate, cardinality,
  ordering, frontier, and membership dependency between planning and commit;
  each attempt returns typed stale/replan before effects commit.
- Unrelated-drift test: mutate unrelated entities, aspects, partitions, and
  artifact families; the attempt commits with identical semantic output and
  exact zero false-conflict counters.
- Completeness sabotage test: omit one decision fact from provider/domain
  reporting and prove attempt preparation denies rather than silently accepting
  an incomplete set.
- Canonicality test: alternate traversal, chunk, and parallel discovery order
  yields the same canonical decision read-set identity.

**Engineering decisions**

- Query owns fact binding, canonicalization, freshness comparison, and attempt
  integration. Domains own which admitted semantic facts influence their
  algorithms.
- Read-set receipts remain evidence until consumed by a prepared attempt; they
  cannot independently mint mutation authority.
- Comparison uses exact lower-authority fact/version evidence where available,
  not human-readable values or Query-restamped digests.

**Open questions**

- None.

### Phase 9: Provisional Graph Attempt And Proposed Post-State

Create a managed speculative world in which a domain can stage, inspect, and
revise one proposed graph program without mutating authoritative truth.

**Relevant subsystems**

- graph composition and symbolic same-batch references
- preview/basis lifecycle
- provider session and decision read-set
- artifact and managed run owners

**Relevant APIs**

- graph composition declaration and lowering
- stage mutation execution context
- preview/speculative basis
- proposed-state read and inspection
- basis-bound candidate or repair proposal inspection

**Required progression**

```text
PreparedProviderSession
  + BasisCompleteDecisionReadSet
  + LoweredProvisionalEffectProgram
  -> ProvisionalAttempt
  -> ProposedPostState
  -> ProposedStateInspection
  -> RevisedProvisionalAttempt | InvariantExecutionAdmission
```

The provisional attempt binds one provider session, basis, read-set, declared
touch/effect closure, symbolic identities, effect order, artifact dependencies,
resource envelope, attempt generation, and cleanup/discard authority. Reads
against the proposed state explicitly distinguish authoritative base facts,
staged replacements, staged creations, staged retirements, and derived
provisional views.

A domain candidate or repair proposal may lower into the provisional effect
program only while retaining its exact search occurrence, candidate identity,
source/transformation evidence where applicable, semantic-world basis, expected
target generation, installed policy, and identity-consequence map. Proposal
inspection is not approval. The owning entry/Relational authority must still
admit the effect program and validate the proposed post-state.

**Warnings**

- A preview branch label alone is not a proposed-state transaction overlay.
- Several ordinary `execute_mutation` calls are not one provisional atomic
  program.
- Domain code cannot hold a raw mutable graph or provider transaction object.
- Discard must revoke every promotion path and clean every provisional resource.
- A selected candidate, repair suggestion, transformation record, loss ledger,
  or advisory response is not a durable resolution command.
- This phase may reject a stale single-basis proposal; it may not invent branch-
  aware carry-forward or resolution-session state.

**Test requirements**

- Visibility test: proposed-state reads observe all and only the staged changes
  in canonical effect order while ordinary readers continue to observe the
  authoritative basis.
- Discard test: discard at every provisional stage leaves exact zero
  authoritative mutations, publications, lineage promotions, live resources,
  provider sessions, and promotable artifacts.
- Generation test: a stale proposed-state view or artifact cannot inspect,
  revise, validate, or promote a newer attempt generation.
- Revision parity test: a revised provisional program that is semantically
  equivalent to direct construction yields the same proposed-state identity and
  later invariant input.
- Proposal-basis test: changing source occurrence, candidate-search occurrence,
  policy, target generation, correspondence, or identity consequence invalidates
  exactly the affected proposal before provisional mutation.
- Suggestion-authority test: a provider-marked recommendation or caller-selected
  candidate cannot bypass entry admission, invariant execution, or commit.

**Engineering decisions**

- Query owns attempt lifecycle and authority. Relational/provider owns overlay
  mechanics; domain owns effect-program meaning.
- Provisional identity is explicitly non-authoritative and cannot enter ordinary
  publication or consumer projection lanes.
- Durable acceptance, rejection, deferral, supersession, participant approval,
  and carry-forward are intentionally absent; they consume the cross-runtime
  conflict/resolution authority when that substrate exists.
- Durable speculative checkpoints remain Store scope.

**Open questions**

- None.

### Phase 10: Real Domain Invariant Execution

Replace selection-backed success with actual domain/provider invariant execution
over the exact proposed post-state.

**Relevant subsystems**

- graph touch obligation selection from 9.9
- installed invariant contracts from 9.13/9.14
- Relational commit invariant authority
- proposed-state provider session

**Relevant APIs**

- graph obligation executor registration
- invariant execution provider
- state-load/access-plan admission
- workflow invariant outcome

**Required progression**

```text
SelectedInstalledInvariant
  + ProposedPostState
  + AdmittedInvariantStateLoadPlan
  -> BoundInvariantExecution
  -> PassedInvariantReceipt
     | AdvisoryInvariantReceipt
     | ViolatedInvariantReceipt
     | IndeterminateInvariantReceipt
     | ExhaustedInvariantReceipt
```

The receipt binds installed invariant family/version, domain executor/provider,
attempt and proposed-state generation, base basis, exact state loaded, access
plan, structural counters, verdict, affected scope, diagnostic disposition, and
provider/Relational execution evidence. Only passed blocking invariants and
admitted advisory outcomes can progress according to the installed contract.

**Warnings**

- `selected-for-execution`, registered support, or a zero-row state-load plan
  cannot produce a passed invariant receipt.
- A successful generic commit or mutation receipt cannot be reinterpreted as a
  particular domain invariant verdict.
- Query does not implement loop closure, manifoldness, geometric tolerance, or
  any other domain invariant.
- Validators cannot read outside their admitted state-load closure.

**Test requirements**

- Sabotage matrix: inject a single post-state violation for each registered
  blocking invariant and prove the exact invariant denies before prepare/commit.
- Execution proof test: a fake provider returning selected/support evidence
  without state load and validator execution cannot mint a passed receipt.
- Full/incremental parity test: admitted regional/incremental certificates and
  independent full recomputation agree across valid and corrupted worlds.
- Budget test: invariant state load or execution exhaustion yields typed
  exhausted/indeterminate outcome, never pass or a generic mutation failure.

**Engineering decisions**

- Query owns selection, binding, state-load admission, progression, and public
  outcome. Domain owns validator and verdict semantics; Relational/provider owns
  authoritative proposed-state access and commit enforcement.
- If Relational executes the invariant at commit, its exact invariant receipt
  is readmitted rather than replaced by a generic write receipt.
- Regional invariant certificates are derived artifacts governed by Phases 1,
  12, 13, 14, and 18.

**Open questions**

- None.

### Phase 11: Compare-And-Commit, Abort, Partial Effect, And Indeterminate Outcome

Bind decision freshness, proposed effects, invariant receipts, and provider
session into one commit attempt with honest failure topology.

**Relevant subsystems**

- Relational transaction/MVCC authority
- provider prepared session
- decision read-set freshness
- reversal/compensation/recovery posture from 9.14

**Relevant APIs**

- prepared commit call and token
- compare-and-commit provider
- abort and reconciliation provider
- canonical mutation/publication envelope

**Required progression**

```text
PreparedProviderSession
  + FreshDecisionReadSet
  + ProposedPostState
  + CompleteRequiredInvariantReceipts
  -> PreparedCommitAttempt
  -> CommittedAttempt
     | StaleAttempt
     | AbortedAttempt
     | PartialEffectAttempt
     | IndeterminateAttempt
```

A committed attempt contains one provider/Relational canonical commit artifact
binding all declared effects, exact read basis, freshness comparison, invariant
receipts, realized footprint, lineage, artifact disposition, publication
inputs, structural cost, and session identity. Atomicity is claimed only when
the authoritative provider proves all effects committed as one transaction.

`PartialEffectAttempt` is permitted only for declared escaping effects and names
completed effects, unperformed effects, compensation/reconciliation authority,
and remaining risk. `IndeterminateAttempt` means commit state is unknown and
requires authoritative reconciliation; it cannot be retried blindly.

**Warnings**

- Do not call `admit_commit` and then sequentially call touch providers under an
  atomic label.
- Do not assemble an atomic receipt from several independently successful
  mutation receipts.
- Do not retry an indeterminate commit with the same logical effects unless the
  provider proves idempotent reconciliation.
- Compensation is not rollback and never upgrades a multi-authority operation
  to atomic.

**Test requirements**

- Interleaving matrix: relevant drift after read, after proposed-state build,
  after invariant execution, and before prepare returns stale/replan; unrelated
  drift commits.
- Atomic sabotage test: provider failure after each staged effect produces
  either provider-proven abort with zero authoritative residue or a typed
  partial/indeterminate outcome; it never reports atomic success.
- Receipt substitution test: invariant, read-set, proposed-state, or session
  receipts from another attempt cannot prepare or commit even with colliding
  reporting fields.
- Reconciliation test: lost commit response resolves through provider truth to
  committed or aborted exactly once, and duplicate retry cannot double-apply.

**Engineering decisions**

- Query owns attempt phase types, required joins, outcome topology, and public
  envelope. Relational/provider owns authoritative transaction result.
- Commit success produces one canonical artifact; publication and downstream
  evidence derive from it.
- Multi-provider same-authority execution requires one shared commit session.
  Otherwise the installed operation declares compensation/reconciliation.

**Open questions**

- None.

### Phase 12: Managed Domain Access Product

Close the gap between an access plan that says a capability is required and the
derived provider product that actually satisfies it.

**Relevant subsystems**

- graph-read access planning from 9.10
- artifact owner and lifecycle
- provider capability registry
- sharing, compatibility, and invalidation from 9.14

**Relevant APIs**

- domain operation capability registration
- persistent/ephemeral index requirement
- access-product build and lookup provider
- access-plan consumption receipt

**Required access-product contract**

- domain predicate/access-path family and version
- physical provider/strategy identity and profitable regime
- declared source facts, basis, and compatibility/equivalence basis
- conservative candidate-completeness contract
- exact-refinement family and false-positive posture
- coverage/membership witness family
- build, lookup, update, refit, rebuild, eviction, and disposal capabilities
- fallback and capability-unavailable posture
- resident, transient, maintenance, lookup, amplification, and coordination
  cost contracts
- sharing and exclusivity posture

The access product is a managed derived artifact. Query admits and tracks it,
binds it into access plans, and retains its dependency/lifecycle evidence. The
provider/domain chooses and executes BVH, R-tree, hash, adjacency, continuation,
or other physical mechanisms.

**Warnings**

- `persistent_index_required` without a product lifecycle is an incomplete
  execution contract.
- Query must not choose a universal index family or learn domain predicate
  semantics.
- A caller-owned cache or local lookup table cannot satisfy a Query access-plan
  requirement.
- Destroy-and-rebuild cannot be the only posture when an installed provider
  claims incremental maintenance.

**Test requirements**

- Strategy parity test: two admitted physical strategies produce equivalent
  candidate completeness and exact-refined semantic results with distinct
  honest cost evidence.
- Lifecycle test: build, share, invalidate, update/refit, rebuild, evict, and
  dispose preserve one owner and leave no stale lookup authority.
- Capability test: missing build, membership, maintenance, budget, or exact-
  refinement capability yields the named required-capability posture before
  lookup.
- Rebuild test: destroying every access product and rebuilding from authority
  reproduces semantic results without preserved hidden truth.

**Engineering decisions**

- Access-product identity, artifact identity, access-plan identity, and semantic
  query identity remain distinct.
- Store owns durable bytes and restart-stable product restoration; Query owns
  the portable/runtime semantic contract and runtime-backed lifecycle.
- Bounded ephemeral products use the same contract with a shorter lifecycle,
  not a weaker untracked lane.

**Open questions**

- None.

### Phase 13: Coverage And Membership Witnesses

Make negative-space dependence explicit so access-product reuse and incremental
maintenance cannot be justified only by rows previously returned.

**Relevant subsystems**

- domain access products
- Query dependency impact and invalidation
- Relational truth-delta publication
- region/partition-aware live narrowing

**Relevant APIs**

- access-product coverage provider
- membership dependency token
- impact classifier
- re-execution/rebuild admission

**Required witness meaning**

A coverage witness binds:

- access product and provider generation
- exact source basis and applicable region/partition universe
- domain predicate/access-path family
- completeness posture and false-negative guarantee
- admitted widening/approximation and exact-refinement requirement
- source membership/version facts sufficient to detect entry, exit, or motion
- invalidation granule and conservative fallback
- expiry, rebind, rebuild, and unavailable posture

The witness never claims domain truth. It proves only that the provider's
candidate universe is complete for the declared family and basis under the
installed contract.

**Warnings**

- Dependencies on current result rows cannot detect a new member entering from
  outside the result.
- Arbitrary provider invalidation callbacks are not membership witnesses.
- A bounding-region label or partition ID without authority-bound version and
  completeness posture is insufficient.
- Approximate candidates may admit false positives only when exact refinement is
  mandatory before semantic result or effect.

**Test requirements**

- Entry/exit/motion test: insert, delete, and move an entity across the covered
  membership boundary; impact chooses exact maintenance or conservative
  re-execution without a false negative.
- Negative-space sabotage test: a provider that tracks only returned-row
  dependencies is rejected by certification.
- Approximation test: declared candidate widening preserves exact refined
  results while undeclared false negatives fail the provider oracle.
- Expiry test: stale, foreign, partially rebuilt, or coverage-unknown witness
  cannot support reuse, commit, or incremental maintenance.

**Engineering decisions**

- Domain/provider mints the coverage proof under its installed contract; Query
  binds, retains, and invalidates it.
- When local impact cannot be proven, Query escalates to typed re-execution or
  rebuild. It does not guess local safety.
- Membership witnesses feed both decision read-sets and realized dependency
  footprints.

**Open questions**

- None.

### Phase 14: Realized Footprint Subset Proof And Dependency Integration

Retain the exact data-dependent work actually consumed while preserving the
predeclared authority closure as a hard upper bound.

**Relevant subsystems**

- declared graph read/touch/effect contracts
- decision read-set
- dependency-impact compilation
- sharing, invalidation, reuse, and publication

**Relevant APIs**

- provider/domain realized-footprint reporter
- subset verifier
- canonical footprint receipt
- dependency-role compiler

**Required footprint families**

- exact consumed entity/relation/aspect/field locators
- negative, predicate, ordering, cardinality, closure, and membership facts
- artifact/access-product dependencies
- exact staged and committed touch/effect locators
- provider partitions and structural proof dependencies
- declared-but-unrealized residue counts

The footprint canonicalizes independently of discovery order. Query verifies
each element against the installed declared closure before accepting it. The
accepted footprint can narrow reuse, invalidation, conflict, and incremental
maintenance but never expand execution authority.

**Warnings**

- Do not use only the declared upper bound for dependency impact when a verified
  exact footprint exists.
- Do not let the provider report opaque hashes or callbacks instead of admitted
  fact locators.
- Footprint omission cannot silently mean the whole declared closure; absence is
  a typed unsupported posture with conservative consequences.
- A realized footprint is evidence, not a new read/touch capability.

**Test requirements**

- Subset sabotage test: provider reports one undeclared read, artifact,
  membership region, touch, or effect; Query denies before that widened action
  can influence commit or publication.
- Order parity test: serial, chunked, resumed, and parallel discovery produce
  the same canonical footprint and dependency-impact result.
- Narrowing test: local operations invalidate/recompute proportional to exact
  realized scope rather than unrelated model or component size.
- Conservative fallback test: unavailable exact footprint causes named broad
  invalidation/re-execution with explicit cost, not false precision.

**Engineering decisions**

- Query owns subset verification, canonicalization, and integration. Domains own
  the semantic reason each admitted fact was consumed.
- Declared closure and realized footprint remain separate artifacts with
  separately reported breadth.
- The canonical commit envelope binds both, permitting certification to detect
  scope leakage and under-reporting.

**Open questions**

- None.

### Phase 15: Correlated Heterogeneous Path Programs

Support provider-planned graph neighborhoods that cross mixed relation families
without whole-row materialization and host-local path reconstruction.

**Relevant subsystems**

- Query declaration and validation
- graph-read access planning
- installed domain operation graph reads
- provider access products

**Relevant APIs**

- relation traversal expression
- reusable read family
- graph read access shape and requirement set
- correlated result shape

**Required path algebra**

- forward and reverse steps over admitted relation families
- alternation/union with typed branch results
- bounded repetition and bounded closure
- correlated captures and joins across path steps
- uniqueness, existence, absence, cardinality, and closure assertions
- path-local predicate and projection
- explicit anchor, frontier, ordering, and result grouping
- provider-backed access-plan/index admission and typed unsupported posture

The algebra describes generic graph structure. Domain packages bind legal
relation roles and typed result contracts; Query never learns that one path
means a shell boundary, radial ring, component neighborhood, routing corridor,
netlist cone, or compiler dependency chain.

**Warnings**

- Do not provide arbitrary host closures for path evaluation.
- Do not materialize every entity/relation row and resolve joins locally on a
  covered path.
- Unbounded transitive closure and regex-like path languages without cost
  contracts are prohibited.
- Alternation cannot erase branch-specific cardinality or failure meaning.

**Test requirements**

- Provider/local oracle parity: provider-backed path execution matches an
  independent small-world oracle across mixed directions, alternation,
  repetition, capture, absence, and cardinality.
- No-local-drain test: covered reference-domain paths report exact zero whole-
  graph materialization and host-local join scans.
- Bound test: excessive depth/frontier/result width returns typed required
  streaming/access-product/async posture or denial before traversal.
- Ambiguity test: uniqueness, closure, and cardinality violations localize to
  the exact path clause rather than returning a partial correlated result.

**Engineering decisions**

- Canonical path meaning belongs to Query declaration; relation semantics remain
  installed-domain/Relational authority.
- Execution consumes the graph-read access plan and provider session.
- This phase extends existing traversal families and cannot create a parallel
  domain path engine.

**Open questions**

- None.

### Phase 16: Conflict Partitions And Set-Oriented Installed Execution

Make high-cardinality operation orchestration bulk-native and proof-driven
without quadratic pair enumeration or domain-unsafe identity heuristics.

**Relevant subsystems**

- installed workflow and parallel-frontier admission
- lower Relational/runtime-bridge bulk execution
- decision read-sets and realized footprints
- provider sessions and resource envelopes

**Relevant APIs**

- installed operation batch declaration
- conflict-key/disjointness proof registration
- partition plan and canonical reduction
- per-partition outcome/evidence

**Required batch progression**

```text
BoundOperationSet
  -> DeclaredConflictClosure
  -> DomainConflictEvidence
  -> VerifiedConflictPartitions
  -> AdmittedPartitionExecutionPlan
  -> PartitionedExecution
  -> CanonicalReducedOutcome
```

Domain conflict evidence may use installed keys, regions, partitions, or
disjointness certificates. Query verifies that evidence against read/touch/
effect closures, resource limits, and provider capability. Different target IDs
alone never prove independence. Dynamic work-item expansion is bounded,
canonically keyed, and admitted under the parent operation's authority.

**Warnings**

- Do not loop over scalar installed operations outside Query when the semantic
  request is one set operation.
- Do not require explicit all-pairs independence coverage.
- Do not infer spatial/topological disjointness from representation or identity.
- Parallel execution cannot change effect order, conflict meaning, outcome
  reduction, artifact identity, or evidence canonicalization.

**Test requirements**

- Serial/parallel parity test: reordered inputs, partition widths, worker counts,
  yields, and retry schedules produce equivalent semantic outcomes, footprints,
  effects, and canonical evidence.
- Hostile overlap test: same target ID with disjoint proven scope may admit;
  different IDs with hidden overlapping scope must conflict. Identity heuristics
  cannot decide either case.
- Scale test: partition admission and verification avoid quadratic pair counts
  and expose slope across batch width, actual conflicts, partitions, and
  coordination depth.
- Partial partition failure test: one denied, stale, cancelled, exhausted, or
  indeterminate partition produces the declared batch atomic/partial/recovery
  posture without laundering successful partitions.

**Engineering decisions**

- Query owns batch authority, partition verification, scheduling admission, and
  canonical reduction. Lower runtimes own bulk physical execution; domains own
  conflict/disjointness proof meaning.
- Static workflow-stage parallelism and dynamic set partitioning remain distinct
  mechanisms under one installed operation lifecycle.
- Cross-partition artifact sharing follows managed artifact and access-product
  ownership, not global caches.

**Open questions**

- None.

### Phase 17: Domain Decision Attachments And Boundary Evidence

Attach domain reasoning to the Query run/stage/attempt evidence graph without
promoting diagnostic payloads into state reconstruction or execution authority.

**Relevant subsystems**

- installed decision-record schemas from Phase 4
- causal inspection and decision logs
- canonical boundary envelopes
- privacy/redaction/retention policy

**Relevant APIs**

- stage material and attempt outcome
- decision attachment sink
- causal-parent index
- incremental decision summary
- candidate-search and transformation-evidence attachment

**Required attachment**

Each decision record binds installed schema/version, domain decision kind,
semantic reason family, affected artifact keys, domain payload reference or
typed omission, run/stage/attempt/basis, causal parent, sequence/order posture,
classification, redaction, retention, and integrity evidence. Query supplies
canonical attachment and indexing; domain supplies semantic payload.

Candidate-search attachments additionally bind universe, search occurrence,
termination, completeness, feasibility, comparator, optimality, incumbent, and
rejection-summary posture. Transformation attachments bind immutable source
occurrence, transformation contract/version, source/output correspondence
cardinality, domain disposition and loss/error posture, and any proposed
identity consequence. These attachments remain derived explanations.

The required incremental summary permits O(1) lookup by decision identity and
bounded lookup by affected artifact key without forcing full sidecar
materialization. The boundary core names sidecar presence, omission reason,
classification, retention, and summary identity.

**Warnings**

- A decision log explains why; it cannot reconstruct authoritative state or
  replace checkpoints and journals.
- Free-form prose is rendering, not canonical decision meaning.
- Diagnostic materialization cannot widen data or artifact authority.
- High-cardinality logs cannot be mandatory ordinary-path payload.
- A displayed candidate, recommendation, transformation, correspondence, or
  repair explanation cannot become selection, admission, identity, or
  publication authority.
- Durable participant decisions and resolution-session journals are not decision
  sidecars; they belong to their cross-runtime authoritative owners.

**Test requirements**

- Causal localization test: one injected domain decision divergence localizes to
  exact run, stage, attempt, basis, causal parent, and affected artifact without
  scanning unrelated records.
- Policy test: full, summarized, redacted, omitted, expired, deleted, and legal-
  hold sidecars preserve the same mandatory outcome/authority/recovery/cost
  core.
- Authority test: serialized decision rows, summaries, or matching affected
  artifact keys cannot authorize replay, mutation, artifact access, or commit.
- Evolution test: old and new decision schemas coexist or migrate
  deterministically with provenance; unsupported versions deny before attach.
- Candidate/loss localization test: one altered universe, termination,
  comparator, source occurrence, correspondence, or loss disposition localizes
  to the exact attachment and cannot silently preserve an old claim.
- Resolution-substitution denial test: matching attachment payload, candidate
  identity, or rendered recommendation cannot satisfy an admitted resolution
  command or recovered session decision.

**Engineering decisions**

- Query owns causal attachment, indexing, policy enforcement, and public
  inspection shape. Domain owns decision vocabulary and explanation rendering.
- Durable decision-log storage is a Store handoff; runtime-backed attachment is
  complete without claiming restart survival.
- Query 9.15 owns single-run evidence only. The cross-runtime roadmap owns
  durable conflict/resolution decision state, participant authority,
  supersession, carry-forward, checkpoint, and journal semantics.
- Certification uses decision attachments for divergence localization, not as
  an oracle derived from the implementation under test.

**Open questions**

- None.

### Phase 18: Artifact Reuse, Incremental Maintenance, Eviction, And Rebuild

Permit reuse of expensive stage/subartifact products only after their artifact,
access-product, membership, and footprint contracts can justify sameness.

**Relevant subsystems**

- 9.14 compatibility, sharing, leases, dependency impact, and invalidation
- managed artifacts and access products
- coverage witnesses and realized footprints
- resource admission

**Relevant APIs**

- artifact reuse admission
- occurrence-substitution admission
- reproducibility-class comparison admission
- stage/subartifact equivalence provider
- incremental maintenance plan
- eviction/rebuild/disposal outcome

**Required reuse basis**

- installed operation and stage semantic identity
- artifact/access-product family and version
- exact semantic input basis and canonical dependency order
- domain comparator/equivalence contract
- occurrence-substitution policy and any fresh-execution, independent-provider,
  or independent-verification requirement
- installed reproducibility class and comparison authority
- realized footprint and coverage/membership witness
- provider/runtime compatibility
- retained-memory and ownership budget
- incremental maintenance versus rebuild profitable regime
- staleness, eviction, disposal, and recovery posture

Reuse produces a new disposable lease over a managed owner, not a clone of
execution authority. Incremental maintenance yields a new generation and exact
maintenance footprint; it cannot mutate an artifact beneath consumers of an
older semantic generation unless the installed contract explicitly provides a
snapshot-safe persistent representation.

**Warnings**

- Do not add a general cache API.
- Equal artifact type, stage name, reporting digest, or payload bytes are
  insufficient equivalence.
- Computational equivalence does not imply evidentiary substitution. Reuse
  cannot collapse distinct required occurrences or manufacture independent
  verification.
- A comparison performed under the wrong reproducibility class, environment
  basis, entropy stream, reduction contract, or domain comparator denies reuse.
- Eviction cannot destroy authoritative state or invalidate active handles
  without typed lifecycle transition.
- Background maintenance queues remain bounded and accounted.

**Test requirements**

- Reuse parity test: fresh construction, shared reuse, incremental maintenance,
  eviction/rebuild, and cert re-execution yield equivalent semantic projections
  and exact dependency meaning.
- Equivalence sabotage test: one basis, comparator, footprint, membership,
  version, provider, or lifecycle difference denies reuse before owner/lease
  mutation.
- Occurrence-substitution sabotage test: an equal cached artifact is reusable
  for a consumer permitting computational substitution but denies for a
  consumer requiring a fresh occurrence or independent verification.
- Reproducibility sabotage test: byte-different outputs admit reuse only through
  their installed canonical-reduction, bound, domain-comparator, or
  distributional contract; byte equality cannot bypass a conflicting
  environment or occurrence basis.
- Density crossover test: incremental and rebuild strategies expose their
  profitable regimes and transition by policy with measured total work.
- Eviction schedule test: arbitrary lease disposal and memory pressure evict
  exactly when permitted, preserve active generations, and leave no orphan
  resource or hidden authority.

**Engineering decisions**

- This phase extends 9.14 shared execution rather than creating a domain cache
  registry.
- Query owns admission, owner/lease lifecycle, dependency integration, and cost
  evidence. Domain/provider owns comparator, maintenance algorithm, and physical
  storage.
- Query preserves semantic identity, occurrence identity, and certification
  identity as separate bindings throughout reuse and lease minting.
- Reuse is optional optimization. Destroying all reusable products preserves
  complete reconstruction from authority.

**Open questions**

- None.

### Phase 19: Public Facade, DX, And Reference-Domain Cutover

Expose one declarative ordinary journey and prove it by deleting the legacy
side-channel machinery from representative domain paths.

**Relevant subsystems**

- `worth-query-decl` and `worth-query-host` audience facades
- installed operation/workflow DX
- reference-domain consumer kit
- documentation and AI orientation

**Relevant APIs**

- domain package installation
- workflow artifact declaration and stage executor context
- managed run/attempt handle
- access-product registration
- result, receipt, and inspection journeys

**Required DX transcript**

The ordinary consumer can:

1. install domain artifact, access-path, invariant, counter, and decision
   contracts with its operation package
2. declare one operation/workflow and resource posture
3. execute through a managed run and provider session
4. consume/produce typed artifacts without local stores or payload bags
5. construct and inspect a provisional post-state
6. receive real invariant outcomes and one commit/abort/recovery result
7. inspect footprint, membership, structural cost, and policy-governed decisions
8. use set-oriented execution without local scalar scheduling

The first hostile adoption is one realistic planar-boolean workflow. The second
is a chip/netlist cone recomputation and incremental update that carries a typed
compiled-cone artifact, traverses mixed dependency relation families, observes
membership entry/exit, partitions a set-oriented update, and distinguishes
relevant from unrelated concurrent drift. The second path is mandatory proof
that the substrate is domain-agnostic rather than geometry vocabulary in
generic clothing.

The third hostile adoption is a research-style multi-site observation and
analysis workflow, with a biological workload as the reference fixture. It must
retain two byte-identical independent observations as distinct occurrences,
refuse to let cached computation satisfy an independent-evidence obligation,
and admit byte-different resumed or alternate-provider analysis outputs only
through an installed domain comparator, bound, or distributional equivalence
contract. Query must gain no specimen, assay, cohort, statistical, or biology
vocabulary.

**Warnings**

- The facade must not expose internal phase constructors or raw provider tokens.
- Domain-friendly builders cannot guess budgets, lifecycle, equivalence,
  transaction, or artifact policy.
- Adoption is deletion, not wrapping: local artifact stores, transaction
  packets, stage ledgers, access-product lifecycle catalogs, invalidation
  mirrors, and decision-log attachment plumbing must disappear on covered paths.
- Reference fixtures must build realistic worlds rather than hard-code identity
  receipts or fabricate invariant outcomes.

**Test requirements**

- Golden transcript test: geometry and non-geometry consumers complete the
  journey through the audience facade with exact semantic/evidence assertions
  and no internal Query or lower-runtime imports.
- Research-domain transcript test: the multi-site reference completes the same
  artifact/run/reuse lifecycle while preserving occurrence independence and
  declared non-bitwise reproducibility without Query-owned research semantics.
- Adoption residue test: covered reference paths contain zero local workflow
  artifact bags, provider-session emulation, invariant-pass fabrication,
  access-product lifecycle mirrors, scalar batch loops, or Query-authority
  reconstruction.
- DX denial test: every missing contract or unsupported provider capability
  yields a typed next action before expensive work, not a panic or generic
  string.
- Fixture honesty test: relevant drift, invalid post-state, cancellation,
  resource exhaustion, access membership change, and commit uncertainty arise
  from realistic world setup and real provider behavior.

**Engineering decisions**

- The public facade remains capability-oriented; operational phases stay
  internal and compiler-ordered.
- Reference adoption may require causally necessary changes in lower providers
  or shared vocabularies; those changes are part of honest closure.
- The non-geometry reference must exercise artifacts, resources, transaction
  attempts, and footprints rather than a trivial scalar workflow.
- The research reference is a hostile domain-agnosticity proof, not an
  invitation to add biology or statistical policy to Query. Its domain package
  owns occurrence independence and comparison meaning.

**Open questions**

- None.

### Phase 20: Mechanical Prohibitions And Provider-Independent Certification

Close the milestone with enforcement and hostile oracles that distinguish
semantic correctness from provider self-reporting.

**Relevant subsystems**

- `worth-query-certification`
- boundary-check and audience-facade enforcement
- provider-independent Milestone 13 oracle handoff
- support/profile and documentation surfaces

**Relevant APIs**

- domain computation certification bundle
- hostile provider/domain fixtures
- facade snapshots and compile-fail boundaries
- structural counter and residue reports

**Required permanent prohibitions**

- no `Any`/blob/string/digest workflow artifact authority
- no selected/support-only invariant pass
- no generic write receipt as domain invariant evidence
- no atomic label without session-bound prepare/commit/abort proof
- no provider callback widening declared read/touch/effect/resource scope
- no eager-complete result presented as provider streaming/progress
- no returned-row-only membership dependency for coverage-sensitive results
- no runtime footprint widening or authority minting
- no caller-owned access-product cache or unbounded maintenance queue
- no scalar loop over an admitted bulk operation
- no domain vocabulary in Query declarations, execution, or public evidence
- no decision/counter sidecar widening disclosure authority
- no Store durability claim from runtime-only handles or serializable payloads
- no content, reporting digest, retry, or cache identity substituted for a
  production/acquisition occurrence or independent-certification identity
- no cached/shared result satisfying a required fresh occurrence or independent
  verification
- no universal bitwise-replay requirement and no uninstalled tolerance,
  statistical, distributional, or host comparator granting equivalence
- no `best`, `optimal`, `unique`, `complete`, or `Pareto` claim without its
  installed universe, termination, completeness, and comparison evidence
- no transformation, correspondence, loss, repair, candidate, convergence, or
  advisory artifact promoted into admission, identity, resolution, or
  publication authority
- no Query-local durable conflict model, participant approval state, resolution
  session, carry-forward rule, or session journal competing with
  `_docs/cross-runtime/merging-and-branching-roadmap.md`

**Required certification matrix**

- managed artifact contract, carriage, lifecycle, and schema evolution
- bulk/chunk native access and actual memory bounds
- resource admission, saturation, cancellation, yield/resume, and cleanup
- provider session phase order and failure topology
- complete positive/negative/structural decision read-sets
- provisional-state visibility, discard, and generation safety
- real post-state invariant execution
- relevant/unrelated drift and compare-and-commit
- abort, partial effect, indeterminate commit, and reconciliation
- access-product strategy parity, coverage, membership, and rebuild
- realized-footprint subset proof and invalidation narrowing
- correlated path semantics and no local row-drain
- set-oriented partitioning, serial/parallel parity, and non-quadratic slope
- domain structural counters, decision attachments, privacy, and retention
- artifact reuse, incremental/rebuild parity, eviction, and disposal
- semantic artifact, occurrence, certification, and permitted-substitution
  identity separation
- exact, seeded, canonical-reduction, bound/comparator, distributional, and
  observational/non-replayable reproducibility posture
- candidate-universe, termination, completeness, feasibility, comparison,
  optimality, incumbent, and search-cost posture
- single-basis convergence progress, iteration bounds, oscillation, exhaustion,
  cancellation, and incumbent preservation
- transformation occurrence, source/output correspondence, loss/disposition,
  and authority-denial posture
- branch/resolution boundary proving no durable resolution authority exists in
  Query 9.15
- geometry, chip/netlist, and research-style reference adoption residue

**Warnings**

- Provider receipts cannot be the sole oracle for provider honesty.
- Compile tests remain limited to genuinely compiler-enforced authority and
  phase boundaries; runtime semantics require integration/adversarial evidence.
- Passing local feature tests cannot substitute for cross-phase hostile
  scenarios.
- Certification cannot weaken plans, budgets, basis, or provider obligations to
  make alternate providers pass.

**Test requirements**

- Hostile-provider test: seed scope widening, false completeness, fabricated
  invariant pass, synthetic progress, under-reported bytes, false atomicity,
  stale token reuse, and indeterminate-as-success; each is rejected at its
  responsible boundary.
- Independent-oracle test: at least two provider implementations or one provider
  plus an independent small-world oracle agree on semantics without sharing the
  implementation's comparator or decision logic.
- Mutation-sensitivity test: sabotage every material invariant, membership,
  counter, decision, and lifecycle field and prove at least one decisive test
  fails for the intended reason.
- Identity-substitution test: collide payload bytes, semantic projections,
  reporting digests, retries, caches, providers, and occurrence labels; only the
  installed substitution contract may admit reuse, and required independent
  occurrences remain distinct.
- Reproducibility-matrix test: reorder parallel reduction, vary admitted
  hardware/provider posture, perturb outputs within and outside installed
  bounds or distributions, and prove each family converges or denies according
  to its declared class rather than raw byte equality.
- Search-claim sabotage test: vary universe, candidate order, termination,
  comparator, pruning, bound, and omitted candidates; completeness and
  optimality never strengthen beyond the installed evidence.
- Convergence sabotage test: inject slow progress, repeated-state cycles,
  misleading provider completion, cancellation, and exhausted budgets; typed
  epoch posture and incumbent evidence remain exact.
- Resolution-boundary residue test: production Query 9.15 contains no durable
  conflict/session authority, participant approval state, resolution journal,
  branch-aware carry-forward, or recovery path.
- Scale-ladder test: vary total graph size, touched footprint, path length,
  valence/fan-out, candidate density, artifact bytes, chunk width, batch width,
  occurrence count, comparison breadth, candidate-universe breadth, search
  pruning, convergence iterations, transformation breadth, and consumer pressure
  independently with exact structural counters and slope assertions.

**Engineering decisions**

- Milestone 13 consumes these provider-independent oracles and cannot certify
  the old naive provider lane as equivalent.
- Support rows, docs, AI orientation, facade snapshots, implementation, and
  certification must agree before closure.
- The milestone closes only after reference adoption and permanent residue
  enforcement, not when APIs merely exist.

**Open questions**

- None.

## Must Ship

- installed, schema-versioned domain artifact contracts and runtime-affine
  move-only managed handles
- distinct semantic artifact, production/acquisition occurrence, and
  independent-certification identities with installed substitution policy
- installed exact, seeded, canonical-reduction, bound/comparator,
  distributional, and observational/non-replayable reproducibility classes
- installed candidate-search contracts exposing universe, termination,
  completeness, feasibility, optimality, incumbent, and exact search cost
- managed single-basis convergence epochs with bounded iteration, progress,
  oscillation, cancellation, exhaustion, checkpoint, and incumbent evidence
- derived transformation-occurrence, correspondence-cardinality, and
  loss/disposition evidence with zero admission or resolution authority
- bulk/chunk native access with actual physical memory and work evidence
- installed structural counter and decision-record schemas
- multi-axis resource admission and managed run lifecycle with cancellation,
  backpressure, yield/resume, cleanup, and Store handoff
- sealed provider execution plans and prepared provider sessions
- complete positive, negative, membership, predicate, cardinality, ordering,
  traversal, artifact, and structural decision read-sets
- provisional graph attempts and proposed-state reads
- real domain invariant execution over the exact proposed post-state
- provider-proven compare-and-commit with stale, abort, partial-effect, and
  indeterminate outcomes
- managed domain access products with completeness, membership, lifecycle,
  memory, maintenance, rebuild, and disposal contracts
- verified realized footprints integrated with dependency impact, invalidation,
  conflict, reuse, and publication
- correlated heterogeneous path programs
- conflict-proof, set-oriented installed operation execution
- policy-governed domain decision attachments and mandatory structural cost core
- stage/subartifact reuse, incremental maintenance, eviction, and rebuild
- public facade and realistic planar-boolean, chip/netlist cone, and
  research-style multi-site reference adoption
- permanent prohibitions and provider-independent hostile certification

## Must Preserve

- the one installed operating-world root and all 9.14 authority boundaries
- Query's ownership of installed contracts, admission, workflow/attempt
  progression, public evidence, dependency carriage, resource lifecycle, and
  certification
- Relational ownership of authoritative truth, MVCC, transaction execution,
  constraints, and schema evolution
- runtime-bridge and provider ownership of physical route/session mechanics
- Signal ownership of scheduling and reactive evaluation semantics
- Store ownership of durable payloads, indexes, checkpoints, continuations,
  journals, restart, recovery, and reconciliation persistence
- domain ownership of payload schemas, algorithms, validators, decision
  semantics, structural counter meaning, equivalence, conflict/disjointness,
  replanning, compensation, and physical strategy
- authority/derived separation for artifacts, access products, proposed state,
  footprints, decision records, counters, and reporting projections
- authority/derived separation between candidate/search/transformation evidence
  and durable conflict, resolution, participant, approval, and publication state
- separation of semantic content, occurrence, certification, and permitted
  computational substitution across execution, reuse, and publication
- domain ownership of determinism, numerical/statistical comparison, evidence
  independence, and observational meaning behind Query-carried contracts
- the cert-only replay fence and the distinction among retry, re-execution,
  replay, reversal, compensation, reconciliation, and reconstruction
- exact Foundational value/aspect meaning and `worth-proof` phase/basis/
  freshness/witness law behind stronger owner-specific types
- privacy, disclosure, redaction, retention, deletion, legal hold, schema
  evolution, and compatibility law at every boundary
- cross-runtime ownership of durable conflict and resolution semantics,
  participant authority, session persistence, carry-forward, and recovery

## Allowed Debt

- durable artifact payloads, persistent access-product bytes, restart-stable
  continuations, checkpoint/journal survival, distributed recovery, and
  reconciliation history remain Store-owned follow-on implementation
- provider-specific zero-copy layout, allocator, index, transaction, and
  maintenance strategies may vary behind the complete contracts
- additional domain artifact, access-path, validator, counter, decision, and
  partition families may be installed later without weakening the shipped
  generic lifecycle
- no runtime-backed artifact authority, resource dimension, cancellation state,
  provider-session phase, decision dependency family, proposed-state invariant
  proof, transaction outcome, membership coverage, footprint proof, bulk
  boundary, or mandatory evidence field described by this milestone may remain
  as generic debt, a support-only row, a fabricated receipt, or a documented
  convention
- no unsupported runtime-backed family may fall back to strings, blobs, raw
  provider handles, host callbacks, broad scans, caller caches, scalar loops,
  generic success, or silent widening; it must expose its exact missing
  capability owner and typed denial

## Acceptance Evidence

Milestone 9.15 is complete only when Query can prove:

- a realistic planar-boolean or equivalent geometry/topology workflow carries
  candidate, intersection, split, continuation, classification, provisional
  topology, and invariant products as managed artifacts without primitive/blob
  smuggling or a domain-local side store
- a chip/netlist cone recomputation and incremental update completes the same
  lifecycle without Query gaining domain-specific vocabulary
- a research-style multi-site biological reference retains byte-identical
  independent observations as distinct occurrences, rejects cached computation
  as independent evidence, and admits byte-different equivalent results only
  through an installed domain comparison contract without Query gaining
  research vocabulary
- selected obligation support cannot produce a passed invariant outcome; every
  blocking invariant result binds real proposed-state execution evidence
- relevant concurrent drift returns stale/replan while unrelated drift commits,
  with exact read-set and false-conflict counters
- atomic success exists only for one provider-proven transaction; abort,
  partial-effect, compensation, reconciliation, and indeterminate outcomes
  remain distinct and hostile failure injection cannot collapse them
- cancellation at every safe point, repeated yield/resume, backpressure,
  exhaustion, and degradation preserve declared guarantees and leave no orphan
  resources or promotable stale results
- candidate-producing runs report exact universe, termination, completeness,
  feasibility, comparison, optimality, incumbent, and search-cost posture; no
  heuristic or incomplete search can present itself as uniquely best
- iterative runs converge, remain merely stable or feasible, oscillate,
  exhaust, cancel, or become indeterminate according to installed single-basis
  contracts without creating durable resolution authority
- transformation and loss evidence preserves source occurrence,
  correspondence cardinality, disposition, and proposed identity consequences
  while remaining unable to admit or publish repaired truth
- access-product candidates remain complete under insertion, deletion, motion,
  membership change, rebuild, and eviction, with exact refinement preserving
  semantic results
- serial, chunked, resumed, partitioned, replay-certified, and alternate
  provider execution converge on canonical semantic output, footprint,
  invariant, commit, artifact, and evidence meaning
- exact and non-bitwise reproducibility families converge or deny according to
  their installed class, while semantic equality never erases required
  occurrence or independent-certification identity
- bulk execution avoids external scalar loops and quadratic all-pairs proof;
  scale evidence varies every independent axis named by the milestone
- mandatory envelope core remains present under every diagnostic policy while
  sidecars obey disclosure, redaction, retention, deletion, and legal-hold
  posture
- facade, support matrix, docs, AI orientation, boundary enforcement, residue
  reports, realistic fixtures, and hostile certification agree on one
  execution-grade installed operation path

## Sequencing Notes

Milestone 9.15 follows Milestone 9.14 because it extends the installed operation,
workflow, dependency, publication, compatibility, sharing, and lifecycle
authorities frozen there. It must not reopen 9.14 semantics or invent a parallel
domain entry root.

Phases are intentionally ordered:

1. Phases 1-4 freeze artifact and evidence meaning before operational handles.
2. Phases 5-6 freeze resource admission and managed execution before provider
   work can claim cancellation, progress, or boundedness.
3. Phases 7-11 close the hard transactional attempt from prepared provider
   session through real invariant execution and honest commit outcomes.
4. Phases 12-14 close derived access products, negative-space membership, and
   exact realized dependency carriage.
5. Phases 15-16 close expressive provider-backed neighborhoods and bulk
   partitioning without domain leakage.
6. Phases 17-18 close explanation and reuse only after the authoritative
   execution and dependency boundaries exist.
7. Phases 19-20 close public adoption, deletion, enforcement, and certification.

Milestone 9.15 precedes Milestone 13 because provider-independent certification
must certify the execution-grade provider/session, artifact, invariant,
resource, access-product, footprint, and batch contracts rather than the naive
callback surfaces they replace.

Runtime-backed managed artifacts, sessions, attempts, resource lifecycle,
ephemeral access products, and evidence are not blocked on Store. Durable
artifact bytes, persistent access products, restart-stable continuations,
checkpoint/journal survival, distributed recovery, and reconciliation history
are explicit Store handoffs. Store integration must implement the same contracts
without redefining semantic meaning or weakening provider obligations.

Candidate search, bounded convergence, and transformation evidence are complete
single-semantic-world execution capabilities here. Durable conflict identity,
resolution alternatives and decisions, participant roles, approval, deferral,
supersession, branch-aware carry-forward, session checkpoints/journals, and
resolution recovery are blocked on the distinct cross-runtime semantic Git
program. That dependency is an authority boundary, not permitted generic debt:
Query 9.15 must expose typed absence/unavailability rather than a temporary
local session model.
