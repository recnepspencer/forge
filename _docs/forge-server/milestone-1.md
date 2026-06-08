# Milestone 1: Server Runtime Front Door, Typed Middleware, And Facade Boundary

## Goal

Create one typed `forge-server` runtime front door so every future
Forge-native facade, compatibility API, lease, sync, binary, and integration
surface enters through the same server-owned policy, tenant, branch,
workspace, diagnostics, and observability pipeline before any product-local
surface logic executes.

## Why This Milestone Exists

The first server mistake would be to let the earliest useful surfaces grow as
separate entry paths: one shape for Forge-native work, another for HTTP
compatibility routes, another for future sync sockets, and then ad hoc policy
and diagnostics glue around each one.

If that happens, the whole roadmap rots early:

- Query meaning gets rediscovered at the network edge
- remask and branch posture become route-local folklore
- regulated evidence posture becomes bolt-on logging
- future leases, sync, and files inherit incompatible request context models
- Forge-native ergonomics become "fewer endpoints" instead of "no endpoint
  glue for ordinary product work"

This milestone exists to prevent that failure before any higher-level surface
ships.

## Governing Summaries

- `MENTALITY.md`: the spec must solve the hostile server failure mode first,
  not optimize for a quick endpoint demo.
- `arch_laws.md`: `forge-server` needs one facade and one typed authority path;
  surface-local helpers may not become parallel semantic entrypoints.
- `composition_laws.md`: facade ownership, middleware progression, Query
  handoff, denial shaping, and diagnostics each need separate named homes.
- `domain_structure_laws.md`: request context truth, derived response shaping,
  diagnostics evidence, and future transport families must stay structurally
  distinct from one another.
- `perf_laws.md`: the front door must resolve policy, topology, and Query
  handoff before execution rather than re-deciding them inside hot route code.
- `forge_server_roadmap.md`: Milestone 1 belongs first because every later
  server capability needs one forced path through auth, tenant, branch,
  diagnostics, and observability.
- `AI_README.md`: Query is the ordinary domain-facing runtime, so the server
  must project Query-owned meaning instead of creating local runtime folklore
  above lower layers.
- `milestone-9.4.md`: Query 9.4 closes one runtime-backed downstream delivery
  contract for downstream runtimes and makes temporal/async/mixed-cause
  semantics part of ordinary Query surfaces rather than sidecar APIs.

## Adversarial Constraint

For the same authenticated principal, tenant/workspace target, branch/basis
posture, remask posture, and canonical Query operation, `forge-server` must
admit, deny, hand off, and explain the request through the same typed server
pipeline regardless of whether the caller arrived through a Forge-native
surface, a compatibility HTTP route, a future sync path, a future lease path,
or a future binary/integration surface.

This milestone fails if any admitted server surface:

- resolves auth, tenant, branch, remask, or diagnostics differently from the
  shared pipeline
- reinterprets Query meaning locally instead of handing canonical work to
  Query-owned surfaces
- flattens denial or provenance into route-local strings
- lets future surfaces bypass server evidence and counters
- or requires regulated evidence posture to be reconstructed from host logs

## Product Decision Lock

- `forge-server` is Query-first from the first line of code. It does not define
  read meaning, live meaning, mixed-cause meaning, or mutation meaning.
- Milestone 1 is not the HTTP milestone. It is the forced-entry milestone that
  all later HTTP, Forge-native, sync, lease, file, and integration work must
  build on.
- Forge-native ergonomics are first-class, but they still travel through the
  same server context and policy pipeline as compatibility APIs.
- Runtime-backed-now versus durable-later honesty must be preserved in typed
  contracts from the start. No surface may imply durable restart-stable
  semantics that upstream contracts have not closed.
- Regulated-industry posture starts here through typed evidence, denial, and
  routing artifacts, even though the larger regulated milestone lands later.

## Phase Plan

### Phase 1: Server Facade And Public Entry Boundary

Freeze the single public server entry boundary so `forge-server` begins life as
one facade over subsystem-shaped responsibilities rather than a handler bag or
framework-shaped directory tree.

**Relevant subsystems**
- `forge-server` public facade
- server configuration and bootstrap
- route-family registration

**Relevant APIs**
- `ForgeServer`
- `ForgeServerConfig`
- `ForgeServerRuntime`
- `ForgeServerSurfaceFamily`
- `ForgeServer::serve(...)`

**Warnings**
- Do not make `axum` routers the real public architecture.
- Do not expose surface registration through unconstrained closure hooks.

**Test requirements**
- Add a facade-entry parity test proving multiple surface families register
  under one server facade contract and not through separate bootstrap paths.
- Add an adversarial bypass test proving a surface cannot attach executable
  server behavior without declaring its surface family through the shared
  facade-owned registry.

**Engineering decisions**
- The public entrypoint is a typed server facade, not a direct web-framework
  export.
- Surface families are named and registered explicitly so later milestones can
  stay structurally separate while still sharing one front door.

**Open questions**
- None.

### Phase 2: Typed Request Context And Resolution Artifact Boundary

Freeze the canonical request-resolution artifact that every admitted request
must produce before Query-facing execution can begin.

**Relevant subsystems**
- authentication identity resolution
- tenant/workspace targeting
- branch and preview targeting
- transport classification
- diagnostics and provenance posture

**Relevant APIs**
- `ForgeServerRequestContext`
- `ForgeServerAuthenticatedPrincipal`
- `ForgeServerWorkspaceTarget`
- `ForgeServerBranchTarget`
- `ForgeServerTransportClass`
- `ForgeServerDiagnosticsPolicy`

**Warnings**
- Do not let route handlers invent their own context structs.
- Do not collapse branch targeting, basis posture, and workspace targeting into
  one ambiguous request blob.

**Test requirements**
- Add a hostile context-equivalence test proving equivalent requests from two
  surface families produce byte-for-byte equivalent canonical request-context
  artifacts.
- Add a drift-localization test proving malformed auth, tenant, workspace, or
  branch inputs fail before Query handoff and identify the exact failed
  resolution artifact rather than surfacing as generic request errors.

**Engineering decisions**
- Request context is a typed artifact produced once and consumed downstream.
- Transport class is carried explicitly so later sync, binary, and HTTP
  mechanics can diverge without redefining policy or Query meaning.

**Open questions**
- None.

### Phase 3: Middleware Progression And Denial Ordering Boundary

Freeze the server-owned middleware progression so auth, targeting, budget,
authorization, validation, Query handoff preparation, response shaping, and
observability execute in one intentional order with typed denial boundaries.

**Relevant subsystems**
- middleware pipeline orchestration
- rate and budget posture
- request validation
- authorization and remask preconditions
- response exit pipeline

**Relevant APIs**
- `ForgeServerMiddlewarePipeline`
- `ForgeServerPipelineStep`
- `ForgeServerAdmissionOutcome`
- `ForgeServerDenial`
- `ForgeServerValidationOutcome`

**Warnings**
- Do not let middleware become an untyped stack of incidental framework layers.
- Do not reorder branch/policy validation after execution just because a route
  surface finds that more convenient.

**Test requirements**
- Add a miserable-path ordering test that injects auth, tenant, branch,
  authorization, and validation failures in overlapping combinations and proves
  the pipeline returns the same canonical denial priority every time.
- Add a hidden-bypass test proving no route or surface-local adapter can enter
  Query handoff without a completed shared pipeline receipt.

**Engineering decisions**
- Pipeline progression is modeled as typed server steps, not just framework
  middleware composition.
- Denial ordering is part of the server contract and must be mechanically
  stable across surface families.

**Open questions**
- None.

### Phase 4: Surface Family Segregation Boundary

Freeze the separation between Forge-native surfaces, compatibility APIs,
future sync surfaces, future lease surfaces, future binary surfaces, and future
integration surfaces so they can share one front door without sharing local
semantics or file topology.

**Relevant subsystems**
- surface-family registration
- surface-family routing roots
- server module topology
- public export boundaries

**Relevant APIs**
- `ForgeServerSurfaceFamily`
- `ForgeServerSurfaceRegistration`
- `ForgeServerSurfaceRoot`
- `ForgeServerSurfaceCapabilities`

**Warnings**
- Do not create a generic `routes` or `handlers` bucket.
- Do not let one surface family reach into another family's internal request
  models just because the transport looks similar.

**Test requirements**
- Add a structural isolation test proving each surface family can be deleted or
  disabled without editing unrelated family modules or shared pipeline truth.
- Add a hostile cross-family import test proving surface-local code cannot
  depend on deep internals from a sibling surface family rather than the shared
  facade contracts.

**Engineering decisions**
- Shared entry is not shared semantics; each surface family gets a named home.
- The facade owns cross-family registration while each family owns only its
  local shape above the shared pipeline.

**Open questions**
- None.

### Phase 5: Query-First Execution Handoff Boundary

Freeze the canonical server-to-Query handoff so the server consumes Query-owned
meaning through typed public seams instead of rebuilding read, mutation, state,
or delivery semantics locally.

**Relevant subsystems**
- Query-backed request lowering
- workspace acquisition and binding
- downstream delivery contract intake
- support and capability posture

**Relevant APIs**
- `ForgeServerQueryOperation`
- `ForgeServerQueryHandoff`
- `ForgeServerQuerySupportPosture`
- `ForgeQueryWorkspace`
- `workspace.public_downstream_delivery_contract()`
- `workspace.downstream_delivery(...)`

**Warnings**
- Do not hand raw route payloads directly to lower runtimes.
- Do not let the server mint its own meaning for temporal, async, or
  mixed-cause delivery just because downstream transport work has not landed
  yet.

**Test requirements**
- Add a Query-handoff parity test proving equivalent operations from distinct
  surface families lower to the same canonical Query handoff artifact and the
  same support posture.
- Add an adversarial unsupported-capability test proving runtime-backed-now
  versus durable-later distinctions fail typed at handoff time instead of being
  hidden behind optimistic server wrappers.

**Engineering decisions**
- Query handoff artifacts are server-owned mechanism over Query-owned meaning.
- The server may add transport/session/routing mechanics but not a second
  semantic runtime.

**Open questions**
- None.

### Phase 6: Response, Denial, And Provenance Envelope Boundary

Freeze the typed response and denial envelope family so every admitted and
denied operation leaves the server with machine-checkable provenance instead of
route-local text shaping.

**Relevant subsystems**
- success envelope shaping
- denial and validation envelope shaping
- provenance attachment
- response transformation

**Relevant APIs**
- `ForgeServerResponseEnvelope`
- `ForgeServerSuccessEnvelope`
- `ForgeServerDenialEnvelope`
- `ForgeServerProvenance`
- `ForgeServerResponseTransform`

**Warnings**
- Do not flatten typed denials to status-code-plus-string as the only
  preserved artifact.
- Do not make provenance an optional debug-only add-on for ordinary server
  flows.

**Test requirements**
- Add an envelope-parity test proving the same canonical Query operation yields
  equivalent provenance-bearing response envelopes independent of route surface
  or transport-specific formatter choice.
- Add a denial-localization test proving auth, tenant, branch, remask, support,
  and validation failures remain distinguishable in canonical denial envelopes
  and cannot collapse into the same broad error class.

**Engineering decisions**
- Response shaping is a server responsibility, but it must preserve upstream
  meaning rather than overwrite it.
- Provenance and denial artifacts are first-class contracts because regulated
  posture and later sync diagnostics depend on them.

**Open questions**
- None.

### Phase 7: Observability, Counters, And Operator Evidence Foundation

Freeze the minimum operator-facing evidence surface for Milestone 1 so request
admission, denial, handoff, and response behavior can be explained through
typed counters and evidence artifacts without relying on ad hoc host logs.

**Relevant subsystems**
- server metrics and counters
- request trace artifacts
- operator evidence surfaces
- diagnostics export policy

**Relevant APIs**
- `ForgeServerCounters`
- `ForgeServerRequestTrace`
- `ForgeServerEvidenceRecord`
- `ForgeServerAdmissionCounterSet`
- `ForgeServerExecutionCounterSet`

**Warnings**
- Do not make counters a later observability chore.
- Do not couple hot-path execution to rich free-form logging as the only
  explanation path.

**Test requirements**
- Add a counter-honesty test proving admissions, denials, and Query handoffs
  increment exact narrow counters and do not rely on broad "request failed"
  aggregates as primary proof.
- Add a hostile evidence-reconstruction test proving an operator can classify
  why a request was denied or admitted from canonical evidence artifacts even
  when free-form logs are missing, reordered, or disabled.

**Engineering decisions**
- Observability begins as typed evidence and counters, not string logs.
- The server must preserve enough evidence now to support later regulated and
  distributed milestones without retrofitting core request flow.

**Open questions**
- None.

### Phase 8: Hostile Certification Closure Boundary

Close Milestone 1 with certification that proves the server front door really
forces one path under hostile variation rather than merely working for one
happy-path route.

**Relevant subsystems**
- server certification harness
- cross-surface parity harness
- denial/evidence certification
- structural import and bypass guards

**Relevant APIs**
- certification harness artifacts
- canonical request-context digests
- canonical denial/evidence digests
- pipeline certification bundle surfaces

**Warnings**
- Do not close Milestone 1 on basic route tests.
- Do not use broad response equality as the primary proof that the front door
  is honest.

**Test requirements**
- Add one mixed-hostility certification matrix that varies surface family,
  malformed identity, tenant mismatch, branch mismatch, remask pressure,
  unsupported capability posture, and diagnostics policy while asserting exact
  canonical context, denial, provenance, and counter digests.
- Add one structural sabotage suite that attempts direct route execution,
  sibling-surface deep imports, skipped middleware receipts, and ad hoc Query
  handoffs, and proves each attempt fails at the narrowest expected boundary
  with exact zero assertions for forbidden counters and forbidden evidence
  artifacts.

**Engineering decisions**
- Certification closes the milestone only if the shared path is mechanically
  enforced.
- The proof bar for Milestone 1 is hostile parity and hostile denial, not
  nominal handler success.

**Open questions**
- None.

## Must Ship

- one typed `forge-server` public facade and runtime bootstrap boundary
- one canonical typed request-context artifact for auth, tenant/workspace,
  branch, transport, and diagnostics posture
- one intentional middleware progression with typed denial ordering
- explicit surface-family segregation for Forge-native, compatibility, sync,
  binary, lease, and integration lanes
- one Query-first execution handoff boundary
- typed response, denial, and provenance envelopes
- typed counters and operator-evidence foundation
- hostile certification proving the forced-entry model actually holds

## Must Preserve

- Query remains the semantic runtime authority for ordinary product meaning
- no server surface bypasses auth, tenant, branch, validation, authorization,
  diagnostics, or evidence production
- runtime-backed-now versus durable-later posture remains explicit
- shared entry does not collapse surface-family separation
- regulated evidence posture is derived from typed artifacts, not reconstructed
  from route-local strings or ad hoc logs

## Acceptance Evidence

- cross-surface parity bundles proving equivalent requests lower to the same
  canonical request-context and Query-handoff artifacts
- typed denial bundles proving auth, tenant, branch, validation, support, and
  remask failures localize to the correct shared pipeline boundary
- structural proof that surface families remain separate above the facade while
  sharing one forced-entry pipeline
- counter and evidence bundles proving operator-visible classification without
  log archaeology
- hostile certification suites proving bypass attempts fail with exact zero
  assertions for forbidden handoffs and forbidden evidence artifacts

## Sequencing Notes

This milestone belongs first because every later capability in the
`forge-server` roadmap depends on one forced server entry path. If Milestone 2
or Milestone 3 came first, they would be tempted to define their own request
context, denial rules, and Query integration shape, which would poison later
lease, sync, remask, regulated, and distributed work.

It also belongs immediately after Query 9.4 because the server now has a real
runtime-backed downstream delivery contract and an ordinary Query-facing public
surface to consume. The right first server step is therefore not "add
transport"; it is "force every future transport and facade through one honest
server-owned boundary."
