# Milestone 9.16: Authenticated Async Bank World And The Ordinary Query Front Door

## Goal

Prove that a small team can build a legitimate authenticated, multi-user,
asynchronous application through the ordinary Query API without reconstructing
runtime authority, writing stringly semantic adapters, or reaching into Query
internals.

The proving application is an in-memory bank and person-to-person payment world.
It has real users, personal, business, institution, branch, and estate scopes,
capability-constrained customer and employee authority, field-level disclosure,
double-entry monetary effects, deposits, withdrawals, transfers, approvals,
compensating reversals, linear undo and redo, concurrent requests, live
updates, and a real Authentik OIDC boundary. Its hostile authorization world
includes an employee whose institutional power conflicts with a personal
interest in a deceased relative's estate, plus a governed break-glass path that
cannot become superuser authority.

The application is not a showcase shell. It is the adversarial consumer that
defines whether Query has a front door.

## Roadmap Placement

Milestone 9.15 ends with an installed operation whose proposed state has passed
real invariants. Milestone 9.16 adds the authority and public composition needed
to make that work useful to an ordinary application:

```text
schema-bound typed intent
    -> authenticated principal
    -> installed capability and disclosure context
    -> scoped authorization or governed elevation
    -> admitted touched graph
    -> canonical application query / operation basis
    -> prepared and invariant-approved proposal
    -> provider compare-and-commit
    -> typed outcome
    -> ordinary read / mutation / workflow / history / live result
    -> governed recovery / compensation / linear redo
```

Milestone 9.17 follows this milestone. Advanced access and computation features
must be built through the same public front door proven here.

## Work Types And Phase Identity

Milestone 9.16 contains three visibly different tracks. Each track has its own
ordinary phase sequence:

```text
Runtime Hardening Track   Phase 1 -> Phase 2 -> ... -> Phase N
Bank World Track          Phase 1 -> Phase 2 -> ... -> Phase N
Closure Track             Phase 1 -> Phase 2 -> ... -> Phase N
```

The Runtime Hardening Track owns generic Query product work:

- Runtime Phase 1 establishes schema-bound typed application references;
- Runtime Phase 2 establishes generic authenticated-principal admission;
- Runtime Phase 3 establishes installed permission and touched-graph authority;
- Runtime Phase 4 establishes provider compare-and-commit;
- Runtime Phase 5 establishes ordinary read, mutation, workflow, and live
  facades;
- Runtime Phase 6 establishes installed application queries, basis controls,
  Query-owned continuations, and one-shot/history/live/preview parity;
- Runtime Phase 7 establishes purpose-bound capabilities, field disclosure,
  conflict-of-interest, delegation, and break-glass authority;
- Runtime Phase 8 establishes actionable recovery plus linear undo and redo;
  and
- Runtime Phase 9 performs public policy cutover and workaround deletion.

Bank phases consume and pressure those runtime capabilities:

- Bank Phase 1 freezes the product, courtroom, and gap ledger;
- Bank Phase 2 builds the Authentik adapter and dynamic identity fixture;
- Bank Phase 3 installs the banking domain, accounting, roles, and invariants;
- Bank Phase 4 adds estate administration, capability delegation,
  conflict-of-interest, emergency elevation, and aftermath semantics;
- Bank Phase 5 builds the temporary HTTP boundary and independent user nodes;
  and
- Bank Phase 6 proves the complete consumer journeys.

Closure Phase 1 runs hostile milestone certification and permanent
prohibitions. Track and phase together form the phase identity; the interleaved
order below describes causal execution across tracks.

## Discovery Intake And Phase Amendment Rule

The bank world is expected to discover missing behavior. Discovery is an input
to the milestone, not permission for an ad hoc fix inside whichever phase
happened to expose it.

Before implementing a discovered requirement, update the gap ledger and
classify it:

1. **Existing runtime guarantee is incomplete.** Reopen the affected Runtime
   phase and every downstream ledger guarantee that depended on it.
2. **New generic Query behavior, API, authority, lifecycle, or performance
   capability is required.** Add the next appropriately sized phase to the
   Runtime Hardening Track before implementation.
3. **Bank-domain meaning or behavior is missing.** Add or reopen the appropriate
   Bank World phase. Banking semantics must not be generalized into Query.
4. **Authentik, HTTP, process-fixture, or user-node mechanism is missing.** Add
   or reopen a Bank World adapter/fixture phase. Transport mechanics do not
   become runtime authority.
5. **The discovery changes public cutover, deletion, documentation, or the
   decisive courtroom.** Add or reopen a Closure phase.
6. **The discovery has an independent advanced-computation purpose.** Assign it
   to Milestone 9.17 rather than expanding the bank milestone.

A new phase must be an appropriate vertical slice with one causal guarantee,
not a ticket-sized patch. It states what proof it consumes, what architecture it
establishes, what it mechanically forbids, what evidence closes it, and which
later bank or runtime phase it unblocks.

The bank phase that exposed a generic gap remains blocked until the corresponding
Runtime Hardening phase closes. The application may not carry a local workaround
forward to keep the demo moving.

## Why This Milestone Exists

Milestones 9.12 through 9.15 built substantial authority, lifecycle, workflow,
replay-certification, artifact, and pre-commit machinery. Dogfooding began with
complex consumers, so it was possible for those consumers to tolerate local
builders, raw aspect strings, implicit principals, handwritten permission
checks, or deep runtime knowledge.

That is the unfinished front face of the house. A real bank world makes the
missing fundamentals impossible to hide:

- a caller identity must come from an external authentication boundary;
- authorization must depend on current graph relationships and the exact graph
  the operation can touch;
- concurrent money movement must commit atomically or fail with an honest
  outcome;
- retries must not duplicate effects;
- reads and live updates must not disclose unauthorized accounts or postings;
- business authorization must support multiple users with different scopes;
- employee powers must be institution-scoped and distinct from customer powers;
- a role name must not silently grant every action, field, purpose, or resource
  inside its organizational scope;
- a principal whose employee authority conflicts with a personal interest must
  not gain power by combining otherwise valid relationships;
- emergency access must be time-, purpose-, field-, action-, and
  resource-bounded and must remain incapable of authorizing unrelated money
  movement;
- named application queries, their cursors, and their current, historical,
  live, and preview forms must retain one canonical meaning;
- indeterminate work must expose a lawful recovery action rather than a
  dead-end status;
- undo must preserve committed history through a declared inverse or
  compensation, and redo must be a freshly authorized execution rather than
  ordinary replay;
- application code must use schema-derived types instead of string aspect keys;
- transport adapters must translate rather than own policy or runtime
  authority; and
- several independent processes must communicate asynchronously over real
  network boundaries.

If implementing this world still requires local Query folklore, the milestone
remains open and the workaround becomes a Query requirement or an explicitly
owned non-Query concern.

## Adversarial Constraint

The fixture creates its bank, users, employees, accounts, role assignments, and
OIDC subjects dynamically. No application principal, account, role assignment,
balance, or authorization outcome is hard-coded into product logic.

Each fixture participant runs in a separate user-node server process with its
own async runtime and network listener. User nodes authenticate through a real
Authentik issuer and call one authoritative in-memory bank server over TCP.
Requests race, retry, disconnect, reconnect, lose authorization, and observe
live changes.

At minimum, the courtroom must include:

- two unrelated personal-account customers;
- a joint personal account whose owners have non-identical transaction and
  disclosure authority;
- one business with an owner, an initiator, an approver, and a read-only user;
- a delegated signer whose authority is narrower than the account owner's and
  expires during the fixture;
- a bank teller, branch manager, estate specialist, compliance officer, and
  bank auditor with non-equivalent branch, case, institution, action, and
  disclosure scopes;
- a user who has both customer and employee relationships;
- a deceased customer with an estate case, a court-recognized executor, at
  least two beneficiaries, restricted identity and account fields, and an
  account subject to probate controls;
- a branch manager who is also a beneficiary of that deceased customer's
  estate and therefore has a real conflict of interest;
- a non-conflicted compliance officer capable of approving only the exact
  exceptional access or estate action allowed by the case;
- two concurrent transfers competing for the same available funds;
- an idempotently retried transfer whose response is lost after commit;
- a business payment that requires approval by a different authorized user;
- authorization revoked while a live subscription is active;
- a purpose-bound capability narrowed to one estate case, one account family,
  selected fields, allowed operations, and a finite validity interval;
- an attempted use of branch-manager authority against the manager's own
  beneficial interest, including attempts to combine employee and beneficiary
  grants;
- a break-glass request for an urgent account-preservation investigation,
  including approval, expiry, revocation during live delivery, field masking,
  and attempted escalation from restricted read/freeze authority into
  disbursement;
- an inserted relationship that changes a previously negative authorization
  decision;
- a stale request whose read-set is relevant to the concurrent mutation;
- an unrelated mutation that must not cause a false conflict; and
- a committed transfer undone through a compensating journal entry, a redo
  attempted after intervening relevant drift, and an irreversible operation
  that exposes no fabricated undo action;
- an indeterminate response that returns an actionable recovery capability
  without granting commit, rollback, or retry authority to copied receipt data;
  and
- a crashed or disconnected user node that cannot leak a session, queue, or
  executable authority.

The system must fail closed. Authentication success is not authorization,
permission declaration is not permission evaluation, and policy evaluation is
not commit authority.

## Product Decision Lock

### Application boundary

1. The bank world is production-shaped example code with the same public
   dependencies available to an ordinary consumer.
2. The bank domain owns banking vocabulary, schemas, operations, invariants,
   authorization contributions, and presentation-independent application
   semantics.
3. Query owns canonical declarations, installation, admission, touched-graph
   policy composition, execution progression, compare-and-commit progression,
   result meaning, live-query meaning, and explanation contracts.
4. The temporary HTTP layer owns HTTP, OIDC transport adaptation, request
   deadlines, cancellation, serialization, and status-code mapping. It owns no
   banking policy and mints no Query or graph authority.
5. The fixture owns dynamic environment provisioning and teardown. It cannot
   introduce a privileged mutation or authentication bypass.

### Identity and authentication

6. Authentik is the authentication authority for the reference world.
7. An external principal is identified by the typed pair `(issuer, subject)`.
   Email, username, display name, token text, and locally assigned fixture
   names are not identity.
8. JWT validation requires issuer, audience, signature, time, and nonce/state
   posture appropriate to the selected OIDC flow.
9. The Authentik adapter produces an authenticated external-principal proof. It
   does not produce bank authorization or Query execution authority.
10. Mapping an external principal to a bank principal is an installed,
    graph-backed operation. Unknown, disabled, ambiguous, or stale mappings fail
    closed.
11. Automated tests acquire tokens through a real supported OIDC flow. A test
    helper may provision users and clients but may not mint, decode-and-trust,
    or bypass validation of identity tokens.

### Authorization

12. Authorization is capability-, relationship-, operation-, and context-
    scoped. A role may contribute capabilities but is never itself ambient
    permission. Flat role strings, route-local conditionals, caller booleans,
    and token claims open no application action.
13. An installed capability grant names the semantic action family, resource
    scope, admissible field or relation scope, purpose constraints, validity
    interval, delegation posture, grant provenance, and any amount,
    cardinality, or workflow-stage limit that changes what it authorizes.
    Omitted dimensions do not default to global authority.
14. Customer powers derive from current personal, business, executor,
    beneficiary, and authorized-user relationships. Employee powers derive
    from institution, branch, case, teller, auditor, manager, and compliance
    assignments. No grant silently crosses those scope or authority families.
15. Query composes explicit allow, deny, separation-of-duty,
    conflict-of-interest, distinct-actor, purpose, and field-disclosure
    predicates. A principal holding several valid grants receives only their
    lawful composition under installed policy; incompatible grants cannot be
    stacked into a stronger unnamed authority.
16. Relational owns authoritative graph facts and exact touched-graph evidence.
    The runtime bridge owns installed aspect correspondence and lowering.
    Signal owns local policy-node evaluation and decision evidence. Query owns
    their legal composition into operation admission.
17. No Query-local “super permission,” host callback, token claim, or route
    middleware result may replace that composition.
18. Authorization is evaluated against the graph the operation can actually
    read or affect. The executor cannot touch an entity, relation, aspect,
    field, case, or purpose outside the admitted capability and touched graph.
19. Negative authorization facts, membership dependencies, grant validity,
    purpose, conflict relationships, case state, approval state, and
    revocation enter the decision read-set whenever they influenced admission,
    so every authority-widening or authority-narrowing mutation is causally
    visible at commit and delivery.
20. Read projection, mutation admission, explanation, activity history,
    cursor continuation, preview, and live delivery distinguish internal
    computation authority from consumer disclosure authority. A protected
    fact may influence an installed policy, predicate, ordering, or aggregate
    only under separately admitted internal access and noninterference
    contracts; that access does not authorize disclosure. Consumer-visible
    result construction applies field-disclosure scope before projection or
    serialization. Masked or omitted fields carry typed posture; post-
    projection redaction is forbidden.
21. Break-glass is a governed state transition with requester, approver,
    reason, purpose, exact scope, allowed actions, disclosed fields, issue and
    expiry time, revocation, audit, and mandatory review posture. It cannot
    erase conflict-of-interest, distinct-actor, invariant, or commit
    requirements; the bank reference path is read/freeze-limited and cannot
    authorize estate disbursement or personal benefit. Revocation or expiry
    closes or narrows active delivery before the next payload.

### Typed schema and aspects

22. Installed schema declarations generate or bind typed entity, relation,
    aspect, field, operation, and policy references for ordinary consumers.
23. Application code does not pass semantic aspect names as strings.
24. Values use Foundational-native exact scalar and struct families. Money uses
    an exact typed minor-unit representation with an explicit currency.
25. Dynamic extension, if retained, uses an explicitly dynamic key type and a
    checked schema-readmission boundary. It is not an escape hatch in generated
    application code.
26. Typed references are runtime- and schema-version-bound where use could
    otherwise cross installations.
27. Generated or derived APIs expose only legal operators and writes for the
    referenced schema capability.

### Monetary model

28. Monetary effects are represented by immutable balanced postings. Account
    balances are derived from postings rather than independently mutable
    balance fields.
29. Every journal entry balances to zero in one currency and contains at least
    two postings.
30. Transfer, deposit, withdrawal, opening funding, compensating reversal,
    estate freeze, estate release, and estate disbursement each have a distinct
    typed operation, purpose, capability requirement, and aftermath posture.
31. Deposits and withdrawals move value through explicit bank cash or settlement
    accounts; they do not mint or destroy money implicitly.
32. Amounts are positive, currency-compatible, and bounded. Floating-point
    money is forbidden.
33. Available-funds and account-status invariants execute over proposed state.
34. Business payments and estate disbursements may require separate initiator,
    approver, executor, compliance, or beneficiary-exclusion predicates
    according to installed policy. One principal cannot satisfy a
    distinct-actor or conflict-free rule by holding multiple roles or grants.
35. A request-scoped idempotency key is bound to authenticated principal,
    operation family, semantic payload, and institution. Reuse with a different
    payload denies; a retry of the same committed intent returns the same
    semantic result without duplicating postings.

### Compare-and-commit

36. An invariant-approved proposal still cannot mutate authoritative state.
37. Compare-and-commit consumes the provider session, complete decision
    read-set, proposed state, invariant proofs, idempotency posture, and exact
    effect set.
38. The provider proves one atomic transaction or returns a typed non-commit
    outcome.
39. Relevant drift returns stale or denied posture. Unrelated drift may commit
    without widening the declared or realized authority.
40. Committed, stale, policy-denied, invariant-violated, aborted,
    partial-effect, indeterminate, cancelled, and already-committed-idempotent
    outcomes remain distinct.
41. An in-memory provider may honestly prove atomicity within its process. The
    milestone does not turn process memory into a stronger guarantee.
42. A provider that cannot prove atomicity may not return `Committed`.

### Async and transport

43. All network-facing and provider-facing operations are async and carry
    deadlines, cancellation, bounded queues, resource budgets, backpressure,
    overflow posture, and partial-effect meaning.
44. One user node equals one OS process, one async runtime, one network
    listener, and one independent authenticated session boundary.
45. The fixture uses dynamic ports and discovered identities. Shared process
    globals cannot coordinate users.
46. The bank server is the only authoritative application server. User nodes
    are clients/proxies and cannot cache or reconstruct bank authority.
47. Live updates use a real streaming transport such as SSE. Polling may exist
    only as an explicit degraded posture, not as fake live delivery.
48. Disconnect, timeout, and response loss do not imply rollback. The client
    resolves uncertainty through the typed idempotency/outcome API.
49. HTTP response mapping preserves semantic outcome distinctions and never
    treats every denial as `500` or every accepted request as committed.

### Replay and history

50. Ordinary users may query authorized account activity and receive live
    updates through normal Query surfaces.
51. Runtime replay remains certification-only. The bank certification suite may
    reconstruct results from retained in-memory fixture history through
    `worth-query-replay`; product and user-node crates may not import replay
    authority.
52. Replay evidence must agree with ordinary result meaning without becoming an
    application mutation or recovery API.

### Application queries, recovery, and aftermath

53. A named application query is an installed schema-bound definition with
    typed parameters, admitted root, result shape, cardinality, ordering,
    dependency and disclosure contracts, supported truth bases, and declared
    one-shot, history, live, or preview eligibility. A host-local marker plus
    an unregistered projection closure is not the completed front door.
54. Query owns canonical application-query identity and opaque continuation
    identity. A cursor is bound to the installed query, parameters, scope,
    basis, ordering, result shape, and compatibility generation. It carries no
    authorization; every continuation receives fresh admission.
55. Current, pinned historical, live, and admitted preview execution consume
    the same canonical query meaning where supported. A lane may strengthen
    lifecycle or consistency requirements but may not silently change
    projection, disclosure, ordering, or membership meaning.
56. Callers explicitly own result limits, work limits, deadline,
    cancellation, requested consistency, freshness posture where meaningful,
    truth basis, and mutation preconditions. Query turns expected-version or
    expected-fact preconditions into provider-recompared facts rather than
    host-side races.
57. Read and disclosure outcomes identify truth posture, provider version,
    work, result count, truncation, typed omissions, and a governed access
    receipt sufficient to explain what was disclosed under which installed
    authority. The receipt is evidence, not reusable authority.
58. Capability support is inspectable before execution: supported, supported
    with named degradation, provider capability required, Store capability
    required, advanced access product required, or unsupported. Lack of
    support never widens into a scan, host loop, post-read filter, or local
    cache.
59. Installed application-query predicates, filters, ordering, traversal, and
    result shape lower into the existing Milestone 9.10 graph-read proof
    chain: admitted schema references, canonical read graph, selectivity and
    access shape, `PredicateSupport` and `OrderingSupport` requirement rows,
    cost and inventory admission, one admitted access plan, and receipt-backed
    execution. Application queries do not create a parallel planner, index
    inventory, or execution lane.
60. Relationship expansion, nested result construction, filtering, and sorting
    execute through one admitted graph-read access plan or return a typed
    required-capability, streaming, materialization, or denial posture. A
    facade-side loop, per-result neighbor lookup, undeclared
    post-materialization sort, or repeated child query cannot satisfy the
    contract. Covered receipts prove exact-zero caller-owned N+1 work and zero
    undeclared fallback.
61. Every installed mutation declares `Reversible`, `Compensatable`,
    `Reconcilable`, or `Irreversible` aftermath posture. The public result
    exposes only next actions valid for that operation, outcome, current
    authority, and provider posture.
62. Undo never deletes or rewrites committed truth. It is a newly admitted
    operation derived from the exact committed receipt and installed inverse
    or compensation contract, with fresh authentication, authorization,
    policy, decision facts, invariants, idempotency, and compare-and-commit.
63. Bank money movement uses compensating journal entries. Authorization
    changes use explicit inverse operations when still lawful. Account
    creation, approval, audit, or externally escaped effects expose
    compensation, reconciliation, or irreversible posture rather than a fake
    inverse.
64. Redo is not replay and does not reuse prior authority. A successful undo
    may expose a descriptive redo intent for fresh execution against current
    truth. Relevant drift, changed policy, expired capability, or changed
    invariant posture may deny it.
65. Milestone 9.16 guarantees only a linear, current-head, receipt-linked
    undo/redo journey.
    A new divergent operation may invalidate the convenience redo chain
    without deleting history. Branch selection, branch-local inversion,
    branch-shaped redo, merge interaction, and history navigation are owned by
    the [cross-runtime merging-and-branching roadmap](../cross-runtime/merging-and-branching-roadmap.md).
    This milestone creates no placeholder API, directory, support posture, or
    implied authority for them.
66. Indeterminate and externally unresolved outcomes carry a framework-owned
    recovery handle naming the legal next actions: inspect, resolve by
    idempotency, retry safely, compensate, reconcile, or dispose. A bare status
    or copied receipt cannot manufacture any of those transitions.
67. Provider commit, emitted application causality, and external completion
    remain distinct typed facts. Local commit may authorize managed dispatch,
    but it cannot claim that a device, payment rail, notification system, or
    other external authority completed its effect.

## Destination Topology

### Query authority packages

```text
worth-query-decl
    schema-derived typed references
    typed external identity and principal-binding declarations
    installed application-query and result-shape declarations
    capability, purpose, disclosure, delegation, and elevation declarations
    operation, read, mutation, workflow, history, live, and aftermath declarations

worth-query-installation
    installed principal-binding and authorization contracts
    touched-graph policy compilation
    query identity, basis, cursor, disclosure, and lane-eligibility contracts
    capability composition, conflict, break-glass, and review contracts
    inverse, compensation, redo, and external-effect posture contracts
    monetary operation and invariant contribution admission

worth-query-admission
    admitted authentication adapters and sealed external-principal proof
    authenticated-principal-bound query and operation admission
    purpose-bound capability and field-disclosure admission
    governed elevation and current-review admission
    touched-graph and policy decision admission
    compare-and-commit admission

worth-query-execution
    one installed primary Relational graph authority
    indexed external-principal-to-application-principal binding
    policy-evaluated attempts
    canonical application-query basis and continuation progression
    provider compare-and-commit progression
    current capability and disclosure revalidation
    idempotent typed outcomes and managed recovery
    linear inverse, compensation, and redo progression

worth-query-publication
    capability- and disclosure-scoped read, mutation, activity, explanation,
    history, live, recovery, and aftermath contracts

worth-query-host
    ordinary host composition without raw Relational or authority exposure

worth-query-certification
    hostile public-consumer, capability, disclosure, elevation, aftermath,
    concurrency, and replay proof
```

### Reference-world packages

The exact package count may follow implementation pressure, but semantic
ownership must remain visible:

```text
workspaces/worth-query-bank-world/
    crates/
        bank-domain/
            banking and estate schemas, typed operations, invariants,
            capability contributions, disclosure meaning, and aftermath posture
        bank-server/
            runtime composition, installed ordinary queries,
            and authoritative in-memory provider
        bank-http-adapter/
            Axum transport and Authentik OIDC adaptation
        bank-user-node/
            independent authenticated client/server process
        bank-courtroom/
            environment provisioning and cross-process certification
```

No `common`, `shared`, `helpers`, or generic “app core” package may obscure
ownership. If a responsibility remains local, it remains in the owning package.

### Cross-runtime authority flow

```text
Authentik
    -> authenticated external-principal proof
    -> installed graph principal binding
    -> typed purpose and requested capability
    -> installed capability, conflict, and disclosure decision
    -> canonical Query query or operation intent
    -> Relational touched-graph facts
    -> runtime-bridge installed lowering
    -> Signal policy decision
    -> Query admitted operation
    -> Relational proposed state
    -> installed invariants
    -> provider compare-and-commit
    -> Query publication
    -> managed recovery / inverse / compensation / redo admission
```

Every arrow changes authority. The receiving boundary validates the proof it
needs; it does not trust an adjacent receipt or reproduce the previous owner's
decision.

## Phase Plan

### Bank World Track — Phase 1: Freeze The Bank World And Build The Gap Ledger

**Requirement**

Define the legitimate product behavior, fixture topology, public consumer
transcript, and requirement/evidence ledger before changing Query.

**Must establish**

- exact bank entities, relationships, operations, invariants, roles, and
  outcome families;
- the supported OIDC flow and authenticated-principal boundary;
- dynamic Authentik, bank-server, and per-user-node topology;
- one ordinary DX transcript for each read, mutation, approval, activity, and
  live path;
- a front-door gap ledger classifying every required workaround as a Query gap,
  bank-domain concern, transport concern, fixture concern, or intentionally
  unsupported capability; and
- the cheapest commands for declaration, installation, execution, consumer,
  cross-process, and hostile certification loops.

**Proof before Runtime Hardening Phase 1**

The ledger covers every courtroom actor and behavior, every requirement has one
owner, and no test plan relies on a hard-coded principal or privileged fixture
mutation.

### Runtime Hardening Track — Phase 1: Schema-Bound Typed Application References

**Requirement**

Make the bank domain expressible without raw semantic strings or caller-cast
values.

**Must establish**

- typed entity, relation, aspect, field, operation, policy, and currency
  references derived from the installed schema;
- typed read/projection/filter and mutation/effect authoring;
- explicit checked dynamic-extension posture, if one exists;
- early operator, value-family, runtime, and schema-version rejection; and
- compile-time denial for representative cross-schema, wrong-value,
  wrong-operation, and illegal-write examples;
- dependency compilation distinguishes primary-logical-graph reads and touches
  from separate-authority provider calls: primary reads and runtime mutations
  retain their native evidence, while remote reads, touches, and commits still
  require exact provider receipts.

**Proof before Runtime Hardening Phase 2 and Bank World Phase 2**

The bank-domain package contains no application aspect strings, the public DX
transcript compiles using only declaration-facing types, and a typed local
mutation with an installed primary read completes dependency compilation
without inventing a provider receipt.

### Runtime Hardening Track — Phase 2: Authenticated Principal Admission

**Requirement**

Define the generic boundary through which a validated external identity becomes
a sealed, time-bounded principal proof eligible for installed graph binding.
Query must not learn Authentik, OIDC transport, JWT, cookie, or HTTP semantics.

**Must establish**

- typed issuer-and-subject identity;
- a collision-free canonical representation of the pair for indexed graph
  lookup, without exposing a raw application string key;
- a first-class principal-binding member in canonical application-schema
  identity, closure, installation, and runtime-generation validation;
- application schemas carried through the ordinary host domain-package grammar
  into the exact installed portable package index; a declaration that exists
  only in a side-built installation index is not host composition;
- one compatible canonical owner grammar across host domain packages and
  application schemas; namespace-qualified owners are allowed without allowing
  dots in schema or member identifiers;
- a sealed proof constructor available only to an admitted authentication
  adapter boundary;
- audience, validation-time, expiry, authentication-method, and adapter identity
  bound into the proof where later admission depends on them;
- one execution-owned primary Relational graph root shared by ordinary runtime
  paths, rather than a fixture registry, parallel map, or separate-provider
  graph-participation lane;
- a purpose-scoped host composition terminal that installs and publishes that
  primary graph without requiring bridge, read, mutation, live, signal, or
  inspection adapters whose authority is not exercised in this phase;
- publishing the purpose-scoped terminal consumes the raw execution root and
  installation authority; unrelated ordinary facade families and explicit
  backend/backend-part assembly are absent from its public type at compile
  time;
- the primary graph retains the Relational schema, invariant, and runtime
  configuration already installed by ordinary Query composition rather than
  replacing it with a freshly defaulted runtime;
- a consumed, installation-only typed bootstrap path for principal and mapping
  rows; no privileged identity mutation path survives runtime publication;
- installed, indexed mapping from authenticated external principal to
  application principal, with exact mapping, relation, and typed
  target-principal identity freshness evidence;
- the target-principal identity field is a first-class, read-only,
  equality-capable principal-binding dimension whose scalar family and Rust
  value type participate in declaration closure, canonical schema identity,
  installation, runtime resolution, and freshness;
- the sealed Query principal proof returns that typed application identity from
  the exact mapped primary-graph row; a parallel external-identity-to-product-ID
  registry is forbidden;
- target-principal identities are canonically unique within each installed
  principal binding, and bootstrap admission of external identity, principal
  key, and typed identity is atomic on denial;
- a Relational-owned bounded equality-index lookup contract whose ordinary
  path examines at most the declared candidate cap and whose cold
  certification path proves the same classification against authoritative
  storage;
- unknown, disabled, ambiguous, expired, and cross-runtime denial;
- request deadline and cancellation carriage; and
- no deserialization, fixture, token-claim, or host assertion path that can mint
  the proof.

**Proof before Bank World Phase 2**

A causally admitted test authentication adapter can produce a usable proof,
while forged data, copied fields, wrong runtime, stale proof, and direct
deserialization open no principal or operation authority. The indexed mapping
must prove storage parity and bounded candidate work. Mutable or wrong-typed
target-principal identity fields and copied binding identifiers paired with a
different identity type must deny. Changing the mapped principal's typed
identity must stale the proof. Two different Principal rows carrying the same
typed application identity must deny without poisoning a corrected bootstrap
retry. Primary identity facts
must not require a graph-participation provider receipt. A lawful
`worth-query-host` consumer must build the purpose-scoped runtime without
constructing unused adapter ceremony, while attempted ordinary read, mutation,
workflow, live, raw execution-root extraction, and replay actions fail to
compile against the purpose-scoped terminal.

This phase intentionally establishes the narrow primary-graph composition that
later authorization and ordinary-facade phases consume. It does not claim the
general read, mutation, workflow, or live front door owned by Runtime Hardening
Phase 5.

### Bank World Track — Phase 2: Authentik Adapter And Dynamic Identity Fixture

**Requirement**

Adapt real Authentik OIDC identity into the generic Runtime Hardening Phase 2 boundary
and provision all bank identities dynamically.

**Must establish**

- issuer/audience/signature/time validation and key rotation behavior;
- HTTPS is mandatory for the issuer, introspection, and revocation endpoints,
  and secret-bearing control endpoints must share the issuer origin; the cold
  courtroom may trust its ephemeral self-signed certificate only through
  certification-gated clients;
- cryptographic ID-token/access-token pair binding plus exact introspection
  client affinity; two independently valid users' credentials must not compose
  when their token halves are crossed;
- `(issuer, subject)` identity, with display claims retained only as attributes;
- unknown, disabled, ambiguous, expired, revoked, and wrong-audience denial;
- access-token introspection or equally authoritative online revocation
  evidence; decoding a locally valid ID token is not revocation proof;
- a real Authorization Code with PKCE browser flow carrying state and nonce;
- one bounded JWKS refresh-and-retry on an otherwise admissible token whose
  signing key is absent from the installed cache;
- explicit request deadline and cancellation propagation;
- no Authentik-specific type or token claim beyond the adapter boundary;
- no transport or test-only constructor for authenticated proof; and
- dynamic fixture provisioning, signing-key rotation, token revocation, and
  teardown through real Authentik administration and token acquisition
  boundaries;
- teardown proof covers containers, volumes, networks, and the secret-bearing
  fixture directory after both success and an intentionally failed body;
- a cold Docker courtroom lane that runs real Authentik, PostgreSQL, and browser
  automation without entering ordinary bank edit/test loops.

**Proof before Runtime Hardening Phase 3**

Tokens from the real issuer admit the correct principal, forged or malformed
tokens fail, and application code cannot create or deserialize the proof.

### Runtime Hardening Track — Phase 3: Scoped Authorization And Touched-Graph Admission

**Requirement**

Compile installed bank abilities into the existing Relational, runtime-bridge,
Signal, and Query authority chain, then bind each admitted operation to its
exact allowed touched graph.

**Must establish**

- personal, business, and institution-scoped ability declarations;
- schema-typed application-operation requirements that compile into the
  existing installed domain-operation graph-read, touch, effect, invariant,
  and conditional contracts rather than creating a parallel application lane;
- a move-only, schema-typed primary-graph installation phase for the entity,
  relation, and field facts required by policy evaluation, with no retained
  product mutation bypass after publication;
- current relationship-backed role evaluation;
- distinct-actor approval rules;
- positive, negative, membership, and revocation dependencies;
- Relational-owned touched-graph evidence for the policy facts actually read;
- Query-owned binding of the admitted operation to its installed allowed
  touched-graph contract, without pretending that not-yet-proposed record
  identities are realized effects;
- runtime-bridge lowering through installed correspondence;
- Signal-owned decision evidence;
- Query-owned composition into an operation admission proof; and
- retention of the exact authentication lifetime and live request
  cancellation/deadline authority in that operation proof, so later governed
  phases can reject authority that expired or was interrupted after admission;
- a Query-minted stable operation-scope fingerprint over the runtime,
  installed operation, authenticated principal, and typed scope for downstream
  idempotency binding; it excludes the observation snapshot so an equivalent
  authorized retry remains the same intent; and
- one lane-independent admitted-scope proof family that later read, mutation,
  explanation, history, and live facades must consume; Phase 3 does not invent
  those Phase 5 facades merely to claim integration.

**Proof before Bank World Phase 3**

Forged roles, token claims, caller-declared touched sets, route middleware
booleans, cross-account access, role combination, and post-revocation delivery
all fail. Valid customer and employee combinations remain usable.

### Bank World Track — Phase 3: Double-Entry Banking Operations And Invariants

**Requirement**

Implement the bank domain as ordinary installed operations that emit typed
effects and execute real monetary invariants over proposed state. This phase
produces immutable invariant-approved proposals; it does not create a
bank-local commit path ahead of Runtime Hardening Phase 4.

**Must establish**

- personal and business account creation;
- explicit opening funding;
- direct transfer by stable recipient identity;
- deposit and withdrawal through bank accounts;
- business initiation and separate approval;
- immutable journal entries and postings;
- exact balance, available-funds, currency, account-status, and balancing
  invariants; and
- private causal binding of each invariant witness to the exact in-memory
  snapshot basis used to mint it; an independently built snapshot with the
  same descriptive version is not the same basis;
- typed idempotency intent bound to the authenticated operation.
- no authoritative mutation or `Committed`/`AlreadyCommitted` outcome before
  provider compare-and-commit.

**Proof before Runtime Hardening Phase 4**

Independent accounting oracles recompute proposed balances and journal
conservation.
Overdraft, unbalanced entries, currency mismatch, duplicate approvals,
self-approval where prohibited, and idempotency-key payload drift all deny.
Equivalent payloads produce the same typed idempotency intent; Runtime
Hardening Phase 4 owns retry resolution and `AlreadyCommitted`.

### Runtime Hardening Track — Phase 4: Provider Compare-And-Commit

**Requirement**

Advance an invariant-approved Milestone 9.15 proposal through one
provider-proven compare-and-commit progression.

**Subphases**

- **4.1 — Application-to-provider progression.** Bind the Runtime Phase 3
  admitted application operation to the existing Milestone 9.15
  basis-complete read-set, proposed-state, and installed-invariant
  progression. The resulting commit candidate is a concrete Query authority
  that consumes the exact application admission, proposal, provider session,
  and invariant progression; none of those ingredients can authorize commit
  independently.
- **4.2 — Atomic compare-and-commit.** Recompare only the exact decision facts
  retained by that candidate while the installed provider owns the commit
  serialization boundary, then apply the entire effect program or none of it.
  Relevant drift stales the candidate; unrelated drift does not.
- **4.3 — Idempotent terminal resolution.** Bind the typed operation intent to
  an authoritative idempotency record in the same provider transaction and
  resolve equivalent retries, payload drift, response loss, and ambiguous
  provider outcomes without guessing.
- **4.4 — Derived accounting truth and conflict dependency.** Remove stored
  available balance as an authoritative decision field. Reconstruct monetary
  balance only from the installed journal, posting, and posting-account graph.
  Retain a typed per-account accounting revision as coordination metadata,
  derived from that graph and advanced atomically for every account touched by
  a journal append. The revision may stale a competing attempt, but it never
  carries, replaces, or independently mutates monetary truth. Journal effect
  lowering reads the governed current revision from the exact projected read
  attempt and advances it by the emitted posting count; it never derives a
  successor revision from a deliberately partial domain proposal snapshot.
- **4.5 — Bounded application projection.** Give installed application
  composition a provider-derived, version-bound projection or access strategy
  whose ordinary preparation cost follows the admitted touched graph and the
  smallest honest adjacency granule rather than total provider state.
  Projection state is reconstructible acceleration, never a second truth
  source or commit authority. Its version and invalidation basis must be exact,
  and its structural work must be observable. For signed incoming aggregates,
  Query returns a typed summary containing both the checked scalar and exact
  contributing-source count. The exclusive form proves that each contributing
  source has exactly one relation of the declared kind, so malformed
  multi-target postings cannot be counted as lawful money. Warm maintenance
  examines only authoritative changed sources; a cache miss reconstructs from
  retained journal/posting truth, reports its input breadth, and denies on
  malformed scalar, cardinality, budget, count, or arithmetic state.
- **4.6 — Collision-free created identity.** Created domain objects derive
  stable typed identities from the operation's exact idempotency-key identity
  and an explicit within-operation role. Creation does not scan a global
  maximum, race through a caller-owned counter, or serialize unrelated
  operations merely to mint an identifier. Equivalent retries derive the same
  identities; distinct admitted idempotency identities cannot alias.
- **4.7 — Operation-scoped projection authority.** Bind provider-derived
  invariant projection to one installed operation's declared read capability.
  The locked reader exposes a field or relation only when its typed member
  implements that operation's `OperationReads` capability, and the resulting
  version-bound projection snapshot carries the same operation type into
  application read-set construction. Keep that compile-time projection
  ceiling distinct from the installed decision-read manifest: the former says
  which typed facts may inform bounded invariant construction, while the
  latter names the smaller exact dependency set that every realized attempt
  must observe and recompare before commit. Projection implementation details
  do not become mandatory conflict facts, and omitting a genuine decision
  dependency from the installed manifest remains invalid. A broad schema
  projection, a snapshot for another operation, or composition-root access to
  undeclared facts cannot mint mutation authority.
- **4.8 — Realized touched-graph identity closure.** Bind every
  provider-derived operation projection to the exact admitted operation and
  retain the entity identities actually resolved or reached through its
  bounded typed traversals. The application read attempt consumes that
  move-only closure together with the pinned projection lease: an unprojected
  attempt may observe only the admitted root scope, while a projected attempt
  may additionally observe only identities carried by that exact projection.
  An operation-typed but admission-unbound inspection, a projection bound to
  another admitted scope, or a newly resolved same-type entity outside the
  retained closure cannot become a decision fact or effect target. The
  installed decision-fact budget rejects an additional distinct fact before
  its authoritative value or relation is read and is rechecked when the
  complete read-set proof is minted.
- **4.9 — Current projection and lifecycle authority.** An admitted operation
  projection validates exact runtime and binding affinity plus current
  authentication, cancellation, and deadline authority before its closure can
  read provider truth. Admission-unbound projection is a separate inspection
  result type that carries output and work but no consumable snapshot or
  mutation progression. Framework-owned snapshot pins are released on success,
  denial, early drop, and unwind; a panicking domain projector cannot leak
  retention authority.
- **4.10 — Compiler-visible mutation progression.** Ordinary root-scoped
  application reads and admission-bound projected mutation reads are different
  sealed phase types. Completing an ordinary read set can authorize a read
  result, but cannot expose effect-program construction. Only a complete read
  set that consumed the exact admitted projection may advance to installed
  effect authoring, so raw admission plus a narrow read set is not an implicit
  higher-level mutation authority.
- **4.11 — Exact dynamic decision dependency plan.** The admission-bound
  invariant projection records the exact entity, field, and relation fact
  identities that the domain declares as commit dependencies while it resolves
  the admitted touched graph. The installed manifest remains the compile-time
  ceiling of permitted dependency families; it is not treated as proof that
  one arbitrary instance of each family is complete. Application read-set
  completion requires equality with the projection-carried dynamic dependency
  plan, including multiplicity across distinct entity identities, before
  mutation authority can be minted.
- **4.12 — Pinned, budgeted projection execution.** Every adjacency and
  endpoint lookup explicitly names the retained projection version rather than
  consulting current provider head. Structural counters count the records
  actually read, without self-certifying undercounts, and the installed
  operation contract supplies a distinct hard projection-work limit enforced
  during traversal. Ordinary mutation preparation explicitly disables
  replay/reconstruction; reaching the projection limit denies without widening
  into a whole-world or reconstructive lane.
- **4.13 — Typed admitted-root projection entry.** An admission-bound
  projector receives the exact typed scope entity that authorization admitted,
  already bound to the projector's private occurrence authority and realized
  identity closure. Domain projection begins from that value when the admitted
  scope is its root; it does not re-resolve the root from caller input, pay a
  redundant equality lookup, or risk planning a different same-type entity.
  Inspection-only projection has no admitted root, and no public identifier or
  equivalent entity value can manufacture one.
- **4.14 — Authorization dependency commit closure.** The exact Relational
  authorization observations and Runtime Bridge decision evidence that minted
  an application admission remain proof-carrying commit dependencies. Query
  joins those sealed observations to a separately budgeted installed provider
  decision-fact family and the provider recomparisons their exact relevant
  entity, typed-adjacency, relation, and predicate-field basis before effects
  can commit. A permission-revoking mutation, a newly matching deny path, or a
  removed allow path therefore stales the admitted attempt; unrelated
  relation-kind growth does not create a false conflict. Query neither trusts
  detached dependency identifiers nor replaces the installed
  Relational–Bridge–Signal composition with a host callback or an ad hoc
  policy re-evaluation.

**Must establish**

- complete decision read-set validation;
- mutation preparation from the exact retained branch snapshot, with the
  pinned Relational snapshot and Runtime Bridge authority basis joined before
  provider admission so unrelated head movement does not create a false
  conflict;
- request-local Runtime Bridge execution lanes over the same installed truth
  authority, so thread-affine Signal state cannot reject concurrent server
  requests before provider admission;
- exact proposed-effect evidence from the Bank Phase 3 proposal and proof that
  every realized read and effect is a subset of the operation admission's
  installed allowed touched graph;
- projection capability reads and commit-decision reads remain distinct.
  Ordinary field and relation traversal never capture a commit dependency
  implicitly. The installed decision manifest is the allowed target-family
  ceiling; explicit decision reads seal the exact realized dependency keys,
  and completion requires equality with that dynamic key set rather than one
  arbitrary observation from every allowed family;
- one exact realized identity closure joining the admitted root scope to the
  identities reached by its admission-bound operation projection; schema- and
  operation-compatible identities outside that closure open no read or effect
  authority;
- relevant versus unrelated drift classification;
- atomic effect application by the in-memory provider;
- one terminal attempt outcome;
- response-loss/idempotent resolution;
- explicit abort, partial-effect, indeterminate, stale, cancelled, denied, and
  committed posture; and
- no public construction path from plan, receipt, read-set, invariant result,
  or provider token fragments.
- one installed primary-graph provider authority owns bank decision facts,
  proposed-state comparison, and committed effects. `BankSnapshot` may remain
  an isolated proposed-state projection and independent accounting oracle, but
  it is not a caller-supplied commit basis, a parallel mutable runtime, or an
  authority that can be updated beside the provider graph.
- derived facts such as available balance are computed from the authoritative
  journal and posting state under the installed provider contract. They are
  not independently mutable fields that can diverge from the accounting
  history that proves them.
- every invariant-approved Bank proposal and its Query decision read-set share
  one consumed pinned projection lease; a same-version snapshot reacquired
  after proposal approval is not an equivalent causal basis.
- invariant projection reads are compiler-constrained to the operation's
  declared projection-read capability; every genuine commit dependency is
  separately admitted by the installed decision-read target manifest, and
  the retained projection lease is typed for that same operation before it
  can enter application read-set construction.
- an operation-typed projection that is not bound to the exact admitted
  operation remains inspection-only. The application read-set consumes both
  admission affinity and the bounded set of entity identities realized by the
  retained projection, and enforces the installed distinct-fact budget before
  another authoritative fact is read.
- stale, cancelled, expired, foreign-runtime, or foreign-binding admission
  cannot execute an invariant projection closure. Inspection output cannot be
  converted into a retained projection lease, and every framework-owned pin is
  released if projection unwinds.
- a complete ordinary read set has no effect-program transition. Mutation
  authoring is available only after the exact admission-bound projection,
  realized identity closure, and dynamic decision dependency plan have all
  been consumed into the projected read-set phase.
- the complete realized decision-fact keys equal the projection-carried
  dependency keys, not merely the set of installed target families. If an
  operation depends on the same field family for two accounts, both exact
  account-field facts are mandatory and neither can stand in for the other.
- projection adjacency and endpoint access is explicitly pinned to the retained
  version, hard-limited by an installed projection-work budget, and counted
  according to actual provider reads. Ordinary compare-and-commit planning
  selects a replay-disabled Runtime Bridge lane.
- the admitted projection closure receives its exact typed root scope directly
  from the operation proof. Root-dependent plans neither re-resolve that entity
  through a caller value nor substitute another same-type identity, and warm
  projection work excludes the eliminated lookup.
- authorization observations retained by the admitted operation enter the
  provider progression as their own installed, cardinality-bounded
  decision-fact family. Their exact relevant graph basis is recomparison
  evidence: revocation or a newly effective deny dependency prevents commit,
  while unrelated relation kinds are neither scanned by the warm path nor
  treated as causal drift.
- account accounting revisions are exact conflict dependencies for journal
  mutation, advance in the same provider transaction as their postings, and
  are reconstructible from authoritative journal topology. Bank projection
  requires equality between the revision and Query's exact aggregate source
  count before using the aggregate value.
- ordinary proposal preparation does not repeatedly scan or clone the entire
  installed application world. Provider-derived access paths carry an exact
  version basis, declare their synchronous maintenance and invalidation cost,
  and expose structural counters that distinguish touched work from unrelated
  world size.
- journal, posting, payment, authorization, and account identities created by
  operations have an exact typed derivation basis. Two unrelated proposals
  prepared from the same provider version neither choose the same created
  identity nor acquire a false shared allocation conflict.
- application effect authoring cannot call a raw Relational commit surface.
  Relational mechanics may implement the installed provider, but only the
  sealed invariant-approved provider candidate can reach its transaction
  boundary.

**Proof before Runtime Hardening Phase 5**

Concurrent transfers cannot overspend, unrelated writes avoid false conflicts,
the same idempotent request cannot post twice, and failure injection never
claims atomic commit without provider proof. Destroying or perturbing a derived
balance projection cannot change accounting truth, and rebuilding it from the
committed journal restores the same value and source count. Repeated public
mutations beyond the former history-proportional decision-fact ceiling retain
constant warm aggregate work, while malformed multi-account postings and
revision/count drift deny.

### Runtime Hardening Track — Phase 5: Ordinary Read, Mutation, Workflow, And Live Facades

**Requirement**

Expose the bank's ordinary work through one declarative Query front door whose
valid next actions follow typed phase progression.

Runtime Phase 5 is implemented in the following authority-ordered internal
slices. These are one runtime phase, not independent roadmap tracks.

**5.1 — Typed bounded read front door**

- declare account discovery, account summary/detail, authorized-user, activity,
  pending-payment, and institution-audit reads as typed installed application
  work;
- every read consumes the same Runtime Phase 3 admitted-scope proof family as
  mutation and begins from the exact typed admitted root;
- account discovery starts from the authenticated principal's graph identity
  and traverses only declared visibility relationships. It never enumerates the
  installed account world and redacts afterward;
- current-consistency, deadline, cancellation, maximum-result, and work-budget
  controls belong to the caller; and
- results expose typed data plus provider-read, touched-scope, truncation, and
  degradation metadata.

**5.2 — Complete typed mutation front door**

- transfer, deposit, withdrawal, opening funding, account creation,
  authorization grant/revoke, business initiation/approval/rejection, and
  reversal all consume Runtime Phase 4's sealed projected-mutation progression;
- private per-operation lowering retains compiler-visible installed
  create/delete/write/link/unlink/emit capabilities rather than dynamically
  interpreting a bag of bank effects;
- every invariant-approved proposal effect shape is covered by that exact
  operation's installed program. Proposal/program drift is an installation or
  compile-time failure, never a runtime fallback to untyped effect dispatch;
- typed application emits enter the same atomic provider commit causality and
  recoverable receipt as graph changes. They are never a best-effort
  post-commit side channel; Runtime 5.4 may consume this causality but does not
  retroactively create it;
- mutation decisions seal the exact causal field, edge, and bounded adjacency
  predicates carried by the admitted projection. In particular, a decision
  based on an empty outgoing or incoming relation set must become stale if a
  matching edge appears before commit; exact endpoint-pair observations are
  not accepted as proof of whole-adjacency absence;
- projected dependency completion re-observes only those sealed predicate
  keys. Callers cannot manufacture, widen, omit, or replace an adjacency
  predicate, and provider compare-and-commit validates its current membership
  against the pinned observation before applying effects;
- the installed mutation decision-read manifest is an allowed target-family
  ceiling, while the projection-carried keys are the exact realized
  dependencies. Conditional branches are not forced to invent one arbitrary
  fact from every allowed family; all actual admitted field and adjacency reads
  seal themselves, and completion requires equality with that dynamic set;
- the caller supplies typed input, request controls, and idempotency intent but
  cannot assemble admissions, read sets, provider candidates, receipts, or
  support snapshots; and
- every mutation returns the bank-domain typed outcome family with bounded work
  metadata.
- the published primary-graph provider support envelope admits every installed
  bank mutation contract within its declared decision-fact and effect-program
  ceilings, or publication fails. Per-attempt semantic width remains distinct
  from the fixed concurrent-attempt capacity; a contract must not install
  successfully only to be unconditionally rejected by provider admission.

**5.3 — Workflow continuations and typed explanations**

- business initiation may return a typed pending-payment continuation whose
  only legal decisions are approval or rejection under a newly authenticated
  principal and request scope;
- the continuation is a private-field descriptive payment handle, not retained
  authentication, admission, projection, snapshot, or provider authority. A
  successful or authoritatively recovered initiation and a current
  approval-required payment read may produce it so a separate user process can
  resume the workflow without transferring authority;
- approval and rejection markers contain no caller-selected actor identity.
  Execution derives the deciding principal from the newly supplied
  authenticated proof, consumes fresh request controls, and re-enters the
  installed authorization and invariant progression. A copied or stale payment
  identity therefore opens no workflow authority;
- failed, stale, cancelled, aborted, partial-effect, or indeterminate
  initiation cannot mint a continuation, while an `AlreadyCommitted`
  initiation recovers the same descriptive continuation;
- denial, invariant violation, stale attempt, abort, cancellation,
  partial-effect, and indeterminate outcomes retain distinct typed meaning; and
- explanation data is derived from the governed typed outcome and retained
  decision evidence through one total typed mapping. Invariant violation is not
  hidden inside a generic denial, and no public caller or adapter parses
  diagnostic strings to recover semantics.

**5.4 — Authorized history and live delivery**

- account activity history is a bounded ordinary projection of committed
  journal/posting topology, ordered by an authoritative per-account posting
  sequence committed atomically with the account journal revision. It is not
  replay, does not sort by idempotency-derived identity, and imports no
  certification-only reconstruction surface;
- history continuation is a descriptive, account- and provider-version-bound
  cursor rather than retained snapshot authority. A changed provider version
  produces an explicit stale-cursor outcome; continuation never silently skips
  or duplicates entries across a moving graph;
- account and activity live leases retain the exact admitted scope and installed
  Runtime Bridge/Signal authorization evidence;
- provider commit causality enters one bounded delivery source with explicit
  overflow, cancellation, deadline, and closure outcomes; and
- every declared effect payload carries a typed retained-byte contract. Effect
  authoring counts inline representation and recursively owned allocation
  capacity, rejects the cumulative committed payload before provider mutation
  when it exceeds the installed `RetainedBytes` envelope, and seals the
  validated batch as the only input accepted by provider commit causality. The
  bounded delivery source tracks those admitted bytes through publication and
  eviction; a batch-count ceiling alone is not a memory bound;
- every matching typed emission in one committed batch is delivered exactly
  once and in batch order. A lease may pause within a batch at its admitted
  buffer ceiling, but it cannot advance the source cursor past undelivered
  matching causes;
- the bank activity facade binds each delivered typed cause to its exact
  journal identity, account identity, and account posting sequence and performs
  one fresh, authorized, one-item projection for that conjunction. A
  counterparty posting may have the same per-account sequence and is skipped,
  not mistaken for absence. The lease retains the cause until projection succeeds;
  an oldest-first bounded activity snapshot that omits the cause cannot be
  relabeled as that commit's live update;
- a caller may narrow the installed live buffer ceiling but cannot enlarge it
  or manufacture queue capacity outside the installed execution contract; and
- current permission dependencies are re-evaluated before every delivered
  projection, so revocation closes or narrows the lease before the next
  unauthorized event. Queued payloads carry no authority past that check.

**5.5 — Public consumer closure**

- consumer transcripts compile and run through bank-domain types and the
  `worth-query-decl` / `worth-query-host` audience path only;
- no consumer assembles runtime identities, policies, read sets, provider
  sessions, receipts, or support snapshots; and
- the transcript covers every read, mutation, workflow, explanation, history,
  and live family plus cancellation, deadline, consistency, idempotency,
  bounded work, queue overflow, and live revocation.

**Destination module skeleton**

```text
workspaces/worth-query/crates/worth-query-execution/src/domain_computation/primary_graph/
    ordinary_read/
        controls.rs
        outcome.rs
        projection.rs
    ordinary_mutation/
        controls.rs
        outcome.rs
        progression.rs
    live_delivery/
        controls.rs
        lease.rs
        outcome.rs
        provider_causality.rs

workspaces/worth-query-bank-world/crates/bank-server/src/ordinary/
    read/
        account_discovery.rs
        account_detail.rs
        account_access.rs
        activity.rs
        payment.rs
        institution_audit.rs
    mutation/
        account_access.rs
        account_creation.rs
        business_payment.rs
        money_movement.rs
        reversal.rs
    workflow/
        continuation.rs
        explanation.rs
    live/
        account.rs
        activity.rs
```

Files may be combined only where they retain one semantic responsibility and
remain within the workspace line cap. The stable axes above are not flattened
into `helpers`, `common`, route-local policy, or a second mutable bank runtime.

**Proof before Bank World Phase 4**

Consumer transcript tests compile and run using only `worth-query-decl` and
`worth-query-host`. No consumer assembles runtime identities, policies,
read-sets, provider sessions, receipts, or support snapshots.

### Bank World Track — Phase 4: Estate, Capability, And Emergency-Access World

**Requirement**

Extend the bank into a production-shaped estate and exceptional-access world
that forces capability composition, conflict-of-interest, field disclosure,
delegation, elevation, review, and aftermath to become real domain semantics.
This phase declares the bank meaning and hostile worlds; it does not create a
bank-local authorization or history engine.

The Phase 4 commands remain typed bank-domain semantics rather than installed
executable application operations. Runtime Hardening Phase 7 must install
their capability, purpose, disclosure, conflict, and elevation contracts
before any host can obtain execution authority for them.

**Must establish**

- typed death notice, estate case, executor, beneficiary, joint owner,
  authorized signer, branch, estate specialist, compliance, legal-authority,
  capability-grant, emergency-access, and mandatory-review entities or
  relationships at the narrowest truthful domain boundary;
- capability grants for exact account, estate, institution, branch, operation,
  purpose, field, amount, validity, delegation, and workflow-stage scopes;
- deny, conflict-of-interest, separation-of-duty, distinct-actor, and
  beneficiary-exclusion policy contributions;
- restricted customer identity, beneficiary, document, account, posting, and
  audit fields with domain-owned disclosure classifications and typed
  application result shapes;
- death notification, account freeze, case opening, executor recognition,
  capability delegation/revocation, emergency request/approval/revocation,
  mandatory review, estate release, and estate disbursement operations;
- the branch-manager/beneficiary world in which ordinary employee authority,
  beneficiary status, delegated authority, and emergency elevation remain
  separately attributable and cannot compose into self-benefiting power;
- bank aftermath declarations identifying compensatable money movement,
  explicitly reversible authorization changes, reconcilable external effects,
  and irreversible audit or legal decisions; and
- independent estate, capability, disclosure, conflict, and accounting oracles
  that do not call the production policy or projection implementation.

**Proof before Runtime Hardening Phase 7**

The bank domain can state every legal and illegal case without a superuser role,
route predicate, redaction callback, mutable permission map, generic metadata
bag, or local undo stack. The conflicted branch manager cannot read restricted
estate fields, approve their own elevation, disburse estate funds, or combine
grants to do so; a causally valid non-conflicted path remains expressible.

### Runtime Hardening Track — Phase 6: Installed Application Queries, Bases, And Continuations

**Requirement**

Join schema-bound application meaning to Query's existing canonical read,
collection, graph-read access-planning, cursor, history, live, and preview
machinery so an application declares one query rather than hand-wiring
parallel lane-specific projectors or access paths.

**Must establish**

- installed application-query identity with typed parameters, admitted root,
  result shape, cardinality, predicates, ordering, dependency ceiling,
  disclosure contract, basis support, and lane eligibility;
- one installed binding from that identity to the existing canonical
  `WorthQueryReadFamily` / `WorthQueryReadGraph` meaning and Milestone 9.10
  graph-read access-planning proof chain;
- parameter-bound predicate normalization and selectivity, plus
  `PredicateSupport`, `OrderingSupport`, traversal, result-buffer, and
  lifecycle requirement rows derived before execution rather than
  rediscovered by an application-query executor;
- domain-owned projection or derived-field semantics behind that installed
  contract without host-local execution dispatch becoming query authority;
- Query-owned opaque continuation bound to query identity, parameters, scope,
  basis, ordering, result shape, and compatibility generation;
- fresh installed scoped-authorization admission on every page, history
  request, live delivery, and preview read; a cursor or prior result carries
  no authority;
- a typed access-context input that Runtime Hardening Phase 7 can strengthen
  with capability, purpose, disclosure, conflict, and elevation proof without
  changing canonical query identity;
- explicit caller controls for current or pinned basis, consistency, freshness
  posture where supported, deadline, cancellation, result count, and work;
- one canonical result meaning across one-shot, historical, live, and admitted
  preview lanes, with typed support or unsupported posture per lane;
- consumption of one admitted graph-read access plan on every covered one-shot,
  continuation, historical, and preview execution, with separately admitted
  live-maintenance posture for the same canonical read meaning;
- expected-version and expected-fact mutation preconditions lowered into exact
  provider-recompared decision facts;
- governed read-access receipts carrying the Milestone 9.10 plan,
  requirement-set, support, strategy, fallback, edge-scan, and
  per-result-neighbor-lookup evidence alongside typed field omissions; and
- deletion or migration of application-local cursor identity, lane-specific
  query meaning, host pagination, host-side post-read filtering or sorting,
  repeated child queries, per-result relation lookup, and duplicated
  one-shot/live/history projectors where Query owns the generic contract.

**Destination module skeleton**

```text
worth-query-declaration/src/application_query/
    definition.rs
    parameters.rs
    result_shape.rs
    basis_support.rs
    lane_eligibility.rs

worth-query-installation/src/application_query/
    canonical_identity.rs
    installed_contract.rs
    read_family_binding.rs
    graph_access_contract.rs
    disclosure_contract.rs
    continuation_contract.rs

worth-query-execution/src/domain_computation/primary_graph/application_query/
    admission.rs
    graph_read_plan_binding.rs
    one_shot.rs
    continuation.rs
    historical.rs
    live.rs
    preview.rs
    access_receipt.rs

worth-query-bank-world/crates/bank-domain/src/queries/
    account.rs
    payment.rs
    estate.rs
    audit.rs
```

The lane files are committed sibling responsibilities with different lifecycle
and truth-basis posture. They bind lane progression to the existing canonical
read graph and admitted graph-read access plan; they are not separate query
engines. Implementation creates only populated responsibilities but may not
flatten them into one mode-switched executor.

**Proof before Runtime Hardening Phase 7**

The same installed account or estate query has identical result shape,
ordering, declared access-context boundary, and scope across every supported
lane. Foreign, cross-query, stale-basis, wrong-order, or wrong-generation
cursors deny before projection. Unsupported search, Store, preview, or
access-product posture cannot silently widen into a scan, host loop, or local
cache. Predicate and ordering changes produce the correct Milestone 9.10
selectivity, requirement, inventory, and access-plan evidence. Nested account,
payment, estate, and actor views prove exact-zero covered per-result neighbor
lookups and zero fallback across ordinary and lane-specific receipts. Phase 6
does not fabricate the richer capability and disclosure proofs owned by Phase
7.

### Runtime Hardening Track — Phase 7: Capability, Disclosure, And Governed Elevation

**Requirement**

Make capability-based authorization powerful enough for medical-grade
disclosure and emergency access while preserving exact graph authority,
conflict-of-interest, currentness, and fail-closed phase progression.

**Must establish**

- typed semantic capability identity distinct from role, relationship,
  authentication, policy result, and operation authority;
- scope dimensions for action, resource, relation, field, purpose, amount,
  cardinality, workflow stage, validity, delegation, grant provenance, and
  application-defined constrained context;
- compiler-visible installed composition of allow, deny, conflict,
  separation-of-duty, distinct-actor, delegation, and disclosure predicates;
- delegation that can narrow but never widen the grantor's exact authority,
  validity, disclosure, purpose, or downstream delegation posture;
- purpose-bound request context introduced at an explicit entry boundary and
  thereafter Query-carried without ambient or adapter-owned policy;
- separate typed internal-computation and consumer-disclosure admission,
  including noninterference posture for protected facts that influence
  predicates, ordering, cursors, counts, aggregates, explanations, history,
  or live membership without appearing in the result;
- typed field disclosure and omission before consumer-visible projection or
  serialization, with no post-projection redaction path;
- break-glass typestate from requested, approved, active, expired or revoked,
  and review-required to reviewed, with exact requester, approver, reason,
  scope, purpose, fields, actions, time, and audit identity;
- an installed upper bound on what emergency elevation can authorize; elevation
  cannot bypass conflict, distinct-actor, invariant, irreversible-commit, or
  provider requirements;
- exact authorization and disclosure decision facts carried through commit,
  history, continuation, and every live payload; and
- re-admission of the strengthened access context on one-shot, continuation,
  historical, preview, and live query lanes without changing canonical query
  identity or result meaning;
- typed explanations that distinguish missing capability, scope mismatch,
  purpose mismatch, conflict, separation-of-duty, field omission, elevation
  required, elevation denied, elevation expired, and review required.
- installation of the Bank World Phase 4 estate and emergency-access commands
  only after their complete capability, disclosure, conflict, delegation, and
  elevation contracts are compiler-visible and fail closed.

**Destination module skeleton**

```text
worth-query-declaration/src/application_capability/
    capability.rs
    scope.rs
    purpose.rs
    disclosure.rs
    delegation.rs
    elevation.rs

worth-query-installation/src/application_capability/
    composition.rs
    conflict.rs
    disclosure.rs
    delegation.rs
    elevation.rs

worth-query-execution/src/domain_computation/authorization/
    capability_admission.rs
    purpose_context.rs
    disclosure_admission.rs
    delegation_admission.rs
    elevation_progression.rs
    currentness.rs
```

**Proof before Runtime Hardening Phase 8**

The complete estate courtroom proves lawful ordinary, delegated, and emergency
paths while every grant-combination, conflict, field-widening, purpose-swap,
self-approval, expiry, revocation, and copied-elevation attack fails at the
earliest governed boundary. Every installed query lane applies the same
capability, purpose, disclosure, and conflict meaning. Growing unrelated
grants, relationships, fields, or cases does not widen warm authorization
work.

### Runtime Hardening Track — Phase 8: Recovery, Linear Undo, And Redo

**Requirement**

Expose actionable recovery and receipt-linked linear aftermath without
rewriting history, importing certification replay, or pretending that a local
commit proves an external effect.

**Must establish**

- installed reversible, compensatable, reconcilable, and irreversible
  aftermath contracts with operation-specific next-action types;
- a framework-owned recovery handle for indeterminate outcomes, bound to exact
  runtime, operation, attempt, principal scope, idempotency identity, provider
  posture, and expiry or disposal lifecycle;
- typed inspect, resolve, safe-retry, compensate, reconcile, and dispose
  transitions, exposing only those admitted by the outcome and installed
  contract;
- undo as a fresh admitted inverse or compensation operation that consumes the
  exact committed receipt and re-enters current capability, policy,
  touched-graph, invariant, idempotency, and compare-and-commit progression;
- compensating journal entries for bank money movement, explicit inverse
  operations for eligible capability changes, and honest denial for
  irreversible legal, audit, approval, or escaped-effect cases;
- redo as a descriptive intent available only after a proved undo, requiring
  fresh authority and current-truth validation and never importing replay;
- one linear parent-causality chain with explicit redo invalidation after a
  divergent current-head operation; and
- provider commit, emitted application causality, dispatch, external
  acknowledgement, external completion, compensation, and reconciliation as
  distinct typed postures.

**Destination module skeleton**

```text
worth-query-installation/src/application_aftermath/
    posture.rs
    inverse_contract.rs
    compensation_contract.rs
    recovery_contract.rs

worth-query-execution/src/domain_computation/application_aftermath/
    recovery_handle.rs
    undo_admission.rs
    undo_progression.rs
    redo_intent.rs
    redo_admission.rs
    linear_lineage.rs
    external_effect.rs

worth-query-publication/src/application_aftermath/
    outcome.rs
    explanation.rs
    access_and_disclosure.rs
```

Branch-aware aftermath is intentionally absent from this topology. Its
semantic-history, reference, inversion, merge, publication, recovery, and
product-surface responsibilities belong to the cross-runtime
merging-and-branching roadmap and do not enter through a dormant Query
directory.

**Proof before Bank World Phase 5**

A committed transfer produces one compensating reversal and preserves both
journals; an equivalent retry does not compensate twice. Redo after the proved
undo is freshly authorized and can stale or deny after relevant drift. A copied
receipt, foreign principal, expired capability, conflicted beneficiary,
irreversible operation, lost response, or unresolved external effect cannot
manufacture undo, redo, rollback, or completion authority.

### Bank World Track — Phase 5: Temporary HTTP Boundary And Per-User Async Nodes

**Requirement**

Run the ordinary public Query surface across real asynchronous process and
network boundaries.

**Must establish**

- one authoritative bank-server process;
- one independently authenticated user-node process per fixture participant;
- an Axum adapter that maps HTTP and SSE onto the public Query facade;
- typed wire representations for query identity, basis, opaque continuation,
  capability purpose, disclosure omissions, elevation progression, recovery,
  undo, and redo without serializing runtime authority;
- bounded request and stream queues, cancellation, deadlines, backpressure, and
  disconnect handling;
- dynamic ports, health/readiness, deterministic teardown, and leak detection;
- typed wire representations for semantic outcomes and legal next actions; and
- no route-local banking policy or direct provider access.

**Proof before Runtime Hardening Phase 9**

The full courtroom runs over TCP with separate process IDs and runtimes.
Disconnects, restarts of non-authoritative user nodes, response loss, queue
saturation, token expiry, and live revocation preserve semantic outcomes.

### Runtime Hardening Track — Phase 9: Public Policy Cutover And Workaround Deletion

**Requirement**

Make the proven front door canonical and delete the local reconstruction paths
that the bank world or existing consumers no longer need.

**Must establish**

- contracted declaration and host facade snapshots;
- public API documentation for typed schema use, authentication adaptation,
  installed application queries, capability and disclosure composition,
  break-glass progression, mutation outcomes, recovery, linear undo/redo,
  history, preview posture, and live delivery;
- `AI_README.md` orientation links that lead agents from the runtime model to
  the relevant feature documents;
- migration of relevant Worth UI or other reference-consumer workarounds where
  the new surface owns the capability;
- deletion of raw aspect strings, manual permission registries, local Query
  authority builders, application-local generic cursors, lane-specific query
  copies, undo stacks, break-glass booleans, post-projection redaction, and
  duplicate outcome assembly; and
- residue checks that prevent their return.

**Proof before Bank World Phase 6**

A fresh consumer can discover and implement the supported bank paths from the
public facade and docs without architectural archaeology.

### Bank World Track — Phase 6: Complete Consumer Journeys

**Requirement**

Assemble the runtime phases and bank mechanisms into the complete public bank
application without internal imports, local authority, or fixture shortcuts.

**Must establish**

- authenticated account discovery and scoped account detail;
- personal transfer, deposit, and withdrawal;
- business payment initiation and distinct-user approval;
- employee teller and auditor journeys with non-equivalent authority;
- estate-case discovery, executor and beneficiary views, restricted-field
  disclosure, account freeze/release, and conflict-free disbursement;
- branch-manager conflict-of-interest, narrowed delegation, governed
  break-glass approval, expiry, revocation, and mandatory review;
- concurrent transfer, idempotent retry, response-loss, and stale outcomes;
- authorized activity history and live delivery;
- permission grant, revocation, and live-stream narrowing; and
- one installed query exercised as one-shot, paged continuation, historical,
  live, and admitted preview work wherever its support posture permits;
- compensating undo, freshly authorized redo, divergent-redo invalidation, and
  honest irreversible posture; and
- typed explanations and actionable recovery for denial, conflict,
  cancellation, disclosure omission, elevation, and indeterminacy.

**Proof before Closure Track Phase 1**

Every basic front-door question is answered through public Query and adapter
surfaces, every process boundary is real, and the gap ledger contains no
unclassified workaround.

### Closure Track — Phase 1: Hostile Certification And Permanent Prohibitions

**Requirement**

Close the milestone through consumer-real, adversarial evidence and permanent
enforcement.

**Must establish**

- the complete requirement/evidence closure ledger;
- cross-process public-consumer tests;
- compile-fail authority and phase-order probes;
- hostile OIDC, policy, touched-graph, invariant, concurrency, idempotency,
  cursor, basis, disclosure, delegation, conflict-of-interest, break-glass,
  aftermath, live-revocation, and transport scenarios;
- certification-only replay parity for authorized bank results;
- warm-path work counters and targeted compile/test timing;
- boundary, context, facade, dependency, residue, and file-size enforcement;
  and
- explicit ownership for every discovered but deferred capability.

**Closure rule**

A green suite does not close an unproved ledger row. Every high- or
critical-impact finding reopens the guarantees it can invalidate and the
downstream rows that depend on them.

## DX Target

The application-facing target should be recognizable to a normal framework
consumer while preserving typed authority:

```rust
bank.operation(operations::send_money)
    .as_principal(request.authenticated_principal())
    .with_input(SendMoney {
        from: accounts::AccountRef::from_path(request.path_account()),
        to: customers::PaymentHandle::parse(request.body.recipient)?,
        amount: Money::<USD>::from_minor(request.body.cents)?,
    })
    .idempotency(request.idempotency_key())
    .deadline(request.deadline())
    .execute()
    .await?
```

Business approval should expose the legal next action from a typed result:

```rust
match payment.initiate().await? {
    InitiatedPayment::Committed(receipt) => publish(receipt),
    InitiatedPayment::ApprovalRequired(pending) => {
        pending.approve_as(request.authenticated_principal()).await?
    }
    InitiatedPayment::Denied(reason) => deny(reason),
}
```

Authorized live delivery should remain query-shaped:

```rust
let activity = bank
    .query(queries::account_activity(account))
    .as_principal(request.authenticated_principal())
    .subscribe(LiveDelivery::server_sent_events())
    .await?;
```

One installed query should retain its meaning across supported bases and
lifecycles:

```rust
let query = queries::estate_activity(estate_case)
    .ordered_by(activity::committed_sequence())
    .page_size(50)?;

let current = bank.query(query.clone())
    .as_principal(principal)
    .purpose(purposes::estate_administration())
    .at(ReadBasis::Current)
    .run()
    .await?;

let historical = bank.query(query.clone())
    .as_principal(principal)
    .purpose(purposes::estate_administration())
    .at(ReadBasis::Historical(current.version()))
    .run()
    .await?;

let live = bank.query(query)
    .as_principal(principal)
    .purpose(purposes::estate_administration())
    .live(LiveControls::bounded(64)?)
    .open()
    .await?;
```

Emergency elevation should remain narrower than ordinary institutional power
and should expose mandatory review as a legal next action:

```rust
let requested = bank.break_glass(emergency::preserve_estate_account(account))
    .as_principal(branch_manager)
    .for_purpose(purposes::suspected_asset_dissipation())
    .fields(disclosure::restricted_account_preservation())
    .expires_within(Duration::minutes(20))
    .request()
    .await?;

let active = requested
    .approve_as(non_conflicted_compliance_officer)
    .await?;

let inspection = active.query(queries::estate_preservation_view(account)).await?;
let review = active.close()?.mandatory_review();
```

Undo and redo should expose only the operation's installed aftermath:

```rust
let committed = bank.mutate(commands::send_money(input))
    .as_principal(principal)
    .controls(controls)
    .run()
    .await?
    .require_committed()?;

let undone = committed
    .aftermath()
    .compensate_as(principal, undo_controls)
    .await?;

let redo = undone.redo_intent()?;
let redone = bank.redo(redo)
    .as_principal(principal)
    .controls(redo_controls)
    .run()
    .await?;
```

These blocks are design targets. The plan for each phase must reconcile them
against the real public APIs and make any necessary semantic differences
explicit before implementation.

## Basic Front-Door Questions

The completed application must answer, through public typed APIs:

- Who am I in this bank?
- Which personal and business accounts may I see?
- Which estate cases, accounts, documents, and fields may I see for this exact
  purpose?
- What is the exact current and available balance of an account?
- Which capability grants, denials, conflicts, delegations, employee
  assignments, and case relationships affect this operation?
- May I initiate, approve, deposit, withdraw, or transfer from this account?
- May I freeze, release, inspect, or disburse this estate account, and which
  distinct actor must act next?
- Why is a field omitted even though the containing account is visible?
- May a protected fact influence an authorized filter, ordering, count, or
  policy decision without becoming visible, and what prevents it from leaking
  through membership, rank, cursor, summary, or timing posture?
- Can I delegate this capability, how narrowly, for how long, and with what
  downstream delegation posture?
- Does my personal interest conflict with my employee authority?
- What exact emergency access may I request, who must approve it, when does it
  expire, and what review remains mandatory?
- Why was an operation denied?
- What payments are pending my approval?
- Did my request commit, fail, become stale, or remain indeterminate?
- If I retry after losing the response, will money move twice?
- What account activity may I inspect?
- Can I ask the same installed query at current, historical, live, or preview
  basis without its meaning or disclosure changing?
- Is this cursor valid for this exact query, ordering, basis, and scope?
- Which consistency, freshness, result, work, deadline, and cancellation
  controls belong to me?
- Can I subscribe to the same authorized result and receive query-shaped
  changes?
- Did this filtered, sorted, or relationship-expanded result consume one
  admitted graph-read access plan without per-result child reads or hidden
  fallback?
- What changes when my role is granted or revoked?
- Can two users race without overspending or observing impossible balances?
- Can an auditor inspect allowed evidence without gaining mutation power?
- Can an employee who is also a customer keep those authorities distinct?
- If the employee is also a beneficiary, can conflict-of-interest prevent
  self-benefiting access even though each relationship is individually valid?
- If the outcome is indeterminate, what typed recovery action remains legal?
- Can I undo this committed action without erasing history, and can redo be
  denied when current authority or truth changed?
- Which operations are reversible, compensatable, reconcilable, or
  irreversible?

If one of these requires an internal import, local registry, raw string, hidden
callback, or result reinterpretation, the front door is not finished.

## Must Preserve

- Milestone 9.15 prepared-state and invariant authority;
- one installed operating-world root;
- Query-agnostic schema and domain meaning;
- Relational ownership of authoritative graph facts and transaction mechanics;
- runtime-bridge ownership of exact installed correspondence;
- Signal ownership of policy-node evaluation truth;
- authentication distinct from authorization;
- roles and relationships distinct from scoped capability authority;
- ordinary capability distinct from governed emergency elevation;
- entity visibility distinct from field disclosure;
- one canonical application-query identity across supported execution lanes;
- committed history preserved by inverse or compensation rather than erased by
  undo;
- redo as fresh execution rather than replay or retained authority;
- exact Foundational value meaning;
- cert-only replay imports;
- typed outcomes over exceptions or generic error strings; and
- bounded ordinary warm paths with cold certification isolated.

## Explicit Non-Goals

- loans, credit scoring, interest, exchange rates, card networks, chargebacks,
  fraud modeling, or regulatory reporting;
- production identity administration or a custom identity provider;
- replacing the final WORTH Server architecture;
- browser UI polish;
- multi-currency conversion;
- distributed consensus or multi-bank settlement;
- branch- or tree-shaped undo/redo navigation, branch-local inversion, branch
  merge, or conflict resolution. Those are cross-runtime semantic-history
  capabilities, not a deferred Query-local extension of linear aftermath;
- durable recovery handles, restart-stable cursors, or restart-stable
  undo/redo history before the Store handoff;
- advanced domain access products, correlated paths, conflict partitions, or
  geometry/provider certification governed by Milestone 9.17.

The absence of those capabilities cannot justify fake authentication, fake
money, fake concurrency, or fake authorization in the supported world.

## Acceptance Evidence

Milestone 9.16 closes only when:

- the bank courtroom runs against a real Authentik issuer, one bank server, and
  separate user-node processes over TCP;
- all actors and relationships are provisioned dynamically;
- public consumer code contains no semantic aspect strings or internal Query
  imports;
- the Relational -> runtime-bridge -> Signal -> Query authorization chain is
  exercised and independently challenged;
- capability scope, purpose, delegation, conflict-of-interest, field
  disclosure, break-glass approval, expiry, revocation, and review are
  exercised and independently challenged;
- one installed application query preserves canonical identity, result shape,
  ordering, disclosure, and scope across every supported one-shot, continuation,
  historical, live, and preview lane;
- installed-query filters, sorts, traversal, and nested expansion lower through
  the existing Milestone 9.10 requirement, inventory, budget, admitted-plan,
  and receipt chain, with exact-zero covered per-result neighbor lookups and
  zero undeclared fallback;
- read, mutation, explanation, history, and live surfaces enforce identical
  scoped authority;
- monetary invariants and an independent double-entry oracle agree;
- concurrency, stale detection, idempotent retry, response loss, and failure
  injection produce honest typed outcomes;
- revocation prevents subsequent unauthorized live delivery;
- compensating undo preserves original truth, redo requires fresh authority,
  divergent or relevant change can deny redo, and irreversible actions expose
  no fake inverse;
- every indeterminate outcome exposes an actionable governed recovery posture;
- certification-only replay agrees with ordinary authorized result meaning;
- all workaround deletions and permanent prohibitions are enforced; and
- the closure ledger has no unresolved high- or critical-impact row.

## Handoff To Milestone 9.17

Milestone 9.17 may add advanced computation only through the public typed
declaration, admission, execution, publication, and certification path proven
here. Advanced search, spatial access, membership, paths, bulk execution,
decision attachments, and reuse bind to the installed application-query,
capability, disclosure, basis, recovery, and aftermath contracts established
here. It may extend that path; it may not reintroduce a specialist-only
authority lane, a provider-owned cursor, a field-disclosure bypass, or replay
disguised as redo.
