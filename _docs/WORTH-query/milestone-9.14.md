# Milestone 9.14: Installed Operation Semantics And Bound Downstream Authority

## Goal

Make the safe downstream Query path mechanically complete. Stable domain views,
workflows, and operation semantics are installed once as typed Query
definitions. One non-detachable runtime-affine operation capability then
carries declaration, installation, cross-domain binding, basis, workflow
progression, execution trace, replay admission, reversal posture, lineage,
consumption, native refinement, dependency impact, sharing, and lifecycle
proof through the consumer journey without permitting callers to reassemble
authority from individually valid parts. The same capability mints its
consumer support contract, managed observation leases, invalidation deltas,
and collection/window delivery so downstream runtimes never reconstruct Query
posture, operation meaning, stage ledgers, replay catalogs, undo scope,
identity lineage, impact closure, shared work, or patch semantics locally.

## Why This Milestone Exists

Milestone 9.13 makes ordinary Query declarative, runtime-installed, and
Foundational-native. Downstream integrations can still make serious mistakes
if stable domain operations rebuild schema, selectors, result shape, workflow
graphs, stage ledgers, replay catalogs, reversal scope, or lineage meaning in
extension methods; if independently valid domain capabilities are treated as
atomic cross-domain authority; if consumers maintain local dependency graphs
or recompute policy; if equivalent projections create duplicate execution and
subscription resources; or if separately exposed definitions, executions,
consumed facts, basis identities, support families, lifecycle receipts,
invalidation surfaces, collection patch posture, and reporting digests must be
coordinated correctly. This milestone closes those remaining assembly and
duplication hazards before Query exports its final runtime-backed
certification oracle and before Store integration multiplies the same boundary
across physical providers. It does so generically: Query owns semantic
progression and proof, domain packages own domain algorithms and correspondence
policy, and Store continues to own durable journals, checkpoints, and restart.

## Governing Summaries

- `MENTALITY.md` protects foundation-first correctness under adversarial
  pressure; the milestone therefore begins by installing stable domain view
  and workflow meaning, then binds the non-detachable capability instead of
  adding ergonomic wrappers around consumer-reconstructed declarations.
- `arch_laws.md` protects proof-carrying phase progression and preserved
  identity authority; each transition consumes the proof from the prior phase,
  and representations cannot promote themselves.
- `composition_laws.md` protects one predictable semantic responsibility per
  file and function; installed definitions, binding, support, compatibility,
  refinement, dependency impact, sharing, lifecycle, invalidation, collection
  delivery, identity, and accounting remain separately named responsibilities.
- `domain_structure_laws.md` protects physical boundaries that reveal
  authority, lifecycle, truth source, and dependency direction; the public
  facade is narrower than the internal capability topology.
- `perf_laws.md` protects cost honesty and bounded execution breadth; installed
  operation lookup, native access, dependency closure, sharing admission,
  invalidation, window resolution, and patch delivery expose exact counters
  and cannot hide broad scans or duplicate equivalent maintenance.
- `WORTH_query_roadmap.md` protects one canonical runtime-backed Query meaning
  that later providers must preserve; 9.14 closes downstream consumer safety
  after 9.13 and before Milestone 13 exports provider-independent oracles.

## Adversarial Constraint

Given any number of independently valid installed-domain handles, stage
receipts, portable or locally rebuilt operation definitions, execution
completions, replay scopes, reversal scopes, lineage candidates, basis
projections, consumed fact receipts, support projections, dependency labels,
equivalence tokens, diagnostic digests, invalidation labels, collection
patches, and lifecycle artifacts from foreign runtimes, stale generations, or
different declarations, a consumer must be unable to combine them into an
atomic multi-domain operation capability, legal workflow advancement,
semantic replay, reversal or compensation authority, persistent naming
authority, operational projection capability, compatibility proof, shared
execution, observation lease, replacement admission, rebind admission, native
value access, impact closure, invalidation delta, collection window, patch
delivery, or lifecycle transition.

Equivalent installed definitions and bound capabilities must converge on one
canonical meaning and, where Query admits sharing, one framework-owned
execution resource with independently disposable consumer leases. The admitted
path must remain proportional to declared dependency, consumption, affected-
capability, window, and patch width. Foreign, stale, detached, ambiguous, and
representation-only inputs must deny before planning, sharing, lower-runtime
contact, native refinement, invalidation, or lifecycle work.

## Product Decision Lock

- Query exports a complete installed operation semantic closure, not separate
  catalog, workflow, replay, reversal, lineage, or authority ingredient lists
  that consumers must assemble correctly.
- Stable domain-native views, workflows, and operations are installed package
  definitions; downstream extension traits may expose vocabulary but may only
  resolve those definitions through an installed handle.
- Installed operation definitions declare dependency, workflow, read, touch,
  effect, invariant, replay, reversal, lineage, result, failure, and cost
  posture as one semantic closure. Portable definitions never contain
  executable callbacks.
- Multi-domain operation authority is bound atomically. A tuple of separately
  valid domain capabilities is not an operation capability.
- Workflow stages and run traces are Query-minted proof progression; domain
  packages retain ownership of stage algorithms and semantic lowering.
- Replay and reversal are distinct. Replay creates a new attempt identity and
  proves declared semantic equivalence; reversal executes a declared inverse
  or compensation posture and may be explicitly unavailable.
- Persistent naming and identity continuity consume typed lineage evidence;
  raw identities, strings, digests, coordinates, rendered values, and debug
  representations cannot authorize continuity.
- Operational compatibility is decided by Query and returns typed proof or a
  typed denial; consumers do not compare identity components.
- Native access is derived from the admitted declaration and Foundational
  contract; callers do not scan facts or reinterpret paths.
- Consumer support and required Query surfaces are minted once by Query from
  the bound capability; consumers do not rebuild them from hooks or digests.
- Query emits capability-bound invalidation deltas; consumers decide their own
  consequences but do not reinterpret which Query meaning changed.
- Query compiles semantic dependency roles and impact closure from the
  installed definition, admitted plan, native contracts, and installed domain
  invariants; consumers do not maintain a second dependency graph.
- Equivalent capability sharing is admitted by a Query-owned equivalence
  contract and represented by framework-managed consumer leases; matching
  digests or cloned packets never justify reuse.
- Collection identity, ordering, cursors, windows, and incremental patches are
  Query-shaped authority; UI virtualization owns mounting policy only.
- Live promotion, replacement, rebind, and disposal remain attached to the
  capability that established their installation and execution authority.
- Reporting and diagnostic identities are observable derivations with zero
  operational authority.
- Exact public type names may be refined during implementation, but the single
  capability, proof progression, authority ownership, and prohibition grammar
  are not open.

## Phase Plan

### Phase 1: Installed Domain Operation Definitions

This phase extends the installed domain package from capability, invariant,
obligation, graph-operation, declaration-family, and contribution meaning to a
complete installed operation semantic closure. A definition declares its typed
identity and version, parameter contract, native aspect contract and projection
mask, canonical Query intent, result shape, collection/order posture, required
installed capabilities, optional typed workflow-stage graph, graph-read
contract, touch and effect contract, invariants, replay posture, reversal or
compensation posture, lineage contract, terminal result and failure classes,
cost contract, support, and lowering family. The runtime validates and installs
the definition once and returns only an installed typed operation capability.

Generic ad hoc Query authoring remains legitimate. A downstream operation with
stable domain meaning, however, cannot repeatedly construct schema, selectors,
aliases, result shape, or mutation paths inside an extension method and call
that local construction domain-native authority.

**Relevant subsystems**
- domain package validation, canonical identity, and runtime installation
- installed-domain operation indexes and handle lookup
- ordinary read, collection, mutation, and workflow declarations
- Foundational-native schema contracts, masks, selectors, and result shapes

**Relevant APIs**
- `WorthQueryDomainPackage<D>` and installed-domain handle surfaces
- installed graph-read operation definitions and declaration-family definitions
- ordinary read, collection, mutation, and workflow authoring
- proposed typed domain view definition, workflow definition, installed
  operation key, and installed operation capability
- existing graph-read, graph-touch, authority-scoped effect, identity-
  evolution, and deterministic execution contracts

**Warnings**
- An extension trait that hides local reconstruction of a stable query is still
  a competing declaration authority even when its method name is ergonomic.
- Installing a closure, callback, backend plan, or executable consumer object
  would move volatile execution mechanics into portable domain meaning.
- A workflow catalog, replay catalog, reversal catalog, and lineage catalog
  maintained independently would create multiple semantic authorities even if
  every catalog refers to the same operation name.
- The semantic closure declares what must be true; domain implementations
  register or lower algorithms separately and cannot smuggle executable
  callbacks into portable definitions.
- A raw operation name plus version is not installed authority. Portable
  definitions become operational only when resolved by the installing runtime.
- Query must not hard-code product domains. Domain crates author typed generic
  definitions against Query's contracts and may expose their own vocabulary.
- Installed domain operation definitions are versioned product semantics, not
  user-saved query instances or durable provider records.

**Test requirements**
- Adversarial definition convergence test: equivalent package view and workflow
  definitions install to identical canonical meaning regardless of declaration
  order, ergonomic authoring path, or derived-index rebuild.
- Adversarial conflict and reconstruction test: duplicate versions with
  different schema, masks, parameters, result shape, support, lowering, or
  workflow meaning fail package installation atomically, while a locally
  reconstructed lookalike cannot resolve as the installed operation.
- Adversarial authority test: raw names, copied definition fields, foreign-
  runtime handles, stale installation generations, and portable definitions
  cannot execute, bind, mutate, or mint an installed operation capability.
- Adversarial lookup-bound test: resolving one installed operation performs one
  indexed lookup independent of unrelated domains and operations and performs
  no planning or lower-runtime work on a miss.
- Adversarial semantic-closure test: removing or drifting a required capability,
  workflow edge, graph-read, touch/effect, invariant, replay, reversal,
  lineage, result/failure, or cost role changes canonical operation meaning and
  conflicts atomically rather than producing a partially installed operation.
- Adversarial callback test: portable operation definitions cannot carry or
  recover executable callbacks, backend objects, or consumer-owned stage
  implementations.

**Engineering decisions**
- The package is the canonical portable source for stable domain operation
  meaning; the installed operation capability is the runtime-affine authority.
- A typed downstream extension method may resolve an installed operation and
  improve vocabulary, but it cannot construct or override Query meaning.
- Definitions describe semantic intent and lowering family, never a physical
  provider plan or executable callback.
- The operation definition is the single canonical source for workflow,
  replay, reversal, lineage, dependency-impact, and certification semantics;
  later phases compile derived indexes and proof artifacts from it.
- Installed view and workflow indexes are derived, rebuildable, and powerless
  without the retained installed package artifact and runtime authority.

**Open questions**
- Exact generic type names and whether view and workflow definitions share a
  common sealed definition envelope are implementation choices; their distinct
  result, cost, lifecycle, and failure contracts must remain visible.

### Phase 2: Non-Detachable Bound Projection Authority

This phase introduces the one downstream authority root from which every later
operation proceeds. A bound operation capability, with projection-specialized
views where applicable, retains the exact installed-domain authorities,
canonical operation definition, native value contract, runtime generation,
basis, policy, tenant, preview or branch scope, and required cross-domain
capability set as private state. Only Query can construct it by atomically
binding every installed capability declared by the operation.

**Relevant subsystems**
- installed domain capability and runtime-affine handle authority
- downstream basis and projection authority
- atomic multi-domain capability binding
- public domain/read facade

**Relevant APIs**
- `WorthQueryInstalledDomainHandle<D>`
- installed-domain view declaration surfaces
- proposed bound operation/projection capability and binding denial

**Warnings**
- A convenience struct with public fields is still an ingredient list, not a
  capability.
- Phantom tags cannot substitute for retained runtime-affine authority.
- Do not make the capability serializable or reconstructible from canonical
  definition data; persistence later readmits portable meaning through the
  owning runtime.
- Do not erase legitimate view-family or domain-family distinctions behind one
  dynamically typed bag.
- A tuple, struct, or map of independently bound topology, spatial, relational,
  or other domain capabilities is not atomic operation authority.
- Query must reject mixed runtime, installation generation, basis, policy,
  tenant, preview, and branch scope before producing the bound capability.

**Test requirements**
- Adversarial convergence test: equivalent installed declarations in the same
  runtime produce equivalent bound capability meaning regardless of declaration
  order or facade authoring path.
- Adversarial construction test: external code cannot mint, field-assemble,
  deserialize, or clone-and-restamp a bound capability from an installed
  handle, view definition, runtime identity, or digest.
- Adversarial separation test: independently valid domain and view artifacts
  from different runtimes or installation generations cannot bind and perform
  exact zero planning, execution, and refinement work.
- Adversarial atomicity test: individually valid required domain capabilities
  with mismatched basis, policy, tenant, preview, branch, or installation
  generation cannot be bundled, reordered, partially bound, or substituted to
  mint operation authority.

**Engineering decisions**
- The bound capability owns the complete prerequisite set for downstream
  operation and projection work and exposes read-only semantic inspection.
- Cross-domain binding is one Query decision and one proof artifact, not a
  consumer-authored aggregation of successful independent binds.
- Construction is sealed behind the installed-domain declaration path.
- Canonical portable declarations remain non-authoritative inputs; the bound
  capability is runtime-affine operational authority.

**Open questions**
- Exact generic parameters and final public name remain open until compiler
  inference and facade transcripts prove the smallest honest surface.

### Phase 3: Query-Minted Consumer Support Contract

This phase makes the bound projection mint one typed consumer contract that
states the Query meaning and support required downstream. The contract carries
the admitted basis capability, live and continuation posture, result-state and
async posture, projection-consumption contract, recovery and inspection
capabilities, dependency-impact contract, sharing posture, invalidation
contract, and collection-delivery posture that apply to this exact bound
projection.

Downstream runtimes may add presentation, graph attachment, allocation, or host
policy, but they do not enumerate Query subsystems, derive support from local
hook kinds, or fold Query reporting digests into a local posture product.

**Relevant subsystems**
- Query support and admission matrices
- bound projection declaration and installed-domain capability
- live, async/result-state, recovery, inspection, and projection consumption
- downstream consumer contract facade

**Relevant APIs**
- Query support snapshots and admitted capability artifacts
- proposed `WorthQueryConsumerProjectionContract` or equivalent
- bound projection support and requirement inspection

**Warnings**
- A struct containing one string digest per Query subsystem is still a
  consumer-authored Query mirror.
- The consumer contract must express admitted semantic requirements, not an
  inventory of whichever internal modules currently implement them.
- Consumer-local presentation posture may not alter Query support,
  compatibility, recovery, continuation, or result-state meaning.
- Support must be evaluated by Query; a downstream default that treats every
  registered hook as supported is structurally dishonest.

**Test requirements**
- Adversarial convergence test: equivalent bound projections mint identical
  consumer contract meaning, required capabilities, support posture, and exact
  counters regardless of declaration order or facade path.
- Adversarial spoofing test: local hook enums, copied support reports, matching
  digest strings, and consumer-authored requirement lists cannot construct or
  satisfy the consumer contract.
- Adversarial support drift test: basis, live, async/result-state, recovery,
  inspection, projection-consumption, dependency-impact, sharing,
  invalidation, or collection-delivery drift produces a Query-owned typed
  compatibility denial before downstream planning rather than a consumer-side
  digest comparison.
- Adversarial derived-state test: support indexes and reporting summaries can be
  destroyed and rebuilt from the bound capability and consumer contract
  without changing admission, compatibility, denial, or lifecycle outcomes.

**Engineering decisions**
- Query mints exactly one consumer contract per bound projection generation.
- The contract is proof-bearing operational input; reporting and human-readable
  support summaries are derived projections with no construction power.
- Downstream-specific requirements compose beside the Query contract through a
  named consumer boundary rather than being inserted into Query authority.

**Open questions**
- Exact grouping of live, continuation, recovery, and result-state views may
  follow existing Query capability families; the one-contract authority is
  fixed.

### Phase 4: Installed Execution And Consumption Progression

This phase makes execution and projection consumption a proof-widening chain
owned by the bound capability. The ordinary progression is structurally
equivalent to `bound -> executed -> consumed -> settled`; each transition
consumes or immutably borrows the exact prior proof and returns the only type
accepted by the next transition.

**Relevant subsystems**
- ordinary read execution and installed-domain execution receipts
- projection consumption declarations, contracts, facts, and receipts
- downstream authority settlement and result-state handling

**Relevant APIs**
- `WorthQueryInstalledDomainExecutionReceipt`
- `WorthQueryConsumedProjectionAuthority`
- proposed bound execution, consumed projection, and settlement capabilities

**Warnings**
- A tuple containing the same artifacts is not a proof-bearing phase type.
- The executor must not accept a detached definition or independently supplied
  basis to override the capability's authority.
- Advisory, partial, pending, and violation posture must remain attached to the
  same chain instead of being flattened into booleans or side-channel errors.
- Do not make consumers replay Query's internal phase graph manually under new
  method names.

**Test requirements**
- Adversarial continuity test: installation authority, declaration identity,
  basis, execution receipt, consumption contract, materialized facts, warnings,
  and settlement remain traceable through one chain with no restamping.
- Adversarial mix-and-match test: a completion, fact receipt, or consumption
  authority from another bound capability cannot advance the chain even when
  its query text, view shape, basis label, and terminal digest match.
- Adversarial phase-order test: consumption before execution, settlement before
  consumption, and reuse of a moved single-use transition are uncallable or
  deny at the earliest trust boundary with zero later-phase work.

**Engineering decisions**
- Phase types encode what has been proven, not merely which fields are present.
- Execution and consumption transitions preserve the installed authority
  privately and expose semantic outcomes through contracted views.
- Multi-consumer observation, when legitimate, uses an explicit shared
  authority lifecycle rather than arbitrary packet cloning.

**Open questions**
- None. Snapshot settlement remains move-only unless a later explicit
  observation lease is admitted through Phase 13.

### Phase 5: Installed Workflow Graph And Query-Minted Run Trace

This phase turns an operation's optional workflow posture into an installed,
validated graph and a proof-bearing execution trace. An installed operation may
declare typed stage identities, stage dependencies and legal successors,
required installed capabilities across domains, typed input and output
contracts, graph-read roles, touch and effect roles, invariant checkpoints,
cost roles, and terminal result and failure classes. Package installation
proves that the stage graph is acyclic, complete, uniquely identified, and
dependency-closed before any runtime work can begin.

Starting a run from the atomically bound operation capability mints a Query-
owned workflow-run capability. Each admitted stage consumes the exact required
predecessor receipt set and produces a Query-minted stage receipt carrying the
workflow run, installed operation, bound authority, predecessor continuity,
typed native outputs, graph reads, touch and effect evidence, invariant
outcomes, and exact counters. A consumer cannot skip a stage, splice runs, or
maintain a parallel evidence ledger.

**Relevant subsystems**
- installed operation workflow definitions and package validation
- atomic multi-domain operation binding
- graph-read planning, graph-touch obligations, and authority-scoped effects
- deterministic parallel admission and exact workflow accounting

**Relevant APIs**
- installed domain operation capability and bound operation capability
- existing graph-read access plans, graph-touch obligations, and lowered
  authority-scoped effect plans
- existing deterministic parallel admission proofs
- proposed installed workflow graph, workflow-run capability, stage admission,
  stage receipt, and completed execution trace

**Warnings**
- A global product-specific stage enum or hard-coded stage slot map is not a
  generic workflow graph.
- Independently valid stage receipts are not composable unless Query proves
  they belong to the same run, operation, authority, and predecessor frontier.
- Portable workflow definitions declare semantic roles and dependencies; they
  never carry executable callbacks or backend objects.
- Do not rebuild graph-read, touch, effect, or parallel-admission semantics in
  this phase. The workflow trace consumes the existing proof-bearing contracts
  established by Milestones 9.10, 9.9, 9.3.3, and 5.3.
- Diagnostic stage timelines are derived views of the retained trace and have
  no authority to advance a run.

**Test requirements**
- Adversarial graph-convergence test: equivalent workflow definitions install
  to the same validated DAG and stage semantics regardless of declaration
  order, while cycles, missing predecessors, duplicate stage identities,
  unreachable stages, and incomplete terminal paths fail installation
  atomically.
- Adversarial progression test: stage skipping, duplicate advancement,
  predecessor omission, cross-run receipt mixing, foreign-runtime receipts,
  and wrong-basis or stale-generation receipts cannot advance a run.
- Adversarial multi-domain binding test: every required domain capability must
  share runtime, installation generation, basis, policy, tenant, and preview or
  branch scope before the first stage; partial success mints no run capability.
- Adversarial trace-convergence test: valid serial and deterministically
  parallel schedules produce the same semantic trace, outputs, touch/effect
  closure, invariant outcomes, and terminal result for the declared workflow.
- Adversarial cost test: stage admission inspects only the declared predecessor
  frontier and required capabilities, and exact counters expose graph, read,
  touch, effect, and parallel-admission work without scanning unrelated runs.

**Engineering decisions**
- The installed workflow graph is part of the operation semantic closure; the
  run trace is the single Query-minted canonical execution artifact.
- Domain implementations own stage algorithms and lower through admitted
  Query and lower-runtime capability seams. Query owns legal progression,
  predecessor continuity, trace assembly, and stage authority.
- The trace derives from existing graph-read, touch, effect, and parallel
  proofs rather than storing consumer-authored summaries of them.
- Stage receipts are proof-bearing inputs to later stages and to replay,
  reversal, lineage, dependency impact, inspection, and certification.

**Open questions**
- The concrete representation of joins and parallel frontiers remains an
  implementation choice, but it must preserve exact predecessor authority and
  deterministic semantic trace ordering.

### Phase 6: Semantic Replay And Re-execution Authority

This phase admits semantic replay from the installed operation and a completed
Query-minted execution trace. It distinguishes four different mechanisms:
idempotent retry of one stage attempt, full semantic re-execution against the
same basis, admitted historical re-execution against a related basis, and
durable restart replay from a persisted journal. The first three receive Query
contracts here. Durable journal persistence, checkpoint recovery, and restart
reconstruction remain Store-owned.

A completed trace mints replay capability only when the installed operation
declares an admitted replay posture. Replay consumes normalized intent, the
installed operation definition, exact bound context, and the original trace,
then creates a new run and attempt identity. Equivalence is determined by the
operation's declared semantic comparator over typed results, effects, lineage,
and explicitly allowed nondeterministic noise—not by raw execution identity,
digest equality, or a consumer-maintained parity checklist.

**Relevant subsystems**
- installed operation replay posture and normalized intent
- workflow run trace and terminal execution outcome
- basis lifecycle, historical evaluation, and correspondence
- result, effect, lineage, divergence, and exact-counter inspection

**Relevant APIs**
- completed workflow execution trace and bound operation capability
- Query basis use and historical evaluation capabilities
- proposed replay admission, replay run capability, semantic equivalence
  contract, replay comparison, and typed divergence result

**Warnings**
- Replay is not reversal and does not promise to undo the original operation.
- Raw run identifiers, trace digests, output digests, and byte equality cannot
  define semantic equivalence unless the installed operation explicitly names
  byte identity as its domain guarantee.
- A new replay attempt must never reuse the original run or attempt identity,
  even when every semantic result converges.
- This phase must not imply durable journal storage, checkpoint survival, or
  restart reconstruction; those remain Store contracts.
- Allowed nondeterminism must be typed and bounded. An open-ended "ignore
  differences" callback would make replay unfalsifiable.

**Test requirements**
- Adversarial same-basis convergence test: an admitted semantic re-execution
  creates distinct run and attempt identities while proving equivalent typed
  intent, result, effect, lineage, and exact-counter posture.
- Adversarial historical-basis test: admitted historical re-execution records
  the exact basis relationship and correspondence evidence; a foreign,
  unrelated, stale, or unsupported basis denies before stage execution.
- Adversarial divergence test: omitted stages, changed semantic outputs,
  changed effects, broken lineage, or meaningful ordering changes localize a
  typed divergence even when reporting digests collide.
- Adversarial allowed-noise test: explicitly declared ordering or diagnostic
  noise converges without masking semantic output, effect, lineage, or
  invariant drift.
- Adversarial path-parity test: snapshot, live-triggered re-execution, and
  admitted replay paths consume the same installed operation semantics and
  produce comparable traces without a replay-specific operation catalog.

**Engineering decisions**
- Replay admission derives exclusively from the installed operation semantic
  closure and completed trace.
- A replay result carries original and replay identities, shared normalized
  intent, basis relationship, trace comparison, output/effect/lineage
  equivalence, divergence locus, and exact counters.
- Idempotent retry, semantic re-execution, historical re-execution, and durable
  restart replay remain separately named because their authority, cost, and
  failure contracts differ.
- Query owns replay admission, semantic comparison orchestration, and proof
  artifacts; domains own the comparator semantics they declare, and Store owns
  persistence and recovery mechanics.

**Open questions**
- None. Any future cross-process replay transport must readmit portable meaning
  through the runtime rather than serializing operational replay authority.

### Phase 7: Reversal, Compensation, And Irreversibility Contracts

This phase makes aftermath posture explicit and binds any reversal path to the
original installed operation and completed execution trace. The operation
definition must distinguish provisional discard, exact inverse, compensating
operation, rebuild or recovery procedure, and irreversible completion. Query
does not infer inverse behavior from touched scope and does not pretend every
operation is undoable.

When reversal or compensation is declared, the domain supplies its semantic
lowering and postcondition contract. Query admits it only against the exact
original operation, execution trace, effect and touch closure, basis,
installation generation, and current lifecycle. A successful inverse or
compensation is a new ordinary authoritative operation with its own run and
receipt referencing the original; it is never a mutation of history or a
consumer-authored "undo completed" label.

**Relevant subsystems**
- installed operation aftermath and irreversibility posture
- completed workflow trace, graph touches, effects, and invariants
- authority-scoped operation execution and lifecycle
- recovery, inspection, and exact accounting

**Relevant APIs**
- completed execution trace and installed effect/touch contracts
- ordinary bound operation execution and authority-scoped effects
- proposed reversal admission, exact-inverse capability, compensation
  capability, irreversible denial, and aftermath receipt

**Warnings**
- Touched scope proves where an operation acted; it does not prove how to undo
  that action.
- Replay repeats semantic intent. Reversal or compensation changes truth under
  a separately declared semantic contract. The two capabilities must not share
  a misleading generic "recovery" entrypoint.
- Compensation may restore a declared invariant or business postcondition
  without restoring byte-identical prior state. Its guarantee must state which.
- Irreversibility is a typed operational posture, not a missing callback or a
  runtime string error.
- Rebuild and Store recovery must not be presented as exact inverse operations.

**Test requirements**
- Adversarial scope-forgery test: copied touch sets, effect labels, operation
  names, or matching digests cannot mint exact inverse or compensation
  authority.
- Adversarial continuity test: a reversal capability from another execution,
  runtime, installation generation, basis, or lifecycle cannot act on the
  target operation even when touched scope and outputs match.
- Adversarial posture test: exact inverse, compensation, provisional discard,
  rebuild/recovery, and irreversible operations expose distinct callable
  surfaces, results, postconditions, failures, and exact counters.
- Adversarial semantic test: exact inverse proves its declared restoration
  semantics; compensation proves its declared postcondition without claiming
  byte equality; irreversible operations deny before lowering or mutation.
- Adversarial partial-failure test: failed reversal or compensation preserves
  the original trace, reports partial effects and required recovery posture,
  and cannot mint a successful aftermath receipt.

**Engineering decisions**
- Domain packages own inverse and compensation semantics and lowerings. Query
  owns their installed declaration, binding to original authority, progression,
  denial, receipt, and inspection contracts.
- Every reversal or compensation executes as a new operation with distinct
  identity and an explicit relation to the original execution.
- Touch/effect closure is necessary evidence but never sufficient authority.
- Durable rollback journals, checkpoints, and restart recovery remain Store-
  owned and consume these semantic contracts without redefining them.

**Open questions**
- None. Operations whose honest aftermath posture is not yet known must install
  as explicitly irreversible or unsupported rather than exposing a weak undo.

### Phase 8: Typed Lineage And Identity-Evolution Binding

This phase binds Query's existing lineage, correspondence, and identity-
evolution semantics to installed operation outputs and the Query-minted
execution trace. Operations declare how produced, preserved, split, merged,
retired, or ambiguous identities are evidenced. Query then carries typed
lineage outcomes from the exact stage and effect that established them rather
than allowing consumers to reconstruct continuity from geometry, labels,
digests, array position, or reporting representations.

The standard outcome family distinguishes preserved identity, singular
successor, plural split successors, merged predecessor set, generated identity,
retired identity, ambiguous or advisory correspondence, and continuity break.
Persistent naming, replay comparison, dependency impact, compatibility, UI
selection continuity, and derived-product reuse consume this same lineage
contract. Ambiguous or advisory correspondence may be presented for
adjudication but cannot silently authorize persistent naming.

**Relevant subsystems**
- existing lineage, structural correspondence, and identity evolution
- installed operation output and effect contracts
- workflow execution trace and typed native result carriers
- persistent naming, compatibility, invalidation, replay, and consumer
  continuity

**Relevant APIs**
- lineage traversal and correspondence artifacts from Milestones 5.4 and 7
- runtime authoritative mutation evidence and existing-truth binding
- Foundational native identity and value types
- proposed operation-lineage declaration, trace-bound lineage evidence,
  identity-evolution result, and persistent-naming admission

**Warnings**
- Coordinates, topology summaries, rendered values, debug strings, raw IDs,
  labels, and digests are observations or representations, not continuity
  authority.
- Similar geometry or equal output values may support correspondence but cannot
  promote advisory evidence into preserved identity.
- Persistent naming policy remains domain-owned; Query owns the proof-bearing
  binding and typed evolution result, not product-specific naming rules.
- Do not create separate lineage stories for replay, invalidation, UI
  continuity, and derived-product reuse. They must consume the same trace-bound
  evidence.
- Plural split and merge outcomes must not collapse into an optional singular
  successor.

**Test requirements**
- Adversarial representation test: raw strings, raw identifiers, digests,
  coordinates, rendered geometry, debug output, and copied correspondence rows
  cannot authorize persistent naming or operational identity continuity.
- Adversarial split/merge test: one-to-many splits, many-to-one merges,
  generated identities, retirements, and breaks produce exact typed outcomes
  with no silent singularization or predecessor loss.
- Adversarial trace-continuity test: lineage evidence from another execution,
  stage, operation, runtime, basis, or generation cannot bind to the target
  output even when its reporting representation matches.
- Adversarial replay-parity test: semantically equivalent replay produces the
  declared lineage-equivalent outcome under a distinct execution identity;
  lineage divergence localizes replay failure.
- Adversarial ambiguity test: ambiguous or advisory correspondence remains
  inspectable but cannot mint persistent naming, compatibility, reuse, or
  preservation authority without an admitted domain adjudication.
- Adversarial boundedness test: lineage binding is proportional to declared
  output and correspondence width and does not scan unrelated execution
  traces, identities, or consumer state.

**Engineering decisions**
- Existing Query identity-evolution semantics are reused and bound to installed
  operation traces; this milestone does not invent a parallel lineage engine.
- Foundational owns native identity and value carriers; domain packages own
  naming and correspondence semantics; Query owns admission, trace binding,
  typed evolution outcomes, and proof-preserving transport.
- Persistent naming accepts only typed lineage evidence from an admitted
  operation trace or an explicitly adjudicated correspondence capability.
- The same lineage result feeds replay, reversal inspection, dependency impact,
  compatibility, reuse, invalidation, and consumer continuity.

**Open questions**
- The exact generic vocabulary for domain-specific persistent names remains an
  implementation choice, but no public surface may accept representation-level
  substitutes for typed lineage evidence.

### Phase 9: Declaration-Indexed Native Access

This phase derives native value access from the admitted projection declaration
and Foundational contract. Declaring a projected field produces a typed access
key or materially equivalent proof that can refine the exact selected fact in
constant time without consumer-side path matching, fact scans, string parsing,
or local value-family switches.

**Relevant subsystems**
- Foundational native aspect contracts and canonical scalar/struct carriers
- Query projection declaration and materialized fact consumption
- native refinement and absence posture

**Relevant APIs**
- `ProjectionFactFieldPath`
- `ConsumedFieldValueFact` native refinement methods
- `AspectValue`, `StructAspectValue`, and canonical Foundational wrappers
- proposed declaration-derived native access key and access denial

**Warnings**
- A raw field path is location vocabulary, not proof that a value was declared,
  admitted, materialized, and retained by this capability.
- Do not introduce a Query-owned mirror of Foundational scalar or struct
  families to improve autocomplete.
- Absence, null, missing materialization, unsupported family, and struct shape
  mismatch are distinct typed outcomes.
- Typed access must not conceal a linear scan of all facts or source rows.

**Test requirements**
- Adversarial native parity test: every Foundational scalar family,
  representative nested structs, canonical floats, and admitted absence
  postures observed through the bound capability equal Query's native
  consumption oracle exactly.
- Adversarial access rejection test: a key from another declaration, field,
  contract revision, domain installation, or value family cannot access or
  refine a fact, even when its printable field path is identical.
- Adversarial bounded lookup test: accessing `k` declared native values performs
  exactly `k` indexed accesses and refinements as unrelated facts, rows, views,
  and installed domains grow.

**Engineering decisions**
- Query derives access keys from the admitted declaration and contract; callers
  never manufacture them from paths.
- Native carriers remain Foundational-owned and are borrowed or moved without
  stringify/reparse or JSON-shaped intermediates.
- Unsupported refinement preserves field, contract, source, and projection
  authority context in a typed denial.

**Open questions**
- The final accessor grammar may use generated typed selectors, declaration
  return values, or both; the no-scan and no-forging properties are fixed.

### Phase 10: Authority-Native Compatibility Decisions

This phase moves downstream sameness and transition decisions into Query.
Consumers ask semantic questions such as whether two capabilities share an
installation, may replace one another, may rebind, may share execution, or
belong to a compatible basis. Query returns a sealed typed witness for the
admitted relationship or a typed denial identifying the first incompatible
authority dimension.

**Relevant subsystems**
- installed-domain authority and generation compatibility
- basis capability lifecycle and projection contract equivalence
- replacement, rebind, reuse, and execution-sharing admission

**Relevant APIs**
- installed-domain authority witnesses
- basis capability envelopes and use receipts
- proposed installation, replacement, rebind, and basis compatibility proofs

**Warnings**
- Boolean helpers discard the proof later work needs and encourage callers to
  repeat the comparison.
- Equality of labels, canonical definitions, fact-set digests, or reporting
  projections cannot prove operational compatibility.
- Do not collapse replacement, rebind, reuse, execution sharing, and same-
  installation into one generic compatibility enum; their proofs and failure
  modes differ.
- Compatibility checks may inspect retained authority but may not reopen lower
  runtimes or re-execute Query work.

**Test requirements**
- Adversarial equivalence test: every admitted compatibility witness agrees
  with the canonical Query authority oracle across equivalent facade paths,
  declaration order, and derived-index rebuild.
- Adversarial collision test: matching labels, definitions, digest text,
  reporting output, and copied metadata cannot admit foreign-runtime,
  stale-generation, wrong-basis, wrong-contract, or wrong-lifecycle pairs.
- Adversarial proof-use test: replacement, rebind, reuse, and execution-sharing
  entrypoints reject a witness for the wrong relationship or capability pair at
  compile time or before operational work.

**Engineering decisions**
- Compatibility witnesses bind both exact capabilities and the named semantic
  relationship they prove.
- Denials preserve machine-readable installation, generation, basis, contract,
  lifecycle, and view-shape mismatch categories without leaking forgeable
  authority components.
- Derived compatibility indexes may accelerate lookup but are rebuildable and
  never sufficient without retained authority.

**Open questions**
- None.

### Phase 11: Capability-Bound Lifecycle Transitions

This phase attaches live promotion, replacement, rebind, cancellation, and
disposal to the bound projection capability. The framework tracks every managed
resource created from it, while transition-specific capability states make
illegal lifecycle calls unavailable.

**Relevant subsystems**
- live query promotion and managed resource lifecycle
- installed-domain execution drift and explicit rebind
- replacement, cancellation, disposal, and continuation

**Relevant APIs**
- managed live query resources
- installed-domain rebind receipts and drift counters
- proposed live-bound, rebound, replaced, and disposed capability states

**Warnings**
- A live subscription identifier beside a snapshot capability is detached
  lifecycle state.
- Disposal must invalidate operational use without pretending to erase durable
  or diagnostic evidence.
- Automatic retargeting across installation generations, bases, or runtimes is
  forbidden; drift requires a typed explicit transition.
- Lifecycle convenience must not erase the distinct costs or failure modes of
  snapshot execution, live maintenance, rebind, and replacement.

**Test requirements**
- Adversarial lifecycle convergence test: one-shot, promoted-live, explicitly
  rebound, and replacement paths preserve canonical declaration and authority
  meaning where the respective transition is admitted.
- Adversarial drift test: stale generation, foreign runtime, disposed resource,
  wrong basis, and changed contract deny before planning, lower-runtime,
  maintenance, or delivery work with exact-zero counters.
- Adversarial ownership test: dropping, cancelling, replacing, or disposing a
  managed capability leaves no orphan subscription or resource, and double
  disposal or use-after-disposal is uncallable or typed-denied.

**Engineering decisions**
- The framework owns resource registration, continuation, and disposal.
- Each lifecycle transition consumes the preceding state and returns the next
  proof-bearing state plus retained inspection evidence.
- Explicit rebind requires Query-minted compatibility proof from Phase 10.

**Open questions**
- None.

### Phase 12: Query-Compiled Semantic Dependency And Impact Closure

This phase makes every bound projection carry the complete semantic dependency
contract that determines how changes affect its meaning. Query compiles the
contract from the installed operation definition, validated workflow graph,
Query-minted stage trace, admitted graph-read plan, touch and effect closure,
lineage outcomes, Foundational-native contracts and masks, result shape,
collection ordering, window policy, support posture, and installed domain
invariants. Consumers may inspect the resulting roles and impact but cannot
author a competing graph or recompute policy.

Dependency roles remain explicit: operational identity, selection or
membership, ordering, projected value, grouping, window boundary, support and
lifecycle, installed domain invariant, and advisory-only context do not have
the same invalidation consequence. The compiled closure converts a lower change
into the narrowest honest impact class before consumer invalidation or patch
delivery begins.

**Relevant subsystems**
- installed view and workflow definitions
- validated workflow graph, stage trace, touch/effect closure, and lineage
- read planning, projection consumption, result shape, ordering, and grouping
- native aspect contracts, masks, and installed domain invariants
- live relevance, invalidation narrowing, and patch classification

**Relevant APIs**
- declaration aspect and projection consumption contracts
- query-shaped live relevance and region-scoped narrowing artifacts
- installed domain invariant and graph-obligation definitions
- installed operation semantic closure and completed execution trace
- proposed projection dependency contract, dependency role, compiled impact
  closure, and impact decision

**Warnings**
- A flat list of changed fields does not explain whether identity, membership,
  ordering, value, grouping, or window meaning changed.
- Consumers may widen their own consequences, but they may not reinterpret the
  Query impact class or declare a narrower dependency closure than Query proved.
- Domain semantic prerequisites may enter through installed definitions and
  invariants; domain crates may not install executable invalidation callbacks
  or consumer-specific recompute policy.
- Replay scope, reversal scope, lineage scope, and invalidation scope must not
  be maintained as parallel graphs. They are derived roles and closures over
  the same installed operation and execution trace.
- Cycles, ambiguity, unsupported dependency roles, and incomplete lower-runtime
  locality must deny or conservatively escalate explicitly, never silently
  produce an exact local impact.

**Test requirements**
- Adversarial closure convergence test: direct recomparison, live maintenance,
  and replay compile identical dependency roles, transitive closure, impact
  class, and counters for the same installed capability and change.
- Adversarial trace-derivation test: the compiled impact closure contains every
  declared stage read, touch, effect, invariant, output, and lineage dependency
  and cannot be narrowed by omitting a stage receipt or substituting a
  consumer-authored replay or invalidation scope.
- Adversarial role matrix test: identity, membership, ordering, projected
  value, grouping, window-boundary, support/lifecycle, invariant, and advisory
  changes produce exactly their specified impact and never silently promote
  advisory context into operational authority.
- Adversarial foreign and ambiguity test: a closure from another runtime,
  installation, generation, view definition, basis, or plan cannot classify
  impact for the target capability; cyclic, conflicting, or incomplete
  declarations fail before live or consumer work.
- Adversarial boundedness test: compiling or applying an impact for `d`
  declared dependency edges and `a` affected edges performs work proportional
  to `d` at binding and `a` at change time, independent of unrelated views,
  aspects, consumers, graph nodes, and diagnostic projections.

**Engineering decisions**
- Installed definitions and plans are authoritative for Query dependency
  meaning; Query-minted traces carry the realized dependency evidence, and
  compiled closure indexes are derived and rebuildable.
- Query emits the narrowest proven impact class: unaffected or suppressed,
  value patch, membership splice, reorder or regroup, window shift, reexecute,
  explicit rebind, replacement, retirement, or unsupported escalation.
- Foundational owns native aspect shape and contract meaning; Query owns how an
  admitted query consumes that meaning and how changes affect its result.
- Impact closure is computed before invalidation fan-out so consumers receive a
  decision, not ingredients from which they must rediscover one.
- Replay, reversal inspection, lineage, invalidation, patching, and reuse
  consume the same dependency/effect closure instead of maintaining separate
  authority maps.

**Open questions**
- Exact eager versus borrowed closure representation is an implementation
  choice governed by the same semantic and exact-counter contract.

### Phase 13: Equivalent Capability Coalescing And Managed Consumer Leases

This phase turns canonical equivalence into framework-owned shared work. Query
may coalesce bound projections only when a sealed equivalence decision proves
the exact runtime, installed package and generation, operating context, basis,
policy and tenant posture, canonical view definition, support contract, native
projection contract, dependency closure, and lifecycle state required by the
shared execution family. Each consumer receives an independently disposable
lease over the shared resource rather than a clone of execution authority.

Sharing is an admitted optimization and lifecycle contract, never a change in
canonical Query meaning. A capability that is not proven share-compatible runs
independently or returns a typed unsupported-sharing decision; matching labels,
digests, definitions, or output rows cannot authorize coalescing.

**Relevant subsystems**
- canonical declaration and bound-capability equivalence
- snapshot execution, live subscription, continuation, and managed lifecycle
- shared observer registry, delivery fan-out, cancellation, and disposal
- exact sharing, suppression, fan-out, and resource counters

**Relevant APIs**
- compatibility witnesses and managed live resources
- subscription-family sharing and lifecycle contracts
- bound execution and settlement capabilities
- proposed share-admission proof, shared execution owner, consumer observation
  lease, lease disposal receipt, and sharing counter snapshot

**Warnings**
- Cloning a packet, handle, subscription identifier, or shared pointer does not
  establish semantic equivalence or managed lifecycle.
- Automatic sharing across basis, policy, tenant, runtime, installation,
  generation, contract, dependency closure, result shape, or lifecycle
  boundaries is forbidden even if current results happen to match.
- Lease disposal must not cancel work still observed by another admitted lease,
  and a leaked consumer must not make resource ownership unknowable.
- Reference counting or global locking on every delivery can hide coordination
  cost; the sharing design must expose fan-out and lifecycle work structurally.

**Test requirements**
- Adversarial equivalence convergence test: direct and independently authored
  share-eligible equivalent capabilities admit one shared execution meaning
  and identical lease semantics regardless of declaration order or sharing-
  index rebuild.
- Adversarial non-equivalence test: foreign runtime, stale generation, changed
  basis, policy, tenant, support, projection, dependency closure, window, or
  lifecycle posture refuses sharing before execution or resource mutation.
- Adversarial lifecycle test: arbitrary lease create/drop/cancel/dispose order
  leaves exactly one resource while any lease remains, disposes it exactly once
  after the last lease, and makes use-after-disposal or cross-owner release
  uncallable or typed-denied.
- Adversarial fan-out and contention test: one shared change delivered to `l`
  leases performs exactly one underlying maintenance pass plus `l` admitted
  deliveries, exposes coordination counters, and does not scan unrelated
  capabilities, resources, or consumers.

**Engineering decisions**
- Query alone decides whether capabilities may share and retains the shared
  execution owner; consumers own only their lease and downstream consequences.
- Sharing indexes are rebuildable projections over retained active capability
  and resource authority and cannot mint or prolong a resource by themselves.
- Base projection work may share independently of consumer presentation.
  Window-specific maintenance shares only when the later bound window contract
  proves equivalent window, cursor, ordering, and continuation meaning.
- Snapshot observation, live maintenance, and continuation sharing retain
  distinct failure and cost contracts even when they reuse lifecycle machinery.

**Open questions**
- Exact registry and allocation strategy must be selected from measured
  observer topology, but it must preserve the phase, authority, and counter
  contracts above.

### Phase 14: Capability-Bound Consumer Invalidation Delta

This phase makes Query emit one typed invalidation delta from the managed bound
capability or shared execution owner whenever Query meaning relevant to a
consumer changes. The delta carries the Phase 12 impact decision and names the
exact capability generation, shared-resource generation where applicable,
target lease, affected declared access keys, result-state or lifecycle cause,
locality/region posture where admitted, continuation posture, and the Query-
owned compatibility evidence required for preservation, patching, reexecution,
rebind, replacement, or retirement.

Consumers retain authority over their consequences. Worth UI maps the delta to
UI graph nodes, allocation work, preservation policy, and mounted receipts; it
does not rediscover which Query subsystem changed or manufacture a local list
of Query invalidation surfaces.

**Relevant subsystems**
- query-shaped live invalidation and region-scoped narrowing
- bound projection lifecycle and compatibility
- compiled dependency impact and managed shared-execution fan-out
- declared native access keys and projection-consumption facts
- downstream invalidation subscription and consumer indexes

**Relevant APIs**
- query-shaped live delivery and invalidation metadata
- projection impact decisions and consumer observation leases
- proposed `WorthQueryConsumerInvalidationDelta` or equivalent
- capability-bound invalidation subscription/observation surface

**Warnings**
- A consumer enum that mirrors live, signal, async, recovery, inspection, and
  projection subsystems is a second interpretation of Query change meaning.
- A derived lookup key may locate a candidate, but the delta must retain the
  bound authority that proves the candidate is current.
- Consumers may widen their own consequence scope explicitly; Query may not
  silently broaden a narrow declared change to a whole-view invalidation.
- A consumer may not downgrade a reexecute, rebind, replace, retire, or
  unsupported impact into a local value patch.
- Invalidation lookup must not scan all bound projections or all consumer
  bindings to rediscover an exact capability match.

**Test requirements**
- Adversarial replay-convergence test: the same canonical Query evolution emits
  byte-equivalent semantic impact, invalidation deltas, and compatibility
  outcomes across one-shot recomparison, live maintenance, and replayed
  delivery.
- Adversarial foreign/stale test: a delta from another runtime, installation,
  capability generation, declaration, or lifecycle cannot invalidate,
  preserve, rebind, replace, or retire the target consumer binding.
- Adversarial boundedness test: delivering changes to `k` affected bound
  projections and `l` admitted leases performs exactly `k` indexed capability
  lookups plus `l` targeted deliveries and does not grow with unrelated
  projections, consumers, graph nodes, rows, or diagnostics.
- Adversarial authority-separation test: two consumers may derive different
  downstream consequences from the same delta without changing its Query
  meaning, compatibility evidence, identity, or counters.

**Engineering decisions**
- Query owns change meaning through a capability-bound delta; consumers own the
  semantic consequences inside their domains.
- The compiled impact decision is preserved, not re-derived, across shared-
  resource fan-out and per-lease delivery.
- Delta identity is operational only while paired with retained bound
  authority and lifecycle generation.
- Subscription and continuation are framework-managed extensions of the bound
  capability, not consumer-owned hook tables.

**Open questions**
- The exact balance between eager affected-key materialization and a bounded
  borrowed iterator must be decided from allocation and delivery cost, with
  identical semantic and counter contracts.

### Phase 15: Operational Identity Opacity

This phase separates operational identity authority from reporting and
diagnostic representation. Operational handles and witnesses expose semantic
comparison only through Query-owned methods. Digests, labels, formatted traces,
portable operation names, dependency fingerprints, equivalence tokens, lease
labels, and diagnostic fingerprints remain observable but cannot key, compare,
install, bind, share, admit, replace, rebind, reuse, release, or resume
operational state.

**Relevant subsystems**
- canonical Query identity and evidence identity
- facade reporting, diagnostics, and inspection projections
- installed-operation, dependency, sharing, replacement, and rebind indexes

**Relevant APIs**
- typed Query authority and basis identities
- evidence/reporting digest projections
- installed-operation lookup, compatibility, sharing, and lease entrypoints

**Warnings**
- Hiding a raw integer behind a newtype is insufficient if consumers can copy
  it into operational constructors or compare it as authority.
- Reporting digests and equivalence explanations may be stable for diagnostics
  without becoming installed-operation, sharing, or compatibility contracts.
- Internal indexes must retain or revalidate the authority their derived keys
  locate; a key hit alone opens no door.
- Do not remove useful inspection merely to prevent misuse; separate power from
  observability structurally.

**Test requirements**
- Adversarial representation test: copied digest bytes, debug output, labels,
  portable operation names, dependency/equivalence tokens, lease labels,
  serialized diagnostics, and colliding test fingerprints cannot satisfy any
  operational constructor, lookup result, sharing admission, lease release, or
  compatibility requirement.
- Adversarial observability parity test: reporting and inspection projections
  remain deterministically derivable from retained authority and may be
  destroyed and rebuilt without changing operational outcomes.
- Adversarial API test: external consumers cannot extract raw operational key
  material or call equality/ordering traits that substitute for the named
  Query compatibility decisions.

**Engineering decisions**
- Operational identity types remain opaque and non-promotable.
- Reporting identity types are explicitly named as projections and have no
  conversion into operational authority.
- Operational indexes return candidates that must remain paired with retained
  authority and a Query-owned validation step.

**Open questions**
- Exact trait exposure for opaque identities must be decided by the minimum
  requirements of legitimate collections without reopening generic authority
  comparison.

### Phase 16: Consumption Cost Evidence And Exact Counters

This phase makes the bound capability's costs visible at every downstream
boundary. Installed operation lookup, binding, support-contract minting,
execution handoff, compatibility, native access, dependency compilation and
impact resolution, sharing admission and lease fan-out, lifecycle transition,
invalidation delivery, collection-window resolution, patch delivery, and
denial outcomes carry structural counters sufficient to prove their declared
bounds without timing-based guesses or consumer instrumentation.

**Relevant subsystems**
- projection fact extraction and native refinement
- installed-domain operation lookup and compatibility admission
- dependency closure and impact decision
- shared execution, lease lifecycle, and delivery fan-out
- managed capability transitions and downstream settlement receipts
- consumer invalidation and collection window/patch delivery
- Query complexity contract registry and certification counters

**Relevant APIs**
- `ProjectionFactExtractionCounters`
- installed-domain drift and execution counters
- proposed downstream capability, dependency-impact, sharing, and native-access
  counter snapshots

**Warnings**
- End-to-end elapsed time cannot prove bounded semantic breadth.
- Counters added only to diagnostics are optional observability, not a
  performance contract visible to the consumer.
- Do not charge unrelated source execution work to the downstream consumption
  boundary; each boundary accounts for the work it owns.
- Exact-zero denial claims require counters for planning, lower-runtime contact,
  fact access, refinement, sharing/resource mutation, lifecycle mutation,
  invalidation, window/patch application, and authority reopen.

**Test requirements**
- Adversarial slope test: resolving an installed operation, binding it,
  compiling `d` dependencies, and accessing `k` declared facts performs one
  indexed operation lookup plus work proportional to `d + k` and remains
  unchanged as unrelated rows, fields, views, domains, installations,
  diagnostic projections, and consumers grow.
- Adversarial denial-budget test: foreign, stale, detached, wrong-contract, and
  wrong-lifecycle inputs report exact zero downstream planning, execution,
  access, refinement, and lifecycle work beyond the minimum indexed authority
  check.
- Adversarial accounting parity test: ordinary facade and internal oracle paths
  produce identical counter snapshots for the same capability journey.
- Adversarial incremental breadth test: `a` affected dependency edges, `k`
  affected capabilities, `l` admitted leases, window width `w`, and patch width
  `p` report exact independent counters and remain insensitive to unrelated
  registry, collection, consumer, and graph size.

**Engineering decisions**
- Every critical result or denial embeds its boundary-local counter snapshot.
- Native access is `O(k)` for `k` requested access keys and `O(1)` per key after
  declaration admission; compatibility is bounded by the named retained proof
  dimensions rather than global registry size; impact resolution is bounded by
  affected dependency edges; shared maintenance plus delivery is bounded by
  semantic maintenance breadth plus admitted lease fan-out; invalidation and
  patch work is bounded by affected capability and semantic patch width.
- Rich inspection may derive summaries from counters but may not alter or
  become the source of operational accounting.

**Open questions**
- None.

### Phase 17: Bound Collection Identity And Window Declaration

This phase specializes the bound projection for collection-shaped results. A
bound collection capability retains canonical result shape, entity and view-
local row identity, ordering authority, membership semantics, cursor basis,
and admitted window policy. Consumers request semantic windows through that
capability resolved from its installed view definition rather than assembling
ranges from offset pagination, result digests, copied row keys, or host-local
collection caches.

The consumer may declare presentation concerns such as viewport overscan or
mounting budget. Query owns which ordered, cursor-bound result window those
concerns select from canonical Query meaning.

**Relevant subsystems**
- collection result shape, ordering, membership, and cursor authority
- installed collection-view definition, bound projection, and declaration-
  indexed access
- view-local identity and materialized projection facts
- downstream virtualization/window request boundary

**Relevant APIs**
- collection query results and opaque cursor surfaces
- entity identity and view-local identity projection facts
- proposed bound collection capability and admitted window declaration

**Warnings**
- Array position, UI node order, display text, and host widget identity cannot
  become row identity.
- A numeric offset is not a stable cursor or window authority under live
  membership and ordering change.
- Consumer overscan policy may affect requested breadth but not Query ordering,
  membership, or continuation meaning.
- The collection capability must not require the consumer to scan all rows to
  recover stable identity or locate a declared window.

**Test requirements**
- Adversarial identity convergence test: direct collection execution, bound
  collection execution from the installed view, and equivalent window requests
  produce identical canonical row identity, ordering, membership, and cursor
  meaning.
- Adversarial reorder test: insertion, removal, ordering-key change, grouping
  movement, and unrelated row change preserve stable entity/view-local identity
  without relying on prior array position.
- Adversarial construction test: offset ranges, copied cursor bytes, consumer-
  authored row keys, reporting digests, and foreign window declarations cannot
  construct an admitted window.
- Adversarial bounded request test: declaring and resolving a window of width
  `w` performs work proportional to the admitted access plan and `w`, with no
  consumer-side full collection scan.

**Engineering decisions**
- Collection identity and ordering remain Query-owned parts of the bound
  capability; consumers receive typed row handles and declared native access.
- Window admission binds cursor, ordering, basis, capability generation, and
  consumer breadth policy into one proof-bearing request and extends Phase 12's
  dependency roles with collection membership, ordering, grouping, and window-
  boundary impact.
- Unsupported grouping, cursor, or live-window neighbors deny explicitly rather
  than degrading into offset pagination or full materialization.

**Open questions**
- Whether the ordinary DX exposes cursor-first, anchor-first, or viewport-first
  authoring remains open to transcript testing; all must lower to the same
  admitted window contract.

### Phase 18: Query-Shaped Window And Patch Delivery

This phase delivers collection evolution as capability-bound Query patches.
Insert, remove, move, update, reset-required, result-state, warning,
continuation, and window-shift posture remain attached to the exact bound
collection authority, cursor basis, compiled impact decision, and target lease
where sharing is admitted. The consumer receives a patch that can be applied to
its own graph or mounted representation without reinterpreting raw CDC, diffing
full collections, or hashing a local Query patch posture.

Worth UI translates admitted row and patch meaning into UI graph touches,
virtualized node preservation, measurement invalidation, and mounted receipts.
Query does not own UI mounting, overscan policy, or allocation.

**Relevant subsystems**
- query-shaped live collection patches and suppression
- bound collection windows, result state, and continuation
- dependency impact, managed leases, consumer invalidation deltas, and native
  row access
- Worth UI virtualized data and graph-touch handoff

**Relevant APIs**
- live collection patch artifacts and delivery receipts
- proposed bound window delivery and consumer patch capability
- exact patch-width, suppression, and continuation counters

**Warnings**
- A bundle of support, live, async, inspection, consumption, and recovery
  digests is not a collection patch contract.
- Raw CDC or source-row events cannot be exposed as consumer patch meaning.
- Full reset is an explicit typed posture with a reason and cost contract, not
  a silent fallback when incremental handling is inconvenient.
- Patch application must validate capability generation and cursor basis before
  touching consumer state, and shared delivery must validate the exact target
  lease without repeating underlying maintenance.

**Test requirements**
- Adversarial fresh-execution parity test: applying every admitted patch
  sequence yields exactly the same ordered window, native values, result state,
  and continuation meaning as fresh execution at the resulting basis.
- Adversarial delivery-order test: duplicate, stale, reordered, foreign,
  wrong-window, and wrong-generation patches cannot corrupt the admitted
  consumer window and produce typed next actions.
- Adversarial bounded patch test: a semantic patch of width `p` performs work
  proportional to `p` plus declared window-boundary effects, independent of
  total collection, UI graph, and unrelated consumer size.
- Adversarial virtualization convergence test: Worth UI consumes bound windows
  and patches without local Query posture digests, row-identity reconstruction,
  full-collection diffing, or raw CDC interpretation while preserving its own
  graph and mounting authority.
- Adversarial shared-window test: equivalent admitted windows may share exactly
  one maintenance pass while distinct leases receive independently applicable
  patches; a different cursor, ordering, basis, or window contract cannot join
  that shared resource.

**Engineering decisions**
- Query patches carry stable row identity, operation meaning, compiled impact,
  affected declared facts, result-state posture, continuation, authority,
  target lease where applicable, and exact counters.
- The consumer applies patches through a capability-validated handoff and owns
  only domain-local consequences.
- Suppression and reset admission are Query decisions; mounting and preservation
  policy remain consumer decisions.

**Open questions**
- None.

### Phase 19: Public Facade, DX, And Reference-Consumer Cutover

This phase makes installed domain operations and the bound capability the
ordinary discoverable path. The facade lets a consumer install a domain with
typed view and workflow definitions, resolve a domain-native operation, bind
it atomically across required domains, advance its typed workflow, inspect its
Query-minted trace, request admitted replay or reversal, consume typed lineage,
execute or promote it, access declared native values, ask named compatibility
questions, inspect compiled impact, acquire or dispose managed observation
leases, consume support contracts and invalidation deltas, declare collection
windows, apply query-shaped patches, and dispose managed resources without
importing internal phase topology or manually carrying authority ingredients.

Worth UI is the primary hostile reference consumer because its allocation,
rebind, and virtualization boundaries exercise installation, projection,
native refinement, support, compatibility, lifecycle, invalidation, collection
identity, and patch delivery together. The cutover removes its remaining need
to defend against recombined Query authority or reconstructed Query posture
locally.

**Relevant subsystems**
- Query domain/read/live/foundation facade modules
- consumer kit and external compile transcripts
- `worth-ui-query-binding` and other serious downstream reference consumers
- Query discovery documentation

**Relevant APIs**
- installed-domain capability facade
- installed view/workflow resolution, bound projection facade, and declaration-
  derived access grammar
- workflow-run, replay, reversal/compensation, lineage, and persistent-naming
  admission grammar
- consumer support/invalidation contract, collection window, patch delivery,
  named compatibility, dependency impact, sharing lease, and lifecycle methods

**Warnings**
- A facade that reexports every ingredient and internal phase type has not
  simplified or sealed the boundary.
- Do not preserve detached constructors for tests; hostile fixtures belong in
  certification-only support.
- Examples teach only the positive current path, not historical methods framed
  as forbidden alternatives.
- DX measurements must include imports, authority decisions, lifecycle cleanup,
  and denial routing—not only line count.

**Test requirements**
- Adversarial DX transcript: an external crate completes install, declaration,
  installed operation resolution, binding, snapshot execution, native access,
  live promotion, compatibility, shared observation, explicit rebind,
  workflow advancement, replay comparison, aftermath handling, lineage-aware
  continuity, invalidation, collection window delivery, patch application,
  inspection, lease release, and disposal using ordinary facade modules with
  no local schema reconstruction, stage ledger, replay or undo catalog,
  lineage string, dependency graph, manual digest, raw path, hook enum, deep
  import, or detached receipt plumbing.
- Adversarial reference convergence test: Worth UI and at least one non-UI
  consumer produce the same canonical results, authority decisions, denials,
  and counters through the bound capability as Query's internal oracle.
- Adversarial deletion test: consumer-local authority bundles, basis
  comparators, stable domain-query builders, schema/selector/result-shape
  copies, dependency graphs, recompute policies, sharing registries, support-
  hook mirrors, digest folds, fact scans, invalidation surface enums,
  collection patch posture, lifecycle joins, workflow stage ledgers, replay
  catalogs, undo-scope registries, persistent-naming strings, and compatibility
  mirrors can be deleted without preserving fragments elsewhere.

**Engineering decisions**
- Progressive disclosure begins with one installed domain operation and one
  bound capability; advanced inspection does not expose construction power.
- Worth UI retains ownership of Query-to-allocation semantic refinement but no
  longer reconstructs Query installation, basis, compatibility, or lifecycle
  authority, support posture, dependency impact, sharing eligibility,
  invalidation meaning, row identity, or patch semantics.
- Compiler diagnostics and typed denials route callers to the nearest legal
  facade transition.

**Open questions**
- Final method names remain open to golden transcript review; the authority
  grammar and required journey are fixed.

### Phase 20: Mechanical Prohibitions And Hostile Certification

This phase makes regression structurally expensive. Compile-fail probes,
facade snapshots, boundary checks, source residue audits, sabotage fixtures,
and provider-independent semantic oracles prove that authority ingredients
cannot reappear as an alternate public path.

**Relevant subsystems**
- Query facade and module visibility
- compile-pass/compile-fail consumer harnesses
- boundary checker and source/API residue certification
- Milestone 13 provider-independent certification oracle

**Relevant APIs**
- public facade snapshots and semver surface checks
- certification-only hostile fixture constructors
- downstream capability parity bundle
- installed-operation, dependency-impact, sharing, and lease parity bundles
- reusable installed-operation/workflow certification kit with generic
  compile-fail proofs and domain-supplied semantic scenario fixtures

**Warnings**
- Source scans alone cannot prove runtime-affine authority continuity.
- Runtime rejection alone is weaker than making detached assembly uncallable.
- Certification support must not leak hostile constructors into production
  facade modules.
- Do not freeze filenames or incidental topology; enforce authority direction,
  forbidden construction, and responsibility boundaries.
- Do not duplicate hundreds of consumer-local compile harnesses. Query owns the
  generic authority-impossibility matrix; domain packages contribute narrow
  semantic fixtures and runtime scenarios for their installed operations.
- Runtime-generated compile-fail claims are dishonest. Compile-fail proof is a
  central static harness; the reusable consumer kit supplies runtime matrices,
  bounded fixtures, sabotage cases, and semantic parity oracles.

**Test requirements**
- Adversarial sabotage matrix: foreign installation, stale generation,
  locally reconstructed installed operation, detached completion, copied
  consumed facts, wrong declaration key, forged dependency closure, reporting
  digest collision, forged support projection, false sharing-equivalence token,
  wrong compatibility witness, foreign lease or invalidation delta, copied
  cursor, stale collection patch, disposed lifecycle, cross-run stage receipt,
  forged replay or reversal scope, representation-derived lineage, and cross-
  provider artifacts all fail at the earliest boundary with exact counters.
- Adversarial facade/prohibition test: public API snapshots and compile-fail
  probes report zero consumer-executable portable operation definitions,
  detached capability constructors, raw operational identity extractors,
  consumer-mintable access/window/dependency/share keys, support/invalidation
  constructors, patch constructors, or lifecycle entrypoints that skip bound
  predecessors.
- Adversarial residue test: production source and reference consumers contain
  zero stable domain-operation reconstruction, operational digest comparisons,
  consumer-side authority assembly, local dependency or recompute authority,
  consumer-owned sharing registries, broad fact/collection scans, local support
  or invalidation mirrors, local basis compatibility logic, patch-posture
  hashes, raw CDC interpretation, or orphan lifecycle joins.
- Adversarial oracle-export test: Milestone 13 can drive the complete bound
  capability journey against a second admitted provider without weakening or
  reinterpreting Query semantics.
- Adversarial certification-kit test: a representative downstream domain can
  register operation semantics and contribute small workflow, replay,
  reversal, lineage, impact, and counter scenarios without rebuilding generic
  authority compile-fail tests or a product-specific evidence ledger.

**Engineering decisions**
- The certification matrix combines compile-time impossibility, typed runtime
  denial, exact counter proof, and reference-consumer deletion evidence.
- Boundary rules name allowed production owners and reject dependency or
  reexport bypasses mechanically.
- The provider-independent parity bundle becomes a required Milestone 13 input,
  not a separate interpretation of Query meaning.
- Generic construction impossibility is certified once at Query's facade;
  downstream certification is proportional to domain semantics rather than the
  cross-product of authority ingredients.

**Open questions**
- None.

## Must Ship

- complete typed installed operation semantic closure with indexed runtime
  resolution and no consumer-local reconstruction of stable operation meaning
- one sealed runtime-affine bound projection capability
- atomic multi-domain operation binding
- one Query-minted consumer support contract
- proof-bearing installed execution, consumption, and settlement progression
- installed typed workflow DAGs, Query-minted run/stage traces, and legal
  predecessor progression
- semantic replay and re-execution admission with distinct attempt identity and
  typed convergence or divergence
- exact-inverse, compensation, rebuild/recovery, provisional-discard, and
  irreversibility contracts with no scope-only undo
- trace-bound typed lineage and identity-evolution evidence for persistent
  naming and consumer continuity
- declaration-indexed Foundational-native value access and typed denials
- Query-owned installation, basis, replacement, rebind, and reuse decisions
- framework-owned live, replacement, rebind, cancellation, and disposal states
- Query-compiled semantic dependency roles, impact closure, and typed impact
  decisions
- explicit equivalent-capability sharing admission, framework-owned shared
  execution resources, and independently disposable consumer leases
- capability-bound consumer invalidation deltas
- opaque operational identities separated from reporting projections
- embedded exact structural counters and named complexity contracts
- bound collection identity/window declarations and query-shaped incremental
  patch delivery
- contracted facade, positive discovery path, Worth UI reference adoption, and
  reusable operation/workflow certification kit with a hostile certification
  matrix

## Must Preserve

- Foundational ownership of native scalar, struct, absence, contract, and
  canonical value meaning
- Query ownership of declaration, installation, basis, execution, projection
  consumption, installed operation meaning, consumer support, compatibility,
  dependency impact, sharing admission, invalidation, collection identity/
  ordering/cursors, patch meaning, and managed query lifecycle
- Query ownership of atomic cross-domain binding, workflow progression and
  trace assembly, replay admission, reversal binding, lineage evidence
  transport, and typed identity-evolution outcomes
- domain ownership of stage algorithms, inverse and compensation semantics,
  persistent naming policy, correspondence policy, and product meaning
- Store ownership of durable journals, checkpoints, restart replay, and
  recovery persistence
- lower-runtime ownership of truth, reactive scheduling, and physical provider
  semantics
- backend-independent canonical Query meaning and provider parity
- distinct cost, failure, correctness, and lifecycle boundaries for snapshot,
  live, replacement, rebind, reuse, inspection, and disposal
- distinct Query change meaning and consumer-local consequence authority
- canonical equivalence and shared execution remain distinct: equivalence may
  admit reuse, but derived sharing indexes and leases never become declaration
  or execution authority
- portable domain operation definitions remain distinct from installed
  runtime-affine operation capabilities
- replay remains distinct from reversal, execution identity remains distinct
  from semantic equivalence, and touch/effect scope remains distinct from
  inverse authority
- Query remains a generic semantic framework rather than becoming a geometry
  kernel, product workflow registry, or executable callback host
- consumer ownership of viewport, overscan, graph, mounting, allocation, and
  presentation policy
- portable declarations and reporting projections remain non-authoritative and
  readmittable rather than secretly runtime-affine

## Acceptance Evidence

- equivalent installed view and workflow definitions converge, conflicting
  definitions fail atomically, and local lookalikes cannot execute
- invalid workflow graphs fail package installation, and independently valid
  stage receipts or domain capabilities cannot be combined into legal
  progression or atomic operation authority
- admitted replay creates a distinct run/attempt identity while proving the
  installed operation's declared semantic equivalence; meaningful divergence
  localizes to typed trace, result, effect, or lineage evidence
- scope-only undo is impossible; exact inverse, compensation, rebuild/recovery,
  and irreversibility remain typed and separately testable
- persistent naming cannot be authorized from raw strings, identifiers,
  digests, coordinates, rendered values, or advisory correspondence
- independent valid artifacts cannot be recombined into operational authority
- the ordinary facade carries one exact installation-to-settlement authority
  chain with no consumer restamping
- declared native access preserves complete Foundational parity and bounded
  indexed cost
- named compatibility decisions return pair-bound witnesses or typed denials
- live and rebind lifecycle states are framework-owned and leave no orphans
- compiled dependency roles and impact decisions converge across one-shot,
  live, and replay paths without consumer-local closure or recompute authority
- equivalent capabilities share only through Query admission; one underlying
  maintenance pass fans out through independently disposable typed leases with
  exact counters
- support requirements and invalidation meaning come from Query-minted typed
  artifacts rather than consumer hook enums or digest bundles
- collection windows and query-shaped patches preserve row identity, ordering,
  cursor, result-state, continuation, and fresh-execution parity
- reporting representations have zero operational effect
- invalid, stale, foreign, detached, and disposed inputs deny before expensive
  work with exact counter proof
- Worth UI deletes local Query authority, support, invalidation, row-identity,
  stable operation reconstruction, dependency/recompute policy, sharing
  registry, workflow stage ledger, replay and undo catalogs, lineage-string
  reconstruction, and patch-posture reconstruction while preserving its graph,
  virtualization, allocation, mounting, and presentation semantics
- a representative operation/workflow certification kit proves generic
  authority impossibility centrally and accepts domain-owned semantic fixtures
  without duplicating a product-specific compile-test matrix
- compile-fail, facade, residue, sabotage, parity, and provider-oracle suites all
  agree on the single capability path

## Sequencing Notes

This milestone follows 9.13 because it hardens the installed-domain and
Foundational-native product boundary that 9.13 establishes; it must not invent
parallel declaration or value semantics while 9.13 remains open. Installed
domain operation definitions come first because atomic binding, workflow
progression, replay, reversal, lineage, dependency impact, sharing equivalence,
invalidation, and patch delivery must all consume one canonical operation
meaning rather than reconcile locally authored lookalikes. Workflow graph and
trace, replay, reversal, and lineage close before native access and downstream
lifecycle work because every later compatibility, impact, reuse, invalidation,
collection, and certification decision must consume the trace rather than
retrofit a parallel ledger.

This milestone consumes the graph-read planning of Milestone 9.10, graph-touch
authority of Milestone 9.9, deterministic parallel admission of Milestone 5.3,
authority-scoped effect execution of Milestone 9.3.3, and existing lineage and
identity-evolution contracts from Milestones 5.4 and 7. It binds and composes
those capabilities; it does not rebuild them.

It precedes Milestone 13 because Query's provider-independent certification
oracle must export the non-detachable capability journey, not the lower-level
ingredient surfaces that this milestone removes. It must include consumer
support, workflow trace, replay, reversal, lineage, dependency impact, sharing
and lease lifecycle, invalidation, collection windows, and patch delivery so
providers do not reinterpret them.
Store integration consumes that same oracle and capability contract through
the Store roadmap. The milestone is not blocked on Store and makes no physical
persistence claim.
