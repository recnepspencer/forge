# forge-signal-wasm Router And Navigation Projection Plan

> **Status:** Planned engineering spec
>
> **Roadmap parent:** [wasm_product_roadmap.md](./wasm_product_roadmap.md)
>
> **Web runtime parent:** [web_runtime_spec.md](./web_runtime_spec.md)
>
> **Composition prerequisites:**
>
> - [composition-api-plan.md](./composition-api-plan.md)
> - [controller_scope_and_graph_lifecycle_plan.md](./controller_scope_and_graph_lifecycle_plan.md)
> - [opaque_identity_and_ergonomic_authoring_plan.md](./opaque_identity_and_ergonomic_authoring_plan.md)
>
> **API-surface prerequisite:**
>
> - [api_surface_plan.md](./api_surface_plan.md)
> - [api_surface_closeout.md](./api_surface_closeout.md)
>
> **Core vision:** [_docs/forge_signal/forge_signal_vision.md](../../../_docs/forge_signal/forge_signal_vision.md)
>
> **Core test requirements:** [_docs/forge_signal/test-requirements.md](../../../_docs/forge_signal/test-requirements.md)

## Goal

Build a graph-native router and navigation surface in `forge-signal-wasm` that
can replace framework-router-shaped usage without inventing a second
navigation, guard, loader, or browser-history truth engine.

The target product shape is:

- URL state is graph-owned input, not ambient window folklore
- route matching is derived truth, not framework-local orchestration state
- route params, search params, hash fragments, and canonical URL equivalence
  are typed and normalized
- route-local prerequisites, redirects, and not-found/forbidden/unavailable
  outcomes are explicit typed route results
- speculative branch navigation can admit, redirect, or discard candidate
  navigations before visible truth flickers
- route-local resources consume the completed resource/API surface rather than
  recreating loaders, resolvers, and cache folklore
- nested layouts and outlets are graph-projected route composition, not
  framework glue
- reversible navigation can restore admitted route/outlet composition and the
  route-scoped graph truth the snapshot boundary honestly owns
- diagnostics, history, replay, restore, and breadcrumb/back semantics are
  real product surfaces rather than debug-only implementation accidents

This milestone is not complete if it only wraps `window.history` and path
matching behind a prettier TypeScript facade.

## Why This Milestone Exists

The completed wasm product line now has enough substrate to support a much
better router than the standard frontend stack:

- `createSignals()` is already the app-first product surface
- controller-first composition and graph publication are real
- graph-owned lifecycle, branch, replay, and restore are real
- host capability exists for explicit browser-derived facts
- resource lines already own request posture, continuity, diagnostics,
  delivery, and branch/restore truth

Without a router milestone, application code still faces the old split:

- one engine owns URL and navigation state
- another engine owns resources and loaders
- another layer owns auth/permission guards
- the UI framework owns outlet composition and route-local continuity
- browser history sits beside them as a loosely synchronized imperative system

That split recreates exactly the kind of convenience-era drift this roadmap has
been removing:

- route guards become imperative middleware instead of declared graph truth
- route loaders become a second resource lifecycle story
- tenant and workspace switching produce orphaned local state
- breadcrumbs and back buttons restore strings instead of admitted route truth
- search-param-heavy enterprise screens fall back to ad hoc string plumbing
- diagnostics explain resources and graph state, but not the navigation story

This milestone exists to prevent that collapse and make routing a consumer of
the completed graph/resource substrate rather than a rival state machine.

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects here is starting from the hostile
  navigation failure mode instead of from path-matching ergonomics. The spec
  therefore starts from deep tenant switching, speculative admission, stale
  deep links, and restore parity rather than from "make `navigate(...)` nice."
- `arch_laws.md`
  The most important laws here are 2, 7, 20, 24, 33, 34, 40, and 41. Route
  prerequisites must declare what they consume, browser-history boundaries must
  surface explicit envelopes, rejection must happen before route-local work is
  treated as admitted, and route, outlet, breadcrumb, redirect, and restore
  meanings must remain distinct proof-bearing categories.
- `perf_laws.md`
  The most important thing it protects is breadth honesty. Cheap-looking route
  transitions must not hide broad route-tree rescans, broad loader/cache work,
  repeated URL reparsing, or rich-history reconstruction on the hot path.
- `domain_laws.md`
  The most important thing it protects is responsibility shape. URL authority,
  route schema, admission, browser-history integration, outlet composition,
  reversible navigation, and diagnostics must have distinct homes instead of
  collapsing into one giant router helper module.
- `forge_signal_vision.md`
  The most important thing it protects is that `forge-signal` remains derived
  execution substrate rather than truth storage. The router must consume graph
  truth and snapshots; it must not become a new authority store or a framework
  orchestration engine in disguise.
- `web_runtime_spec.md`
  The most important thing it protects is the framework-agnostic wasm product
  thesis. The router must be a real wasm product surface that React, Angular,
  Vue, and plain TypeScript can consume, not a React-first convenience layer.
- `wasm_product_roadmap.md`
  The most important thing it protects is sequencing. Router work belongs after
  the API surface because it must consume completed resource/history/branch
  truth instead of inventing loaders, guards, and continuity semantics ad hoc.
- `api_surface_plan.md`
  The most important thing it protects is that request posture, continuity,
  redirects, diagnostics, replay, and restore are already solved at the
  resource line boundary. The router must consume those capabilities rather
  than reinterpret them as route-local middleware.
- `api_surface_closeout.md`
  The most important thing it protects is that the resource line model is now
  closed and trustworthy. Route-local resource transitions, prefetch, and
  stale-deep-link recovery should lower through that line model rather than
  through a second client cache.
- `composition-api-plan.md`
  The most important thing it protects is controller-first graph publication.
  Route-local feature graphs, layouts, and outlets must compose through
  published graph handles instead of inventing a private router composition
  model.
- `host_callback_computed_spec.md`
  The most important thing it protects is callback-first app authoring without
  a second reactive engine. Router projections, prerequisites, and route-local
  controllers must remain callback-authored consumers of runtime truth.
- `_docs/forge_signal/test-requirements.md`
  The most important thing it protects is certification rigor. The router must
  close through hostile branch/replay/restore/navigation proof, not through
  nominal path examples.

## Adversarial Constraint

This milestone must survive the following hostile condition:

> A long-lived web application with nested layouts, multiple outlets,
> parameterized routes, typed search state, resource-backed route transitions,
> auth and policy prerequisites, speculative navigations, branch restore,
> reversible back/breadcrumb flows, tenant or workspace switching, stale deep
> links, and direct browser-history edits must converge to the same admitted
> route truth, the same visible outlet composition, the same route-local
> resource continuity truth, and the same diagnostics/history artifacts
> regardless of whether the navigation was driven by typed app navigation,
> direct URL mutation, browser back/forward, speculative branch evaluation,
> route-local redirect, restore, or replay.

If semantically equivalent navigation histories can produce:

- different admitted route identity for the same canonical URL truth
- different visible route or outlet composition after equivalent redirects
- partial cross-tenant state after deep workspace/project switching
- different breadcrumb/back outcomes depending on whether history was browser-
  driven or graph-driven
- loader/resolver/guard semantics that disagree with resource lifecycle truth
- or a second navigation/cache authority beside the runtime and browser-history
  boundary

then the milestone has failed.

## Product Decision Lock

- the router is a first-class wasm product surface, not framework glue
- URL truth is graph-owned and explicitly admitted through a browser-history
  boundary rather than being ambient runtime context
- route matching, param extraction, search-param normalization, hash handling,
  and route projection are derived graph truth
- route schema and route identity are typed and canonicalized; ad hoc string
  path concatenation is not the primary public story
- navigation actions are typed intent classes such as push, replace, soft
  refresh, same-route param mutation, and canonicalization redirect
- route outcomes are typed and explicit:
  - admitted
  - redirect
  - notFound
  - forbidden
  - unavailable
  - denied
- route prerequisites are declared graph consumers, not imperative middleware
- access-policy composition may consume auth, tenant capability, feature flags,
  licensing, environment posture, maintenance posture, and regional posture
  through existing graph/resource/host-capability truth
- speculative branch navigation is a first-class product capability, not an
  optional optimization
- nearest-valid-truth recovery for stale deep links, deleted entities, and
  disabled modules is part of the product contract
- route-local continuity and prefetch posture must lower through completed
  resource line semantics rather than inventing a second loader cache or route
  pending grammar
- nested layouts and outlets are route-scoped graph composition artifacts, not
  framework-owned slot glue
- reversible navigation is a first-class strength of the router:
  - back and breadcrumb return restore admitted route/outlet composition through
    graph restore/snapshot truth
  - the router must promise an explicit restore boundary rather than hand-wave
    "full app state"
- semantic draft preservation/discard across authority changes is part of the
  contract; the router must not preserve or discard route-local drafts by
  accidental component survival
- canonical route equivalence is a real product law for duplicate URL forms,
  canonicalization redirects, analytics correctness, and redirect-loop
  prevention
- URL writeback from graph state must happen only through explicit declared
  projection contracts
- SSR and hydration posture must be reserved honestly even if full SSR product
  work is later
- cross-tab and external navigation coherence must be stated explicitly at the
  authority boundary, even if the first implementation scope focuses on one tab

Normative consequence:

- any implementation that treats browser location as an ambient read instead of
  an admitted host boundary is out of spec
- any implementation that rebuilds guard/loader semantics as framework-local
  middleware or cache glue is out of spec
- any implementation that restores only raw URL strings while claiming route
  restore semantics is out of spec
- any implementation that treats search params and hash fragments as ad hoc
  string bags instead of typed route state is out of spec
- any implementation that preserves route-local drafts across tenant or entity
  authority changes without declared policy is out of spec
- any implementation that hides stale deep-link recovery or canonicalization as
  undocumented framework behavior is out of spec

## Architectural Model

### Ownership split

This milestone freezes the intended ownership boundary:

1. **`forge-signal`**
   - owns branch, replay, restore, snapshot, and diagnostics truth
   - owns derived evaluation and transaction semantics
2. **browser host boundary**
   - owns raw `location`, `history`, `popstate`, and direct URL mutation
   - does not own typed route meaning
3. **`forge-signal-wasm` router surface**
   - owns typed route schema authoring, URL normalization, route projection,
     navigation intent lowering, browser-history integration, and route-facing
     diagnostics/history surfaces
   - hosts route truth inside the same graph/runtime substrate rather than a
     second router state machine
4. **resource and forms products**
   - remain the owners of resource lifecycle and draft/product semantics
   - may be consumed by route-local controllers and route transitions
   - must not be semantically redefined inside the router

The router is therefore not:

- a framework-owned orchestrator
- a browser-history wrapper with added type sugar
- a second loader cache
- a second guard engine
- a stringly breadcrumb generator

It is the local product layer that turns URL and navigation into graph-native
route truth.

### URL authority model

The router should organize URL truth into explicit typed categories rather than
one raw string:

1. **Raw location authority**
   - browser-provided pathname, search, hash, and navigation type
2. **Canonical URL projection**
   - normalized path segments
   - normalized search params
   - normalized hash posture
   - canonical equivalence digest
3. **Route match projection**
   - matched route identity
   - typed params
   - typed search state
   - typed fragment/hash state
4. **Admitted route truth**
   - route outcome after prerequisites, redirect posture, and nearest-valid
     recovery have been resolved

The normalization contract must not collapse to path strings too early. The
router needs typed canonical artifacts to justify:

- canonical redirect detection
- same-route param mutation
- duplicate URL equivalence
- cache and prefetch coherence
- replay and restore parity

### Route composition model

Route matching alone is not enough. The router must project:

- route shell
- nested layout stack
- outlet composition
- route-local controller graph handles
- route capability boundary for child routes and outlet consumers

The intended product direction is:

- layouts are first-class route composition units
- outlets are explicit route-scoped projection slots
- route-local feature graphs publish through the existing composition API
- route capability handles prove which params, prerequisites, and route-local
  resources are admitted before downstream code can use them

This is where ordinary routers usually collapse back into framework-local
component trees. This milestone must not.

### Admission and prerequisite model

The router must treat route admission as a declared graph-evaluated boundary.

Prerequisite sources may include:

- auth/resource truth
- tenant capability
- feature flags
- licensing posture
- environment posture
- maintenance windows
- regional or deployment constraints
- route-local resource existence checks

Prerequisite resolution must structurally precede admitted route construction.

The route product should therefore lower through explicit categories such as:

- route intent
- validated route candidate
- prerequisite evaluation plan
- admitted route outcome
- redirect or denial artifact

This keeps route-local work, layouts, resources, and drafts from being treated
as admitted before the prerequisite story has actually been resolved.

### Navigation model

The router must distinguish navigation intent from navigation outcome.

Required intent classes:

- `push`
- `replace`
- `softRefresh`
- `sameRouteMutation`
- `canonicalize`
- `restoreBack`
- `breadcrumbReturn`

Required policy dimensions:

- speculative branch or direct commit
- continuity posture while pending
- redirect-on-failure or hard-denial
- prefetch/warmup posture
- URL writeback posture

This allows navigation semantics to stay explicit instead of hiding multiple
history and continuity behaviors behind one `navigate(...)` helper.

### Reversible navigation model

The router must treat reversible navigation as a product capability built on
real snapshot and restore semantics.

That means:

- back and breadcrumb return are not just prior URL lookup
- route history entries may carry route-composition restore truth
- multi-outlet and nested-layout pages can restore the previously admitted
  route shell and outlet composition
- route restore boundaries are explicit about what is guaranteed:
  route truth, outlet composition, and graph-owned state within the admitted
  restore boundary

The router must not promise arbitrary non-graph local UI restoration by
accident. But it also must not under-sell the snapshot substrate by pretending
that route restore is only string navigation.

## Phases

### Phase 1: URL Authority, Canonicalization, And Route Schema Lock

Purpose:

- make URL truth and route-schema truth explicit typed product categories
- prevent the milestone from collapsing into string path helpers

This phase must ship:

- typed URL state vocabulary for pathname, search params, and hash fragments
- canonical normalization and route-equivalence contracts
- route-schema authoring with typed params and search state
- explicit route identity and navigation-intent vocabulary
- compile-time boundaries that keep raw URL strings distinct from canonical
  route artifacts where the product contract requires it

Phase 1 gate:

- no later phase begins until canonical path/search/hash equivalence is frozen
  strongly enough that redirect loops, duplicate URLs, and route identity drift
  are structurally prevented

### Phase 2: Route Projection, Layouts, And Outlet Composition

Purpose:

- turn route matching into real route composition truth
- prevent layouts and outlets from falling back into framework glue

This phase must ship:

- route projection from canonical URL truth into matched route identity
- nested layout and outlet composition vocabulary
- route-scoped controller/publication integration through the existing
  composition API
- typed route capability boundaries for admitted params, outlet contracts, and
  route-local APIs
- typed route outcomes for admitted, redirect, notFound, forbidden,
  unavailable, and denied paths

Phase 2 gate:

- no later phase begins until one route can project one explicit layout/outlet
  composition story without requiring framework-local orchestration state

### Phase 3: Admission, Access Policy, And Nearest-Valid Recovery

Purpose:

- make prerequisites, denial, redirect, and stale-link recovery real product
  semantics instead of middleware folklore

This phase must ship:

- declaration-driven prerequisite posture
- layered access-policy composition over graph/resource/host-capability truth
- explicit denial/redirect/not-found/unavailable artifacts
- nearest-valid-truth recovery for stale deep links, deleted entities, disabled
  modules, and tenant/workspace/project switching
- typed diagnostics for why admission succeeded, denied, redirected, or fell
  back to a nearest valid route

Phase 3 gate:

- no later phase begins until route-local work is structurally impossible to
  treat as admitted before prerequisite resolution finishes

### Phase 4: Browser History Integration, Typed Navigation, And Transition Policy

Purpose:

- make browser-history input and app-issued navigation one coherent product
  boundary
- freeze declared transition semantics instead of per-screen improvisation

This phase must ship:

- typed navigation intents for push, replace, soft refresh, same-route
  mutation, canonicalize, breadcrumb return, and restore-back
- explicit transition policy for speculative branch, direct commit, continuity,
  and redirect handling
- typed browser-history integration for push, replace, popstate, manual URL
  edits, and external navigation entry
- URL writeback contracts from graph state to browser URL
- route-local breadcrumb/back provenance built from route history truth rather
  than path slicing

Phase 4 gate:

- no later phase begins until browser-driven and app-driven navigations can be
  shown to converge on one route truth and one history story

### Phase 5: Speculative Navigation, Resource-Native Transitions, And Prefetch

Purpose:

- turn the router into a consumer of the completed resource substrate instead
  of a second loader/cache engine

This phase must ship:

- branch-native speculative navigation
- route-local resource admission and continuity through the completed resource
  API surface
- route-local prefetch and warmup posture driven by hover, focus, viewport, or
  explicit intent without inventing a second loader cache
- transition semantics that can preserve visible truth while pending when the
  declared continuity/resource policy allows it
- canonical diagnostics for whether a visible route change came from direct
  navigation, speculative branch commit, redirect, prefetch admission, or
  resource continuity preservation

Phase 5 gate:

- no later phase begins until route transitions can consume resource continuity
  and prefetch posture without inventing framework-local resolver grammar

### Phase 6: Reversible Navigation, Draft Preservation, And Restore Parity

Purpose:

- expose the snapshot/restore substrate as a real navigation strength instead
  of treating browser back as the only history story

This phase must ship:

- route-history entries that preserve admitted route/outlet composition truth
- reversible navigation through restore-backed back and breadcrumb return
- semantic draft preservation/discard policy across route and authority changes
- typed restore-boundary truth for what route-local graph state is guaranteed to
  restore
- parity between direct restore-backed navigation and equivalent replay/history
  inspection artifacts

Phase 6 gate:

- no later phase begins until multi-outlet and nested-layout navigation can be
  restored honestly without overclaiming arbitrary non-graph local UI state

### Phase 7: SSR/Hydration Boundary, Cross-Tab Coherence, And Certification Closeout

Purpose:

- reserve the larger deployment boundaries honestly and close the milestone with
  hostile proof rather than examples

This phase must ship:

- explicit SSR and hydration posture for initial URL admission and route truth
  handoff
- explicit cross-tab and external-navigation coherence contract at the browser
  authority boundary
- canonical verification-package vocabulary for route truth, route outcomes,
  outlet composition, navigation provenance, and restore parity
- docs that teach the router as a graph-native product lane rather than a
  framework-specific helper

Phase 7 gate:

- the milestone is not closed until speculative branch navigation, stale-link
  recovery, resource-native transitions, reversible navigation, and browser
  history all converge under hostile replay/restore proof

## Must Ship

- typed URL, search-param, and hash-fragment route state
- canonical route equivalence and normalization contracts
- typed route schema authoring and typed navigation builders
- route-scoped layout and outlet composition
- typed route capability boundaries
- declaration-driven prerequisites and access-policy composition
- explicit typed route outcomes for admitted, redirect, notFound, forbidden,
  unavailable, and denied
- speculative branch navigation
- nearest-valid-truth recovery for stale deep links and invalid tenant/project/
  module targets
- typed navigation intent classes and transition policy
- browser-history integration and URL writeback contracts
- resource-native route transitions and route-local prefetch/warmup posture
- semantic draft preservation/discard across authority changes
- reversible navigation backed by route restore truth
- diagnostics/history/replay/restore surfaces for navigation and route outcomes
- SSR/hydration and cross-tab/external-navigation posture stated honestly at
  the product boundary

## Must Preserve

- `forge-signal` remains the owner of branch, replay, restore, and diagnostics
  truth
- the browser host remains the owner of raw URL/history mutation, but not of
  typed route meaning
- resources remain the owner of request, continuity, freshness, delivery, and
  diagnostics semantics
- forms remain the owner of draft semantics outside the explicit route-side
  preservation/discard contract
- the router does not become a second cache, loader engine, guard engine, or
  authority store
- route ergonomics must not hide broad tree rescans, broad resource work, or
  rich-history reconstruction behind cheap-looking APIs

## Required Named Proof Families

- `The Canonical Route Equivalence Test`
- `The Typed Search And Hash Normalization Test`
- `The Route Projection And Layout Composition Test`
- `The Route Capability Boundary Compile-Time Test`
- `The Prerequisite And Access Policy Admission Test`
- `The Nearest Valid Recovery And Stale Deep Link Test`
- `The Browser History And App Navigation Convergence Test`
- `The Speculative Branch Navigation And Flicker Suppression Test`
- `The Resource Native Route Transition And Continuity Test`
- `The Route Prefetch Without Second Loader Cache Test`
- `The Semantic Draft Preservation Across Authority Change Test`
- `The Reversible Outlet Composition Restore Test`
- `The Breadcrumb And Back Provenance Test`
- `The Navigation Diagnostics And Auditability Test`
- `The SSR Hydration And External Navigation Boundary Test`

## Acceptance Evidence

This milestone is complete only when the wasm product surface can prove:

- canonical path/search/hash normalization yields one route identity story for
  semantically equivalent URLs
- direct URL edits, typed app navigation, browser back/forward, and external
  navigation entry converge to the same admitted route truth
- nested layout and outlet composition are projected from route truth rather
  than framework-local orchestration state
- prerequisite denial, redirect, and nearest-valid recovery happen before
  route-local resources, drafts, or layout truth are treated as admitted
- resource-backed route transitions consume existing continuity, freshness, and
  diagnostics truth instead of inventing resolver/loader folklore
- speculative navigation avoids half-committed route state and yields explicit
  diagnostics about why a candidate navigation committed, redirected, or
  discarded
- deep tenant/workspace/project switching either restores or converges to one
  nearest valid route truth without orphaned cross-authority state
- back, breadcrumb return, and reversible navigation restore admitted
  route/outlet composition through explicit restore-boundary truth
- diagnostics can explain why the current route is visible, which prerequisite
  or redirect shaped it, which navigation intent produced it, and which route
  history or restore boundary currently explains it
- SSR/hydration and cross-tab/external-navigation boundaries are stated
  honestly enough that later work can extend them without semantic drift

## Architectural Notes

- the strongest shape is route-family-first rather than component-instance-first
  so canonical identity, matching, and equivalence stay centralized
- route-local resources should be published graph consumers of route params and
  route search state, not hidden inside loader callbacks
- layout and outlet projection should reuse the existing controller and graph
  publication substrate instead of defining a parallel router composition model
- reversible navigation should prefer explicit restore-boundary artifacts over
  vague promises about "restoring page state"
- if a route transition needs richer state than URL alone to be explainable,
  that state should become a named route artifact rather than hidden framework
  memory

## Sequencing Notes

This milestone belongs after the API surface because:

- route transitions, prefetch, redirects, and continuity need the completed
  resource request/lifecycle/diagnostics substrate
- speculative navigation and reversible back/breadcrumb flows are much stronger
  once branch, restore, and route-local resource truth are already real

This milestone remains separate from forms because:

- forms should consume route authority changes and route-local draft
  preservation rules where appropriate
- the router should not become the first place where general draft semantics
  are solved

Current judgment:

- write the router as a graph-native navigation product, not a browser-history
  helper
- make stale-link recovery, speculative branch navigation, and reversible
  route/outlet restore first-class strengths
- preserve headroom for more ambitious constraint-solved navigation later
  without making it part of the initial milestone contract
