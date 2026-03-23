# Forge Server Vision

## Thesis

`forge-relational` owns truth. `forge-signal` owns derived computation.
`forge-store` owns durability. `forge-server` owns delivery.

`forge-server` is the network-facing runtime that turns durable, reactive,
branch-aware truth into a live service. It owns the sync protocol, the HTTP
surface, the subscription lifecycle, the delivery infrastructure, file and
binary handling, authentication integration, and every contract between the
Forge runtime stack and its external consumers — whether those consumers are
browser clients, mobile apps, CLI tools, API integrations, or other Forge
stores.

It is not just a wire protocol. It is the full server runtime that makes the
Forge stack accessible over the network while preserving the semantic contracts
that make the stack valuable: durable subscriptions, cursor-based resume,
aspect-aware narrowing, typed delivery classes, basis negotiation, branch-aware
reads, and provenance-bearing patches.

The server is intended for systems where connected clients need live,
incremental, explainable state synchronization with the same structural
guarantees the runtime provides locally — without collapsing into generic
pub/sub, crude polling, or dumb WebSocket broadcast.

## What This Server Is For

`forge-server` exists for product surfaces where external consumers need to
observe, mutate, and synchronize truth-state with the precision and
efficiency the runtime makes possible.

It is meant to support:

- collaborative web applications where multiple users edit the same workspace
  and need immediate, minimal, semantically precise state updates instead of
  broad refetch or crude room-based broadcast
- mobile and offline-first applications that need durable subscriptions,
  cursor-based resume, and compound catchup patches instead of full re-sync
  after disconnection
- AI agent interfaces that need branch-aware reads, speculative writes on
  branches, and provenance-bearing responses that explain what changed and why
- dashboard and monitoring surfaces that need live incremental refresh with
  tolerance-aware suppression instead of periodic full-page polling
- integration pipelines that need durable CDC consumption with exactly-once
  cursor semantics and structured schema-aware change feeds
- multi-store replication where one Forge store subscribes to another's commit
  stream through the same protocol that serves browser clients
- geometry and chip-design tool servers that need snapshot-safe concurrent reads,
  branch-local analysis, and large binary asset management alongside structured
  truth sync

The technical thesis is the same across all of them:

- the server must know what changed, who cares, and what is meaningful
- delivery must be minimal, not maximal
- subscriptions must survive connection loss
- resume must be cursor-based, not full re-sync
- binary assets and structured truth are separate concerns with separate
  transport
- authentication and authorization are infrastructure, not features
- the server must not weaken the runtime's semantic guarantees

## Why This Server Is Different

These are not optional add-ons. They are the capabilities that make
`forge-server` strategically different from ordinary application servers:

- signal-native subscription evaluation (server-side reactive narrowing)
- durable leased subscriptions that survive connection loss
- semantic cursor-based resume and compound catchup
- aspect-aware delivery filtering
- tolerance-aware update suppression
- typed delivery classes (authoritative, replaceable, presence, advisory)
- invalidation lane and patch lane separation
- basis negotiation for minimal delta delivery
- branch-aware reads and branch-scoped subscriptions
- structured schema-aware CDC as a protocol surface
- binary asset management alongside structured sync
- freshness mode negotiation (live, coalesced, invalidate-only, pull-on-focus)
- shared subscription bases for overlapping client views
- provenance-bearing patches that explain causality
- server-side view materialization for hot collaborative surfaces
- typed middleware pipeline with declarative policy enforcement
- aspect-level authorization — not just route-level, but per-aspect delivery scoping
- view-shaped patch delivery (collection splices, group membership changes)
- optimistic mutation protocol with branch-based conflict resolution
- multi-tenant workspace routing with automatic branch scoping
- schema-validated mutations with typed error responses

If these are treated as "nice to have later," the server becomes an ordinary
REST API with a WebSocket bolted on, and the entire value of the runtime stack
below it is wasted at the network boundary.

## Mission

`forge-server` exists to deliver truth-state changes to external consumers
with precision, efficiency, and durability that ordinary server architectures
cannot achieve.

It must answer these questions as native server responsibilities:

- How do clients declare durable subscriptions that survive transport
  interruption?
- How does the server evaluate which subscriptions are materially affected by
  a truth commit without broadcasting to all connected clients?
- How do clients resume from exactly where they left off after disconnection,
  receiving one compound patch instead of replaying every missed event?
- How does the server decide whether a change is meaningful enough to deliver,
  based on aspect masks, tolerance thresholds, and comparator policies?
- How are different kinds of updates — authoritative truth patches, replaceable
  latest-state, ephemeral presence, advisory hints — delivered with
  appropriate reliability and ordering guarantees?
- How do binary assets (file uploads, large exports, media) coexist with
  structured truth sync without polluting the sync channel?
- How does authentication and authorization integrate so every subscription,
  every read, and every mutation is policy-checked without per-endpoint
  boilerplate?
- How does the server support branch-aware reads and branch-scoped
  subscriptions so clients can observe speculative truth without muddying
  main-branch state?
- How does the server negotiate what the client already has so it can deliver
  the smallest correct update?

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| `forge-relational` | Truth-state semantics | identity, transactions, MVCC, diffs, CDC, lineage, schema |
| `forge-store` | Durable storage | commit persistence, WAL, snapshots, compaction, backends |
| `forge-signal` | Derived computation | dependency DAG, invalidation, recomputation, scheduling |
| Bridge / integration | Decoupled coordination | patch-to-invalidation, aspect mapping, snapshot evaluation |
| `forge-server` | Network delivery | subscriptions, sync protocol, HTTP surface, file handling, auth, delivery |

### What `forge-server` owns

- the sync protocol: message types, delivery semantics, cursor contracts
- the subscription manager: lease lifecycle, durable subscription state,
  per-client evaluation, outbox management
- the HTTP surface: REST endpoints, file upload, multipart handling,
  binary downloads, traditional request/response
- the WebSocket/WebTransport surface: sync channel upgrade, message framing,
  connection lifecycle
- authentication and authorization integration: middleware, policy enforcement,
  subscription-scoped permission checks
- delivery infrastructure: typed delivery classes, invalidation lane, patch
  lane, presence lane, coalescing, basis negotiation
- freshness mode management: per-subscription freshness contracts
- branch-aware routing: branch-scoped reads, branch-scoped subscriptions
- shared subscription optimization: base view sharing for overlapping clients
- server diagnostics: connection metrics, subscription metrics, delivery
  metrics, protocol health

### What `forge-server` does not own

- truth semantics, transaction semantics, or identity semantics
- storage engine behavior, WAL management, or compaction
- signal evaluation, dependency tracking, or reactive scheduling
- domain-specific meaning of entities, relations, or aspects
- bridge routing logic or aspect mapping rules
- schema validation or integrity enforcement
- application-specific business logic

### Structural rule

`forge-server` translates between the Forge runtime stack and the network.
It does not become a second truth runtime, a second signal scheduler, or a
second storage engine. It consumes the runtime's semantic precision and
delivers it faithfully to external consumers.

## Principles

1. Subscriptions are durable server state, not connection-scoped ephemeral
   state.
2. Delivery must be minimal by default — the server sends only what is
   meaningful for each subscription.
3. Binary assets and structured truth sync are separate transport concerns
   with separate channels.
4. Authentication and authorization are infrastructure middleware, not
   per-endpoint application code.
5. Connection loss must not destroy subscription state.
6. Resume is cursor-based, never full re-sync unless the basis is
   irrecoverable.
7. Different update types deserve different delivery guarantees.
8. The server evaluates subscription relevance using the signal graph, not
   by broadcasting and filtering.
9. Branch-aware reads and branch-scoped subscriptions are first-class, not
   add-ons.
10. Freshness mode is a client-declared contract, not a server-imposed
    default.
11. Shared subscription optimization must not compromise per-client
    correctness or per-client permission scoping.
12. Server diagnostics are a production contract, not debugging
    instrumentation.
13. The sync protocol must be transport-pluggable — WebSocket first,
    WebTransport later, HTTP fallback always.

## Foundational Decisions

These are locked architectural decisions:

- axum is the HTTP and WebSocket framework
- tokio is the async runtime
- postcard is the default binary serialization for sync messages (JSON
  fallback for debugging and development)
- lease creation is HTTP, not WebSocket — leases are durable state, not
  connection events
- the WebSocket channel is for delivery, not for subscription management
- subscription evaluation uses the signal graph through the bridge, not
  custom narrowing logic
- authentication middleware applies uniformly to HTTP, WebSocket, and
  lease management
- file uploads go through standard HTTP multipart; file metadata flows
  through truth commits and triggers sync updates
- the sync protocol defines explicit message types with versioned schemas
- CDC cursor persistence delegates to `forge-store`
- presence and truth updates are separate protocol lanes

## Capability Pillars

### Sync Protocol Architecture

#### Durable leased subscriptions

Technical role:
Clients declare subscriptions as durable server-side state. Each subscription
is a lease with an explicit view definition, aspect mask, tolerance policy,
freshness mode, and CDC cursor. The lease survives transport interruptions
and can be resumed from any compatible connection.

What this enables:

- mobile apps that maintain subscriptions across cellular interruptions
- browser tabs that resume after sleep without full re-sync
- integration pipelines that never lose their place in the change stream
- multi-device scenarios where the same subscription transfers between devices

#### Semantic cursor-based resume

Technical role:
Every lease tracks a cursor in the CDC stream. When a client reconnects, the
server computes a compound patch from the cursor position to HEAD rather than
replaying individual events.

What this enables:

- reconnection cost proportional to actual change, not missed event count
- offline-first architectures with efficient sync-on-reconnect
- background tabs that catch up cheaply when foregrounded
- durable integrations that resume from exactly where they stopped

#### Typed delivery classes

Technical role:
Not all updates deserve the same reliability and ordering guarantees. The
protocol defines explicit delivery classes:

- **authoritative-ordered**: committed truth patches that must arrive in
  causal order and be acknowledged
- **replaceable-latest-state**: live values where only the latest state
  matters and intermediate values can be dropped
- **coalescible-region**: region-scoped updates that can be merged into
  one compound update
- **ephemeral-presence**: user cursor, typing state, activity indicators
  that are lossy-ok and never persisted
- **advisory-hint**: notifications that something may have changed,
  prompting client-side pull

What this enables:

- presence updates that do not consume authoritative delivery bandwidth
- dashboard metrics that deliver only the latest value, not a queue of
  intermediate states
- collaborative editing with lightweight presence alongside heavyweight
  truth patches
- protocol-level efficiency instead of application-level filtering

#### Invalidation lane and patch lane separation

Technical role:
The server maintains two logical delivery lanes: a lightweight invalidation
lane that notifies of staleness with minimal bytes, and a patch lane that
delivers actual content when needed.

What this enables:

- backgrounded tabs receive only invalidation tokens, not full patches
- clients can defer heavy patch consumption until they are active
- bandwidth-constrained connections get awareness without payload
- progressive loading where invalidation arrives immediately and patches
  follow on demand

#### Basis negotiation

Technical role:
The server and client negotiate what the client already has so the server
can compute the smallest correct response. The client declares its cursor,
basis tokens, and region tokens. The server responds with either: no change,
inline patch, region replace, reorder-only, or full rebase.

What this enables:

- GET requests that return 400 bytes instead of 70KB when most data is
  unchanged
- cursor-based state advancement that replaces traditional request/response
  fetch
- partial-region refresh when only part of a view changed
- explicit rebase when the client's basis is too stale for delta delivery

#### Freshness modes

Technical role:
Clients declare a freshness contract per subscription:

- **live_strict**: immediate push of every meaningful change
- **live_coalesced**: push with coalescing window for high-churn sources
- **background_coalesced**: coalesce aggressively while client is inactive
- **invalidate_only**: receive staleness tokens, pull patches on demand
- **pull_on_focus**: receive nothing until client declares focus, then
  advance to HEAD
- **presence_only**: receive only presence updates, no truth patches

What this enables:

- bandwidth-appropriate delivery for different UI surfaces
- mobile data conservation for backgrounded subscriptions
- server-side resource management that matches client attention
- progressive refinement where initial load is fast and detail follows

### HTTP Surface Architecture

#### REST endpoints

Technical role:
Traditional request/response operations — mutations, one-off queries,
exports, health checks — go through standard HTTP routes. These coexist
on the same server and port as the sync protocol.

What this enables:

- familiar API surface for operations that are naturally request/response
- compatibility with existing HTTP tooling, proxies, and load balancers
- clear separation between "do something" (HTTP) and "stay synchronized"
  (sync channel)
- standard HTTP caching headers for cacheable read endpoints

#### File and binary asset handling

Technical role:
File uploads use standard HTTP multipart. File downloads use standard HTTP
range requests. Binary blobs never flow through the sync channel. File
metadata — who uploaded, what entity it is attached to, size, type — flows
through truth commits and triggers sync updates through the normal CDC path.

What this enables:

- large file uploads without blocking the sync channel
- resumable downloads for large exports
- CDN-compatible static asset serving
- truth-aware file metadata updates that trigger reactive sync

#### Streaming responses

Technical role:
Large exports, bulk queries, and initial hydration use HTTP chunked
streaming. The server streams results without buffering the entire response
in memory.

What this enables:

- large data exports that start delivering immediately
- initial lease hydration through paginated HTTP catchup before switching
  to live WebSocket delivery
- bulk read operations that do not require the client to fit the entire
  result in memory

### Subscription Manager Architecture

#### Signal-backed subscription evaluation

Technical role:
Each active lease corresponds to a signal subscription node in the server-side
signal graph. When a truth commit triggers CDC, the signal graph evaluates
which leases are affected. Only leases whose subscribed aspects and entities
are materially changed receive delivery.

What this enables:

- evaluation cost proportional to affected subscriptions, not total
  subscriptions
- zero delivery for subscriptions unaffected by a commit
- tolerance-aware suppression where numerically insignificant changes
  produce no delivery
- provenance-bearing patches that explain why the update was generated

#### Subscription outbox and coalescing

Technical role:
Each lease has an outbox that accumulates pending deliveries. If the client
is disconnected or in a coalescing freshness mode, multiple pending updates
are merged into a single compound delivery using the signal graph's
reduction logic.

What this enables:

- efficient reconnection with one compound patch instead of N queued events
- background coalescing that reduces server memory pressure
- coalescing policies that match client freshness mode
- bounded outbox size through retention-aware eviction

#### Shared subscription bases

Technical role:
When multiple clients subscribe to overlapping views (e.g., 20 users on the
same workspace page), the server can compute a shared base evaluation and
layer per-client narrowing (permissions, aspect masks) on top.

What this enables:

- O(1) base computation for N overlapping subscribers instead of O(N)
- efficient collaborative workspace delivery
- per-client permission enforcement without per-client full evaluation
- horizontal scaling of collaborative surfaces

### Authentication and Authorization Architecture

#### Middleware-enforced authentication

Technical role:
Authentication is a server-wide middleware layer that applies to every HTTP
route, every WebSocket upgrade, and every lease operation. No endpoint
exists without authentication unless explicitly marked public.

What this enables:

- impossible to accidentally create an unprotected endpoint
- uniform authentication across REST, sync, and file operations
- pluggable authentication providers (OAuth, JWT, API keys, SSO)
- session management integrated with lease lifecycle

#### Subscription-scoped authorization

Technical role:
Authorization checks apply at subscription creation time and at delivery
time. A lease cannot subscribe to entities the client is not authorized to
read. Delivery suppresses patches that would expose unauthorized state even
if the subscription was valid when created.

What this enables:

- permission changes take effect on existing subscriptions
- no information leakage through stale subscription permissions
- role-based and entity-based access control at the protocol level
- audit-safe delivery that respects authorization continuously

### Branch-Aware Delivery Architecture

#### Branch-scoped subscriptions

Technical role:
Clients can subscribe to specific branches, not only the main branch. The
server delivers patches from that branch's commit stream and suppresses
updates from other branches.

What this enables:

- preview environments showing branch-specific truth
- AI agent interfaces monitoring speculative branches
- branch comparison views with per-branch sync
- staging environments as branch-scoped subscriptions

#### Branch-aware reads

Technical role:
HTTP reads can target specific branches or specific historical snapshots
through the same API surface. The server resolves reads against the
appropriate branch head or materialized snapshot.

What this enables:

- API consumers reading from feature branches
- historical reads against pinned snapshots
- branch comparison through parallel reads
- time-travel-capable API surfaces

### Observability Architecture

#### Server diagnostics

Technical role:
The server must expose: active lease count, connected client count, delivery
rates per class, outbox depths, coalescing ratios, evaluation latency, CDC
lag, WebSocket connection health, and protocol error rates.

What this enables:

- operational monitoring of live collaborative surfaces
- capacity planning based on subscription and delivery metrics
- performance diagnosis when delivery latency increases
- alerting on protocol health degradation

#### Protocol-level provenance

Technical role:
Delivered patches carry causal metadata: which commit triggered them, which
aspects changed, which comparator policies applied, and why this subscription
was considered affected. This provenance is available at delivery time, not
only through retrospective log analysis.

What this enables:

- client-side debugging of unexpected or missing updates
- admin tooling that explains delivery decisions
- audit surfaces that trace truth changes through delivery
- compliance reporting for regulated collaborative workflows

### Request Pipeline Architecture

#### Typed middleware pipeline

Technical role:
Every request — HTTP, WebSocket message, lease operation, mutation — flows
through a typed middleware pipeline. Middleware layers compose functionally
and execute in declared order:

- **transport decoding**: message framing, deserialization
- **authentication**: identity resolution, session validation, token
  verification
- **tenant resolution**: workspace identification, branch scoping for
  multi-tenant deployments
- **rate limiting**: per-tenant, per-user, per-operation rate enforcement
- **authorization**: route-level, entity-level, and aspect-level permission
  checks
- **query/mutation validation**: schema-aware validation of the request
  payload
- **execution**: query evaluation or mutation commit
- **response transformation**: result shaping, delivery formatting
- **observability**: logging, metrics, provenance tagging

What this enables:

- impossible to accidentally bypass auth, tenant scoping, or validation
- middleware is composable and declarative, not hand-coded per endpoint
- new middleware layers can be added without modifying existing handlers
- the pipeline is inspectable and testable as a typed composition
- developers never write per-endpoint boilerplate for cross-cutting
  concerns — they declare pipeline policy once

#### Policy pipeline and declarative authorization

Technical role:
Authorization is enforced through a declarative policy pipeline that
operates at multiple granularities:

- **route-level**: can this user access this endpoint at all?
- **entity-level**: can this user read or mutate this specific entity?
- **aspect-level**: can this user see this specific aspect of this entity?
- **branch-level**: can this user read or write to this branch?
- **operation-level**: can this user perform this specific mutation type?

Policies are declared against the schema, not coded per handler. The server
enforces them structurally — an aspect the user cannot see is never included
in delivery, not filtered client-side.

What this enables:

- aspect-level security where salary data is never sent to unauthorized
  users, without the developer writing conditional checks
- branch-level access control where draft branches are visible only to
  their owners or reviewers
- policy changes that take effect immediately on existing subscriptions
- authorization that composes with shared subscription bases (shared base
  evaluation, per-client aspect masking on top)
- zero per-endpoint authorization boilerplate

### View-Specific Delivery Architecture

#### View-shaped patch delivery

Technical role:
When a query declares a view shape (table, kanban, timeline, chart), the
server uses that intent to deliver patches in view-appropriate formats:

- **table view patches**: collection splices ("insert at position 3,"
  "remove from position 9," "update row 7 field X")
- **kanban view patches**: group membership changes ("move entity from
  group A to group B," "add entity to group C")
- **timeline view patches**: range intersection updates
- **chart view patches**: aggregation value deltas with tolerance-aware
  suppression
- **detail view patches**: per-aspect field-level diffs

What this enables:

- frontends apply surgical UI updates instead of re-rendering entire lists
- bandwidth reduction since patches carry only the structurally meaningful
  change for the declared view
- the server and query layer collaborate on delivery format without the
  frontend specifying how to apply patches
- different clients viewing the same data as different view shapes receive
  view-appropriate patches from the same underlying truth change

#### Server-side view materialization

Technical role:
For hot collaborative surfaces where many clients share the same view, the
server can materialize and maintain the view result server-side, delivering
incremental patches to all subscribers from the maintained view rather than
re-evaluating per client.

What this enables:

- O(1) view evaluation for N subscribers on the same collaborative page
- reduced server compute for high-fan-out collaborative surfaces
- consistent delivery to all clients viewing the same materialized view
- view materialization as a server-managed optimization, transparent to
  clients

### Mutation Architecture

#### Schema-validated mutations

Technical role:
Every mutation flowing through the server is validated against the current
schema before execution. The server rejects invalid mutations with typed
error responses that explain which fields failed validation and why.

What this enables:

- schema-enforced data integrity at the server boundary
- typed validation error responses that frontends can render into
  field-level error messages
- no possibility of invalid truth commits through the server API
- validation rules that derive from schema declarations, not hand-coded
  per endpoint

#### Optimistic mutation protocol

Technical role:
The server supports an optimistic mutation protocol where clients can:

1. Apply a mutation locally and render the result immediately
2. Send the mutation to the server
3. Receive confirmation (mutation committed) or rejection (conflict or
   validation failure)
4. On confirmation: the local state is already correct
5. On rejection: roll back the local state and apply the server's
   canonical state

For branch-aware clients, optimistic mutations can be committed to a
local branch and merged on confirmation, providing clean rollback
semantics.

What this enables:

- zero-latency perceived mutations for interactive applications
- clean rollback semantics backed by branch isolation
- server-side validation without sacrificing responsive UI
- collaborative editing where local edits appear immediately and
  settle against canonical truth asynchronously

### Multi-Tenant Architecture

#### Tenant workspace routing

Technical role:
The server resolves tenant identity from request context (subdomain,
header, path, token) and routes all operations to the correct workspace.
Tenant resolution is a middleware layer that applies to every request,
ensuring all reads, mutations, subscriptions, and file operations are
scoped to the correct tenant workspace.

What this enables:

- multi-tenant SaaS deployment from a single server instance
- tenant isolation enforced at the routing layer, not the application
  layer
- per-tenant policy, rate limiting, and budget enforcement
- workspace-scoped subscriptions that never leak data across tenants

#### Tenant-scoped delivery

Technical role:
Subscription delivery is scoped to the resolved tenant workspace. A
subscription in tenant A never receives patches from tenant B, even if
the underlying store uses branch-based tenant isolation with shared base
structure.

What this enables:

- shared base data across tenants (e.g., system configuration, default
  templates) with per-tenant branch-local customization
- tenant-specific CDC streams that only include changes relevant to that
  tenant
- cross-tenant isolation that is structural, not filtered
- tenant-aware shared subscription optimization within a single tenant's
  user base

## Domain Fit

### Collaborative Web Applications

`forge-server` should support:

- immediate multi-user state synchronization
- durable subscriptions across page refreshes and tab sleep
- cursor-based resume after network interruption
- tolerance-aware suppression for noisy real-time surfaces
- presence lanes for collaborative awareness
- shared subscription optimization for hot collaborative pages

Revolutionary use:
collaborative web apps can stop building custom realtime infrastructure and
instead consume a server that natively knows what changed, who cares, and
what is meaningful — with durable subscriptions, cursor-based resume, and
typed delivery classes as the default, not as afterthoughts.

### Mobile and Offline-First Applications

`forge-server` should support:

- offline branch persistence with cursor-based sync-on-reconnect
- compound catchup patches instead of event replay
- bandwidth-appropriate freshness modes
- background coalescing for inactive subscriptions

Revolutionary use:
mobile apps can maintain persistent, efficient truth synchronization across
cellular interruptions and device sleep without custom sync logic.

### AI Agent Interfaces

`forge-server` should support:

- branch-scoped subscriptions for speculative evaluation
- provenance-bearing patches that explain changes
- branch-aware mutations for sandboxed agent writes
- snapshot-scoped reads for stable evaluation contexts

Revolutionary use:
AI systems can interact with truth state through a protocol that supports
branching, provenance, and speculative exploration natively, instead of
wrapping CRUD APIs with custom state management.

### Integration Pipelines

`forge-server` should support:

- durable CDC consumption with exactly-once cursor semantics
- schema-aware change feeds with aspect-tagged deltas
- structured subscriber resume after interruption
- contract-grade stream delivery with ordering guarantees

Revolutionary use:
downstream systems consume structured, schema-aware, cursor-resumable
change feeds from the same server that serves browser clients, instead of
bolting CDC onto a separate integration layer.

## Roadmap Direction

This file is a vision document, not the execution roadmap. But the future work
should be derivable from it.

The highest-signal server programs are:

- axum server with HTTP routes and WebSocket upgrade
- typed middleware pipeline with declarative composition
- lease management and durable subscription state
- CDC-triggered subscription evaluation through the signal graph
- sync protocol message types and cursor semantics
- typed delivery classes and lane separation
- basis negotiation and compound catchup
- policy pipeline with route-, entity-, aspect-, and branch-level enforcement
- schema-validated mutations with typed error responses
- optimistic mutation protocol with branch-based rollback
- authentication middleware and subscription-scoped authorization
- file upload handling and binary asset management
- freshness mode negotiation
- view-shaped patch delivery (table splices, kanban moves, chart deltas)
- server-side view materialization for collaborative surfaces
- tenant workspace routing and tenant-scoped delivery
- shared subscription optimization
- branch-scoped subscriptions and branch-aware reads
- postcard binary encoding with JSON fallback
- server diagnostics and protocol health metrics
- WebTransport support (future transport upgrade)

If a capability is named here and not yet built, it is roadmap work.

If a capability is built but not yet proven under concurrent load, connection
churn, permission change, and network failure scenarios, it is certification
work.

## Non-Goals

- turning the server into a truth runtime or storage engine
- owning signal evaluation logic (the server consumes the signal graph
  through the bridge)
- implementing domain-specific business logic in the server layer
- requiring WebTransport or HTTP/3 for initial delivery (WebSocket first)
- inventing a new transport protocol (the innovation is the application-layer
  sync model, not the transport)
- replacing HTTP for operations that are naturally request/response
- building a generic pub/sub system (subscriptions are signal-backed, not
  topic-based)
- treating presence and truth updates as the same delivery class

## Companion Documents

- [forge_relational_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_vision.md)
- [forge_signals2.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/forge_signals2.md)
- [forge_runtime_bridge_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_runtime_bridge_vision.md)
- [forge_store_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_vision.md)

Signal-backed subscription evaluation, durable leased subscriptions,
cursor-based resume, typed delivery classes, basis negotiation, and
branch-aware delivery are what make this server more than "REST plus
WebSocket." If those are weak, the entire Forge runtime stack delivers its
precision and efficiency only to local in-process consumers, and the network
boundary becomes the place where all the architectural investment is thrown
away.
