# Forge Query Vision

## Thesis

`forge-relational` defines truth. `forge-store` persists it. `forge-signal`
derives from it. `forge-query` is how consumers ask for it.

`forge-query` is the typed, composable, aspect-aware query layer for
graph-native truth systems. It translates developer intent — "give me
these entities, with these aspects, filtered by these conditions, projected
to this shape" — into efficient reads against the truth runtime and store,
with native support for live subscription promotion, branch-scoped reads,
time-travel, lineage traversal, and incremental result maintenance.

It is not SQL. It is not GraphQL. It is a query model designed from the
ground up for truth systems that understand identity, aspects, branches,
lineage, schema evolution, and reactive change propagation.

The query layer exists because the gap between "truth exists" and "developers
can efficiently ask for exactly the truth they need" is where most systems
collapse into either too-powerful-to-optimize (arbitrary SQL) or
too-limited-to-express (REST endpoints for everything). `forge-query` fills
that gap with a query model that is expressive enough for real applications
and constrained enough for the runtime to optimize, narrow, and subscribe.

## What This Query Layer Is For

`forge-query` exists for every consumer that needs to read, filter, project,
traverse, or subscribe to truth — whether that consumer is a web frontend,
a server-side handler, an AI agent, a geometry kernel, a CLI tool, or another
Forge component.

It is meant to support:

- web applications that need typed, filtered, projected reads of current
  truth with automatic promotion to live subscriptions
- geometry kernels that need subgraph traversal, topological neighborhood
  queries, and aspect-scoped reads over branch-local truth
- AI systems that need branch-scoped reads, historical comparison queries,
  and lineage traversal for explanation and decision support
- dashboard and reporting surfaces that need aggregated, tolerance-aware
  reads with incremental refresh
- workflow and editor tools that need collection queries with sorting,
  pagination, and live updates
- integration pipelines that need structured CDC-shaped query output for
  downstream consumption
- admin and operator tools that need branch inspection, commit history
  traversal, and snapshot diffing

The technical thesis is the same across all of them:

- queries must be typed and composable, not string-based or ad hoc
- the query model must be aware of aspects, not force whole-entity reads
- live subscription promotion must be a native query capability
- branch-scoped and time-travel reads must be expressible without separate
  APIs
- the query layer must give the runtime enough structure to optimize,
  narrow, and incrementally maintain results
- queries must not bypass the truth runtime's semantic contracts

## Why This Query Layer Is Different

These are not optional add-ons. They are the capabilities that make
`forge-query` strategically different from ordinary query languages and ORMs:

- aspect-aware projection (query specific aspects, not whole entities)
- typed composable query expressions, not SQL strings
- native live query promotion (read → subscribe with no API change)
- branch-scoped reads as a query parameter, not a separate API
- time-travel reads as a query parameter (at commit, at snapshot, at time)
- lineage-aware traversal (follow identity evolution across versions)
- subgraph-scoped queries (topological neighborhood, dependency cone,
  assembly region)
- incremental result maintenance for live subscriptions
- tolerance-aware aggregation (suppress updates below threshold)
- schema-aware query validation at construction time
- CDC-shaped output for integration consumption
- query-to-signal bridging for reactive derived computation
- pagination and cursor-based traversal as first-class query features
- diff queries across branches, versions, and time ranges
- correspondence queries across branches via lineage

If these are treated as separate APIs bolted onto traditional read paths,
the developer experience fractures into "reads here, subscriptions there,
history somewhere else, branches through a different endpoint."

## Mission

`forge-query` exists to make asking for truth as precise, composable, and
efficient as the truth runtime makes storing it.

It must answer these questions as native query responsibilities:

- How do developers express exactly which entities, aspects, and relations
  they need without over-fetching or under-fetching?
- How does a one-time read become a live subscription without changing the
  query expression?
- How do developers read from a specific branch or a specific point in
  history without using a different API?
- How does the query layer validate queries against the current schema at
  construction time?
- How do subgraph and neighborhood queries express topological scope without
  falling back to arbitrary graph traversal?
- How do diff and comparison queries express "what changed between these
  two versions" as a first-class operation?
- How does the query layer give the runtime enough information to narrow
  CDC, optimize reads, and incrementally maintain results?
- How do aggregation and collection queries express tolerance and
  suppression policies for live delivery?
- How does the query layer support pagination, cursors, and bounded result
  sets for large collections?
- How do lineage queries traverse identity evolution without exposing
  internal lineage graph structure?

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| `forge-relational` | Truth semantics | identity, transactions, MVCC, diffs, CDC, lineage, schema |
| `forge-store` | Durable storage | commit persistence, snapshots, WAL, recovery, backends |
| `forge-signal` | Derived computation | dependency DAG, invalidation, recomputation, scheduling |
| `forge-query` | Query expression and planning | query types, composition, validation, planning, execution, result shapes |
| Bridge / integration | Decoupled coordination | patch-to-invalidation, aspect mapping, snapshot evaluation |
| `forge-server` | Network delivery | subscriptions, cursors, delivery classes, HTTP surface |
| Forge Cloud | Managed platform | workspaces, tenants, policies, admin, observability |

### Ownership boundary

`forge-query` owns:

- query expression types and composition API
- schema-aware query validation
- query planning and optimization
- aspect projection and narrowing
- result shape definition and projection
- pagination, cursor, and bounded traversal
- branch-scoped and time-travel read parameterization
- subgraph and neighborhood scope expressions
- diff and comparison query expressions
- lineage traversal query expressions
- aggregation expressions with tolerance policies
- live query promotion semantics
- query-to-signal bridging for incremental maintenance
- CDC-shaped output formatting
- query execution against the truth runtime and store

`forge-query` does not own:

- truth semantics, identity semantics, or transaction semantics
- storage layout, compaction, or recovery
- signal evaluation, dependency tracking, or reactive scheduling
- sync protocol delivery or subscription management
- authentication or authorization enforcement
- domain-specific meaning of entities, relations, or aspects
- network transport or message framing

### Structural rule

`forge-query` translates developer intent into efficient, validated,
narrowed reads against the truth runtime and store. It does not define
truth semantics, own storage behavior, or manage delivery. It produces
typed, structured result sets that higher layers (server, cloud) can
deliver, cache, and subscribe to.

## Principles

1. Queries are typed, composable expressions — not strings, not magic, not
   framework conventions.
2. Aspect projection is the default — whole-entity reads are a special case,
   not the norm.
3. A query that works as a one-time read must work as a live subscription
   with no API change.
4. Branch and time-travel are query parameters, not separate APIs.
5. Schema validation happens at query construction time, not at execution
   time.
6. The query model must be constrained enough for the runtime to optimize
   and incrementally maintain.
7. Subgraph scope, lineage traversal, and diff queries are first-class
   expression types, not afterthoughts.
8. Pagination and bounded results are native query features, not
   application-layer workarounds.
9. Tolerance and suppression policies are expressible per query for live
   subscriptions.
10. Queries must not bypass or weaken the truth runtime's semantic contracts.
11. The query layer is the primary consumer-facing API for truth reads —
    direct runtime access is an escape hatch, not the encouraged path.
12. Query results must carry enough metadata for the server to deliver them
    efficiently (aspect masks, entity scopes, tolerance policies, cursor
    positions).

## Foundational Decisions

These are locked architectural decisions:

- queries are Rust types, not strings or DSLs parsed at runtime
- aspect projection is explicit in every query expression
- live promotion is a property of the query execution context, not a
  different query type
- branch and version targeting are parameters on the query context, not
  separate query APIs
- schema validation is compile-time where possible, construction-time
  otherwise
- query planning operates on the truth runtime's knowledge of schema,
  aspects, and storage layout
- result shapes are typed and declared, not arbitrary dynamic maps
- pagination uses opaque cursors, not offset/limit
- subgraph scope uses typed boundary expressions, not arbitrary traversal
  predicates
- diff queries produce structured change sets, not raw deltas
- the query layer does not cache results — caching is a store and server
  responsibility
- queries carry enough metadata for the signal graph to evaluate
  subscription relevance

## Capability Pillars

### Query Expression Architecture

#### Typed composable query expressions

Technical role:
Every query is a typed Rust expression that declares: what entity types to
read, which aspects to project, what filters to apply, what ordering and
pagination to use, and what result shape to produce. Queries compose through
typed combinators, not string concatenation.

What this enables:

- compile-time or construction-time validation against the schema
- IDE autocompletion and type checking for query construction
- query composition without injection risks or parsing ambiguity
- the runtime can inspect query structure for optimization and narrowing

#### Aspect-aware projection

Technical role:
Every query explicitly declares which aspects it needs. The query layer
never reads whole entities by default — it reads the declared aspect
projection and nothing more.

What this enables:

- reads that touch only the aspects the consumer needs
- CDC narrowing scoped to the declared aspects
- storage-aligned reads when the store uses aspect-aware layout
- bandwidth reduction for network delivery of query results
- clear ownership of what data flows to which consumer

#### Filter and predicate expressions

Technical role:
Queries support typed filter predicates over aspect fields. Filters compose
through logical operators and can express equality, comparison, range,
membership, and pattern conditions over typed fields.

What this enables:

- server-side filtering before results reach the consumer
- filtered live subscriptions where only matching changes trigger delivery
- pushdown of filter evaluation to the store when possible
- typed filter composition without stringly-typed predicate building

#### Ordering, pagination, and cursors

Technical role:
Collection queries support typed ordering expressions, cursor-based
pagination, and bounded result sets. Cursors are opaque tokens that encode
position without exposing internal offsets.

What this enables:

- efficient large-collection traversal without unbounded memory
- stable pagination under concurrent mutations (cursor-based, not offset)
- ordered live subscriptions where new results arrive in sort position
- bounded initial load followed by cursor-based advancement

### Live Query Architecture

#### Read-to-subscribe promotion

Technical role:
Any query that can be evaluated as a one-time read must be promotable to a
live subscription by changing the execution context, not the query itself.
The promoted subscription receives incremental patches as truth changes.

What this enables:

- the simplest possible developer experience — write one query, use it for
  both read and subscribe
- no separate "subscription query" API or syntax
- live dashboards, collaborative surfaces, and real-time lists from the
  same query that serves one-time loads
- gradual adoption where developers start with reads and promote to live
  when they need it

#### Incremental result maintenance

Technical role:
Live query subscriptions must receive incremental patches that correspond
to the query's projection, filters, and ordering — not raw CDC events that
the consumer must re-evaluate.

What this enables:

- consumers receive "row 7 changed field X to value Y" not "entity 42
  aspect Foo was modified" — the result is query-shaped, not event-shaped
- collection splices ("inserted at position 3," "removed from position 9")
  for ordered live collections
- filtered subscriptions that suppress changes that do not match the
  filter predicate
- tolerance-aware suppression for aggregation queries where small changes
  below threshold are held

#### Query-to-signal bridging

Technical role:
Live queries are backed by signal graph nodes. The query layer translates
query expressions into signal subscriptions so that the reactive evaluation
infrastructure handles dependency tracking, invalidation, and
recomputation automatically.

What this enables:

- live queries benefit from all signal graph optimizations (suppression,
  coalescing, aspect-scoped invalidation)
- no custom change-tracking code for live query maintenance
- derived computation can depend on query results through the same signal
  graph
- the server can evaluate subscription relevance through the signal graph
  rather than per-client re-evaluation

### Branch and Time-Travel Architecture

#### Branch-scoped queries

Technical role:
Every query context can target a specific branch. If no branch is specified,
the query reads from the current branch head. Branch targeting is a
parameter on the query context, not a different query type or API.

What this enables:

- reading draft branch truth with the same query expressions used for
  main-branch truth
- branch comparison views where multiple query instances target different
  branches
- preview environments served by branch-scoped queries
- AI agent evaluation against speculative branches

#### Time-travel queries

Technical role:
Query contexts can target a specific commit, a specific snapshot, or a
relative time point. Time-travel is a context parameter that applies to
any query expression.

What this enables:

- historical reads with the same query expressions used for current reads
- "what did this entity look like at this commit?" without separate APIs
- audit and compliance reads against specific historical points
- regression analysis by comparing query results across time

#### Diff queries

Technical role:
Diff queries express "what changed between version A and version B" as a
first-class query type. The result is a structured change set shaped by
the query's aspect projection, filters, and scope — not raw storage deltas.

What this enables:

- "what changed in this collection since my last read?" as a query
- branch-to-branch comparison with typed, projected results
- audit reports generated from structured diff queries
- AI agents comparing branches with precise, query-shaped diffs.

### Subgraph and Traversal Architecture

#### Subgraph-scoped queries

Technical role:
Queries can express topological scope: a neighborhood, a dependency cone,
an assembly region, a connected component, or a bounded traversal from a
starting entity. Scope is expressed through typed boundary expressions, not
arbitrary graph traversal predicates.

What this enables:

- geometry kernel reads that load exactly one assembly region
- chip design reads that load exactly one timing cone
- dependency-aware reads that follow relation edges to a bounded depth
- efficient reads that match the store's multi-resolution materialization

#### Relation traversal expressions

Technical role:
Queries can follow typed relations between entities, expressing joins,
adjacent entity reads, and multi-hop traversals through the relation graph.
Traversals are typed by relation kind and bounded by explicit depth.

What this enables:

- loading an entity with its related entities in one query
- following parent-child, containment, or dependency relations
- bounded graph traversal without unbounded fan-out
- query-shaped results that include traversed related entities

### Lineage and Correspondence Architecture

#### Lineage traversal queries

Technical role:
Queries can follow identity evolution: "what did this entity used to be?"
"what replaced this entity?" "what did this entity split into?" Lineage
traversal operates through the lineage graph without exposing internal
lineage event structure.

What this enables:

- identity history for any entity
- tracing entity evolution across schema changes and refactors
- merge tooling that understands what corresponds to what
- audit trails that follow identity through splits and replacements

#### Correspondence queries

Technical role:
Queries can express cross-branch correspondence: "what in branch B
corresponds to this entity in branch A?" Correspondence queries use
lineage and structural fingerprint data to resolve cross-branch identity.

What this enables:

- merge UIs that show corresponding entities across branches
- analysis reuse detection across branches
- cross-branch diff views backed by identity correspondence
- AI agents finding equivalent structures across speculative branches

### Aggregation Architecture

#### Typed aggregation expressions

Technical role:
Queries can express count, sum, min, max, average, and custom aggregations
over projected aspect fields. Aggregations can be grouped by typed
group-by expressions.

What this enables:

- dashboard metrics computed by the query layer, not the consumer
- server-side aggregation that reduces result size
- grouped aggregations for reporting and analytics
- aggregation results that are incrementally maintainable for live
  subscriptions

#### Tolerance-aware aggregation

Technical role:
Aggregation queries for live subscriptions can declare tolerance thresholds.
Changes below the tolerance threshold are suppressed — the consumer does
not receive a delivery until the aggregate changes meaningfully.

What this enables:

- dashboard metrics that do not flicker on trivial changes
- bandwidth-appropriate live aggregation for high-churn sources
- tunable precision per consumer need
- server-side suppression that reduces delivery cost

### Result Shape Architecture

#### Typed result shapes

Technical role:
Every query declares its result shape as a typed structure. Results are not
arbitrary dynamic maps — they are typed projections shaped by the query's
aspect projection, traversal, and aggregation expressions.

What this enables:

- compile-time or construction-time type safety for query consumers
- serialization efficiency for known result shapes
- typed SDK generation from query definitions
- clear contracts between query producers and result consumers

#### CDC-shaped output

Technical role:
Query results can be formatted as CDC-shaped output for integration
consumption: structured change feeds shaped by the query's projection
and filters, suitable for downstream pipeline consumption.

What this enables:

- integration pipelines that consume query-shaped change feeds
- downstream systems that receive structured, filtered, projected changes
- durable subscription-based integrations through the same query model
- clear separation between "query for humans" and "query for pipelines"
  while using the same underlying query expressions

## Domain Fit

### Web Applications

`forge-query` should support:

- typed collection queries with filtering, sorting, and pagination
- detail queries with aspect projection
- live promotion for real-time lists and detail views
- aggregation queries for dashboard metrics
- branch-scoped reads for draft and preview workflows

Revolutionary use:
web developers write one typed query that works for initial load, live
updates, and historical reads — instead of separate REST endpoints, GraphQL
resolvers, WebSocket event handlers, and polling loops.

### Geometry and CAD

`forge-query` should support:

- subgraph-scoped queries for assembly regions and topological neighborhoods
- relation traversal for dependency and containment graphs
- lineage traversal for identity evolution tracking
- branch-scoped reads for design experiments
- diff queries for version comparison

Revolutionary use:
geometry kernels can express "give me this assembly region with its topology
aspects, following containment relations to depth 3, on this branch, and
keep me updated" as a single typed query.

### AI Systems

`forge-query` should support:

- branch-scoped reads for speculative evaluation
- correspondence queries for cross-branch comparison
- lineage traversal for explanation and provenance
- time-travel queries for historical analysis
- diff queries for measuring branch divergence

Revolutionary use:
AI agents can query speculative branches, compare alternatives, trace
decision lineage, and subscribe to truth changes — all through the same
typed query model, instead of cobbling together separate read, diff, and
history APIs.

### Chip Design and Simulation

`forge-query` should support:

- subgraph-scoped queries for timing cones and module neighborhoods
- aggregation queries with tolerance for large netlist metrics
- diff queries for regression analysis
- relation traversal for connectivity graphs
- time-travel queries for analysis at historical points

Revolutionary use:
chip design tools can express "give me the timing cone from this output pin,
with connectivity and timing aspects, diff against last week's version" as
a composable query — instead of writing custom traversal and comparison code.

## Roadmap Direction

This file is a vision document, not the execution roadmap. But the future work
should be derivable from it.

The highest-signal query programs are:

- typed query expression types and composition API
- aspect-aware projection
- filter and predicate expressions
- ordering, pagination, and cursor-based traversal
- live query promotion and incremental result maintenance
- query-to-signal bridging
- branch-scoped and time-travel query contexts
- diff queries
- subgraph-scoped queries and relation traversal
- lineage traversal and correspondence queries
- typed aggregation with tolerance-aware suppression
- typed result shapes
- CDC-shaped output formatting
- schema-aware query validation
- query planning and optimization

If a capability is named here and not yet built, it is roadmap work.

If a capability is built but not yet proven under complex schema, large
graphs, concurrent live subscriptions, and cross-branch scenarios, it is
certification work.

## Non-Goals

- implementing a general-purpose SQL engine
- becoming a GraphQL runtime
- owning truth semantics or transaction semantics
- managing storage layout or compaction
- owning signal evaluation or reactive scheduling
- managing sync protocol delivery or subscription lifecycle
- defining authentication or authorization policy
- replacing the truth runtime's internal read paths for system-level access

## Companion Documents

- [forge_relational_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_vision.md)
- [forge_signals2.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/forge_signals2.md)
- [forge_store_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_vision.md)
- [forge_server_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-server/forge_server_vision.md)
- [forge_cloud_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-cloud/forge_cloud_vision.md)
- [forge_runtime_bridge_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_runtime_bridge_vision.md)

The query layer is where developer experience lives or dies. If queries are
expressive, typed, composable, and natively promotable to live subscriptions,
the entire Forge stack feels like a single coherent product. If developers
must assemble reads, subscriptions, diffs, and traversals from separate APIs,
the architectural coherence below the query layer is invisible and wasted.
