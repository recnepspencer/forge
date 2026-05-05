# forge-signal-wasm API Surface DX Hardening Plan

> **Status:** Planned engineering spec
>
> **Roadmap parent:** [wasm_product_roadmap.md](./wasm_product_roadmap.md)
>
> **Prerequisite milestone:** [api_surface_closeout.md](./api_surface_closeout.md)
>
> **Adjacent milestone:** [router_navigation_projection_plan.md](./router_navigation_projection_plan.md)
>
> **Core vision:** [_docs/forge_signal/forge_signal_vision.md](../../../_docs/forge_signal/forge_signal_vision.md)
>
> **Resource certification parent:** [test-requirements.md](./test-requirements.md)

## Goal

Harden the shipped `forge-signal-wasm` API surface so normal application
authoring becomes materially easier, clearer, and less repetitive without
changing the already-closed runtime semantics underneath it.

This milestone is not about inventing a new async engine, a generic fetch
helper, or a Laravel-style magical resource abstraction.

It is about making the existing resource line model pleasant enough that a
serious frontend team would reach for it first instead of feeling forced back
to TanStack Query plus ad hoc API-client glue.

The target product shape is:

- shared API defaults such as base URL, auth, and headers are declared once
- request defaults can be layered at app, section, and endpoint scope
- common declarations start with one obvious route entrypoint: `url(...)`
- declaration-site types carry the main semantic intent for the common lane
- explicit ids and explicit route intent remain visible in code
- path parameters stay readable and explicit
- request-side URL parameters use `params`, not `query`
- signed upload, multipart upload, deferred processing, and similar advanced
  paths stay inside the same authoring grammar as common reads
- the low-level family declaration surface remains available as the escape
  hatch rather than the default experience

## Why This Milestone Exists

Milestone 6 closed the semantics and proof burden for the resource/API line.
That was the correct first job.

But the current public authoring surface still looks too much like stabilized
substrate and not enough like an application-facing product.

Today the common lane still forces developers to repeat or over-think:

- `resourceParams(...)`
- `normalizeParams(...)`
- `resourceParamIdentity(...)`
- per-resource auth and header setup
- raw route construction inside `load(...)`
- large declaration objects even for ordinary CRUD-shaped reads

That means the current surface is semantically strong but ergonomically weak.

The package has already done this kind of hardening successfully once for the
main signal surface:

- semantics stayed strict
- authoring became much nicer
- the explicit/spec lane stayed available

The API surface now needs the same treatment.

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects here is solving the hostile DX failure
  first, not just shortening examples. The spec therefore starts from large-app
  authoring pressure, repetition, and adversarial nonstandard endpoints rather
  than from cosmetic builder sugar.
- `arch_laws.md`
  The most important laws here are 2, 7, 20, 33, 40, and 41. The new DX layer
  must preserve the existing authority split, keep route/request/builder
  semantics self-describing, and encode proof-bearing distinctions instead of
  flattening them into convenience blobs.
- `perf_laws.md`
  The most important thing it protects is cost honesty. Builder-shaped APIs
  must not hide broad invalidation, broad refresh, repeated reparsing, or a
  second request-planning engine behind cheap-looking calls.
- `domain_laws.md`
  The most important thing it protects is subsystem clarity. Shared API
  defaults, declaration-site route authoring, request parameter serialization,
  upload posture, and raw-family escape hatches need distinct homes instead of
  one mega-builder.
- `forge_signal_vision.md`
  The most important thing it protects is that `forge-signal` remains derived
  execution substrate. This milestone may improve authoring, but it must still
  lower to the same runtime-owned line, lifecycle, and diagnostics truth.
- `wasm_product_roadmap.md`
  The most important thing it protects is sequencing. DX hardening belongs
  after API-surface semantic closeout and before the router, because the router
  should consume a humane API lane instead of compensating for resource
  declaration ceremony.
- `api_surface_plan.md`
  The most important thing it protects is the line model and semantics already
  closed. This milestone must not reopen lifecycle, reconciliation, delivery,
  branch, replay, or compatibility semantics just to make declarations shorter.
- `api_surface_closeout.md`
  The most important thing it protects is closure honesty. The API resource
  milestone is closed semantically; this work is a new product hardening step,
  not an admission that the original closeout was fake.
- `router_navigation_projection_plan.md`
  The most important thing it protects is downstream consumption. The router
  needs a nice API surface for route-local resources and request defaults; it
  should not become the place where API authoring ergonomics are fixed by
  router-specific sugar.
- `forge-signal-wasm/test-requirements.md`
  The most important thing it protects is proof quality. The new authoring lane
  must certify raw-vs-builder equivalence, inheritance honesty, and hostile
  nonstandard endpoint behavior instead of only shipping pleasant docs.

## Adversarial Constraint

This milestone must survive the following hostile condition:

> A large TypeScript application with shared auth and header posture, explicit
> named endpoints, conventional detail/list/paged reads, nested tenant or
> workspace routes, custom action endpoints, signed uploads, multipart uploads,
> deferred processing, pushed delivery, and a small number of ugly
> non-resourceful backend routes must converge to the same family identity, the
> same line lifecycle truth, the same diagnostics/history truth, and the same
> request-shaping truth regardless of whether the resource was authored through
> the new DX `url(...)` lane or the existing raw family declaration surface.

If the DX lane can produce:

- different family or line identity from the raw lane
- different request, freshness, upload, delivery, or restore semantics
- hidden broad work behind cheap-looking builder calls
- repeated per-endpoint header/auth duplication because inheritance is weak
- a pleasant CRUD lane that collapses when one endpoint becomes nonstandard
- or a second route/request planning abstraction that later fights the router

then the milestone has failed.

## Product Decision Lock

- keep explicit endpoint and declaration names visible in application code
- do not add a magical `resource("products")` abstraction as the default lane
- prefer one small route declaration grammar over giant declaration objects
- no `.build()` stage is part of the default lane
- default read/write intent should be expressible from declaration-site types
  plus `url(...)` without redundant verb or shape restatement
- prefer `list` over `index`
- prefer `params` over `query` for request-side URL parameter vocabulary
- preserve explicit detail, list, paged, create, update, and remove
  distinctions
- shared API defaults such as base URL, headers, and auth should be declared
  once and inherited automatically
- nested API scopes should be able to add section-specific headers, auth, base
  URL fragments, and request conventions once and have them lower
  automatically
- the DX lane must degrade one step at a time for weird endpoints rather than
  forcing an immediate fall through to raw `load(...)`
- signed upload, multipart upload, deferred processing, and related advanced
  cases must remain inside the same builder grammar as ordinary reads
- the raw family declaration lane remains the escape hatch and semantic anchor
- the DX lane must lower to the exact same closed resource family and line
  model already certified under Milestone 6

Normative consequence:

- any DX surface that changes runtime semantics relative to the closed raw lane
  is out of spec
- any DX surface that hides costful or broad work behind deceptively cheap API
  shape is out of spec
- any DX surface that makes conventional CRUD pretty but adversarial custom
  endpoints miserable is out of spec
- any DX surface that replaces explicit names and route intent with opaque
  resource magic is out of spec

## Architectural Model

### Ownership split

This milestone freezes the intended ownership boundary:

1. **raw resource family surface**
   - remains the semantic authority at the wasm product boundary
   - continues to describe canonical family, line, lifecycle, reconciliation,
     delivery, download, replay, and restore behavior
2. **API DX declaration surface**
   - owns pleasant authoring, shared defaults, scoped inheritance, path
     interpolation, and common request vocabulary
   - lowers into the raw resource family surface
   - does not define independent lifecycle or cache semantics
3. **future router surface**
   - consumes the DX surface and the raw line model
   - does not own API authoring ergonomics

The DX layer is therefore not:

- a second resource runtime
- a second route system
- a generic HTTP client detached from line semantics
- a replacement for the raw lane's semantic authority

### Authoring model

The intended ergonomic direction is declaration-site semantic typing plus one
explicit route entrypoint.

Representative shape:

```ts
const getUser: Detail<User> = url("/users/:userId");

const getUsers: List<User> = url("/users")
  .params<{ search?: string; page?: number }>();

const createUser: Create<User> = url("/users");

const updateUser: Update<User> = url("/users/:userId");

const uploadReceipt: Create<PreparedUpload> = url("/receipts/upload")
  .signedUpload({ finalize: true })
  .processing("poll");
```

Important constraints:

- the declaration-site type expresses the common semantic intent
- `url(...)` remains explicit and grepable
- shared defaults are inherited unless overridden
- weird endpoints stay in the same grammar by adding small explicit modifiers
  instead of switching to a second declaration API
- the result of the chain is already the usable family

### Parameter model

The API must distinguish:

1. **path params**
   - values consumed by placeholders in `url("/products/:productId")`
   - should be inferred where possible
2. **request params**
   - extra serialized URL parameters for list/search/filter/paging flows
   - should use `params`, not `query`
3. **body payload**
   - explicit for non-GET-style operations when the endpoint takes a request
     body

The authoring goal is:

- infer path params from the URL where possible
- treat `Detail<T>` as identity-bearing by default instead of requiring
  repetitive trivial id restatement
- keep request params explicit and named as `params`
- do not force developers to re-declare trivial identity by hand for the 80%
  case

### Shared-default model

The DX declaration lane must support one explicit API root for shared request
posture:

- base URL
- shared headers
- shared auth
- shared serializers or request conventions
- shared inheritance rules

The intention is that most applications declare headers once and reuse them for
nearly every endpoint, with only explicit local overrides where needed.

The scoped inheritance model must also support intermediate API sections so
large apps can define shared request posture once for one part of the product
without contaminating unrelated routes.

Representative shape:

```ts
const api = signals.api({
  baseUrl: "/api",
  headers: () => ({
    Authorization: `Bearer ${session.token()}`,
  }),
});

const tenantApi = api.scope({
  headers: ({ tenantId }) => ({
    "x-tenant-id": tenantId,
  }),
});

const adminTenantApi = tenantApi.scope({
  headers: () => ({
    "x-admin-area": "true",
  }),
});

const getUser: Detail<User> = tenantApi.url("/users/:userId");

const exportUsers: Create<ExportJob> = adminTenantApi
  .url("/users/export")
  .headers(() => ({
    "x-export-mode": "full",
  }));
```

Required merge semantics:

- app-root defaults apply first
- nested API scopes apply in lexical order
- endpoint-local overrides apply last
- collisions resolve deterministically
- diagnostics and request inspection must reveal what was inherited versus what
  was overridden

### Advanced-path model

The builder lane must handle adversarial but common real-world cases without
forcing immediate escape:

- signed upload preparation
- multipart upload preparation
- finalize-required upload contracts
- deferred processing via poll, callback, or webhook
- nested tenant/workspace/project routes
- custom action endpoints
- nonstandard but still explicit request shapes
- the occasional brutal endpoint that mixes path params, request params,
  headers, body, upload preparation, deferred completion, and push delivery

The degradation ladder must be:

1. standard `url(...)` path
2. `url(...)` plus one extra override or advanced step
3. explicit advanced `url(...)` shape with transport modifiers
4. raw family declaration escape hatch

Not:

1. standard builder path
2. immediate fall to raw `load(...)`

## Phases

### Phase 1: Shared API Root And Inheritance Lock

Purpose:

- remove repeated auth/header/base-URL boilerplate
- make shared request posture a first-class product concept

This phase must ship:

- one explicit API root surface for shared defaults
- nested API scope surfaces for section- or feature-local defaults
- inheritance for base URL, auth, and headers
- inheritance for request-side serializers and related request conventions where
  declared
- explicit local override semantics
- diagnostics-visible evidence of inherited versus overridden request posture

Phase 1 gate:

- no later phase begins until common shared-request posture can be declared
  once, nested per section, and proved to lower identically across multiple
  endpoint families

### Phase 2: Declaration-Site Typing And URL Kernel

Purpose:

- replace giant declaration objects for the common lane
- freeze the core declaration grammar around declaration-site semantic types
  plus `url(...)`

This phase must ship:

- one standard `url(...)` entrypoint for the common lane
- declaration-site semantic type families such as `Detail<T>`, `List<T>`,
  `Paged<T>`, `Create<T>`, `Update<T>`, and `Remove<T>`
- auto-finalizing declaration chains with no `.build()`
- preserved explicit endpoint identity and readable call-site naming
- equivalence with the raw family declaration lane

Phase 2 gate:

- no later phase begins until declaration-site-typed `url(...)` authoring and
  raw-authored conventional declarations can be certified as semantically
  identical

### Phase 3: URL, Path Params, And Request Params Ergonomics

Purpose:

- make routes explicit and readable without repetitive normalization ceremony

This phase must ship:

- `.url(...)` as the standard route declaration step
- path-param inference from route placeholders where possible
- explicit `params(...)` support for request-side URL parameters
- clear separation between path params, request params, and body payloads
- canonical lowering into the closed family/member identity model

Phase 3 gate:

- no later phase begins until URL declaration, param inference, and request
  param serialization can be certified without hidden identity drift

### Phase 4: Common CRUD And Adversarial Endpoint Coverage

Purpose:

- make the 90% case pleasant without making the 10% case fall apart

This phase must ship:

- first-class common semantic shapes for:
  - detail reads
  - list reads
  - paged reads
  - create operations
  - update operations
  - remove operations
- support for explicit custom action routes inside the same grammar
- support for nested workspace, tenant, and project route prefixes
- hostile proof that weird endpoints degrade one step at a time rather than
  forcing immediate raw-lane escape

Phase 4 gate:

- no later phase begins until standard and nonstandard endpoints can coexist in
  one grammar without semantic or readability collapse

### Phase 5: Upload, Processing, And Advanced Transfer Builders

Purpose:

- make advanced transfer contracts pleasant without reopening transport
  semantics

This phase must ship:

- `signedUpload(...)`
- `multipartUpload(...)`
- finalize-required upload declaration
- `processing("poll" | "callback" | "webhook", ...)`
- explicit advanced modifiers such as `verb(...)`, `body(...)`, `headers(...)`,
  and related transport-shaping steps for truly nonstandard endpoints
- equivalence with the existing upload and processing posture semantics
- denial and diagnostics parity for advanced transfer builders

Phase 5 gate:

- no later phase begins until `url(...)`-authored upload and processing flows
  can be certified as semantically identical to the raw lane

### Phase 6: Escape Hatch, Diagnostics, And Certification Closeout

Purpose:

- close the hardening work without trapping the package in one authoring style

This phase must ship:

- a clean raw-family escape hatch that remains first-class and documented
- diagnostics and history parity between builder and raw declarations
- compile-time and runtime proof that builders do not overclaim capability
- docs that teach the pleasant lane first and the raw lane second

Phase 6 gate:

- the milestone is not closed until declaration-site-typed `url(...)` and
  raw-authored resource families converge exactly under hostile certification
  and the docs teach a clear default path

## Must Ship

- one shared API root for common auth, headers, base URL, and inheritance
- nested API scopes for section-specific request defaults
- declaration-site semantic typing for common read and write intent
- explicit `url(...)` declaration as the common entrypoint
- explicit `params(...)` request parameter vocabulary
- path-param inference where possible
- no `.build()` in the normal lane
- signed upload, multipart upload, and deferred processing builder support
- custom action and nonstandard endpoint support inside the same grammar
- a documented raw-lane escape hatch
- diagnostics, type, and runtime parity between builder and raw lanes

## Must Preserve

- the closed resource family and line semantics from Milestone 6
- explicit naming and explicit route intent
- one canonical line model
- runtime-owned lifecycle, freshness, retry, timeout, reconciliation,
  delivery, replay, and restore semantics
- cost honesty at the builder boundary
- future router consumption without router-owned API semantics

## Required Named Proof Families

- `The Shared Request Inheritance Equivalence Test`
- `The Scoped Request Defaults Inheritance Equivalence Test`
- `The Nested Request Defaults Override Honesty Test`
- `The URL And Raw Detail Equivalence Test`
- `The URL And Raw List Equivalence Test`
- `The URL And Raw Paged Equivalence Test`
- `The Declaration Type And Runtime Lowering Equivalence Test`
- `The URL Path Param Inference And Stable Identity Test`
- `The Request Params Serialization And Identity Boundary Test`
- `The Conventional CRUD Declarations Parity Test`
- `The Nonstandard Endpoint Degradation Test`
- `The Signed Upload Builder Parity Test`
- `The Multipart Upload Builder Parity Test`
- `The Deferred Processing Builder Parity Test`
- `The DX Declaration Diagnostics And History Honesty Test`
- `The DX Declaration Capability Overclaim Compile-Time Boundary Test`

## Acceptance Evidence

This milestone is complete only when the wasm product surface can prove:

- shared API defaults lower identically across multiple endpoint families
- nested API scopes lower identically to their explicit fully-written request
  posture equivalents
- declaration-site-typed `url(...)` authoring and raw-authored resources
  converge to the same family identity, line identity, lifecycle truth, and
  diagnostics/history truth
- path-param inference and request-param declaration do not destabilize
  canonical identity
- conventional CRUD-shaped declarations are materially shorter without semantic
  drift or redundant verb/shape restatement
- adversarial custom endpoints still fit the same grammar without forcing
  immediate raw-lane escape
- signed upload, multipart upload, and deferred processing builder steps remain
  exact semantic consumers of the closed transfer substrate
- inherited versus overridden request posture is visible enough in diagnostics
  and request inspection that teams can reason about section-local defaults
- the docs recommend one obvious pleasant lane while preserving the raw lane as
  the explicit escape hatch

## Architectural Notes

- the builder surface should live as a separate authoring subsystem over the
  existing resource facade, not as a rewrite of the resource runtime itself
- route and request declaration should remain explicit enough to be grepable and
  reviewable
- the builder layer should prefer additive lowering helpers over a giant
  conversion blob
- the DX lane should likely become the default teaching lane for the later
  router milestone

## Sequencing Notes

This milestone belongs after API-surface closeout because:

- the semantic line model had to be real before ergonomics could safely harden
- the transfer, delivery, and compatibility lanes are now concrete enough to be
  wrapped honestly instead of guessed at

This milestone belongs before the router because:

- the router should consume a pleasant API declaration lane rather than invent
  its own route-local API sugar
- route-local resources will be much easier to author once shared request
  defaults and the `url(...)` declaration lane already exist

Current judgment:

- treat this as a new product-hardening milestone, not as a rewrite of the
  closed API milestone
- keep the raw lane as the semantic anchor
- make the pleasant lane the default recommendation for real app code
