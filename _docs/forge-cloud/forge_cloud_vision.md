# Forge Cloud Vision

## Thesis

`forge-relational` defines truth. `forge-store` makes truth survive.
`forge-server` delivers truth over the network.
Forge Cloud makes truth operable, collaborative, governable, and instantly
usable at internet scale.

Forge Cloud is not "managed hosting for Forge." It is the control plane,
policy plane, sync plane, and operational plane for graph-native truth
systems.

It makes branches, history, live views, assets, replay, policies, retention,
lineage, and resumable collaboration into first-class cloud-native product
capabilities rather than things teams bolt together from a dozen services.

It is the platform where:

- current truth is live by default
- branching is native
- history is durable
- assets are first-class
- replay and audit are built in
- sync is resumable
- retention is explicit
- policy is typed
- operations are artifact-aware
- collaboration is branch- and history-aware
- deployment does not destroy semantic guarantees

This is not "database as a service." It is truth as a managed operating
environment.

## What Forge Cloud Is For

Forge Cloud exists for teams that need more than hosted records.

It is for systems where:

- truth must be live, durable, and replayable
- branching is a product primitive, not a developer hack
- history is valuable, not accidental
- assets belong to truth, not external bucket plumbing
- collaboration must survive disconnection and resume cleanly
- derived views and reactive systems need durable foundations
- debugging, audit, and policy are part of the product, not afterthoughts

It is meant to support:

- web apps with real-time state and simple CRUD that grows naturally into
  drafts, history, and collaboration
- geometry kernels with durable branchable modeling state across sessions
  and machines
- AI systems with persistent speculative workspaces and replayable decision
  trails
- compilers and build systems with branchable IR/state and resumable derived
  artifacts
- workflow editors and node editors with draft branches, approval branches,
  and replayable change history
- simulation and analysis systems with pinned bases, durable checkpoints,
  and artifact-aware collaboration

## Why Forge Cloud Is Different

The technical thesis is clear.

Today, teams stitch together:

- database
- cache
- WebSocket layer
- object store
- event bus
- audit log
- snapshot strategy
- draft/version model
- background jobs
- sync-resume logic
- support/debug export tooling

Forge Cloud collapses those into one coherent truth platform where:

- reads are live by default
- writes are transactional and durable
- current truth is cheap
- history is retained under policy
- branches are native
- assets are attached as first-class refs
- derived artifacts are basis-pinned and governable
- subscriptions are resumable
- snapshots are exportable
- replay is inspectable
- retention and budgets are visible
- diagnostics are policy-aware
- collaboration is built on canonical truth, not client folklore

If these are assembled piecemeal from separate services, every seam between
them becomes a source of inconsistency, partial failure, and accidental
complexity. The value of Forge Cloud is that no seams exist.

## Mission

Forge Cloud exists to make truth:

- immediately usable
- live by default
- branchable without redesign
- durable without ops heroics
- inspectable without custom tooling
- collaborative without sync folklore
- governable without semantic compromise

It must answer these as native cloud responsibilities:

- How does a developer get live current truth with almost no setup?
- How does a team create draft branches and compare them without custom app
  architecture?
- How does history survive naturally without event-sourcing boilerplate?
- How do assets attach securely without bucket plumbing?
- How do clients resume subscriptions after disconnect without app-specific
  glue?
- How do operators inspect commits, branches, snapshots, and assets as
  first-class cloud objects?
- How do replay and export become support and debug primitives?
- How do budgets, retention, and storage classes become visible policy, not
  hidden infrastructure accidents?
- How does collaboration work across devices and sessions without losing
  basis or branch meaning?
- How do deployment, scaling, and storage stay below the semantic line?

## Core Identity: Truth Workspaces

Forge Cloud is a platform made of truth workspaces.

A truth workspace is the cloud unit that owns:

- schema
- current branch/head truth
- branch graph
- snapshots
- durable history
- assets
- subscriber cursors
- retention policy
- usage budgets
- admin policies
- observability surfaces
- export/import capsules
- collaboration/sync sessions

A workspace is the Forge equivalent of a database, a project, a workspace,
and a tenant environment — but deeper and more coherent than any of them
individually.

## Architectural Model

### Runtime stack

| Layer | Responsibility | Owns |
| --- | --- | --- |
| `forge-relational` | Truth semantics | identity, transactions, CDC, lineage, schema, integrity |
| `forge-store` | Durable substrate | commit persistence, snapshots, WAL, recovery, assets, retention, integrity |
| `forge-signal` | Derived computation runtime | dependency DAG, invalidation, recomputation, policy-tiered diagnostics |
| `forge-server` | Sync and transport | subscriptions, resume, delivery, basis negotiation, session semantics |
| Forge Cloud | Managed operating environment | workspaces, tenants, auth, live serving, control plane, policy plane, admin plane, budgets, observability, collaboration |

### Ownership boundary

Forge Cloud owns:

- workspace and tenant lifecycle
- hosted serving of reads, writes, and live queries
- branch and snapshot administration
- asset upload/download session authority
- policy surfaces for retention, budgets, diagnostics, and storage classes
- resumable subscription hosting
- operator and admin tooling
- usage metering and limits
- support, debug, and export surfaces
- collaboration and sync session management
- cloud observability and alerts
- deployment and upgrade envelope

Forge Cloud does not own:

- truth semantics
- lineage semantics
- schema validation logic
- invalidation semantics
- storage backend semantics
- domain meaning of application entities
- client UI state management beyond protocol/runtime surfaces

### Structural rule

Forge Cloud productizes the canonical capabilities of the Forge stack. It
does not become a second truth runtime, a second storage engine, or a second
sync protocol. It makes the stack operable, administrable, and accessible
as a managed platform.

## Principles

1. Forge Cloud serves truth workspaces, not generic databases.
2. Current truth must be cheap and live by default.
3. The common path must stay tiny even though deeper machinery exists
   underneath.
4. Branching, history, assets, and replay are product primitives, not
   app-specific architecture projects.
5. Assets are first-class refs, not bucket URLs.
6. Policies are visible and typed; hidden costs and accidental retention are
   failures.
7. Resumability is a first-class contract across subscriptions, uploads, and
   collaboration sessions.
8. Export, replay, and support are native capabilities, not afterthought
   tooling.
9. The cloud layer must not invent alternate truth semantics.
10. Cloud hosting must increase usability and operability without flattening
    Forge's deeper contracts.

## Foundational Decisions

These are locked architectural decisions:

- workspace is the hosted unit of truth
- current-head current-branch basis is implicit in ordinary reads and writes
- live query is first-class, not bolted on later
- assets use staged capability-based transfer and transactional attach
- branches are native and durable, but optional in the ordinary path
- history exists by default, though retention depth is policy-governed
- export, replay, and snapshot are cloud-visible operations
- policy plane is first-class
- admin and control plane is first-class
- single-region is acceptable for early hosted versions
- Forge Cloud builds on canonical artifacts from relational/store, not
  shadow models
- the cloud product must remain easier than ordinary stacks for simple CRUD

## Capability Pillars

### Truth Workspace Hosting

#### Workspace as the cloud unit

Technical role:
A workspace is the hosted unit of truth. It packages: branchable current
truth, schema, durable history, assets, live queries, snapshots, policies,
and admin controls.

What this enables:

- instant project environments
- isolation by workspace and tenant
- current-truth CRUD by default
- natural upgrade from simple apps to branch-heavy systems
- durable state that survives process and deployment boundaries

### Live Truth Serving

#### Current truth as the default lane

Technical role:
Forge Cloud must make current truth live by default. Ordinary developer usage
should be: define truth, query truth, mutate truth, subscribe to truth —
without hand-assembling cache invalidation, WebSocket events, polling, resume
logic, or read-model synchronization.

What this enables:

- modern web apps that feel simpler than the usual stack
- real-time UI without bespoke infrastructure
- branch-aware live collaboration
- resumable client sessions after disconnect

Hard rule:
The default current-truth access lane must remain cheap, implicit in basis,
durable across reconnect, and free of forced replay/history/branch jargon
in the common path.

### Branch-Native Product Workflows

#### Branches as product primitives

Technical role:
Forge Cloud must expose branches as product primitives, not internal oddities.

What this enables:

- drafts
- staged approvals
- what-if scenarios
- experiments
- offline workspaces
- review branches
- sandbox environments
- branch compare and diff flows

Hard rule:
Branching must be latent by default and frictionless when needed. A simple
CRUD app should not be forced to think about branches every minute. But when
it needs them, no rewrite should be necessary.

### Durable History, Replay, and Supportability

#### History and replay as native operations

Technical role:
Every workspace should already have durable history and inspectable replay
bases.

What this enables:

- "who changed this?" answerable without custom audit code
- timeline inspection
- exact replay for debugging
- support and export capsules
- auditability
- snapshot-based diagnosis

Hard rule:
History must be available by default, though depth and richness may be
policy-bounded. Replay and export should be first-class cloud operations,
not internal emergency tools.

### Generic Asset Platform

#### Assets as first-class truth refs

Technical role:
Forge Cloud must make assets feel like first-class attached truth, not bucket
plumbing.

Assets include: images, PDFs, ZIPs, videos, CSVs, random binary payloads,
engineering artifacts, exported models, and checkpoints.

What this enables:

- staged upload, finalize, and attach flow
- secure capability-based transfer
- content-addressed dedup
- reference-aware lifecycle
- download without direct bucket management
- future derived asset artifacts

Hard rule:
Apps work with `AssetRef`, not object-store URLs.

### Policy Plane

#### Policy as a native managed surface

Technical role:
Forge Cloud should expose policy as a native managed surface. Policies
include: retention, storage budgets, branch limits, diagnostics tiers, asset
classes, max upload sizes, history windows, subscription and session limits,
and snapshot/export controls.

What this enables:

- controlled cost
- tenant safety
- explicit tradeoffs
- enterprise governance
- operational clarity

Hard rule:
Policies may shape retention, richness, and cost. They may not alter truth
semantics.

### Control Plane and Admin Plane

#### Managed operating surface

Technical role:
Forge Cloud needs a real managed operating surface: create workspaces, manage
tenants and projects, inspect branches, inspect commits, inspect snapshots,
inspect assets, inspect live subscriptions, trigger exports, apply retention
or budget changes, and investigate recovery issues.

What this enables:

- product viability
- support workflows
- operator trust
- customer visibility
- managed governance

This is where Forge Cloud stops being "runtime hosting" and becomes an actual
platform.

### Collaboration and Sync Plane

#### Resumable sync and collaborative sessions

Technical role:
Forge Cloud should make resumable sync and collaborative sessions native.

What this enables:

- reconnectable live sessions
- offline resume
- branch-local edits
- durable cursor-based continuation
- multi-device work
- future edge and local sync

Hard rule:
Synchronization is based on canonical truth artifacts and resumable cursors,
not ad hoc client heuristics.

### Derived Artifact and Execution Substrate

#### Basis-pinned derived artifacts as cloud objects

Technical role:
Forge Cloud should allow workspaces to persist basis-pinned derived artifacts
and resumable execution checkpoints. These are not truth by default. They
are derived, basis-pinned, policy-retained, and rebuildable or invalidatable.

What this enables:

- cached queries and materialized views
- analysis outputs
- previews
- search and index fragments
- AI session artifacts
- simulation and build checkpoints

This is one of the areas where Forge Cloud can become much more powerful
than ordinary backend platforms.

### Observability and Verification Plane

#### Cloud-native observability

Technical role:
Forge Cloud must expose: storage health, branch growth, snapshot usage, asset
volume, retention pressure, subscription counts, replay and export activity,
recovery health, and verification/integrity state.

What this enables:

- capacity planning
- support tooling
- trust
- future certification
- cloud operations that understand Forge-native objects

## Domain Fit

### Web Apps

`forge-cloud` should let ordinary web apps start with:

- simple schema
- current-state CRUD
- live queries
- attached assets
- automatic history

and later grow into:

- drafts
- approvals
- collaboration
- support replay
- offline resume

without replacing foundations.

Revolutionary use:
web developers get a single platform that replaces their database, cache,
WebSocket layer, object store, event bus, and audit log — and the simple
CRUD path is actually simpler than the traditional stack.

### Geometry and CAD

Forge Cloud should support:

- branch-local design sessions
- durable modeling history
- asset-backed imports and exports
- exact replay bases
- collaborative or review branches
- snapshot-based analysis handoff

Revolutionary use:
geometry teams can collaborate on design branches in the cloud with durable
history, attached asset management, and exact replay — instead of passing
files around and losing all internal structure.

### AI Systems

Forge Cloud should support:

- persistent speculative branches
- resumable sessions
- durable uploaded knowledge artifacts
- replayable action history
- basis-pinned derived state and indexes

Revolutionary use:
AI agents get a persistent, branchable, auditable cloud workspace with
attached assets and resumable sessions — instead of rebuilding context from
logs every time.

### Workflows and Editors

Forge Cloud should support:

- draft branches
- approval branches
- diff and compare
- attached evidence and assets
- timeline and history inspection
- exact resume after disconnect

Revolutionary use:
workflow and editor tools get branch-aware drafts, approval flows, and
history inspection as platform primitives — instead of building custom
versioning and approval systems from scratch.

## Roadmap Direction

This file is a vision document, not the execution roadmap. But the future work
should be derivable from it.

The highest-signal cloud programs are:

- truth workspace hosting and tenant lifecycle
- live truth serving with implicit current-branch basis
- branch-native product workflows
- durable history, replay, and export capabilities
- generic asset platform with staged capability-based transfer
- policy plane with retention, budgets, and storage classes
- control plane and admin plane with workspace administration
- collaboration and sync plane with resumable sessions
- derived artifact and execution substrate
- observability and verification plane
- authentication integration and subscription-scoped authorization
- deployment and upgrade automation

If a capability is named here and not yet built, it is roadmap work.

If a capability is built but not yet proven under multi-tenant load,
workspace lifecycle, policy enforcement, and collaboration scenarios, it is
certification work.

## Non-Goals

- becoming a second truth runtime (that is `forge-relational`)
- owning storage backend semantics (that is `forge-store`)
- owning sync protocol delivery logic (that is `forge-server`)
- inventing domain-specific application logic
- managing client UI state beyond protocol and runtime surfaces
- requiring multi-region for initial viability
- replacing existing cloud infrastructure for non-Forge workloads

## Companion Documents

- [forge_relational_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-relational/forge_relational_vision.md)
- [forge_signals2.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge_signal/forge_signals2.md)
- [forge_runtime_bridge_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/engineering/forge_runtime_bridge_vision.md)
- [forge_store_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-store/forge_store_vision.md)
- [forge_server_vision.md](file:///c:/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/forge-server/forge_server_vision.md)

Forge Cloud is where the entire stack becomes a product. Without it, Forge
is a powerful set of libraries that teams must self-host and self-operate.
With it, Forge is a platform where truth is live, branchable, durable,
collaborative, and governable by default — and the simple path is simpler
than any existing alternative.
