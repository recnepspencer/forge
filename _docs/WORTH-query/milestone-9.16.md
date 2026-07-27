# Milestone 9.16: Authenticated Async Bank World And The Ordinary Query Front Door

## Goal

Prove that a small team can build a legitimate authenticated, multi-user,
asynchronous application through the ordinary Query API without reconstructing
runtime authority, writing stringly semantic adapters, or reaching into Query
internals.

The proving application is an in-memory bank and person-to-person payment world.
It has real users, real employee and customer roles, personal and business
accounts, scoped authorized users, double-entry monetary effects, deposits,
withdrawals, transfers, approvals, concurrent requests, live updates, and a
real Authentik OIDC boundary.

The application is not a showcase shell. It is the adversarial consumer that
defines whether Query has a front door.

## Roadmap Placement

Milestone 9.15 ends with an installed operation whose proposed state has passed
real invariants. Milestone 9.16 adds the authority and public composition needed
to make that work useful to an ordinary application:

```text
schema-bound typed intent
    -> authenticated principal
    -> installed scoped authorization
    -> admitted touched graph
    -> prepared and invariant-approved proposal
    -> provider compare-and-commit
    -> typed outcome
    -> ordinary read / mutation / live result
```

Milestone 9.17 follows this milestone. Advanced access and computation features
must be built through the same public front door proven here.

## Migration From The Former 9.15 Draft

The split changes milestone ownership without silently dropping the former
requirements:

- former Phase 10.1, typed aspect references, is now Phase 2;
- former Phase 10.2, permission and touched-graph authority, is now Phase 4;
- former Phase 10.3, ordinary read/mutation/live adoption, is now Phase 7;
- former Phase 10.4, policy cutover and prohibitions, is now Phases 9–10;
- former Phase 11, compare-and-commit, is now Phase 6; and
- the application-relevant facade, cutover, and prohibition obligations from
  former Phases 19–20 are now Phases 9–10.

The bank phases around those moved requirements are causal proof, not extra
showcase work: they demonstrate that the public contracts compose into a real
application.

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
- one business with an owner, an initiator, an approver, and a read-only user;
- a bank teller and a bank auditor with non-equivalent employee scopes;
- a user who has both customer and employee relationships;
- two concurrent transfers competing for the same available funds;
- an idempotently retried transfer whose response is lost after commit;
- a business payment that requires approval by a different authorized user;
- authorization revoked while a live subscription is active;
- an inserted relationship that changes a previously negative authorization
  decision;
- a stale request whose read-set is relevant to the concurrent mutation;
- an unrelated mutation that must not cause a false conflict; and
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

12. Authorization is relationship- and operation-scoped. It is not a flat role
    string, ambient user object, route-local conditional, or caller-provided
    boolean.
13. Customer account powers derive from current account relationships such as
    personal owner, business owner, initiator, approver, and viewer.
14. Employee powers derive from institution-scoped assignments such as teller
    and auditor. Employee authority does not imply customer ownership and
    customer authority does not imply employee authority.
15. An operation declares the semantic capability it needs. Query derives the
    relevant policy dependencies, and the runtime evaluates them through
    installed Signal authority.
16. Relational owns authoritative graph facts and exact touched-graph evidence.
    The runtime bridge owns installed aspect correspondence and lowering.
    Signal owns local policy-node evaluation and decision evidence. Query owns
    their legal composition into operation admission.
17. No Query-local “super permission,” host callback, token claim, or route
    middleware result may replace that composition.
18. Authorization is evaluated against the graph the operation can actually
    read or affect. The executor cannot touch an entity, relation, or aspect
    outside the admitted touched graph.
19. Negative authorization facts and membership dependencies enter the decision
    read-set so permission-granting and permission-revoking mutations are
    causally visible.
20. Read projection, mutation admission, explanation, activity history, and
    live delivery enforce the same scope. Hiding a route is not access control.
21. Revocation closes or narrows active live delivery before subsequent
    unauthorized data can be emitted.

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
30. Transfer, deposit, withdrawal, opening funding, and reversal each have a
    distinct typed operation and accounting purpose.
31. Deposits and withdrawals move value through explicit bank cash or settlement
    accounts; they do not mint or destroy money implicitly.
32. Amounts are positive, currency-compatible, and bounded. Floating-point
    money is forbidden.
33. Available-funds and account-status invariants execute over proposed state.
34. Business payments may require a separate approver according to the
    installed account policy. An initiator cannot satisfy a distinct-actor rule
    by holding multiple roles.
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

## Destination Topology

### Query authority packages

```text
worth-query-decl
    schema-derived typed references
    operation, authz, read, mutation, workflow, and live declarations

worth-query-installation
    installed principal-binding and authorization contracts
    touched-graph policy compilation
    monetary operation and invariant contribution admission

worth-query-admission
    authenticated-principal-bound operation admission
    touched-graph and policy decision admission
    compare-and-commit admission

worth-query-execution
    policy-evaluated attempts
    provider compare-and-commit progression
    idempotent typed outcomes

worth-query-publication
    authorization-scoped read, mutation, activity, explanation,
    and live-publication contracts

worth-query-host
    ordinary host composition

worth-query-certification
    hostile public-consumer, authority, concurrency, and replay proof
```

### Reference-world packages

The exact package count may follow implementation pressure, but semantic
ownership must remain visible:

```text
workspaces/worth-query-bank-world/
    crates/
        bank-domain/
            banking schemas, typed operations, invariants, and policy contributions
        bank-server/
            runtime composition and authoritative in-memory provider
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
    -> Query operation intent
    -> Relational touched-graph facts
    -> runtime-bridge installed lowering
    -> Signal policy decision
    -> Query admitted operation
    -> Relational proposed state
    -> installed invariants
    -> provider compare-and-commit
    -> Query publication
```

Every arrow changes authority. The receiving boundary validates the proof it
needs; it does not trust an adjacent receipt or reproduce the previous owner's
decision.

## Phase Plan

### Phase 1: Freeze The Bank World And Build The Gap Ledger

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

**Proof before Phase 2**

The ledger covers every courtroom actor and behavior, every requirement has one
owner, and no test plan relies on a hard-coded principal or privileged fixture
mutation.

### Phase 2: Schema-Bound Typed Application References

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
  wrong-operation, and illegal-write examples.

**Proof before Phase 3**

The bank-domain package contains no application aspect strings, and the public
DX transcript compiles using only declaration-facing types.

### Phase 3: Authentik And Authenticated Principal Admission

**Requirement**

Adapt real Authentik OIDC identity into a sealed, time-bounded external-principal
proof and bind it to an installed bank principal.

**Must establish**

- issuer/audience/signature/time validation and key rotation behavior;
- `(issuer, subject)` identity, with display claims retained only as attributes;
- unknown, disabled, ambiguous, expired, revoked, and wrong-audience denial;
- explicit request deadline and cancellation propagation;
- no transport or test-only constructor for authenticated proof; and
- dynamic fixture provisioning through real Authentik administration and token
  acquisition boundaries.

**Proof before Phase 4**

Tokens from the real issuer admit the correct principal, forged or malformed
tokens fail, and application code cannot create or deserialize the proof.

### Phase 4: Scoped Authorization And Touched-Graph Admission

**Requirement**

Compile installed bank abilities into the existing Relational, runtime-bridge,
Signal, and Query authority chain, then bind each admitted operation to its
exact allowed touched graph.

**Must establish**

- personal, business, and institution-scoped ability declarations;
- current relationship-backed role evaluation;
- distinct-actor approval rules;
- positive, negative, membership, and revocation dependencies;
- Relational-owned touched-graph evidence for reads and effects;
- runtime-bridge lowering through installed correspondence;
- Signal-owned decision evidence;
- Query-owned composition into an operation admission proof; and
- identical scope enforcement for reads, mutations, explanations, history, and
  live delivery.

**Proof before Phase 5**

Forged roles, token claims, caller-declared touched sets, route middleware
booleans, cross-account access, role combination, and post-revocation delivery
all fail. Valid customer and employee combinations remain usable.

### Phase 5: Double-Entry Banking Operations And Invariants

**Requirement**

Implement the bank domain as ordinary installed operations that emit typed
effects and execute real monetary invariants over proposed state.

**Must establish**

- personal and business account creation;
- explicit opening funding;
- direct transfer by stable recipient identity;
- deposit and withdrawal through bank accounts;
- business initiation and separate approval;
- immutable journal entries and postings;
- exact balance, available-funds, currency, account-status, and balancing
  invariants; and
- typed idempotency intent bound to the authenticated operation.

**Proof before Phase 6**

Independent accounting oracles recompute balances and journal conservation.
Overdraft, unbalanced entries, currency mismatch, duplicate approvals,
self-approval where prohibited, and idempotency-key payload drift all deny.

### Phase 6: Provider Compare-And-Commit

**Requirement**

Advance an invariant-approved Milestone 9.15 proposal through one
provider-proven compare-and-commit progression.

**Must establish**

- complete decision read-set validation;
- relevant versus unrelated drift classification;
- atomic effect application by the in-memory provider;
- one terminal attempt outcome;
- response-loss/idempotent resolution;
- explicit abort, partial-effect, indeterminate, stale, cancelled, denied, and
  committed posture; and
- no public construction path from plan, receipt, read-set, invariant result,
  or provider token fragments.

**Proof before Phase 7**

Concurrent transfers cannot overspend, unrelated writes avoid false conflicts,
the same idempotent request cannot post twice, and failure injection never
claims atomic commit without provider proof.

### Phase 7: Ordinary Read, Mutation, Workflow, And Live Facades

**Requirement**

Expose the bank's ordinary work through one declarative Query front door whose
valid next actions follow typed phase progression.

**Must establish**

- typed account summary, account detail, authorized-user, activity, and payment
  reads;
- typed transfer, deposit, withdrawal, and account-management mutation
  outcomes;
- business initiation/approval workflow progression;
- authorization-scoped live account and activity delivery;
- explanation surfaces for denials, stale attempts, and indeterminate outcomes;
- cancellation, deadline, consistency, idempotency, and delivery controls owned
  by the caller; and
- bounded work and honest degradation exposed in result metadata.

**Proof before Phase 8**

Consumer transcript tests compile and run using only `worth-query-decl` and
`worth-query-host`. No consumer assembles runtime identities, policies,
read-sets, provider sessions, receipts, or support snapshots.

### Phase 8: Temporary HTTP Boundary And Per-User Async Nodes

**Requirement**

Run the ordinary public Query surface across real asynchronous process and
network boundaries.

**Must establish**

- one authoritative bank-server process;
- one independently authenticated user-node process per fixture participant;
- an Axum adapter that maps HTTP and SSE onto the public Query facade;
- bounded request and stream queues, cancellation, deadlines, backpressure, and
  disconnect handling;
- dynamic ports, health/readiness, deterministic teardown, and leak detection;
- typed wire representations for semantic outcomes; and
- no route-local banking policy or direct provider access.

**Proof before Phase 9**

The full courtroom runs over TCP with separate process IDs and runtimes.
Disconnects, restarts of non-authoritative user nodes, response loss, queue
saturation, token expiry, and live revocation preserve semantic outcomes.

### Phase 9: Public Cutover, Documentation, And Workaround Deletion

**Requirement**

Make the proven front door canonical and delete the local reconstruction paths
that the bank world or existing consumers no longer need.

**Must establish**

- contracted declaration and host facade snapshots;
- public API documentation for typed schema use, authentication adaptation,
  authorization, mutation outcomes, and live delivery;
- `AI_README.md` orientation links that lead agents from the runtime model to
  the relevant feature documents;
- migration of relevant Worth UI or other reference-consumer workarounds where
  the new surface owns the capability;
- deletion of raw aspect strings, manual permission registries, local Query
  authority builders, and duplicate outcome assembly; and
- residue checks that prevent their return.

**Proof before Phase 10**

A fresh consumer can discover and implement the supported bank paths from the
public facade and docs without architectural archaeology.

### Phase 10: Hostile Bank Certification And Permanent Prohibitions

**Requirement**

Close the milestone through consumer-real, adversarial evidence and permanent
enforcement.

**Must establish**

- the complete requirement/evidence closure ledger;
- cross-process public-consumer tests;
- compile-fail authority and phase-order probes;
- hostile OIDC, policy, touched-graph, invariant, concurrency, idempotency,
  live-revocation, and transport scenarios;
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

These blocks are design targets. The plan for each phase must reconcile them
against the real public APIs and make any necessary semantic differences
explicit before implementation.

## Basic Front-Door Questions

The completed application must answer, through public typed APIs:

- Who am I in this bank?
- Which personal and business accounts may I see?
- What is the exact current and available balance of an account?
- Which authorized users and employee assignments affect this operation?
- May I initiate, approve, deposit, withdraw, or transfer from this account?
- Why was an operation denied?
- What payments are pending my approval?
- Did my request commit, fail, become stale, or remain indeterminate?
- If I retry after losing the response, will money move twice?
- What account activity may I inspect?
- Can I subscribe to the same authorized result and receive query-shaped
  changes?
- What changes when my role is granted or revoked?
- Can two users race without overspending or observing impossible balances?
- Can an auditor inspect allowed evidence without gaining mutation power?
- Can an employee who is also a customer keep those authorities distinct?

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
- read, mutation, explanation, history, and live surfaces enforce identical
  scoped authority;
- monetary invariants and an independent double-entry oracle agree;
- concurrency, stale detection, idempotent retry, response loss, and failure
  injection produce honest typed outcomes;
- revocation prevents subsequent unauthorized live delivery;
- certification-only replay agrees with ordinary authorized result meaning;
- all workaround deletions and permanent prohibitions are enforced; and
- the closure ledger has no unresolved high- or critical-impact row.

## Handoff To Milestone 9.17

Milestone 9.17 may add advanced computation only through the public typed
declaration, admission, execution, publication, and certification path proven
here. It may extend that path; it may not reintroduce a specialist-only
authority lane.
