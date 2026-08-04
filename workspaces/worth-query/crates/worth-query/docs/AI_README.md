# Worth Query Orientation For AI Agents

This document is the broad orientation corpus for AI agents building real
applications, domain integrations, and downstream runtimes on top of
`worth-query`. It is intentionally comprehensive. An agent should be able to
read it once and know which capabilities exist, who owns their authority, how
the major journeys compose, and where the exact API and examples live.

It answers four questions throughout the Query surface:

1. What category of thing am I touching?
2. What does that category actually do?
3. Which authority owns its meaning and lifecycle?
4. Which docs contain the exact signatures, examples, and limits?

This file does not replace rustdoc or the linked feature guides. It preserves
the whole-system mental model that lets an AI use those references correctly.
Do not shorten it into a table of contents: breadth here is deliberate because
unseen capabilities are easily reimplemented as competing local systems.

## Runtime Stack

Worth Query lives in this stack:

```text
domain crate / application
-> worth-query-decl + worth-query-host
-> worth-query-installation
-> worth-query-admission
-> worth-query-execution
-> worth-query-publication
-> worth-relational + worth-runtime-bridge + worth-signal
```

That layering matters. Query is not a thin read helper over lower runtime
systems. The declaration and host facades are the ordinary application entry;
installation owns sealed meaning, admission owns review and capacity,
execution owns sessions and terminals, and publication owns non-authoritative
receipts. The `worth-query` monolith still owns unrelated generic runtime
features and one-way integration adapters, but it is not the destination owner
of the canonical application graph-obligation progression. None of these
layers absorbs the physical authority of the lower runtimes it coordinates.

`worth-runtime-bridge` owns the causal protocol layer that wires authoritative
truth to derived computation without collapsing either runtime into the other:
patch-to-invalidation and snapshot-backed evaluation over committed truth,
aspect mapping and fine-grained subscriptions, lineage-aware continuity across
replace/split/merge-like identity evolution, historical and branch-aware
evaluation, planned bulk routing with canonical reduction, change-stream and
reactive-source protocols, structural-identity-assisted remapping, merge-bearing
history consumption, speculative branch coordination and preview flows,
cross-runtime policy propagation, bridge-mediated commit strategies and
extensible writeback families, subscription declaration/admission/lifecycle with
checkpointed delivery and shared fanout, temporal basis binding for mixed
truth-and-clock causality, and async/resource completion causality with
generation-safe stale-completion rejection—all expressed through deterministic,
replay-safe routing plus machine-checkable receipts, envelopes, and diagnostics
that record how work crossed boundaries while leaving truth authority,
invariants, merge execution, and signal scheduling to their owning runtimes.
`worth-relational` owns lower truth semantics and authoritative state mechanics:
transactional commit authority, savepoints and rollback, literal MVCC snapshots,
version and history substrate, deterministic patch and replay publication, CDC
and subscriber recovery, schema contracts and schema evolution/reconciliation,
structural identity and historical inspection, relational invariants, joins,
lineage/correspondence, merge-ready history and merge execution, bulk query and
bulk mutation surfaces, and the constraint-bearing truth model that keeps
concurrent reads, historical views, recovery, and authoritative mutation
coherent without collapsing into caller-owned bookkeeping.
`worth-signal` owns derived-computation infrastructure beneath the bridge: an explicit
dependency graph with aspect-aware and maybe-stale invalidation, conditional
and policy-aware evaluation gates, transactional invalidation with hard rewind,
lazy pull and reactive diff propagation with partial recomputation boundaries,
structural memoization and query-style keyed incremental execution, deterministic
versus optimized scheduling with cost and priority shaping and parallel-ready
planning, speculative branchable execution with snapshot/replay/time-travel
state, fixed-point and convergence policies, temporal and previous-value
dependencies, comparator and adaptive tolerance propagation, and first-class
observation with extensible delivery strategies—all over host snapshots without
owning truth storage, while exposing execution traces, graph inspection, and
metrics that explain why work ran, deferred, or delivered change.

Execution-grade installed computation makes this ownership split especially
visible. Query owns the managed-run and provider-session progression. Runtime
Bridge binds the exact request to its causal execution basis. Relational owns
the authoritative basis and eventual truth mutation. Signal owns cancellation,
pressure, and bounded scheduling state. Registered providers own physical
sessions, native artifacts, provisional overlays, and invariant-state loading.
Joining those authorities does not let any participant fabricate another.

Ordinary domain work starts at Query. Use lower layers to understand semantics,
not as permission to bypass Query.

For the complete installed obligation, graph-read plan, session, terminal, and
publication map, read [Canonical Graph Obligation Progression](./domain-capabilities/canonical-graph-obligation-progression.md).

## The Core Rule

The governing Query rule is:

```text
declare intent once
lower it once
execute or inspect it through canonical runtime-owned artifacts
```

That rule explains most of the architecture.

Query wants domain code to express work once, keep that work canonically
identified, and let the runtime lower it through public, typed lanes instead of
forcing every downstream crate to invent local wrappers, local status enums,
local recovery folklore, or local “smart” adapters around lower layers.

If you are about to invent a local pseudo-Query surface, a hidden recovery path,
or a caller-owned translation layer that duplicates a Query lane, stop and
check whether the category you need already exists below.

## How To Use This File

Read this file as a corpus, then use it in two passes while working.

First, find the category that matches the problem you are solving. Each section
explains what that category is for, when to reach for it, and what mistake to
avoid.

Second, jump to the linked docs at the end of that category. Those docs carry
the detailed surface, examples, execution model, debugging guidance, and
current limits. Keep the surrounding categories in mind: Query features share
basis, support, authority, outcome, and lower-runtime boundaries even when
their public namespaces differ.

If you have no idea where to start, read these first:

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Docs README](./README.md)
- [Workspace Overview](./foundations/workspace-overview.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Downstream Runtime Integration](./foundations/downstream-runtime-integration.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)
- [Installed Computation Artifact Contracts](./domain-capabilities/installed-computation-artifact-contracts.md)
- [Managed Artifact Ownership And Native Access](./domain-capabilities/managed-artifact-ownership-and-native-access.md)
- [Execution Resource Admission And Managed Runs](./domain-capabilities/execution-resource-admission-and-managed-runs.md)
- [Provider Sessions And Decision Read-Sets](./domain-capabilities/provider-sessions-and-decision-read-sets.md)
- [Provisional State And Invariant Execution](./domain-capabilities/provisional-state-and-invariant-execution.md)
- [Installed Operation Re-Execution And Replay](./domain-capabilities/installed-operation-reexecution-and-replay.md)
- [Installed Operation Aftermath](./domain-capabilities/installed-operation-aftermath.md)
- [Installed Operation Lineage And Promotion](./domain-capabilities/installed-operation-lineage-and-promotion.md)
- [Bound Projection Lifecycle, Sharing, And Consumer Invalidation](./domain-capabilities/bound-projection-sharing-and-invalidation.md)

## Declarative Capability Surface

Ordinary product code starts in a capability namespace and follows one grammar:

```text
declare intent -> refine it -> using(context) -> run(...) or open(...) -> typed outcome
```

Choose `facade::read`, `facade::aggregate`, `facade::live`, `facade::history`,
`facade::comparison`, `facade::preview`, `facade::mutation`,
`facade::workflow`, `facade::inspection`, or `facade::installed` based on the
product job. Use `facade::domain` only to declare or assemble an installed
domain. Query owns the canonicalization, authority admission, planning,
lower-runtime selection, execution, lifecycle, receipts, and stops behind
those surfaces.

Use this category whenever a consumer is tempted to import phase artifacts,
choose a backend, pair basis digests with receipts, or split live activation,
maintenance, and closeout across local helpers.

Read next:

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Read Composition](./authoring/read-composition.md)
- [Live Views](./runtime-surfaces/live-views.md)
- [Historical Diff And Basis](./capabilities/historical-diff-and-basis.md)
- [Projection Consumption](./capabilities/projection-consumption.md)
- [Inspection](./capabilities/inspection.md)

## Public Runtime Facade

The public facade is a set of explicit capability namespaces, not one barrel
that mirrors Query's implementation tree:

- `worth_query::facade::{read, aggregate, live, history, comparison}` owns
  query-shaped observation.
- `worth_query::facade::{preview, mutation, workflow}` owns declared change and
  promotion journeys.
- `worth_query::facade::domain` owns portable installed-domain package,
  operation-definition, and runtime-assembly grammar.
- `worth_query::facade::installed` owns ordinary installed-operation
  consumption: one operating-world root, typed family views, bound operations,
  and guarantee-specific progression modules.
- `worth_query::facade::inspection` owns outcome-attached inspection.
- `worth_query::facade::runtime` contains workspaces and backend-owned runtime
  products used by those journeys.
- `worth_query::facade::consumer_kit` contains downstream adoption, support
  pinning, evidence, test-runtime, and residue-proof tools.
- `worth_query::facade::certification` contains manifests, generated checks,
  and hostile-test tooling. It is not an ordinary product import.

Start with the capability namespace that names the task. Foundation, policy,
and lower runtime surfaces are substrate and advanced accountability seams;
they are not the ordinary journey to reconstruct in consumers.

Visibility and support are separate. A public type can describe vocabulary for
a deferred neighbor without making that neighbor an admitted runtime lane.
Check the support matrix for the active profile.

Use the installed consumer surface explicitly:

```rust
use worth_query::facade::{installed, runtime};
```

Hosts that declare packages or register volatile execution mechanics use the
assembly surface instead:

```rust
use worth_query::facade::{domain, runtime};
```

An entry-band host can use the narrower host audience:

```rust
use worth_query_host::facade::{domain, installed, runtime};
```

Execution-grade installed computation uses more specific host audiences:

```rust
use worth_query_host::facade::{
    admission::resource_admission,
    convergence_epoch,
    domain,
    installed::{domain_computation, provider_session},
    publication,
    runtime,
};
```

`domain` owns callback-free installed meaning. `resource_admission` owns
capacity admission and reservation. `domain_computation` owns managed artifacts,
runs, provider-plan progression, decision read-sets, provisional attempts, and
invariant progression. `provider_session` is the host integration surface for
physical providers, not an ordinary domain-consumer callback lane.
`convergence_epoch` observes managed terminal posture. `publication` remains a
separate authority boundary.

Pure schema and meaning crates remain Query-agnostic. They expose portable
meaning that an entry-band crate installs through `worth-query-decl` or
`worth-query-host`; they do not import Query to acquire runtime authority.

Reach for this category when the task sounds like ordinary runtime-backed
product behavior: declaring retained surfaces, reading them, mutating truth,
opening preview or branch sessions, inspecting retained handles, or deciding
whether a public family is really supported today.

The mistakes to avoid are importing from `worth_query::facade` as a flat
barrel, using `facade::domain` as the ordinary installed-operation consumer
path, importing raw `worth_proof` progression in product code, using
`facade::certification` in production code, and teaching support from
autocomplete.

Read next:

- [Workspace Overview](./foundations/workspace-overview.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Basis Capability Lifecycle](./capabilities/basis-capability-lifecycle.md)
- [Consumer Kit](./foundations/consumer-kit.md)
- [Query Operating Modes](./foundations/query-operating-modes.md)

## Query Operating Modes

Query deliberately supports more than one honest execution posture. The same
canonical query meaning can run against an admitted relational snapshot,
promote to live maintenance without changing the query expression, or exist as
an ephemeral saved-query or host-bound artifact.

Use this category when the question is not “how do I author the query?” but
“which execution posture is real today, and what completion debt is still open?”

The mistake to avoid is claiming a stronger execution or cursor posture because
a helper name sounds persistent. Only the currently admitted profile is real.

Read next:

- [Query Operating Modes](./foundations/query-operating-modes.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Workspace Overview](./foundations/workspace-overview.md)

## Shared Read Authority And Journal Replay

Shared read authority, mutation intake, derived publication, and replay are
separate named runtime lanes.

This section describes Query's general shared-read and journal replay
capabilities. Installed-operation semantic replay is a separate cert-only
lane: it re-executes a retained installed workflow, compares the exact semantic
trace, exposes replay authority only through `worth-query-replay`, and returns
inspection semantics rather than Query's ordinary completed-trace phase type.

The important rule is that shared reads are real runtime-owned read authority,
not copied snapshot convenience. A shared read context is basis-bound, sealed,
and backed by generation pinning. Readers consume already-published facts; they
do not evaluate derived state, warm caches, repair indexes, or trigger bridge or
signal work as a side effect of looking.

Submission is the single writer lane. Journal order is represented by typed
journal position and journal segment identity, not by parsing commit labels,
receipt strings, or display text. Consumer replay asks the runtime to replay a
typed segment and returns ordinary receipts and artifacts; it does not expose a
raw journal format as the public contract.

Published derived artifacts are read through projection consumption. If a
public bridge or downstream runtime needs materialized facts, it should consume
typed projection receipts rather than spelunking materialization rows or
bridge-only helper state.

Use this category when work touches concurrent reads, submission order, replay,
published derived artifacts, or public-bridge read certification.

The mistakes to avoid are copied-snapshot "pinning," `Mutex` or `RwLock`
around committed-read hot paths, reader-side derived evaluation, string-derived
journal order, and direct materialization-row reads where projection
consumption owns the public lane.

Read next:

- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Projection Consumption](./capabilities/projection-consumption.md)
- [Read Composition](./authoring/read-composition.md)
- [Query Operating Modes](./foundations/query-operating-modes.md)

## Support And Admission

Support and admission explain what the runtime actually promises today.

Query exposes some public vocabulary before every neighbor is fully closed. That
is intentional: it lets downstream runtimes plan against the final public shape
without pretending everything is already implemented. The support matrix exists
to make that distinction explicit. It tells you what is stable, what is
deferred debt, what is visible-but-not-admitted yet, and what must fail closed.

Use this category whenever the real question is not “does this method exist?”
but “can I build on this honestly right now?” This is especially important near
intent-shaped families, temporal neighbors, async/resource neighbors, and
anything that looks like a future extension point.

The mistake to avoid here is assuming that visibility implies support. Query
wants support posture to be machine-checkable, not guessed from API surface
shape.

That same machine-checkable posture applies to identity and denial handling.
If a caller is matching runtime denial text, formatting values into digests, or
passing raw strings into preview or branch entry, it is bypassing the
supported ordinary path even if the surrounding surface compiles.
The same warning applies to workflow preview contribution authoring: use the
typed preview-session identity artifact instead of smuggling preview identity
through free-form strings.
If identity matters to support, replay, inspection, workflow binding, or
recovery, prefer the Query-owned typed artifact over a caller-owned string.
Those typed stops are the authority boundary; diagnostic terminal text is not.

The application support surface publishes
`support_report().identity_boundary_closure()`. Read that posture literally:

- `Closed` means the ordinary runtime-backed identity boundary is live and the
  hostile residue scans are clean
- `Partial` means the typed closure work exists but the current support posture
  does not expose the full ordinary path or a same-class residue class is
  still open
- `Open` is reserved for genuinely unclosed posture, not for "not checked yet"

Read next:

- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Downstream Runtime Integration](./foundations/downstream-runtime-integration.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)

## Consumer Kit

The Consumer Kit is Query's product surface for downstream proof. It is the
ordinary downstream path for any crate that needs to prove it consumes Query
correctly. It is not optional convenience, not a testing helper pile, and not a
wrapper around folklore that the consumer still owns.

The practical meaning is blunt:

```text
consumer owns domain facts and source files
Query owns proof of Query consumption
```

So when a downstream crate needs digest-bearing evidence, hard-prohibition
enforcement, support posture pinning, in-memory Query test workspaces, or
adoption/residue proof, it should enter through
`worth_query::facade::consumer_kit` instead of building local proof machinery.

If the downstream need is read-only proof or diagnostics inspection, the same
boundary still applies: read the canonical public facade artifact and its typed
inspection getters. Do not satisfy that contract through support wrappers, raw
rows, or local helper explanations that rediscover planner-owned routing.

The required Consumer Kit families are:

- `evidence-report-kit`
- `hard-prohibition-registry`
- `boundary-audit`
- `support-snapshot`
- `support-pinning`
- `in-memory-test-backend`
- `consumer-residue-audit`

Use this category when a downstream crate is about to hand-roll report structs,
format digests, grep for forbidden Query seams, assemble required support rows,
or build runtime adapter piles just to test Query behavior. Those patterns are
folklore. The kit gives the consumer typed declarations, sealed evidence,
runtime-owned audits, support-profile pinning, and honestly postured test
workspaces through the public facade.

Choose the surface by proof job:

- use `EvidenceReportDeclaration` for sealed evidence reports and canonical
  report identity
- use `hard_prohibition_registry()` and `hard_prohibition_boundary_audit()` for
  Query hard-prohibition enforcement
- use `project_workspace_support_snapshot(...)` for a schema-versioned
  projection of the live support matrix
- use `support_pinning_contract(...)` when a consumer must fail on support-row
  regressions
- use `in_memory_test_runtime()` and `WorthQueryTestBackendSchema` when tests
  need a real `WorthQueryWorkspace` without adapter or receipt fabrication
- use `query_consumer_residue_audit(...)` when a consumer must prove it did
  not rebuild Query proof locally through report structs, proof structs, raw
  support-row spelunking, support-matrix row searches, debug-derived proof
  strings, or delimiter-derived proof strings
- use the adoption audits when a reference consumer must prove it deleted
  Query-owned folklore rather than merely hiding it

`query_consumer_residue_audit(...)` returns a typed
`WorthQueryConsumerResidueReport`, not a lint string. The report carries typed
findings, finding identities, report identity, audited source paths, skipped
non-Rust source count, and a source-inventory digest. Consumers should assert
that report and inventory evidence directly. They should not build local source
manifests, local residue classes, local scanners, or local replacement
matrices around it.

The generic residue audit owns Query-proof folklore across Consumer Kit
adoption: fake reports, fake proofs, raw support rows, row searches, debug proof
strings, and delimiter proof strings. Installed graph obligations do not add a
second Consumer Kit registry or executor. Use the host facade's read-only
`inspect_installed_graph_obligations` surface and the external-consumer certification
lane for that proof.

Real support pinning means typed row identity, live row digest binding, and a
localized typed failure when a required row regresses. A checked-in list of row
names or a local admission loop is not pinning.

The shipped boundary audit is honest about its mechanism. Associated-path
coverage checks registry public-symbol suffixes. Method-call coverage is
syntax/AST based and method-name resolved, not compiler-backed type
resolution. Do not describe it as closing macro expansion, trait dispatch, or
type-alias resolution.

The closure signal for this family is Query-owned certification evidence.
Consumers inspect the public Consumer Kit reports and manifests; they do not
construct an application facade merely to manufacture a closure signal.

The mistake to avoid is teaching these kit surfaces as nice-to-have wrappers.
For downstream evidence and certification, they are the canonical lane.

Read next:

- [Consumer Kit](./foundations/consumer-kit.md)
- [Canonical Graph Obligation Progression](./domain-capabilities/canonical-graph-obligation-progression.md)
- [Downstream Runtime Integration](./foundations/downstream-runtime-integration.md)
- [Hard Prohibitions](./foundations/hard-prohibitions.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Certification Surfaces

`facade::certification` is the explicit non-ordinary audience for inspecting
machine-checkable Query evidence. It includes the domain-capability inventory,
representative and slope reports, installed-domain closeout bundles, native
value closeout, and capability-specific certification families. These values
describe and summarize public lanes; they do not become runtime authority and
do not belong in product execution.

Use `worth_query_domain_capability_certification_surface()` and
`worth_query_domain_capability_public_surface_inventory()` to inspect the
declared domain-capability surface. Use `certify_domain_capabilities()` for the
corresponding executable evidence bundle. Installed-domain and native-value
closeout use their dedicated certification functions and returned bundles.

Keep the evidence strategy proportional. Ordinary integration tests should
prove ordinary journeys. Use compile-fail fixtures selectively when the actual
product guarantee is compiler rejection, such as private construction or
move-only phase ordering. Use hostile runtime tests for stale, foreign,
cross-runtime, cross-basis, semantic-drift, and exact-counter behavior. Do not
write a second layer of tests merely to prove that these tests or manifests
exist.

Installed-operation certification enters through the cold
`worth-query-certification` crate. That crate depends on
`worth-query-host` and `worth-query-replay`, never Query implementation
modules. It owns the generic hostile matrix, provider-independent parity
oracles, exact denial and counter evidence, compile-fail authority checks, and
executable external-consumer journeys. Ordinary packages cannot depend on this
kit.

Execution-grade installed computation adds consumer evidence for managed
artifacts, resource admission, yield and same-runtime readmission, provider
session ordering, complete decision read-sets, provisional overlays, and real
invariant execution. Compile-fail cases prove that callers cannot construct
session tokens, checkpoints, decision facts, proposal bases, provisional
programs, invariant receipts, or later-phase progression. The passing audience
journey proves the public host facade can advance from a running managed run
through fresh decision evidence and invariant progression without importing
implementation modules. Keep these proofs causal: do not multiply every
scenario across every category when one representative journey plus
boundary-specific hostile cases proves the guarantee.

Read next:

- [Certification Surface And Closeout Bundle](./domain-capabilities/certification/certification-surface-and-closeout-bundle.md)
- [Goldens, Boundaries, And Hostile Certification](./domain-capabilities/certification/goldens-boundaries-and-hostile-certification.md)
- [Installed Operation Certification Kit](./domain-capabilities/certification/installed-operation-certification-kit.md)
- [Installed Domain Closeout Evidence](./domain-capabilities/platform-entry-closeout.md)
- [Domain Capability Documentation Certification](./domain-capabilities/public-doc-coverage.md)

## Canonical Graph Obligation Progression

Every installed application query or operation owns one sealed graph-obligation
set. The set is derived from typed installed contracts; consumers do not
register rows or attach runtime callbacks. Its fixed kinds are `GraphRead`,
`AuthorizationObservation`, `MutationTouch`, `EffectApplication`, and
`InvariantExecution`.

The ordinary progression is:

```text
installed obligation set
  -> exact selection
  -> requirement, cost, budget, and capacity admission
  -> typed-branch and basis-bound managed session
  -> actual Relational / Runtime Bridge / Signal / provider work
  -> read or commit terminal
  -> non-authoritative publication and inspection
```

Selection is installed meaning, not execution. An invariant succeeds only when
the installed provider executes it against the exact proposed state and
Relational seals the validated candidate. A read succeeds only after the
session-owned read port returns actual work and cleanup evidence. Publication
can describe those terminals but cannot execute or commit anything.

Application-query current, continuation, historical, preview, and live lanes
all consume the same installed graph-read planning authority. Each attempt
still has its own plan, session, typed branch, and branch-qualified basis. An
equal numeric version from another branch is foreign evidence.

The `worth-query-host` adoption surface is read-only. It exposes installed row
identity, kind, required owners, terminal requirement, and selector-index size;
it exposes no registration, planning, execution, invariant, commit, or
publication transition. The specialized monolith obligation registry,
selection-backed executor, manual invariant callbacks, and authority-capable
Consumer Kit path are retired. Unrelated generic workspace read and graph
composition features remain preserved.

Read next:

- [Canonical Graph Obligation Progression](./domain-capabilities/canonical-graph-obligation-progression.md)
- [Graph Touch Obligation Authority](./authoring/graph-touch-obligation-authority.md)
- [Graph Read Access Planning](./authoring/graph-read-access-planning.md)
- [Provider Sessions And Decision Read-Sets](./domain-capabilities/provider-sessions-and-decision-read-sets.md)
- [Provisional State And Invariant Execution](./domain-capabilities/provisional-state-and-invariant-execution.md)

## Graph Read Access Planning

Graph read access planning is one of Query's core runtime advantages. It lets
Query take a declared graph-shaped read and prove, before and after execution,
which access structures made that read honest.

The thing Query does here that ordinary ORMs, graph helpers, reactive runtimes,
and application frameworks do not do is make graph-read access a proof-bearing
runtime lane. Query does not merely run a traversal, infer an index behind the
caller, or ask the caller to trust that a helper avoided N+1 work. It derives
the required adjacency, predicate, ordering, frontier, visited-set, result, live
maintenance, streaming, persistent-index, or materialization support from the
read declaration; admits or denies that shape against typed runtime support;
and then emits receipts proving which plan was consumed and which counters were
observed.

The declaration is the authoring surface. The access plan is the accountability
surface. Query derives access requirements from read declarations, admits those
requirements against runtime support rows and budgets, and proves execution
through the read receipt's access-plan consumption counters.

This is not graph touch obligation authority. Obligation authority decides which
graph meaning must be checked. Graph read access planning decides which access
structures are required to read graph-shaped data without caller-owned relation
loops, hidden N+1 traversal, broad RAM expansion, or surface-local graph caches.

This is also not a magic "route resources" or "automatic index everything"
feature. The endpoints and access shapes stay visible as declared read families,
admitted access plans, typed required-capability postures, and receipt fields.
If Query cannot prove a safe runtime-owned path, it returns a typed denial or a
typed required posture instead of silently crossing into caller-owned traversal.

Use this category when a task mentions graph read cost, adjacency indexes,
frontier scans, broad boolean graph predicates, read-family access plans,
streaming graph reads, persistent index requirements, async materialization, or
receipt counters for no-N+1 proof.

The access admission postures are `inline_indexed`,
`bounded_ephemeral_index`, `admitted_paged_streaming`,
`paged_streaming_required`, `persistent_index_required`,
`async_materialization_required`, `store_backed_capability_required`,
`access_capability_registration_required`, and `denied`.

The denial kinds are `budget_exceeded`, `required_async_materialization`,
`required_access_capability_registration`, `required_persistent_index`, and
`unsupported_graph_index_support`.

The required capability owners are `query_runtime`, `lower_runtime`,
`persistent_store`, `domain_registration`, and `async_materializer`.

Representative access requirement rows include `directional_adjacency`,
`reverse_adjacency`, `predicate_support`, `ordering_support`,
`traversal_workset`, `visited_set`, `dedup_set`, `proof_support`,
`result_buffer`, `materialization_lifecycle`, `live_maintenance_support`, and
`domain_operation_capability_registration`.

Receipt proof fields include `graph_read_access_plan_consumption`,
`ephemeral_graph_index_receipt`, `graph_read_streaming_receipt`,
`live_graph_read_access`, and `graph_read_access_summary`.

The mistake to avoid is saying a graph read is safe because the helper is
friendly. A graph read is safe only when the admitted access plan and receipt
prove the selected posture ran and the counters show no caller-owned N+1 work.

Read next:

- [Graph Read Access Planning](./authoring/graph-read-access-planning.md)
- [Read Composition](./authoring/read-composition.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Aspects And Authority Lanes

Aspects and authority lanes are one of the core concepts an AI has to
understand to use Query correctly.

Aspects are the semantic names Query uses for what a surface reads, produces,
writes, routes, or inspects. They are not casual dotted field names. They are
the auditable contract for semantic dependency and change. Authority lanes are
the ownership side of the same story: they tell Query whether state is
authoritative truth, branch-local truth, preview truth, derived runtime state,
effect delivery state, pending write intent, bridge external state, temporal
execution state, or async resource state. The active support profile decides
which lanes are admitted for a concrete runtime.

These concepts show up everywhere. Computeds declare what they read and
produce through aspects. Effects declare what they watch and where delivery
goes. State and inspection surfaces explain which lane a result belongs to.
Write receipts preserve aspect operations so later code can see what was
changed without reconstructing it from raw deltas.

Aspect values keep the exact Foundational scalar or struct vocabulary from
authoring through contract admission, mutation, reads, retained/live results,
and projection consumption. Use `AspectValue` and `StructAspectValue` for
meaning, let the active Foundational contract validate that meaning, and use
proof-bearing consumed facts when another subsystem needs the result. Native
refinement is exact and borrowed; it does not parse strings, widen numbers, or
reconstruct structs.

The word "aspect" names related but distinct identities across the stack:

- Foundational `AspectContract` and `AspectBinding` describe stable semantic
  meaning, including the exact binding and projection mask a consumer needs.
- Relational authoritative-change kinds describe how committed truth changed:
  field, endpoint, identity, structure, or another admitted semantic change.
- Runtime Bridge correspondence proves how an authoritative semantic change
  maps to one executable Signal target, including whether the mapping is exact
  or deliberately widened.
- A Signal aspect is a runtime-local slot on an installed Signal node. It owns
  invalidation and version state for computation; it is not a portable domain
  identity.
- Query retains the semantic dependency, installed correspondence authority,
  and returned conditional provenance without exposing raw Bridge or Signal
  ingredients as caller authority.

These layers must be connected explicitly. Never persist a numeric Signal
aspect as domain meaning, infer semantic identity from a stable-name mapping or
equal slot number, or treat a Relational change label as sufficient proof of a
Signal dependency. Field-level or endpoint-level change may widen to a whole
aspect only when the installed correspondence admits and reports that
widening. This matters for geometry kernels: position, topology, tolerance,
constraint, and derived-measure dependencies need semantic precision even when
their executable invalidation slots are runtime-local.

Reach for this category when the real question is about dependency, production,
writes, triggers, or ownership. If the task depends on understanding what a
surface semantically reads or produces, or whether a result is authoritative,
derived, preview-local, or delivery-local, start here.

The mistake to avoid is treating aspects like incidental syntax and lanes like
debug labels. In Query, both are part of the runtime contract.

Read next:

- [Aspects And Authority Lanes](./modeling/aspects-and-authority-lanes.md)
- [Conditional Installed Operations](./domain-capabilities/conditional-installed-operations.md)
- [Native Aspect Values](./capabilities/native-aspect-values.md)
- [Computed](./runtime-surfaces/computed.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Inspection](./capabilities/inspection.md)

## Query Expressions, Validation, Planning, And Execution

This category is the foundation for honest reads: typed query intent before
execution, schema-aware validation, proof-carrying plans, and snapshot-backed
one-shot execution.

Query expression families give collection/detail reads, aspect projection,
bounded traversal, and typed result shapes one canonical identity. Validation
fails illegal, over-broad, or schema-dishonest queries before planning.
Planning lowers intent once; execution consumes the plan without rediscovering
legality, projection, or scope on the hot path.

Use this category when you are defining what is being asked for, whether it is
legal, how it will be planned, or how one-shot execution binds to truth.

The mistake to avoid is host-local query builders, string predicates, or
execution paths that widen silently when validation should have failed closed.

Read next:

- [Query Expressions And Result Shapes](./authoring/query-expressions-and-result-shapes.md)
- [Read Composition](./authoring/read-composition.md)
- [Workspace Overview](./foundations/workspace-overview.md)

## Collections, Cursors, Ordering, And Aggregation Reads

Large-surface reads need first-class collection semantics, not loops of detail
queries or offset/limit pretending to be pagination.

Query owns typed ordering, opaque cursor pagination, bounded traversals,
aggregation and rollup families where admitted, query-time derived fields as
part of the canonical artifact, and CDC-shaped output that stays query-shaped
rather than raw runtime CDC.

Use this category when the job is list/table scale, stable pages, rollups,
aggregations, or integration-facing change-shaped output.

The mistake to avoid is unstable pagination, unbounded graph walks, or host
post-processing that re-derives fields Query should have planned.

Read next:

- [Collections, Cursors, Ordering, And Aggregations](./authoring/collections-cursors-ordering-and-aggregations.md)
- [Read Composition](./authoring/read-composition.md)
- [Query Expressions And Result Shapes](./authoring/query-expressions-and-result-shapes.md)

## Scopes, Templates, View Shapes, And Saved Queries

This category is Query’s productization layer for reusable query meaning:
named scopes, parameterized templates, admitted view shapes, and frozen
saved-query artifacts with explicit reuse posture.

View shapes are not display tags. They affect planning, invalidation narrowing,
delivery formatting, and live patch semantics. Scopes and templates must expand
to the same canonical query meaning as direct construction.

Use this category when you need reusable fragments, table/detail/grouped/
timeline/inspector presentation intent, or saved-query freeze and reuse
decisions.

The mistake to avoid is treating view shape as UI-only sugar or an ephemeral
saved query as a stronger retained product boundary.
On the runtime-backed application support profile, core view-family support
rows are already verified for `table`, `detail`, inspector detail, and
`kanban_grouped` surfaces, and grouped reusable composition/template support is
now admitted on the same runtime-backed product lane. That does not strengthen
the retention posture of neighboring saved-query surfaces.

Read next:

- [Scopes, Templates, Saved Queries, And View Shapes](./authoring/scopes-templates-saved-queries-and-view-shapes.md)
- [Read Composition](./authoring/read-composition.md)
- [Live Views](./runtime-surfaces/live-views.md)

## Policy, Tenant, And Relationship-Proof Narrowing

Policy masking, tenant truth/schema basis, and relationship-proof queries are
structural query concerns, not post-read filters glued on by hosts.

Masking happens before execution so masked aspects never enter the plan or live
path. Tenant scoping narrows truth basis and schema basis explicitly.
Relationship-proof families stay typed query semantics with explicit denial when
the proof chain breaks. Delivery-shape metadata preserves the same masked and
projected meaning across one-shot, live, and historical lanes where admitted.

Use this category when reads must respect policy, tenant variation, branch
access, or proof-gated relationship access.

The mistake to avoid is over-reading and redacting later, ambient tenant
filters, or authorization callbacks that bypass canonical query artifacts.

Read next:

- [Policy, Tenant, And Relationship-Proof Narrowing](./foundations/policy-tenant-and-relationship-proof-narrowing.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Read Composition](./authoring/read-composition.md)
- [Basis Capability Lifecycle](./capabilities/basis-capability-lifecycle.md)

## Declarations, Contexts, Outcomes, And Managed Resources

The ordinary Query experience starts in a capability namespace: `facade::read`,
`facade::aggregate`, `facade::live`, `facade::history`, `facade::comparison`,
`facade::preview`, `facade::mutation`, `facade::workflow`,
`facade::inspection`, or `facade::installed`. Package declaration and runtime
assembly use `facade::domain` before ordinary installed-operation consumption
begins.

Declare what the application wants, attach explicit authority with `using(...)`,
then call `run(...)` for one-shot work or `open(...)` for a managed resource.
Query owns canonicalization, admission, planning, lower-runtime handoff, and
outcome shaping. Consumer code does not recreate those steps locally.

Outcomes retain distinct completed, advisory, stopped, denied, deferred, and
unavailable postures. Live handles own activation, maintenance, and close; the
consumer holds the handle instead of assembling subscription lifecycle calls.
Domain integrations contribute typed meaning through `facade::domain`, then
consume installed work through `facade::installed`; Query remains the owner of
canonical execution artifacts across both sides of that boundary.

Read next:

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Read Composition](./authoring/read-composition.md)
- [Inspection](./capabilities/inspection.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Basis Capability Lifecycle

A Query basis is a typed capability lifecycle, not a raw branch head,
snapshot id, preview handle, or tenant label passed through host context.

Basis intent normalizes, becomes eligible or denied, admits as a capability,
scopes execution or observation, binds lower-runtime truth, emits use receipts,
and returns self-describing envelopes. Read, mutate, replay, inspect, and
materialize surfaces consume basis proofs rather than rediscovering authority
from identifiers alone.

Start at `worth_query::facade::foundation::basis_lifecycle()`. Declare the truth
world, then call the operation you need, such as `observe()`,
`prepare_mutation()`, `replay()`, `inspect()`, or `materialize()`. The result is
the sealed scoped capability for that operation; consumer code does not
assemble lifecycle phases itself.

Use this category when the job depends on which truth world, preview world,
historical world, or tenant/policy world a surface is allowed to use—and what
transition is legal next.

The mistake to avoid is threading raw relational or bridge ids through product
code when Query expects basis capability artifacts.

Read next:

- [Basis Capability Lifecycle](./capabilities/basis-capability-lifecycle.md)
- [Historical Diff And Basis](./capabilities/historical-diff-and-basis.md)

### Downstream basis and projection authority

If another runtime must carry Query-consumed meaning forward, do not hand it a
basis digest, projection receipt, extracted facts, and source labels as separate
authority inputs. On a completed ordinary read, call
`completion.consume_projection(read::project_facts()...)`. Query returns one
`WorthQueryProjectionOutcome`; move its sealed authority with `into_admitted()`.
Evidence projections and getters are observation only and cannot recreate that
authority.

Read next:

- [Projection Consumption](./capabilities/projection-consumption.md)
- [Downstream Runtime Integration](./foundations/downstream-runtime-integration.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## State Readiness Vs Inspection

`workspace.state(...)` answers typed readiness posture for a retained surface
or public facade family: ready, pending, unsupported, or otherwise not in a
normal ready lane. Inspection answers richer **per-target retained evidence**
after work has run. Cross-runtime causal explanation is a separate lane—see
[Cross-Runtime Causal Inspection](./capabilities/cross-runtime-causal-inspection.md).

Use state when you need a digest-bound posture snapshot without full
running work that requires optional support.

The mistake to avoid is guessing support from handle behavior, or using
inspection when you only needed readiness—or the reverse.

Read next:

- [State And Readiness Surfaces](./foundations/state.md)
- [Inspection](./capabilities/inspection.md)
- [Declarative Query Experience](./capabilities/declarative-query-experience.md)

## Typed Stops And Next Actions

Ordinary outcomes preserve why work did not complete. Stale context, foreign
authority, unsupported capability, policy denial, ambiguity, deferral, and
lower-runtime failure remain distinct typed postures with explicit next actions.
Handle those outcomes in the capability namespace that produced them; do not
flatten them into a consumer-local boolean, string, or generic status enum.

Use `facade::inspection` when you need richer explanation for a retained result
or managed resource. Inspection is observational and cannot promote a stop or
diagnostic into authority.

Read next:

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Inspection](./capabilities/inspection.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Runtime-Installed Domains

Runtime-installed domains are Query's runtime-construction and operating seam. A domain
crate declares one typed package, the runtime builder installs it atomically,
and `WorthQueryWorkspace::domain(...)` returns a handle tied to that runtime
and exact installation generation. Runtime-installed operations extend that
same authority into typed execution, publication, consumption, settlement,
workflow progression, graph participation, and conditional evaluation.

There are two installed execution journeys. The ordinary journey binds an
operation through one operating world and advances through execution,
publication, consumption, and settlement as declared. Execution-grade managed
computation starts from the same installed meaning but adds resource
reservation, managed-run lifecycle, provider sessions, decision read-sets,
provisional state, and actual invariant execution. Do not force either journey
through the other's phase types.

The authority split is deliberate:

- domain code owns typed semantic declarations and ergonomic extension traits;
- portable packages own callback-free operation meaning;
- the runtime builder joins those definitions to exact volatile providers;
- Query validates the package, seals identities, builds rebuildable indexes,
  and retains installation, binding, execution, and progression authority;
- Runtime Bridge owns admitted correspondence between semantic truth and
  executable Signal targets;
- Relational owns authoritative change and publication semantics;
- Signal owns dependency evaluation and conditional decision mechanics;
- consumers execute only through the installed handle and Query-minted phase
  values.

Operating contexts follow the same rule. The domain supplies named semantic
fields through `WorthQueryDomainOperatingContextIdentityDeclaration`; Query
canonicalizes and seals them. Field order is not identity, and caller-authored
digests are not part of the public contract.

### Portable operation meaning

`WorthQueryDomainOperationDefinition<D, O, F>` binds a domain marker,
operation marker, and family marker to one portable semantic closure. The
closure can declare:

- parameters and native projection;
- canonical query and result shape;
- collection and continuation posture;
- required capabilities and installed domains;
- workflow stages and legal stage relationships;
- portable conditional nodes;
- graph reads, graph participation, touches, effects, and invariants;
- publication and projection-consumption meaning;
- replay comparison, reversal, postcondition, and recovery meaning;
- identity evolution, lineage, and sparse promotion meaning;
- terminal result states and failure classes;
- cost and support requirements;
- lowering identity and determinism posture.

Every dimension is explicit. Use typed `NotRequired` when a capability is
absent. Do not imply absence or support with an empty label, a missing hook, or
a provider default. Portable definitions remain callback-free and participate
in canonical package identity, conflict detection, installation artifacts, and
index reconstruction. Equivalent declaration order converges; one-field
semantic drift is a conflict even when two providers could produce similar
output.

### Runtime construction

Runtime construction joins portable definitions to exact volatile mechanics:

```rust
let builder = runtime::WorthQueryRuntime::builder()
    .domain_package(package)?
    .domain_operation_executor(
        GeometryDomain,
        ReadVertex,
        ReadFamily,
        ReadVertexExecutor,
    );
```

Depending on declared meaning, construction may also require graph
participation providers, one Runtime Bridge, one Signal graph bound through
that bridge, conditional correspondences and provider sets, workflow-stage
executors, parallel-admission providers, and explicit consumer-support
posture. Replay comparators and aftermath evaluators are domain-owned hooks
registered beside their installed operation; they do not let Query infer
business truth from executor output. Persistent naming instead consumes the
typed evidence of a naming mutation already executed through Query's naming
surface.
Runtime construction rejects missing, extra, duplicate, foreign, or
same-label/different-marker registrations. Marker types and retained provider
identities are authority inputs; matching strings are not substitutes.

### One operating world

`WorthQueryWorkspace::observe_operating_world()` and
`prepare_mutation_operating_world()` are the owner-issued authority entries for
ordinary installed operations. Callers choose intent; Query admits and seals
the basis internally. A family view borrows that root and can only
bind operations installed in the same runtime generation, basis, domain,
family, provider set, and graph world. A family view cannot mint a runtime,
basis, graph, or installation authority.

Ordinary downstream code imports this grammar through
`worth_query::facade::installed`. The module root teaches the operating world,
typed family view, bound operation, and exact entry or binding denials.
Continue through the guarantee-specific `installed::operation`,
`installed::observation`, `installed::collection`,
`installed::compatibility`, `installed::workflow`, and
`installed::recovery` modules. Runtime/package assembly remains in the host
facades, provider callbacks are absent from the installed consumer surface,
and replay remains certification-only.

The ordinary operation grammar is:

```rust
use worth_query::facade::installed;

let installed_domain = workspace.domain(GeometryDomain)?;
let world = workspace.observe_operating_world()?;
let bound = world
    .family(ReadFamily)
    .bind(&installed_domain, ReadVertex)?;

let consumer = bound.consumer_projection_contract()?;
let executed = match installed::transition::execution(
    bound.execute(input, &mut workspace),
) {
    installed::transition::WorthQueryExecutionTransition::Executed(value) => value,
    stop => return handle_execution_stop(stop),
};
let published = match installed::transition::publication(executed.publish()) {
    installed::transition::WorthQueryPublicationTransition::Published(value) => value,
    stop => return handle_publication_stop(stop),
};
let consumed = match installed::transition::consumption(published.consume(
    consumer,
    installed::operation::project_facts().entity_identities(),
)) {
    installed::transition::WorthQueryConsumptionTransition::Consumed(value) => value,
    stop => return handle_consumption_stop(stop),
};
let settled = match installed::transition::settlement(consumed.settle()) {
    installed::transition::WorthQuerySettlementTransition::Settled(value) => value,
    stop => return handle_settlement_stop(stop),
};
```

Every value is minted by the preceding phase. The phase types are move-only,
runtime-affine, and tied to the exact domain, operation, family, installation
generation, basis, graph authorities, providers, receipts, result posture,
warnings, and counters. Obtain the consumer contract before moving `bound`
into `execute`. Do not cache phase ingredients for later recombination, invoke
an executor directly, or reconstruct publication or consumption authority from
receipts and identities.

The `handle_*_stop` names above are application-owned placeholders for mapping
Query's typed stops into the caller's error or retry boundary. Preserve
`Executed`/`Published`/`Consumed`/`Settled`, `Denied`, `Deferred`, `Stale`,
`RebindRequired`, and `Failed`; do not replace the matches with an unchecked
success extractor. The owner-specific `installed::transition` adapters keep
raw `worth_proof` progression out of the consumer API without erasing any
stop.

An operation whose installed publication contract is `NotRequired` finishes
at its executed terminal value. Its operation marker uses the terminal
operation type, so publication is unavailable in the type-level journey rather
than rejected after detached assembly.

### Query-owned consumer support

The bound operation mints one consumer projection contract from installed
operation meaning and Query-owned runtime support truth. The contract retains
the exact installation generation, operation identity, basis and access
context, projection mask, publication and consumption meaning, and typed
support dimensions for live continuation, async and result state, recovery,
inspection, dependency impact, sharing, invalidation, and collection delivery.

Compatibility admission returns either an exact pair-bound witness or a typed
dimension-specific denial. Reports, summaries, labels, and digests are
observational; none can satisfy admission. Presentation and allocation needs
belong in the separate `WorthQueryConsumerBoundary` and cannot rewrite Query's
requirements.

### Collection windows and patch delivery

Installed collection windows and query-shaped patches are operational
capabilities, not future vocabulary. Use them when a settled installed
operation declares collection delivery and the consumer needs an ordered,
bounded mounted view that can advance incrementally.

A window is not an offset/limit pair. Query binds its opaque cursor and breadth
to the exact collection capability generation, basis, ordering, result shape,
native layout, and collection-delivery contract. The breadth declares viewport
rows, overscan before and after, and the consumer's mounting budget. A cursor
from another capability, generation, basis, or ordering is denied before row
or index work.

For the ordinary first window, prepare the consumer directly from the settled
projection before moving that projection into live lifecycle:

```rust
use worth_query::facade::installed::{collection, observation};

let breadth =
    collection::WorthQueryCollectionWindowBreadth::new(40, 8, 8, 56)?;
let mut mounted = settled.prepare_collection_consumer(breadth)?;

let live = match settled.into_lifecycle().promote(&mut workspace) {
    observation::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
    stopped => return handle_promotion_stop(stopped),
};
let lease = match live.into_managed_lease(&mut workspace) {
    observation::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
    observation::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
        return handle_lease_stop(stop);
    }
};
```

If the application needs explicit paging before live promotion, use the longer
`settled.into_bound_collection() -> declare_window(...) ->
resolve_window(...) -> WorthQueryCollectionConsumerWindow::from_bound(...)`
path. Both paths retain the same Query-owned cursor and collection authority;
neither accepts caller-authored row identities or patch posture.

After an authoritative change, drain the managed lease, derive and readmit its
capability-bound invalidation delta, bind that exact lease target to the
consumer window, then plan and apply the patch:

```rust
let delivery = lease.drain(&mut workspace)?;
if !delivery.delivery().is_empty() {
    let delta = lease.consumer_invalidation_delta(delivery)?;
    let admitted =
        lease.admit_consumer_invalidation_delta(delta, &workspace)?;
    mounted.bind_shared_target(&admitted, &workspace)?;

    match mounted.plan_patch(&admitted, &workspace) {
        collection::WorthQueryCollectionDeliveryOutcome::Patch(patch) => {
            let receipt = mounted.apply_patch(patch)?;
            apply_domain_consequences(receipt);
        }
        collection::WorthQueryCollectionDeliveryOutcome::NoDelivery(stop) => {
            handle_no_collection_delivery(stop);
        }
    }
}
```

The `handle_promotion_stop`, `handle_lease_stop`,
`apply_domain_consequences`, and `handle_no_collection_delivery` names are
application-owned placeholders. Query ends at the typed stop or patch receipt;
the application decides how to retry, report, or translate that result into
local consequences.

The patch can carry insert, remove, move, update, explicit window shift,
result-state, warning, continuation, or reset-required operations together
with native facts and exact counters. Applying it validates lease, capability
generation, window contract, cursor/order, and maintenance order before
touching consumer state. Duplicate, reordered, superseded, foreign, and
wrong-window patches remain typed denials. A reset is an explicit operation
with a reason and replacement-cost contract; it is never a silent fallback.

Equivalent admitted windows may share one underlying maintenance pass, but
each lease receives a separately applicable patch. Query owns suppression,
impact, row identity, patch assembly, and delivery authority. The consumer
owns only application consequences such as UI graph touches, mounting policy,
allocation, and measurement invalidation. Do not expose raw CDC, diff the full
collection, or hash a local patch posture.

### Derived indexes and exact counters

Installed-operation, support, conditional, correspondence, and allocation
indexes accelerate authority that remains elsewhere. Destroying and rebuilding
an index from installed packages, runtime support posture, and exact runtime
registrations must preserve admission, identity, denial, lifecycle result, and
counter outcomes. Never make a derived index the only place where semantic
meaning or provider authority lives.

Exact counters are part of progression evidence. A denial at an earlier
boundary must report zero contact with later providers, graph work, executor
work, publication, or consumption. Matching output with dishonest work counts
is not convergence.

Use this category when a domain needs registered reads, invariants, graph
obligations, declaration families, contributions, domain-native workflows,
conditional computations, or an operation that must remain in one authority
chain from binding through settlement.

Read next:

- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)
- [Conditional Installed Operations](./domain-capabilities/conditional-installed-operations.md)
- [Installed Operation Re-Execution And Replay](./domain-capabilities/installed-operation-reexecution-and-replay.md)
- [Installed Operation Aftermath](./domain-capabilities/installed-operation-aftermath.md)
- [Installed Operation Lineage And Promotion](./domain-capabilities/installed-operation-lineage-and-promotion.md)
- [Aspects And Authority Lanes](./modeling/aspects-and-authority-lanes.md)
- [Projection Consumption](./capabilities/projection-consumption.md)
- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Consumer Kit](./foundations/consumer-kit.md)

## Execution-Grade Installed Domain Computation

Execution-grade installed computation is the Query path for domain work that
cannot honestly be represented as one executor callback. Reach for it when an
installed operation must retain provider-native artifacts, reserve real
capacity, advance through bounded steps, yield at safe points, stage proposed
graph changes, or execute invariants against the exact proposed post-state.

This path extends runtime-installed domains; it does not replace the ordinary
installed-operation journey. A small operation can still follow
`bound -> executed -> published -> consumed -> settled`. Execution-grade work
starts from an installed operation and follows a second, more detailed
authority chain:

```text
portable computation contracts
  -> installed artifact and resource authority
  -> capacity reservation
  -> managed run
  -> sealed provider session
  -> complete fresh decision read-set
  -> provisional proposed state
  -> executed invariant receipts
  -> complete invariant progression for a later publication boundary
```

The stable entry audiences are:

```rust
use worth_query_host::facade::{
    admission::resource_admission,
    domain,
    installed::{domain_computation, provider_session},
    runtime,
};
```

`domain` is portable installation meaning. `resource_admission` is capacity
admission and reservation. `domain_computation` is the ordinary consumer
progression. `provider_session` is the runtime-host integration contract for
physical providers.

### Installed computation artifact contracts

Portable artifact contracts declare what candidate sets, compiled plans,
solver state, checkpoints, or other working artifacts mean before any provider
allocates them. The contract includes:

- semantic and occurrence identity;
- payload owner, producer and consumer roles;
- move, borrow, transfer, retention, reconstruction, and lifecycle posture;
- reproducibility, comparison, search, and convergence meaning;
- transformation and loss evidence;
- installed native access paths;
- structural counters and decision-record schemas;
- classification, redaction, retention, compatibility, and retirement.

Every semantic field participates in canonical identity. Equivalent declaration
order converges; changed units, governance, occurrence policy, comparator,
reconstruction, or loss posture conflicts.

Structural counters explain real work in declared units. Required foundation
rows cannot be shadowed, aggregation must be acyclic and unit-compatible, and
caller-authored work reports cannot satisfy the contract. Decision records
explain pruning, ranking, or candidate choice. Their classification and
redaction rules constrain the containing artifact, and their contents never
authorize execution, mutation, publication, or invariant progression.

Read:

- [Installed Computation Artifact Contracts](./domain-capabilities/installed-computation-artifact-contracts.md)

### Managed artifact ownership and native access

Runtime artifacts are owned through `WorthQueryMoveOnlyArtifactHandle`, not
`Any` bags or caller-managed IDs. Query can mint lifetime-bound borrows,
move-only retained leases, admitted workflow transfers, and explicit disposal
outcomes. The handle proves ownership and lifecycle position; it is not
mutation, invariant, publication, or commit authority.

`WorthQueryStageArtifactReader` admits the installed row-batch, field-slice,
bounded-chunk, or provider-native projection path. Scalar fallback is separate
and explicitly bounded. Borrowed provider memory cannot escape its callback.
Projected chunk memory counts variable-width heap allocations, not only the
outer result vector. Pending chunks must be consumed or abandoned before the
provider advances.

Read:

- [Managed Artifact Ownership And Native Access](./domain-capabilities/managed-artifact-ownership-and-native-access.md)

### Execution resource admission and managed runs

An execution resource request declares scale, memory, concurrency, queue,
chunk, deadline, safe-point, and cleanup needs. Admission selects an immutable
strategy and envelope. Reservation then consumes real capacity, so saturation
is proved by competing reservations and release rather than a caller-supplied
"insufficient" snapshot.

Managed-run admission joins the installed-operation phase proof and reserved
attempt with Query, Runtime Bridge, Relational, Signal, provider-session, and
artifact authority. Query owns the semantic state machine. Bridge owns the
causal execution-basis binding. Relational owns authoritative basis mechanics.
Signal owns cancellation and pressure. Providers own physical execution,
retained memory, and checkpoints.

Providers advance through installed bounded steps. Safe points observe
cancellation, timeout, rejection, and pressure before and after provider work.
Pending output blocks another provider step. Provider rejection or panic
becomes a typed terminal or recovery posture with exact counters.

Yield ends the current attempt but not the logical run. A yielded capability
owns the retained checkpoint, artifacts, capacity, applied-effect evidence, and
affinity required for same-runtime readmission. It cannot execute, publish, or
be reconstructed from checkpoint evidence. Readmission transfers retained
capacity and mints fresh attempt and provider-session generations.

Convergence consumes managed lifecycle outcomes and distinguishes completed,
yielded, cancelled, failed, cleanup-pending, and recovery-required work.
Convergence does not grant decision, invariant, publication, or resolution
authority.

Read:

- [Execution Resource Admission And Managed Runs](./domain-capabilities/execution-resource-admission-and-managed-runs.md)

### Provider sessions and decision read-sets

A running managed run admits one sealed provider execution plan from the exact
installed graph participation authority:

```text
admitted provider plan
  -> plan readmission
  -> prepared provider session
  -> session-bound read and effect authorities
```

The plan binds provider identity and generation, operation, resource attempt,
Bridge basis, graph role, decision-fact closure, provisional dimensions, and
installed invariant requirements. Preparation does not authorize effects.
Only the staged session exposes separate read and effect authorities; neither
can perform the other's job.

Installed application work enters this protocol only after one sealed
obligation selection and graph-work plan. The basis is branch-qualified: an
equal version ordinal from another typed branch cannot satisfy the session or
any later read-set, proposal, invariant, commit, retry, receipt, or publication
transition.

A decision read-set contains every installed fact family and exact fact count
that can influence the proposal. Query canonicalizes requests, verifies
provider evidence affinity, and compares the same evidence again. `Fresh`
continues. `Stale` requires replanning. Raw hashes, arbitrary callbacks,
caller-selected subsets, and independently paired facts cannot replace this
receipt. Freshness still is not mutation or commit authority.

Read:

- [Provider Sessions And Decision Read-Sets](./domain-capabilities/provider-sessions-and-decision-read-sets.md)

### Provisional state and real invariant execution

The session effect authority lowers a provisional program only from the exact
staged session and fresh decision read-set. The provider stages an isolated
overlay bound to the provider, session token, basis, program, and attempt
generation. Ordinary reads continue to observe committed truth.

The caller can inspect, revise, or discard the proposed post-state. Candidate
scores, decision evidence, overlay identity, and proposed-state identity are
descriptive; none can publish the proposal.

Invariant registration and provider capability also do not prove a proposal.
Each installed invariant must select its exact requirement, admit a state-load
plan, load the required state from the proposal, execute the validator, and
return a typed passed, advisory, violated, indeterminate, or exhausted receipt.
Only one complete, exact receipt set for every installed slot can mint
`WorthQueryInvariantProgressionAuthority`.

The low-level provisional-state boundary stops there: it does not expose commit
preparation or commit. The higher installed application-operation progression
consumes the complete invariant evidence as a sealed Relational validated
candidate, then performs provider compare-and-commit before publication. The
proposal remains non-authoritative and every provisional stage retains a
consuming discard path.

The consumer shape is:

```rust
let staged = running
    .admit_provider_execution_plan(&graph)?
    .readmit()?
    .prepare()?
    .bind_reads_and_effects();

let captured = staged
    .read_authority()
    .capture_decision_read_set(requests)?;

let fresh = match staged
    .read_authority()
    .compare_decision_read_set(captured)?
{
    domain_computation::WorthQueryDecisionReadSetFreshnessOutcome::Fresh(fresh) => fresh,
    domain_computation::WorthQueryDecisionReadSetFreshnessOutcome::Stale(stale) => {
        return replan(stale);
    }
};

let program = staged
    .effect_authority()
    .lower_provisional_program(&fresh, effect_steps)?;

let inspection = staged
    .begin_provisional_attempt(fresh, program)?
    .materialize_proposed_state()
    .inspect();

let receipt = inspection
    .select_installed_invariant(invariant_slot)?
    .admit_state_load_plan(state_load)?
    .execute()?;

let progression = inspection.admit_invariant_progression([receipt])?;
inspect_progression(&progression);
handle_discard(inspection.discard());
```

Real operations execute every installed invariant and supply the complete
receipt collection. The example ends with discard because this public
progression has no commit method.

Read:

- [Provisional State And Invariant Execution](./domain-capabilities/provisional-state-and-invariant-execution.md)

The mistakes to avoid across this category are untyped payloads, caller-made
work reports, raw checkpoints or session tokens, scalar loops over an admitted
bulk path, local cancellation protocols, incomplete decision dependencies,
staged overlays treated as truth, selected invariants treated as executed, one
passed receipt treated as a complete set, and convergence treated as approval.

## Conditional Installed Operations

Conditional nodes are portable Query declarations inside an installed
operation or workflow stage. They let a domain author describe when derived
work is eligible without authoring Signal-local slots or running a parallel
condition engine.

A conditional declaration can carry:

- semantic truth dependencies and graph-read roles;
- aspect-filtered eligibility;
- typed delta thresholds and units;
- temporal wake conditions;
- typed on-demand trigger families;
- typed domain-specific condition families and parameters;
- dependency and output comparison requirements;
- maintenance, artifact reuse, and output relationship.

Use `WorthQueryPortableConditionalNodeDeclaration::declare(...)`. Every
semantic dimension is required by the builder; it does not invent an
executable default such as `Always`.

Query dependencies name Foundational and Relational meaning, not Signal slot
numbers:

```rust
let dependency = domain::WorthQuerySemanticTruthDependency::new(
    domain::WorthQueryConditionalGraphReadRole::new("model")?,
    aspect_contract,
    projection_mask,
    aspect_binding,
    domain::WorthQuerySemanticLocality::SourceRecord,
    [relational_change_kind],
)?;
```

At runtime construction, Runtime Bridge admits that exact dependency against
authoritative publication semantics and the actual Signal target allocation.
Signal owns the resulting condition decision and computation-cleanliness
result. Query carries the evidence back as
`WorthQueryConditionalProvenance`; it does not recalculate eligibility from
labels, counters, or output comparison.

Conditional outcomes preserve work honestly. Ineligible, suppressed, or
deferred work performs no computation. Reverted-clean work retains the cost of
computation while recording that no new semantic output was produced. Changed
eligible work advances only through its installed consequences.

Read next:

- [Conditional Installed Operations](./domain-capabilities/conditional-installed-operations.md)
- [Aspects And Authority Lanes](./modeling/aspects-and-authority-lanes.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)

## Domain Capability Contributions

This is the public domain capability contribution seam. It exists so downstream
domains can contribute typed semantic posture while Query remains the owner of
canonical runtime artifacts.

Contribution is not an alternative operation root. Executable domain work that
belongs to an installed package uses the portable operation definition,
runtime provider registration, and operating-world binding described above.
Use contributions for contribution-shaped posture, not to bypass that chain.

That ownership split is the whole point. Domains contribute meaning and
evidence. Query materializes canonical artifacts. This prevents downstream
domains from solving contribution problems by minting local pseudo-Query
artifacts or by exposing canonical constructors directly.

Use this category when the domain needs to add semantic posture to Query-owned
runtime truth. If the problem is “the domain needs to say something important
about runtime posture, but Query should still own the final artifact,” this is
where you start.

The mistake to avoid is solving contribution problems by making canonical
artifact construction local or by flattening contribution meaning into generic
strings.

Read next:

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Relational Truth And Invariants Through Query](#relational-truth-and-invariants-through-query)

## Lower-Runtime Capability Routing

All ordinary Query contact with relational, runtime bridge, signal, and later
store-adjacent surfaces should pass through capability-routed boundary envelopes,
not scattered direct imports or convenience shortcuts.

Routing names authority, route plan, capability, cost posture, failure topology,
and retained evidence. Compatibility debt for any remaining direct path must
stay explicit rather than becoming a silent escape hatch around basis, admission,
projection, effect, or inspection contracts.

Use this category when work must touch lower runtimes and you need the honest
Query-owned route rather than “import bridge/relational/signal for speed.”

The mistake to avoid is choosing a lower crate by convenience and bypassing
basis, admission, and envelope contracts Query already owns publicly.

Read next:

- [Lower-Runtime Capability Routing](./domain-capabilities/lower-runtime-capability-routing.md)
- [Declaration Bridge Continuation Routing](./domain-capabilities/declaration-bridge-continuation-routing.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Live Views And Live Promotion

A live view is a durable, query-shaped runtime surface over authoritative truth.
Live promotion means the same canonical query expression can be maintained
incrementally through query-shaped patches rather than as a separate reactive
product with different meaning.

Live maintenance must converge to the same result as re-executing the canonical
one-shot query on the same basis, with suppression and invalidation explained in
query terms—not raw CDC events or host observer folklore.

Use this category when you need current rows or view-shaped records, query-
shaped write patches, or a retained surface that computeds, effects, previews,
and inspection can reuse.

The mistake to avoid is treating live views as thin subscriptions to raw truth
streams or as a different query language from one-shot reads.

Read next:

- [Live Views](./runtime-surfaces/live-views.md)
- [Reads, Observe, And Materialize](./runtime-surfaces/reads-observe-materialize.md)
- [Scopes, Templates, Saved Queries, And View Shapes](./authoring/scopes-templates-saved-queries-and-view-shapes.md)

## Subscriptions

Subscriptions are first-class query artifacts, not ambient observer glue around
a live view handle.

Query lowers admitted live meaning into subscription declaration families with
their own identity, basis binding, bridge lowering, sharing, continuation,
preview isolation, and family-aware delivery. Automatic family selection must
remain bridge-honest and diagnostically sufficient rather than smuggling a fake
default subscription kind.

Subscription declaration consumes a
`facade::foundation::ScopedSubscriptionDeclarationBasis`. Build the declaration
through `LiveQueryAdmissionArtifact::from_live_promotion(...)`, select and admit
the subscription family through `facade::runtime`, then call
`prepare_subscription_activation(...)`. Activation derives its scoped
activation proof from the admitted declaration basis; callers do not author a
second basis posture or carry basis digests alongside the artifact.

Use this category when work is long-lived observation: shared equivalent
subscriptions, continuation after identity evolution, preview-scoped
subscriptions, or understanding which bridge and signal strategies were selected.

The mistake to avoid is hand-rolling observers, inferring subscription meaning
from CDC, or collapsing all live families into one generic runtime lane.

Read next:

- [Subscription Selection And Diagnostics](./capabilities/subscription-selection-and-diagnostics.md)
- [Live Views](./runtime-surfaces/live-views.md)
- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)

## Region-Scoped Live Invalidation And Stream Contracts

When truth changes only touch a bounded region or partition of a query’s
declared scope, live maintenance should narrow to that region and emit delivery
metadata that stays query-shaped.

Change-stream-backed delivery contracts may lower where the bridge admits them,
but the consumer contract remains query-shaped result maintenance—not raw
partition events or transport-local stream glue.

Use this category for geometry-grade locality, integration feeds, or large
collections where broad aspect invalidation would be disproportionate.

The mistake to avoid is widening to full-aspect or full-collection refresh when
planner-owned region narrowing was available.

Read next:

- [Region-Scoped Live Invalidation And Stream Contracts](./runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md)
- [Live Views](./runtime-surfaces/live-views.md)
- [Live View Vs Subscription](./domain-capabilities/choosing/live-view-vs-subscription.md)

## Workspace Runtime Surfaces

The workspace is the unified facade context for retained runtime-backed product
work: computed state, preview or branch sessions, reads, observation,
materialization, state snapshots, and inspection entrypoints that share one
configuration and support contract.

Live views, subscriptions, effects, basis lifecycle, and write paths have their
own categories because they carry extra identity, lowering, or authority rules.
This section is the umbrella for everything else you declare or consume through
`workspace` without re-deriving lower-runtime wiring.

The important mental model is retained handles and digest-bound evidence, not
throwaway callbacks or host-local stores.

The workspace also provides the public bridge-backed read-runtime bootstrap
used by hostile tests and downstream bring-up. Obtain it through the ordinary
Query builder-owned lane rather than custom runtime scaffolding.

Use this category when you are operating inside the stabilized facade and need
the overview of which workspace methods belong to which retained surface family.

The mistake to avoid is treating the workspace as permission to skip category-
specific contracts for live, subscription, effect, basis, or mutation work.

Read next:

- [Workspace Overview](./foundations/workspace-overview.md)
- [Computed](./runtime-surfaces/computed.md)
- [Reads, Observe, And Materialize](./runtime-surfaces/reads-observe-materialize.md)
- [Branches And Previews](./foundations/branches-and-previews.md)
- [State And Readiness Surfaces](./foundations/state.md)

## Authority-Scoped Effects

Effects are retained delivery or staging surfaces that react to live or computed
changes. They are not a general truth-mutation lane.

The provisional effect program used by execution-grade installed computation
is narrower still: it is bound to one staged provider session and fresh
decision read-set, and it can only stage an isolated provider overlay. General
effect authority cannot be substituted for that program, and staging is not
commit.

Query lowers effect intent once through an authority-scoped pipeline: eligibility,
scoped plan, lowered execution plan, receipt, and self-describing envelope.
Executors must consume lowered proofs rather than re-deciding authority, basis,
invariant scope, preview policy, route strategy, or artifact policy.

Use this category when you need conditional delivery, meaningful-change
suppression, staged pending work, or covered write-intent residue that later
admits through the intent lattice.

The mistake to avoid is hiding business logic inside effects or mutating truth
directly from an effect callback.

Read next:

- [Effects](./execution/effects.md) — authoring and staging
- [Authority-Scoped Effect Execution](./execution/authority-scoped-effect-execution.md) — lifecycle matrix
- [Intent Admission](./execution/intent-admission.md)

## Writes And Intent Boundaries

This category answers the question: how should truth change happen, and when
does that change belong on a direct write path versus an intent path?

Query is explicit here because runtime-heavy domains need more than “a mutation
happened.” Direct writes are the stable everyday path when product code already
knows the mutation to perform. Covered intent families exist too, but they
belong on the admitted intent lattice instead of in a vague “everything is an
intent now” story.

Write receipts are important in their own right. They preserve aspect
operations, target evidence, existing-truth binding evidence, causality, batch
evidence, continuity-aware authority evidence, and touched surface routing.
That is how downstream code can explain what actually happened without
rebuilding the story from raw deltas or lower-runtime logs.

Use this category when you are performing authoritative mutation now, when you
need graph-shaped same-batch authoring, when you need existing-truth binding or
verification, or when you need covered mutation intent families.

Use the aspect-native mutation vocabulary, explicit submission lane, or the
higher-level graph and existing-truth lanes when they are the honest fit. Do
not teach a direct workspace write or batch method as an alternate path.

Read next:

- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Intent Admission](./execution/intent-admission.md)
- [Graph Composition Authoring](./authoring/graph-composition-authoring.md)
- [Existing Truth](./capabilities/existing-truth.md)

## Intent Admission Decision Lattice

**Covered intent families** resolve through a structured admission decision
lattice before construction, lowering, or covered execution—not every public
`Intent` export is admitted; check the matrix per family.

Success, advisory, and violation outcomes carry decision traces and typed
context rather than collapsing into a binary wall. Covered families cross into
real bridge-backed execution through typed admitted handoffs.

Use this category when you need to know whether an intent may proceed, proceed
with advisory posture, or stop with violation evidence—and what trace to
preserve for inspection or recovery.

The mistake to avoid is treating admission as “it returned Ok” or rebuilding
admission logic locally after Query already classified the intent.

Read next:

- [Intent Admission](./execution/intent-admission.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)

## Authoritative Mutation Evidence

Write-heavy work needs more than a mutation succeeded. The public mutation-
evidence contract preserves target identity, causality, existing-truth binding,
aspect operations, batch and continuity evidence, and authority explanation so
downstream code does not rebuild carry-forward folklore above Query.

Write receipts are one visible part of that story; the broader contract keeps
Query and bridge carry-forward aligned for replay and inspection.

Use this category when you must explain what was touched, under which authority,
against which existing truth, and with what continuity-sensitive evidence.

The mistake to avoid is reconstructing mutation stories from raw deltas, logs, or
local side maps when Query already issued receipt-grade evidence.

Read next:

- [Authoritative Mutation Evidence](./capabilities/authoritative-mutation-evidence.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Inspection](./capabilities/inspection.md)

## Workflow, Live Continuation, Merge, And Writeback

Use `facade::workflow` for a domain contribution that must become an admitted
mutation under explicit authority. Use `facade::live` when the application needs
a framework-owned resource that remains active, receives updates, and closes
deterministically. Use `facade::preview` and `facade::comparison` for branch or
historical work without blurring those truth worlds into a current read.

Query owns the public declaration, context handoff, planning, and outcome. The
bridge, relational runtime, and signal runtime retain their lower truth,
mutation, merge, and scheduling authority. `WorthQuerySessionLabel` names a
runtime session; it is not a substitute for retained preview or merge authority.

Runtime-installed workflows are typed DAGs inside the operation definition.
Query owns run identity, legal stage progression, stage receipts, conditional
provenance, warnings, result state, exact counters, and the terminal workflow
trace. Start the workflow and advance the returned run; do not keep an
application stage ledger or reconstruct progression from stage names.
Parallel frontiers use the installed parallel-admission provider. Effectful
stages still require the appropriate mutation-preparation basis and installed
graph and effect authorities.

Conditional workflow stages use the same authored semantic dependencies and
installed correspondence as direct operations. Ineligible, suppressed, and
deferred outcomes do not contact the stage executor. Reverted-clean execution
keeps its work cost but does not claim a changed semantic output.

Fresh re-execution remains an ordinary Query operation. Certification replay
is different: only `worth-query-replay` can admit the original trace, replay
basis, and exact historical correspondence, invoke the domain comparator, and
issue an equivalence or drift result. Exact effects, publication, conditional
observations, retained Signal evidence, lineage bindings, and comparator-owned
semantics all participate; broad effect families or matching output labels do
not establish equivalence.

Aftermath starts from a completed workflow trace, executes the
declared inverse or compensation through the same installed authority chain,
and then asks the domain-owned evaluator to prove the exact target scope and
postcondition. A successful candidate execution is not itself restoration
proof. Failed verification preserves partial-effect evidence and the recovery
posture instead of minting a clean receipt.

Lineage starts from exact effect evidence and is bound to the receipts that
actually produced it. Persistent naming requires an executed typed naming
mutation that already embodies domain policy. Sparse promotion additionally
requires exact publication evidence, Schema Graph's typed promotion grammar,
and Foundational admission of the promoted identity. Raw identities, stage
positions, free-form names, and matching strings are never lineage or promotion
authority.

Read next:

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Branches And Previews](./foundations/branches-and-previews.md)
- [Subscription Selection And Diagnostics](./capabilities/subscription-selection-and-diagnostics.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)
- [Conditional Installed Operations](./domain-capabilities/conditional-installed-operations.md)
- [Installed Operation Re-Execution And Replay](./domain-capabilities/installed-operation-reexecution-and-replay.md)
- [Installed Operation Aftermath](./domain-capabilities/installed-operation-aftermath.md)
- [Installed Operation Lineage And Promotion](./domain-capabilities/installed-operation-lineage-and-promotion.md)

## Relational Truth And Invariants Through Query

This category exists because downstream domains often need relational truth or
invariant posture, but that does not mean the right answer is to import
Relational directly from ordinary domain code.

Query gives public surfaces for relational truth routing, invariant
registration, capability gaps, and invariant denials so that domains can use
lower truth semantics through a real public lane. The ownership split is that
Relational remains the authority, but Query owns the ordinary public access and
orchestration shape.

For execution-grade installed computation, registration and selected support
are only prerequisites. Query must admit a state-load plan for the exact
proposed post-state, the registered provider must load that state and execute
the validator, and a complete exact set of typed invariant receipts must mint
progression. A registered or selected invariant is not a passed invariant.

For conditional and derived work, Relational aspects and Signal aspects are
not interchangeable. Relational publishes authoritative semantic change;
Signal tracks executable invalidation and computation state. Runtime Bridge
installs the correspondence between them, including exact projection and any
admitted widening. Query authors the Foundational semantic dependency and
retains the admitted correspondence as opaque Query authority. This produces
one graph-shaped path from authored meaning to committed change to conditional
decision rather than a Relational graph and a second application-owned Signal
graph that merely happen to share names.

`worth-proof` is used where a transition or participation claim must be
unforgeable and move with the phase that earned it. `worth-foundational` owns
the portable aspect contracts and bindings. Neither crate replaces
Relational's authoritative publication or Signal's execution decision; they
let Query carry those meanings without reducing them to strings and digests.

Use this category when the feature needs invariants, relational truth, joins,
capability-gap posture, or lower truth reasoning that should be visible in the
public domain surface.

The mistake to avoid is deciding “this is about truth or invariants, so I
should skip Query.” In this architecture, a major part of the point is that
domains should not have to invent their own relational-entry folklore above the
runtime.

Read next:

- [Declaration Relational Truth Routing](./domain-capabilities/declaration-relational-truth-routing.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)
- [Aspects And Authority Lanes](./modeling/aspects-and-authority-lanes.md)
- [Capability Gaps And Invariant Denials](./domain-capabilities/invariants/capability-gaps-and-invariant-denials.md)

## Read Composition And Graph Authoring

Read composition turns typed application-query declarations into one installed
read graph shared by every supported lane. Generic graph composition authors
identity-dependent workspace mutations as one ordered batch. The generic
composition engine is preserved, but it is not an installed application
obligation executor.

Expression, validation, planning, and collection semantics have their own
categories above; this section is where you compose reads and graph work that
should remain Query-owned rather than host-local.

Use this category when you are building read bundles or graph-shaped authoring
that must lower through Query’s canonical artifacts.

Graph-owned lookup is part of this authority boundary. If a Query-owned feature
repeatedly needs to find nodes by canonical identity, resolve owner or placement
kind, enumerate legal children or targets, check scope or boundary posture, or
splice/move within a structured authoring graph, first establish one canonical
Query-owned graph/index view and consume that view. Do not start with recursive
tree walks, per-call registry scans, or surface-local lookup helpers and treat a
later index as a mere performance cleanup. In Query, the graph/index is part of
the proof boundary, not just an optimization.

Installed execution follows the same one-graph rule. One logical graph is the
default across Query graph participation, Runtime Bridge correspondence, and
Signal execution. Declare separate graph participation only when another graph
has genuinely independent authority, lifecycle, and provider ownership. A
multi-graph mutation binds only when the complete graph set has one atomic
commit authority or the operation declares the required compensation posture.

One Runtime Bridge binds one Signal graph ownership domain, and one Signal
graph accepts one aspect-lowering owner. Clones of the same Bridge may share
that owner; independent Bridge runtimes may not maintain mutually invisible
allocations over the same graph. Query exposes Query-owned correspondence and
provenance, not raw Bridge witnesses, Signal nodes, aspect slots, or candidate
construction. This prevents an application adapter from quietly becoming a
second graph authority.

The legality rules themselves are installed domain invariants, not consumer
validation code. Declare them in the installed operation contract and let the
provider session execute them against proposed state. A contributed-capability
graph denial may explain a rejected generic composition shape, but it is
non-executable and does not replace installed invariant progression.

Installed application operations should not have to remember which validators
apply. Their typed contracts derive one sealed obligation set; the managed
session constructs proposed state and invokes each installed invariant's real
owner before compare-and-commit. Generic graph composition exposes no manual
invariant callback and cannot mint an application invariant verdict.

The mistake to avoid is duplicating query legality or planning in domain helpers
when read composition or graph composition already owns the lane. A closely
related mistake is host-local traversal folklore: helper loops that reconstruct
graph meaning separately inside commands, scoped editing, legality, or
UI-adjacent surfaces.

Read next:

- [Read Composition](./authoring/read-composition.md)
- [Graph Composition Authoring](./authoring/graph-composition-authoring.md)
- [Graph Touch Obligation Authority](./authoring/graph-touch-obligation-authority.md)
- [Canonical Graph Obligation Progression](./domain-capabilities/canonical-graph-obligation-progression.md)
- [Query Expressions And Result Shapes](./authoring/query-expressions-and-result-shapes.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)
- [Conditional Installed Operations](./domain-capabilities/conditional-installed-operations.md)

## Structural Correspondence And Historical Materialization

Structural correspondence and historical materialization-path metadata make
historical evaluation explicit and ambiguity-honest when truth identity or
materialization path affects what a query may read.

This sits beside branch/historical/diff contexts and lineage work: it is about
how Query names correspondence and historical materialization without host cache
repair or silent basis substitution.

Use this category when historical reads depend on structural match posture,
materialization path identity, or explicit rejection of ambiguous historical
targets.

The mistake to avoid is reconstructing history through ambient host caches
instead of declared basis and materialization contracts.

Read next:

- [Structural Correspondence And Historical Materialization](./capabilities/structural-correspondence-and-historical-materialization.md)
- [Historical Diff And Basis](./capabilities/historical-diff-and-basis.md)
- [Lineage And Correspondence](./capabilities/lineage-and-correspondence.md)

## Cross-Runtime Causal Inspection

This is the **`CausalInspection` lane** (`admit_causal_inspection`,
`request_causal_inspection`) for cross-runtime causal explanation—not
`workspace.inspections()?.inspect`, which is per-target retained evidence only.

`CrossRuntimeCausalExplanation` at reference-only richness is **supported**;
materialized detail is **advisory**. Archive-backed reconstruction is
**deferred**.

Construct `CausalInspection` from both the originating receipt and a
`facade::foundation::ScopedInspectionBasis`. The receipt proves the event chain;
the scoped basis proves which truth world may be inspected. Admission and
materialization consume that combined artifact rather than recovering either
authority from identifiers.

Use this category when the question is end-to-end “why across runtimes?”—not
“what does inspect retain for this handle?”

The mistake to avoid is calling `workspace.inspections()?.inspect` cross-runtime causal inspection,
or using explanation contributions instead of the causal inspection API.

Read next:

- [Cross-Runtime Causal Inspection](./capabilities/cross-runtime-causal-inspection.md)
- [Inspection](./capabilities/inspection.md)
- [Inspection Vs Cross-Runtime Explanation](./domain-capabilities/choosing/inspection-vs-cross-runtime-explanation.md)
- [Lower-Runtime Capability Routing](./domain-capabilities/lower-runtime-capability-routing.md)

## Projection Consumption And Downstream Authority

Projection consumption carries materialized Query facts without reopening
source authority. On `WorthQueryReadCompletion`, declare the facts with
`read::project_facts()` and call `consume_projection(...)`.

Runtime-installed operations use the same production consumption machinery
through a stricter bound progression. Before execution moves the bound
capability, call `bound.consumer_projection_contract()`. After Query-minted
publication, pass that exact contract to `published.consume(...)`; the result
wraps the ordinary `WorthQueryConsumedProjectionAuthority` and can settle only
by consuming the progression value. It is not a second consumer system.

The contract is minted from the bound operation and Query-owned support truth.
It preserves exact installation generation, operation and family markers,
basis and access context, projection mask, publication and consumption
meaning, plus every typed support dimension. A separate consumer-boundary
value can carry presentation and allocation requirements, but cannot mutate
Query requirements. Admission requires the pair-bound compatibility witness;
matching summaries or digests are never authority.

Query returns `WorthQueryProjectionOutcome`. Completed and advisory outcomes
carry one sealed authority that retains basis, source lineage, facts, receipt,
and requirements. Call `into_admitted()` to move it into the downstream owner.
Violation, deferred, and unavailable outcomes remain typed and cannot be
promoted from receipts, IDs, or digests.

The mistakes to avoid are fishing in lower-runtime truth for IDs or passing a
basis digest, receipt digest, source label, and fact list as a replacement
authority tuple.

Read next:

- [Projection Consumption](./capabilities/projection-consumption.md)
- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Projection Consumption Vs Inspection](./domain-capabilities/choosing/projection-consumption-vs-inspection.md)
- [Policy, Tenant, And Relationship-Proof Narrowing](./foundations/policy-tenant-and-relationship-proof-narrowing.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)

## Consumer Extensions

Downstream crates extend Query through typed `facade::domain` contribution
contracts. A contribution declares domain meaning; Query still owns canonical
admission, execution, receipts, and outcomes. Keep family-specific ergonomic
helpers as thin declaration builders over that same package-assembly facade.

Once a host has installed that meaning, downstream execution switches to
`facade::installed`; it does not keep using the package-assembly namespace.
The reference pattern is one thin binding crate that owns
domain-to-application refinement while forwarding Query authority intact.
Worth UI follows this shape through `worth-ui-query-binding`, its sole normal
production Query importer. It translates admitted Query facts and patch
receipts into UI-owned allocation and graph consequences without
reconstructing Query support, compatibility, invalidation, row identity,
patch, or lifecycle posture.

Do not add consumer-local coordinators, backend selectors, canonicalizers,
planners, executors, success-envelope builders, or subscription lifecycle
managers. Do not hide those reconstructions behind a consumer facade: the
facade should reduce imports and route typed denials, not become another
authority owner.

Read next:

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Consumer Kit](./foundations/consumer-kit.md)

## Temporal And Time-Aware Live Queries

Temporal query basis and time-aware subscriptions now ship on the
runtime-backed path as an extension of the same canonical query and
subscription model.

The shipped shape distinguishes truth time-travel from signal execution time,
admits time-only deliveries where no truth patch occurred, and lowers temporal
basis through bridge and signal authorities without making Query the owner of
clocks, wake queues, or reactive scheduling.

Use the support matrix and admission docs to see which ordinary runtime-backed
lanes are shipped and which sibling facade-family roots remain intentionally
support-gated. Do not invent parallel temporal APIs or ambient host timers.

The mistake to avoid is implementing stale-after, interval, deadline, or
rolling-window behavior as UI timers outside canonical query artifacts, or
confusing historical truth reads with clock-driven live reevaluation.

Read next:

- [Support Matrix And Admission](./foundations/support-matrix-and-admission.md)
- [Historical Diff And Basis](./capabilities/historical-diff-and-basis.md)
- [Subscription Selection And Diagnostics](./capabilities/subscription-selection-and-diagnostics.md)

## Async Resources And Result State

Async capabilities are the Query-owned way to model resource-backed or
completion-driven meaning without inventing a second async facade, a local
`loading` taxonomy, or host-owned retry folklore.

The important split is:

- declaration-side async meaning lives on canonical declaration input through
  async clauses such as source family, request identity, loading posture, and
  failure posture
- runtime-backed async state lives on the same live/state/inspection surfaces
  as the rest of Query, where retained result-state can become `pending`,
  `current`, `stale`, `cancelled`, `retried`, `revalidating`, `superseded`, or
  `denied`
- projection consumption, continuation, recovery, and downstream delivery carry
  that async posture forward instead of asking callers to reopen lower-runtime
  artifacts or transport callbacks

Use this category when the question is:

- how do I declare async/resource meaning honestly?
- where do I read current async result-state?
- how does async posture survive materialization or downstream delivery?
- what does replay, stale completion, or async-request drift look like?

The mistake to avoid is assuming async support means “there must be a
`workspace.async(...)` API somewhere.” Query does not work that way. Async
meaning is part of existing declaration, live, inspection, projection, and
continuation lanes.

Read next:

- [Async Resources And Result State](./capabilities/async-resources-and-result-state.md)
- [Runtime-Installed Domains](./domain-capabilities/runtime-installed-domains.md)
- [Inspection](./capabilities/inspection.md)
- [Projection Consumption](./capabilities/projection-consumption.md)
- [Continuation Pipeline](./domain-capabilities/continuation-pipeline.md)

## Decision Rules

Need the shortest path between close surfaces:

- start with the declarative capability guide, then open the owning feature doc

Need platform entry or operating world:

- for installed domain operations, resolve the installed domain and enter
  through `workspace.observe_operating_world()?.family(family).bind(...)`
- for other capability families, use the owning ordinary namespace plus a
  Query-owned workspace and explicit context

Need typed query read meaning:

- use query expressions, validation, planning, collections, scopes/templates/view
  shapes, and read composition—in that dependency order

Need policy, tenant, or proof-gated access:

- use policy/tenant/relationship-proof narrowing before execution, not post-read
  filters

Need domain work/request:

- declare a portable installed operation and register its exact volatile
  providers when the request is executable domain work
- use a typed `facade::domain` contribution only when the request contributes
  posture to another Query-owned artifact
- in both cases, let Query own admission, identity, receipts, and outcomes

Need execution-grade installed computation:

- declare the artifact, evidence, native-access, resource, decision-fact, and
  invariant meaning in the installed operation
- reserve capacity before provider allocation, then enter through
  `WorthQueryExecutionRuntime::managed_run_admission(...)`
- advance the Query-minted run, provider session, decision read-set,
  provisional proposal, and invariant phases in order
- preserve typed cleanup, recovery, stale, discard, and terminal outcomes
- stop at invariant progression; do not invent a public provider commit

Need identity/deduplication:

- use canonical declaration entries and canonical declaration artifacts

Need family support:

- use family taxonomy, capability matrix, readiness, inventory, and support
  admission

Need downstream consumer proof:

- use the Consumer Kit for evidence reports, hard-prohibition audits, support
  snapshots, support pins, in-memory test workspaces, and adoption/residue
  proof; do not hand-roll Query proof in the consumer crate

Need basis for read, mutate, replay, inspect, or materialize:

- use basis capability lifecycle, not raw branch or snapshot ids

Need posture before or after a run:

- use state/readiness before guessing and `facade::inspection` after execution;
  preserve the capability namespace's typed outcome and next action

Need relational invariants/truth:

- use Query relational truth, invariant contribution, or invariant registration
  surfaces first

Need one-shot or live read execution:

- use planning and snapshot-backed execution first; use live views for durable
  query-shaped surfaces; use subscriptions for long-lived admitted live meaning

Need concurrent committed reads:

- use shared read authority and basis-pinned contexts; do not copy snapshots or
  take committed-read locks to imitate concurrency

Need submission order or replay:

- use typed journal position and journal segment identity; do not parse
  `commit_identity`, receipt strings, or display text for order

Need signal/reactive behavior:

- use `facade::live` for a long-lived managed query surface
- use an installed conditional node for operation- or workflow-scoped
  eligibility, triggers, and recomputation
- in both cases, let Query and Runtime Bridge carry semantic meaning to Signal;
  do not author Signal slots as domain identity

Need async/resource-backed declaration meaning or retained async runtime state:

- use async declaration clauses, retained live async result-state, projection
  consumption posture, and continuation/recovery drift surfaces instead of
  inventing host-local loading or retry models

Need effects or staged delivery:

- use authority-scoped effects and intent admission, not ad hoc callbacks

Need graph mutation/writeback/bridge routing:

- use the installed operation's declared graph participation when the work
  belongs to a domain package
- otherwise use `facade::mutation` or `facade::workflow` with explicit
  authority; do not choose or invoke a lower backend in consumer code

Need intent admission or mutation evidence:

- use intent admission lattice and authoritative mutation evidence, not local Ok/
  Err wrappers

Need domain-authored capability posture:

- use the domain capability contribution seam and the relevant admission,
  support, workflow, continuity, aftermath, or explanation lane

Need serious runtime-backed product work:

- use the workspace facade and support/admission contract **per admitted family row**

Need failure/recovery:

- use the typed stop and next action returned by the owning ordinary namespace

Need cross-runtime why:

- use cross-runtime causal inspection

Need materialized facts without reopening authority:

- call `completion.consume_projection(read::project_facts()...)` and move the
  sealed authority from `WorthQueryProjectionOutcome::into_admitted()`

Need public-bridge read certification:

- consume published derived artifacts through projection consumption; do not
  read materialization rows or bridge-only helper state directly

Need support pinning:

- use Consumer Kit support snapshots and `support_pinning_contract(...)`; a
  local required-family list or admission loop is not real pinning

Need lower-runtime contact:

- use lower-runtime capability routing and boundary envelopes
- obtain boundary envelopes from real Query boundary receipts or other
  `WorthQueryLowerRuntimeBoundaryEnvelopeSource` values; do not construct or
  synthesize envelopes from strings

Need temporal or time-aware live behavior:

- check support matrix first; do not invent parallel temporal APIs because the
  shipped runtime-backed temporal surface lives on ordinary Query handles while
  sibling facade-family roots remain intentionally support-gated

Need async capabilities:

- read the async capabilities doc first; do not assume there is a separate
  async facade or that blanket async family visibility means every runtime
  profile admits ordinary async DX

Need public DX:

- keep package declaration and volatile provider registration in host assembly
  through `facade::domain` and `facade::runtime`
- expose installed operation consumption through one thin
  `facade::installed`-backed consumer boundary instead of teaching raw lower
  runtime plumbing or re-exporting assembly ingredients

Need installed native values:

- derive selections and `WorthQueryNativeAccessKey` values from the exact
  consumer projection contract, move the built request through
  `consume_bound(...)`, and access the settled or refreshed projection by key;
  do not turn field paths into consumer-owned selectors

Need installed capability compatibility or lifecycle:

- ask the exact named question: same installation, compatible basis,
  replacement, rebind, or execution sharing; these witnesses are not
  interchangeable
- promote, refresh, replace, rebind, cancel, and dispose through the typed
  bound-projection states; retain stopped or cleanup-pending states for retry
  or rollback instead of tracking resource identifiers locally

Need shared installed live work or consumer invalidation:

- enter through a Query-owned live owner and move-only leases, then derive and
  readmit the capability-bound delta before attaching a consumer consequence
- do not use region-scoped stream metadata, Foundational locators, copied
  generations, or replay inspection as invalidation authority

Need installed collection windows or query-shaped patches:

- prepare the initial consumer window from the settled projection, or bind an
  explicit opaque cursor through the bound collection capability
- bind the current admitted lease delta before planning a patch, apply only the
  exact returned patch, and preserve reset-required as a typed operation
- translate the application receipt into domain-local consequences; do not
  reinterpret raw CDC, diff the full collection, or reconstruct row identity,
  cursor, ordering, generation, or patch posture

## Current Installed-Operation Boundary

The installed-operation surface currently includes portable operation
semantics, graph participation, one operating-world root, bound execution,
publication and consumption, installed workflow DAGs, portable conditional
nodes, aspect-precise authoritative change publication, installed semantic
correspondence, Signal-owned decisions, Query-owned provenance on re-entry,
ordinary fresh re-execution, cert-only semantic replay, typed inverse and
compensation aftermath, exact-effect lineage, persistent naming, and sparse
identity promotion. It also includes declaration-indexed native access,
relationship-specific pair-bound compatibility, proof-carrying promotion,
refresh, replacement, rebind, cancellation and disposal, compiled dependency
impact, shared live execution owners, move-only consumer leases, and
capability-bound invalidation deltas. Bound collection capabilities now admit
opaque cursor windows and capability-, generation-, basis-, ordering-, and
lease-bound query-shaped patch delivery.

The installed-operation surface also includes execution-grade managed
computation. Portable contracts cover artifact identity, ownership, lifecycle,
reproducibility, search, convergence, transformation, native access, structural
counters, decision schemas, resource requirements, bounded steps, and yield
posture. Runtime authorities cover move-only artifact ownership, bounded native
access, real capacity reservation, managed direct and workflow runs, safe-point
cancellation, yield, same-runtime readmission, and managed convergence.

For graph-changing work, a running managed run can admit a sealed provider
plan, prepare a fresh provider session, capture and compare the complete
installed decision read-set, stage an isolated provisional overlay, inspect or
revise the proposed post-state, execute each installed invariant against an
admitted state load, and mint progression from the complete exact receipt set.
The boundary currently ends at
`WorthQueryInvariantProgressionAuthority`. Proposed state is still
non-authoritative, and commit preparation or commit is not exposed by this
progression.

Settled direct and workflow projections also expose a sealed consumption-cost
snapshot. Query owns these operational measurements: lookup, binding, support,
execution, dependency, and optional native-binding rows come from the real
boundary-local counters. Later native access, refresh, invalidation, window,
and patch work stays on its own result or denial. A snapshot may explicitly
materialize a Foundational counter-backed receipt or report, but those derived
artifacts cannot be readmitted as Query execution or lifecycle authority. See
[Consumption Cost Evidence](./domain-capabilities/consumption-cost-evidence.md).

Those downstream capabilities remain one authority chain. Native keys come
from a bound projection request. Sharing requires current pair compatibility
and dependency-closure reuse. One owner impact and epoch fan out to exact
leases, and each delta must be readmitted against its current lease before a
consumer-authored consequence can be attached. Attachment rechecks the owner
epoch and borrows the workspace for the consequence lifetime, so a stale
admitted delta cannot retain usable Query consequence authority. Consumers may
explicitly widen their own response but cannot downgrade Query's required
disposition. Readmission checks both the exact capability identity and its
generation, not just a coincidentally equal declaration or version number.

Ordinary live targets and installed semantic targets are routed through
independent indexes and then deduplicated. Canonical path matching recognizes
ancestor and descendant touches in both directions, and ordinary request
routing covers projection, predicate, and ordering dependencies. The attached
counters report the collection/relevance probes, selected and skipped
candidates, overlap deduplication, per-target routing, fan-out, batches,
touches, and native-key narrowing work that actually occurred.

Foundational locators, masks, provenance, and derived boundary artifacts
describe the delta but cannot act as it. Raw deltas project only
`StaleRetained` provenance; fresh materialization requires a still-current
admitted delta. Semantic convergence uses typed access-key meaning and canonical
Foundational-backed bytes, including the complete conditional declaration,
outcome, artifact-reuse posture, and realized observations while excluding
operational Signal identity. The Foundational semantic boundary is derived
from that same complete semantic projection, while work counters remain
separate. Cert replay may reexecute the same installed mutation, but only the
ordinary live owner and lease route emits and admits invalidation authority.
See [Bound Projection Lifecycle, Sharing, And Consumer Invalidation](./domain-capabilities/bound-projection-sharing-and-invalidation.md).

Collection windows and query-shaped patch delivery are available when the
installed operation's exact consumer-support contract admits collection
delivery. Patches preserve stable row identity, native facts, result state,
warnings, continuation, window shifts, explicit reset posture, compiled
impact, target lease, and exact work counters. `apply_patch(...)` validates
authority and delivery order before consumer mutation. The consumer then owns
only its local consequences. Foundational patch components remain descriptive
inside this stronger Query artifact and cannot authorize delivery by
themselves.

This boundary is specific to installed-operation phase values. Query's general
history, replay, live, subscription, and other capability families remain as
described in their owning sections and support rows; they are not methods that
can be inferred onto the installed progression.

### Operational identity is not its representation

Every current operational identity is minted by the runtime that owns the
operation. Relational owns committed truth; Runtime Bridge owns admitted
crossing and correspondence; Signal owns conditional decision evidence; Query
owns its current basis, receipt, installed-operation progression, compatibility,
lease, invalidation, aftermath, replay, and lineage artifacts.

`worth-proof` carries generic progression and freshness. `worth-foundational`
supplies the shared authority, boundary-bridged, projection, digest-evidence,
and external-token categories. Neither surface upgrades a projection into
owner authority. Boundary crossing weakens identity, and the receiving owner
must validate retained source evidence before minting its own artifact.

Mutation writeback makes that validation exact. Runtime Bridge binds the
authoritative patch's Foundational touch meaning and the target collection,
Relational record, and mutation kind into its causality before execution.
Bridge carries the patch's canonical basis through lowering and denies a
different effect intent before authority execution.
Query admits the returned mutation receipt only when the same Bridge bundle
also retains its exact commit and snapshot and matches its single delta. A
bundle from another mutation, even one with the same broad writeback family or
copied projections, cannot mint current Query identity.

Use named methods such as `is_same_current_identity_as(...)` for operational
identity decisions. Evidence identities, terminal reporting projections,
digests, labels, formatted traces, external tokens, and index keys are for
reporting or candidate selection. Matching bytes do not authorize work. Query
does not export owner marker types or witness factories through its facade.
See [Operational Identity Authority](./foundations/operational-identity-authority.md).

## Current Authority Roots

- Domain setup starts with `WorthQueryDomainPackage::declare`, installs through
  `WorthQueryRuntimeBuilder::domain_package`, resolves from
  `WorthQueryWorkspace::domain`, and binds through
  `WorthQueryWorkspace::observe_operating_world()?.family(...).bind(...)` for
  observation or
  `WorthQueryWorkspace::prepare_mutation_operating_world()?.family(...).bind(...)`
  for mutation.
- Portable operation meaning lives in
  `WorthQueryDomainOperationDefinition`; volatile execution, graph,
  conditional, and workflow mechanics are registered separately and retained
  by the runtime.
- Portable computation-artifact meaning lives in
  `WorthQueryPortableArtifactContract`; installation mints
  `WorthQueryInstalledArtifactContractAuthority`, and runtime values are owned
  by move-only Query artifact handles.
- Execution-resource authority comes from an admitted plan plus a real capacity
  reservation bound to the exact installed operation. Support snapshots and
  counter reports remain descriptive.
- Managed-run authority joins the exact Query operation and resource attempt
  with Bridge execution-basis, Relational basis, Signal request, provider
  session, and artifact authority. Yielded capabilities retain that join for
  same-runtime readmission.
- Provider-plan and decision authority comes from the running managed run,
  installed graph participation, fresh provider-session generation, and
  complete compared decision read-set.
- Provisional and invariant authority comes from the exact staged session,
  fresh decision read-set, provider overlay, proposed-state generation,
  admitted state-load evidence, and complete installed invariant receipt set.
  It does not imply publication or commit.
- Native aspect meaning comes from `worth_foundational::facade::{AspectValue,
  StructAspectValue}` together with Foundational aspect contracts and bindings,
  and is admitted by Query against the active contract.
- Relational authoritative changes reach Signal execution only through an
  installed Runtime Bridge correspondence; Query exposes the semantic
  dependency and opaque Query-owned provenance rather than raw graph pieces.
- Fresh re-execution stays in Query; certification replay authority is
  exported only through `worth-query-replay` after exact semantic comparison
  and historical-basis admission. Its result cannot be converted into an
  ordinary completed trace; publication remains a separately bound ordinary
  progression.
- Aftermath authority comes from installed reversal or compensation meaning,
  normal candidate execution, and a domain-owned exact-scope postcondition
  proof. Partial effects remain visible when verification fails.
- Lineage authority comes from exact effect evidence bound to producing
  receipts. Persistent naming additionally requires the typed naming-mutation
  lane, while sparse promotion requires Schema Graph grammar and Foundational
  promoted-identity admission.
- Ordinary product work starts in the owning capability namespace and retains
  its typed declaration, outcome, stop, receipt, or managed handle.
- Downstream facts cross through `consume_projection(...)` and
  `WorthQueryConsumedProjectionAuthority`.
- Support posture comes from the support matrix and admission surface.
- Downstream certification comes from `facade::consumer_kit`.

## AI Checklist Before Editing Code

Before building on a Query category, answer these:

1. What category am I actually in?
2. What is the public entrypoint for that category?
3. What is the canonical identity boundary?
   If this is preview work: is it `WorthQuerySessionLabel` or
   `BridgePreviewSessionIdentity`?
4. What Query artifact or outcome should be preserved instead of flattened?
5. What support row or admission gate decides whether the surface is real now?
6. If this is shared-read or replay work, what pins the read basis and what
   typed journal identity carries order?
7. If this reads published derived facts, am I using projection consumption
   rather than direct materialization access?
8. Am I using Query to carry lower-runtime semantics, or am I bypassing Query
   and inventing a local runtime path?
9. If this is downstream consumer proof, am I using the Consumer Kit instead of
   a local report, grep, pinning, adapter, or receipt-fabrication path?
10. If another runtime depends on Query facts, am I transferring one
    `WorthQueryConsumedProjectionAuthority` rather than pairing basis, receipt,
    source, fact, label, or digest projections locally?
11. If this is an ordinary installed operation, did I import
    `facade::installed`, enter through one operating world, and preserve the
    move-only `bound -> executed -> published -> consumed -> settled` authority
    chain rather than forcing it through managed-computation phases?
12. If the operation is conditional, did Query author semantic dependencies,
    Runtime Bridge admit the exact correspondence, and Signal produce the
    decision, or did I accidentally build another condition engine?
13. If an aspect crosses runtimes, am I distinguishing Foundational semantic
    meaning, Relational authoritative change, Bridge correspondence, and the
    runtime-local Signal slot?
14. Is there one logical graph and one aspect-lowering owner, or did a facade,
    adapter, provider registry, or host helper quietly become a second graph?
15. Am I relying on an authoritative installed package or retained provider,
    or on a rebuildable index, report, digest, or matching semantic key?
16. Does an early denial prove zero later-phase work through exact counters?
17. If this is installed-operation replay, am I using the cert-only facade and
    comparing exact effects, publication, conditional evidence, and lineage?
18. If this is aftermath, what domain proof establishes the exact target and
    postcondition, and where are partial effects retained on failure?
19. If this is lineage or promotion, what exact effect and publication evidence
    binds the identity, and which Schema Graph and Foundational authorities
    admit promotion?
20. If this is mutation readmission, does one Bridge-owned causality bundle
    match the exact commit, snapshot, collection, Relational record, mutation
    kind, and Foundational touch set, or am I pairing separately valid
    projections into a second authority?
21. If this claims bounded cost, which boundary-local result or denial carries
    the exact counters, and is any Foundational export still treated only as
    derived reporting evidence?
22. If this is collection delivery, did the exact lease-bound invalidation
    admit the patch, and does the consumer apply the Query patch before deriving
    only domain-local consequences?
23. If this produces computation artifacts, does the portable contract declare
    exact identity, ownership, lifecycle, carriage, reproducibility, access,
    counters, decisions, governance, and compatibility?
24. If an artifact crosses stages, am I carrying the move-only handle, admitted
    transfer, borrow, lease, and disposal posture rather than IDs or `Any`
    payloads?
25. If this claims bounded native access, does the installed path govern the
    request, does chunk width bound actual variable-width memory, and is scalar
    fallback explicit?
26. If this starts managed work, was real capacity reserved before provider
    allocation, and which authority releases or transfers that reservation?
27. If this yields or readmits, am I carrying the exact move-only yielded
    capability and minting fresh attempt and session generations rather than
    reconstructing progress from checkpoint evidence?
28. If this stages provider work, did the sealed plan bind the exact operation,
    resources, basis, provider generation, and installed decision closure before
    reads and effects separated?
29. Does the proposal depend on one complete compared decision read-set, and do
    stale facts force replanning before any provisional stage?
30. Is proposed state still non-authoritative, did every installed invariant
    execute against an admitted state load, and does only the complete exact
    receipt set mint progression without implying public commit?

If you cannot answer those, read the owning docs before writing code.

## When In Doubt

Use this decision order:

1. Query public docs
2. Query facade surface
3. support matrix / admission
4. inspection
5. lower-layer docs only to understand semantics

If the current public lane cannot do the job honestly, do not invent a local
runtime above the lower layers. Stop, read the owning docs, and choose the
nearest honest public Query lane first.
