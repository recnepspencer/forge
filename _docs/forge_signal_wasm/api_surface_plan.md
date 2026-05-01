# forge-signal-wasm API Surface Plan

> **Status:** Planned engineering spec
>
> **Roadmap parent:** [wasm_product_roadmap.md](./wasm_product_roadmap.md)
>
> **Lifecycle prerequisites:**
> - [opaque_identity_and_ergonomic_authoring_plan.md](./opaque_identity_and_ergonomic_authoring_plan.md)
>
> **Core vision:** [_docs/forge_signal/forge_signal_vision.md](../../../_docs/forge_signal/forge_signal_vision.md)
>
> **Core async roadmap lineage:**
> - [_docs/forge_signal/milestone-a-plan.md](../../../_docs/forge_signal/milestone-a-plan.md)
> - [_docs/forge_signal/milestone-b-plan.md](../../../_docs/forge_signal/milestone-b-plan.md)
> - [_docs/forge_signal/milestone-c-plan.md](../../../_docs/forge_signal/milestone-c-plan.md)
> - [_docs/forge_signal/milestone-d-plan.md](../../../_docs/forge_signal/milestone-d-plan.md)
>
> **Core test requirements:** [_docs/forge_signal/test-requirements.md](../../../_docs/forge_signal/test-requirements.md)
>
> **Adjacent product visions:**
> - [_docs/forge-query/forge_query_vision.md](../../../_docs/forge-query/forge_query_vision.md)
> - [_docs/forge-server/forge_server_vision.md](../../../_docs/forge-server/forge_server_vision.md)

## Goal

Build a first-class TypeScript API surface in `forge-signal-wasm` that can
replace query-library-shaped frontend usage now, simplify a broad slice of the
frontend API integration layer, and remain structurally aligned with later
`forge-query` and `forge-server` integration.

We are not just replacing TanStack Query.
We are replacing a bunch of the frontend API integration layer too.

The target outcome is:

- resource-backed application code feels native in TypeScript
- request shaping, auth posture, headers/context, callback/redirect handling,
  and delivery posture stop being ambient glue and become part of one typed API
  model
- resource lifecycle, freshness, retry, timeout, cancellation, and output
  continuity remain runtime-owned truth rather than package-local convention
- resource identity, parameterization, and partial update behavior map cleanly
  onto graph-native families and aspect-local invalidation
- structured resource truth can carry binary/asset descriptors, download
  readiness, and live-delivery updates without collapsing structured truth and
  byte transport into one muddy abstraction
- the initial signals-first resource surface can later consume `forge-query`
  definitions and `forge-server` delivery without a second client truth engine
- diagnostics, replay, restore, branch, and export surfaces remain honest

This milestone is not a generic fetch-helper layer.

It is a better generic HTTP/API client surface that productizes the
async-capability substrate already closed in core `forge-signal`, while
refusing to collapse back into a bag-of-options transport wrapper.

## Why This Milestone Exists

The current wasm package has enough runtime substrate to support a much better
frontend resource story than ordinary query libraries:

- temporal meaning is runtime-owned
- async lifecycle is runtime-owned
- async policy families are descriptor-backed
- async capability is attachable to arbitrary nodes
- graph contracts, diagnostics, history, replay, branch, and restore are all
  already real product surfaces

But without an API-surface milestone, app code still faces two bad choices:

- hand-author remote/read lifecycle with lower-level signals and async
  capability plumbing
- or retreat to a generic query cache / HTTP client model that throws away
  graph semantics, aspect-local invalidation, replay truth, diagnostics
  richness, and typed request/context posture

This milestone exists to prevent that collapse.

The intended product direction is:

- in the near term, a signals-first resource product can replace
  TanStack-query-shaped usage without inventing a second async truth model
- in the same near term, the package should also be able to absorb a large
  amount of routine API-client work such as auth posture, headers/context,
  redirects, callbacks, downloads, and deferred completion
- in the longer term, the same resource surface must be able to accept
  `forge-query`-backed read definitions and `forge-server` delivery as the
  authoritative upstream read lane
- the same product surface must also be able to host structured truth that
  references files, exports, attachments, and other binary-backed artifacts
  without forcing app authors into a second file-state or socket-state model

The milestone therefore has to solve three problems at once:

- make resource authoring pleasant now
- make routine API integration materially less annoying and less bureaucratic
- avoid becoming a parallel read/query abstraction that later fights
  `forge-query`

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects here is solving the hostile structural
  problem first. This milestone must start from stale completion denial,
  freshness honesty, branch/restore parity, and no-second-engine discipline,
  not from "make `useQuery` but nicer."
- `arch_laws.md`
  The most important laws here are 2, 7, 20, 27, 33, 40, and 41. Resources must
  declare what they consume, surface boundary crossings honestly, consume
  lowered lifecycle/query plans rather than rediscovering them at runtime, and
  keep authoritative server/store truth distinct from derived local materialized
  state.
- `perf_laws.md`
  The most important thing it protects is breadth honesty. Resource helpers
  must not hide broad refetch, graph-wide invalidation, broad diagnostics
  reconstruction, or cache-key folklore behind cheap-looking APIs.
- `domain_laws.md`
  The most important thing it protects is subsystem shape. Resource identity,
  lifecycle materialization, family parameterization, partial patch
  reconciliation, continuity policy, and diagnostics each need named homes
  instead of one giant query-helper module.
- `forge_signal_vision.md`
  The most important thing it protects is that `forge-signal` remains the
  derived execution substrate. Resources are a product surface over derived
  truth, not a second authority store and not a replacement for the runtime.
- `wasm_product_roadmap.md`
  The most important thing it protects is sequencing. Resources must consume
  the already-closed async, host-capability, graph lifecycle, and ergonomic
  authoring substrate rather than becoming the place where those semantics are
  invented ad hoc.
- `test-requirements.md`
  The most important thing it protects is certification. Resource product work
  is not complete until serial/branch/replay/restore/host-driven/resource-driven
  paths converge under machine-checked proof, not just happy-path examples.
- `milestone-a-plan.md`
  The most important thing it protects is that freshness windows, deadlines,
  stale-after behavior, and wake ordering must remain runtime-owned temporal
  truth.
- `milestone-b-plan.md`
  The most important thing it protects is that request identity, in-flight
  ownership, completion admission, cancellation, retry, timeout, and replay
  truth are already runtime law. The resource surface must consume them rather
  than restating them.
- `milestone-c-plan.md`
  The most important thing it protects is that retry, timeout, supersession,
  revalidation, output continuity, retention, diagnostics, and replay
  compatibility are descriptor-backed policy families, not front-end option
  bags.
- `milestone-d-plan.md`
  The most important thing it protects is that async capability is attachable
  to arbitrary nodes. Resources therefore must be framed as a productized
  capability surface, not as a separate "resource node species."
- `forge_query_vision.md`
  The most important thing it protects is ownership. `forge-query` owns typed
  query expression, live-promotion semantics, result shapes, and read planning.
  Wasm resources must be able to consume query-backed upstream definitions later
  without becoming a rival query DSL now.
- `forge_server_vision.md`
  The most important thing it protects is delivery ownership. `forge-server`
  owns durable subscriptions, cursor resume, basis negotiation, delivery
  classes, and branch-aware network delivery. Wasm resources must be able to
  host delivered truth locally without redefining delivery semantics.

## Adversarial Constraint

This milestone must survive the following hostile condition:

> A long-lived TypeScript application with parameterized resource families,
> partial item/aspect updates, host-driven revalidation, branch-local drafts,
> replay/restore activity, and later query/server-backed delivery must converge
> to the same committed local resource truth, the same freshness/lifecycle
> truth, and the same diagnostics/history artifacts regardless of whether the
> resource line was driven by initial request admission, local refresh,
> partial patch reconciliation, host-capability-triggered revalidation, branch
> restore, or replay from retained history.

Concretely, the design must remain correct when:

- the same parameter set is requested repeatedly through one resource family
- different parameter sets produce neighboring family members under one graph
- old completions arrive after refresh or supersession
- one item in a large collection changes in one aspect only
- output continuity must preserve prior visible data while a refresh is pending
- a branch-local form draft overlays the same underlying resource line
- diagnostics are requested after retention has pruned rich history
- later `forge-query`/`forge-server` integration drives the same resource line
  through shaped patches instead of local callback-issued requests

If the product surface produces:

- unstable resource identity
- broad refetch or broad invalidation where aspect/item-local updates were
  admissible
- local freshness state that can drift from runtime lifecycle truth
- different replay/restore answers depending on which authoring surface created
  the resource
- or a second query/cache authority beside the runtime

then the milestone has failed.

## Product Decision Lock

- the resource surface is a consumer of runtime-owned async capability and
  policy truth, not a second async state engine
- the resource surface may feel query-library-shaped to app authors, but it
  must lower through graph-native families, async capability, and runtime
  diagnostics/history truth
- resource identity must be family- and parameter-aware without reducing to
  untyped cache-key folklore
- same parameters must resolve to the same resource family line under a stable
  normalization contract
- changed parameters must produce either a new family line or an explicit
  continuity/replacement outcome; silent ambiguity is out of spec
- partial patch/update flows should feel automatic once the family declaration
  has supplied enough structural truth; the package must not require repetitive
  per-call "narrow mode" opt-in when item/aspect scope is already declared
- broad replacement remains the honest fallback when the family declaration does
  not prove a narrower patch contract
- structured truth and binary transport must stay distinct semantic categories
  even when one resource line references downloadable or upload-produced assets
- output continuity while pending, after rejection, or after supersession must
  come from runtime policy families rather than UI-local display conventions
- common continuity/freshness/retry postures should be available through named
  policy profiles or equivalent sealed declarations so app code is not pushed
  toward repetitive mini policy bags
- resource diagnostics must explain freshness, retry, timeout, supersession,
  visibility/continuity, and invalidation causes without requiring callers to
  inspect lower-level runtime plumbing
- the initial wasm resource surface may be signals-first, but it must not lock
  the package into a second query language that competes with `forge-query`
- future `forge-query` and `forge-server` integration is a required compatibility
  target; this milestone must leave a clean seam for query-backed resource
  definitions and server-delivered patch streams
- this milestone must define the resource-side contract for binary descriptors,
  downloads, and live delivery now, even though optimistic write intent and
  multipart submission lifecycle remain the adjacent mutation milestone's
  responsibility

Normative consequence:

- any implementation that uses arbitrary string cache keys as the primary
  resource identity story is out of spec
- any implementation that models pending/success/error purely in React-local or
  package-local state without runtime-owned lifecycle evidence is out of spec
- any implementation that guesses narrow patch legality from ad hoc runtime
  heuristics instead of declared item/aspect structure is out of spec
- any implementation that makes partial patch/update behavior a best-effort UI
  optimization instead of a graph-/aspect-aware runtime consequence is out of
  spec
- any implementation that treats binary asset bytes as interchangeable with
  structured resource truth by default is out of spec
- any implementation that makes websocket/session delivery semantics the
  resource layer's private convention instead of an explicit compatibility seam
  with `forge-server` is out of spec
- any implementation that treats the resource surface as a generic fetch helper
  detached from graph contracts, branch history, replay, or diagnostics is out
  of spec
- any implementation that lets the initial signals-first resource surface drift
  semantically from later query-backed resource materialization is out of spec

## Architectural Model

### Ownership split

This milestone freezes the intended ownership boundary:

1. **`forge-signal`**
   - owns temporal, lifecycle, policy, replay, restore, and diagnostics truth
   - owns async capability and family identity at the execution substrate
2. **`forge-query`**
   - owns typed read/query expression, result shapes, live-promotion semantics,
     and query planning
3. **`forge-server`**
   - owns subscription durability, cursor resume, basis negotiation, delivery
     classes, branch-aware transport, and patch delivery
4. **`forge-signal-wasm` resources**
   - own TypeScript-facing resource authoring and local resource materialization
   - host materialized resource lines inside the local runtime
   - expose diagnostics/history/freshness/readiness surfaces for app code
   - may begin signals-first, but must preserve a seam for query/server-backed
     upstream definitions later

The resource surface is therefore not:

- a new query engine
- a transport layer
- a second lifecycle runtime
- a UI-only cache

It is the local product layer that turns runtime-owned async capability into an
application-facing resource model.

### Resource identity model

The resource surface should be organized around explicit families and resource
lines rather than free-form cache keys.

The target authoring shapes should be finite and explicit rather than one giant
resource options bag. The initial intended family shapes are:

1. **Detail resource family**
   - one parameterized line resolves to one shaped detail value
2. **Collection resource family**
   - one parameterized line resolves to a collection of stable items plus any
     declared summaries
3. **Paged collection family**
   - one parameterized line resolves to cursor/page-shaped collection segments
     with explicit accumulation semantics

These are different enough in lifecycle and patch behavior that they should not
be collapsed into one overly-permissive constructor.

The intended identity categories are:

1. **Resource family identity**
   - the stable identity of the resource definition/product lane
   - owned by the package authoring surface
2. **Normalized parameter identity**
   - the canonical identity of one family member
   - derived from typed parameters through an explicit normalization contract
3. **Runtime line identity**
   - the runtime-owned identity of the actual materialized resource line
4. **Public graph contract names**
   - explicit graph boundary names when a resource line is published outward

The normalization contract should not collapse to an arbitrary plain string too
early. The desired direction is a branded or otherwise sealed lowered identity
artifact, for example:

```ts
type ResourceParamIdentity<TParams> = {
  readonly params: TParams;
  readonly canonicalKey: string;
  readonly __resourceParamIdentityBrand: unique symbol;
};
```

The exact spelling is open, but the spec intent is not:

- the package should be able to distinguish raw parameter objects from
  canonicalized family-member identity at compile time
- query-backed and server-delivered resource lines should be able to target the
  same lowered identity artifact later
- package authors should not be encouraged to treat ad hoc string assembly as
  the primary public identity story

The desired authoring direction is:

```ts
const product = signals.resource.detail({
  params: resourceParams<{ workspaceId: string; productId: string }>(),
  normalizeParams: ({ workspaceId, productId }) =>
    resourceParamIdentity({ workspaceId, productId }, `${workspaceId}:${productId}`),
  load: ({ workspaceId, productId }) => fetchProduct(workspaceId, productId),
});

const productLine = product.line({ workspaceId: "demo", productId: "p-123" });
```

```ts
const products = signals.resource.collection({
  params: productParams<{ workspaceId: string; productId: string }>(),
  normalizeParams: ({ workspaceId, productId }) =>
    resourceParamIdentity({ workspaceId, productId }, `${workspaceId}:${productId}`),
  itemIdentity: (product) => product.id,
  aspects: productAspects,
  load: ({ workspaceId, productId }) => fetchProducts(workspaceId, productId),
});

const productList = products.line({ workspaceId: "demo", productId: "all" });
```

The important law is not the exact spelling. It is:

- parameter identity is typed and normalized
- same params reuse the same family line
- changed params produce explicit line replacement or a distinct family member
- line identity is not a stringly cache-key accident

Compile-time consequence:

- detail families should not expose collection-only APIs
- collection families should not expose paged-segment APIs unless they are
  actually paged families
- paged families should carry accumulation/replace behavior in their declaration
  rather than as ad hoc call-site switches
- line construction should consume canonical parameter identity through one
  typed path rather than letting call sites smuggle unstable identity through
  incidental strings

### Resource materialization model

Each resource line should expose one coherent local materialization package
through one canonical line facade rather than family-specific mini APIs.

The line facade should be frozen strongly enough that app code, docs, React
consumers, and later query-backed/server-backed lines all speak the same local
language.

Desired direction:

```ts
const line = product.line({ workspaceId: "demo", productId: "p-123" });

line.value();
line.status();
line.refresh();
line.diagnostics();
line.history();
line.view((product) => product?.inventory);
```

The exact naming may still move, but the shape should not. A resource line
should have one obvious place to ask for:

- current visible value
- lifecycle/status truth
- refresh/revalidate actions
- diagnostics/history explanations
- line-scoped derived views

This is important for both human DX and AI usability. A weakly specified line
surface will drift into multiple competing method/handle shapes and make the
resource product feel bureaucratic instead of obvious.

Each resource line should therefore expose:

- lifecycle truth
  - pending
  - fulfilled
  - rejected
  - cancelled
  - superseded
  - stale / refresh-eligible
- value truth
  - current visible value
  - prior preserved value when continuity policy permits it
  - shaped value suitable for downstream computed/output/view use
- diagnostics/history truth
  - why it refreshed
  - which policy family shaped its behavior
  - what invalidated it
  - what replay/restore basis currently explains it

These may lower to ordinary runtime-backed handles internally, but the public
line facade must stay canonical rather than forcing app code to learn multiple
equivalent access postures.

### Patch and collection narrowing model

The resource surface must expose two update postures:

1. **Whole-line refresh/replacement**
   - honest when a result shape is replaced as a unit
2. **Partial patch reconciliation**
   - honest only when the graph can prove narrower scope
   - typically item-aware, aspect-aware, or field-group-aware

The second posture is strategically important. The target product behavior is:

- one changed item in a collection need not invalidate the whole collection
- one changed aspect on one item need not invalidate unrelated aspect
  dependents
- collection summaries, row views, detail views, and forms should re-evaluate
  only if their declared dependencies intersect the changed scope

This capability must be runtime-derived from graph contracts and aspect-local
dependency truth, not inferred by ad hoc UI heuristics.

The intended DX rule is:

- if the family declaration includes stable item identity and declared aspect
  structure, narrow patch behavior should be automatic
- if the family declaration does not include enough structure to prove narrow
  correctness, the package must fall back to broad replacement or reject the
  narrow patch surface explicitly

That means the declaration, not every patch call, carries the burden of proof.

The resource product also needs one first-class derived-view abstraction so app
authors do not rebuild row views, summaries, and UI-facing slices through ad
hoc `computed(...)` glue at every call site.

Desired direction:

```ts
const inventoryView = line.view((products) =>
  products.map((product) => ({
    id: product.id,
    inStock: product.inventory.inStock,
  })),
);
```

Or for a detail line:

```ts
const priceView = line.view((product) => product?.pricing.currentPrice);
```

The important law is:

- resource-line views are line-scoped derived truth, not detached selector
  folklore
- they preserve the same dependency/aspect narrowing laws as the underlying
  resource line
- they become the obvious product lane for row summaries, detail slices,
  readiness helpers, and UI-facing shaped values

If this abstraction is not named now, developers will rebuild it themselves in
React selectors or ambient `computed(...)` wrappers and quietly lose the clean
resource vocabulary the milestone is trying to establish.

Desired collection shape:

```ts
const products = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) => workspaceId,
  itemIdentity: (product) => product.id,
  aspects: productAspects,
  load: ({ workspaceId }) => fetchProducts(workspaceId),
});

const line = products.line({ workspaceId: "demo" });

line.patchItem("p-123", {
  inventory: { inStock: false },
});
```

In that shape:

- `patchItem(...)` exists because the declaration proved item identity
- aspect-narrow invalidation exists because the declaration proved aspect lanes
- a detail family with no collection identity would not expose `patchItem(...)`
- a family without declared aspect structure would fall back to broader
  replacement semantics

Just as importantly, `patchItem(...)` and related narrow patch APIs must be
understood as read-side reconciliation surfaces rather than generic local write
intent.

They exist for:

- authoritative server patch delivery
- live pushed updates
- refresh reconciliation
- package-owned convergence between old visible value and newly admitted read
  truth

They do not exist to make resource lines the semantic owner of optimistic local
mutation intent. The later mutation milestone may lower through similar runtime
machinery, but the public type surface must make the distinction clear:

- resources reconcile authoritative or admitted read-side change
- mutations own optimistic write intent, rollback, supersession, and commit
  posture

The milestone should therefore prefer naming and typing that makes accidental
crossing harder, for example by reserving resource patch APIs for
reconciliation-flavored inputs instead of generic arbitrary partial object
writes.

### API request and authority posture

The API surface should not stop at resource identity and lifecycle.

If the package still forces app authors to hand-roll:

- auth headers
- workspace or tenant scope headers
- correlation identifiers
- branch/basis request context
- redirect continuations
- callback or webhook completion matching
- multipart or binary delivery posture

then it has left a large part of the frontend API integration problem unsolved.

The target direction is a typed request posture that remains graph-native and
runtime-honest while also functioning as a substantially better generic HTTP/API
client surface.

Desired semantic request categories include:

1. **Auth posture**
   - anonymous
   - authenticated
   - workspace/tenant scoped
   - future delegated or impersonated posture if admitted later
2. **Request context posture**
   - headers
   - branch/basis context
   - correlation ids
   - capability-specific metadata
3. **Delivery posture**
   - one-shot response
   - refresh/polling-backed
   - pushed/live delivered
   - deferred completion
4. **Continuation posture**
   - redirect/browser continuation
   - callback-backed completion
   - webhook-correlated completion
   - external provider handoff
5. **Payload posture**
   - structured read
   - binary/download descriptor
   - multipart-adjacent reconciliation target
6. **Upload transport posture**
   - direct multipart
   - signed upload
   - future resumable signed upload if admitted later

The important law is that these should be declared as typed API semantics, not
reintroduced as arbitrary request bags.

The API surface may lower those semantics to ordinary HTTP concerns such as:

- method
- headers
- auth tokens
- query strings
- body encodings
- multipart bodies
- redirect-follow behavior

But the product surface should own them as a coherent semantic declaration
rather than leaving them as free-floating transport folklore.

Signed uploads deserve explicit treatment here because they are one of the most
common places where otherwise disciplined frontend systems collapse back into
manual glue:

1. ask the app server for a signed target
2. upload bytes to storage
3. finalize or attach the uploaded object
4. maybe wait for later processing completion

The API surface should absorb that sequence as one typed mutation or
continuation-backed API posture, not make every app author restage the dance by
hand.

The intended product law is:

- app code declares an upload intention against a domain surface
- the API layer may lower that into prepare/upload/finalize/process phases
- the public lifecycle remains one coherent typed story
- the resulting structured truth reconciles back into the same resource/form
  substrate

This also creates room for a stronger frontend visibility and access posture:

- the client may know that a family is authenticated, tenant-scoped, or
  capability-scoped
- the client may shape requests and UI posture accordingly
- the server remains the enforcement authority

That is the right direction if the package is going to simplify not just query
replacement, but a broad slice of frontend API integration.

### Query/server compatibility seam

The first shipped resource surface may be signals-first, but it must reserve a
clean seam for future query/server-backed resource definitions.

That means the architecture should eventually support two upstream authoring
sources that converge on one local materialization model:

1. **Signals-first authored resource family**
   - used before `forge-query`/`forge-server` integration is the default
2. **Query-backed delivered resource family**
   - backed by `forge-query` result shapes and `forge-server` patch delivery

Both must converge to:

- the same family/line identity laws
- the same lifecycle and continuity laws
- the same diagnostics/history vocabulary
- the same branch/restore/replay honesty

The downstream local resource line must not care whether its upstream truth was
born from a local callback-authored load or a server-delivered query result.

### Binary asset and live-delivery model

The resource layer must explicitly support the common real-world case where
structured truth references:

- file attachments
- exports
- media variants
- blob-backed previews
- server-generated downloadable artifacts

The target architectural rule is:

- structured truth remains the resource's primary semantic payload
- binary bytes remain transport payload, not the default resource identity

The intended resource-side surface should therefore prefer typed asset
descriptors such as:

```ts
type AssetDescriptor = {
  assetId: string;
  mediaType: string;
  contentLength: number | null;
  digest: string | null;
  download: DownloadDescriptor | null;
  variants?: Record<string, DownloadDescriptor>;
};
```

Where `DownloadDescriptor` or an equivalent branded type describes how bytes may
be fetched without making byte transport the resource line's semantic center.

Likewise, the resource layer must be prepared to ingest live delivered updates
from server transport without becoming the owner of websocket/session logic.
The target shape is:

- server/session layer owns connection lifecycle, resume, basis negotiation,
  and delivery classes
- resource lines own local materialization of delivered structured patches,
  invalidations, and continuity effects
- the same family/line identity and patch narrowing laws apply whether updates
  arrived from a local refresh, an HTTP response, or a pushed delivery packet

Multipart uploads matter here too, but differently:

- the mutation milestone owns multipart submission lifecycle, progress,
  optimistic intent, and commit/rollback behavior
- the resource milestone owns the read-side structured truth contract that
  upload completion must reconcile into

Signed uploads are important enough to deserve an explicit target shape now.

The API surface should be able to represent a signed upload as one declared API
posture rather than three or four unrelated request helpers.

Desired direction:

```ts
const receiptUpload = signals.api.upload({
  params: invoiceAttachmentParams(),
  auth: apiAuth.workspace(),
  file: uploadedFile({
    media: ["image/png", "image/jpeg", "application/pdf"],
    maxBytes: mb(25),
  }),
  transport: signedUpload({
    prepare: prepareInvoiceReceiptUpload,
    finalize: finalizeInvoiceReceiptUpload,
  }),
  reconcileInto: invoiceResource,
});
```

The exact spelling may change, but the contract should preserve:

- one declared upload intention from app code
- typed prepare-target data from the app server
- typed lowering into signed `PUT` or signed `POST` transfer semantics
- explicit finalize posture when the app server must confirm attachment or begin
  processing
- optional later processing completion before the final descriptor is ready
- one progress/processing/error/result lifecycle from the developer point of
  view

The API surface should therefore standardize a signed-upload descriptor shape or
equivalent lowered artifact, for example:

```ts
type SignedUploadDescriptor = {
  url: string;
  method: "PUT" | "POST";
  headers?: Record<string, string>;
  fields?: Record<string, string>;
  objectKey: string;
  expiresAt: string;
};
```

The important law is not the exact fields. It is that app code should not have
to manually assemble `FormData`, guess storage-provider differences, or hand-roll
prepare/upload/finalize correlation logic.

Likewise, the API surface should distinguish these subphases in diagnostics
without making them separate product APIs:

- prepare failed
- transfer failed
- finalize failed
- uploaded and awaiting processing
- processing failed
- ready

That gives the package one honest story for direct multipart, signed upload,
and later delegated/resumable upload flows instead of multiple ad hoc upload
cultures.

This must be designed now so uploads, downloads, and pushed updates do not
later force a second file-state or socket-state abstraction beside resources.

There is also a common "processing job" posture that the API surface should
name explicitly:

- report/export generation
- media processing
- bulk imports
- AI generation or enrichment jobs
- provider-confirmed deferred work

This does not necessarily require a fourth primary family kind, but it does
require a declared lifecycle posture where a line may move through:

- accepted
- processing elsewhere
- refreshed or callback-completed later
- downloadable or structured result ready

That posture should be expressible through the same line facade rather than
becoming a pile of one-off polling glue.

This does not require a fourth primary family kind. The better target shape is
an asset-backed descriptor posture shared across detail, collection, and paged
resource families:

- detail lines may expose one asset-bearing value
- collection lines may expose lists of asset-bearing items
- paged collection lines may expose asset-bearing segments

The product distinction is therefore not "asset resource family" versus "normal
resource family." It is whether a given family's shaped value carries declared
binary/asset descriptors through one common resource-line model.

## Phases

### Phase 1: Resource Family Identity And Materialization Kernel

Freeze the core family/line substrate.

This phase must ship:

- finite family declaration vocabulary for:
  - detail resources
  - collection resources
  - paged collection resources
- typed parameter normalization contract
- branded or sealed lowered parameter-identity artifact rather than plain
  string-only canonicalization
- stable same-params reuse law
- explicit changed-params replacement or new-line law
- a canonical materialized resource-line facade
- core line state surfaces for value, lifecycle, and freshness
- a first-class line-scoped derived-view abstraction
- compile-time separation between family kinds so wrong-shape APIs are
  unrepresentable

This phase must not yet try to solve every higher-level query ergonomics need.
Its job is to make resource identity and local materialization mechanically
honest first.

Phase 1 gate:

- no later phase begins until family identity, parameter normalization, and
  line reuse semantics are fixed strongly enough to support replay and package
  proofs

### Phase 2: Runtime-Lowered Refresh, Revalidation, And Continuity

Attach the resource family substrate to the closed async policy layer.

This phase must ship:

- refresh/revalidate vocabulary on resource lines
- runtime-owned pending/refresh/retry/timeout/supersession truth surfaced
  through the product API
- explicit output continuity behavior while pending and after rejection
- named policy-profile or equivalent sealed posture for common continuity,
  freshness, and retry shapes
- resource-line diagnostics that explain refresh and continuity state

This is where the product stops being "typed fetch family" and becomes a real
runtime consumer of Milestones A/B/C/D.

Phase 2 gate:

- no later phase begins until refresh, retry, timeout, continuity, and
  revalidation are clearly inherited from runtime policy rather than secretly
  package-owned

### Phase 3: Request Context, Auth, And Continuation Posture

Freeze the typed API-shaping surface so ordinary frontend integration concerns
stop leaking out as hand-written request glue.

This phase must ship:

- typed auth posture declarations
- typed request-context posture for headers, basis/branch context, and
  correlation metadata
- typed continuation posture for redirect, callback-backed completion, and
  webhook-correlated completion
- typed upload-transport posture that can express direct multipart and signed
  upload flows through one API model
- explicit deferred-completion / processing-job posture that can be hosted by
  the same resource-line model
- a lowering story that can target ordinary HTTP semantics without turning the
  public API back into a bag of request options

This is the point where the milestone becomes more than query replacement and
starts becoming a real API surface.

Phase 3 gate:

- no later phase begins until request/auth/context/continuation semantics are
  declared strongly enough that ordinary API integration, including signed
  upload preparation/finalization, does not escape into ambient app glue

### Phase 4: Partial Patch Reconciliation And Collection Scope Narrowing

Teach resources to be surgically narrow instead of cache-line blunt.

This phase must ship:

- collection-oriented resource families or equivalent collection-capable line
  behavior
- declaration-driven automatic narrow patch behavior once stable item identity
  and aspect structure have been supplied
- explicit rejection or broad-replace fallback when a family declaration does
  not justify narrow patch legality
- runtime-visible narrowing contracts for partial collection updates
- explicit read-side reconciliation posture for resource patch APIs so they do
  not blur into mutation intent
- diagnostics that explain why only one item/aspect/summary updated
- proofs that broad refresh is not happening where narrow patch admission was
  semantically sufficient

This is one of the milestone's biggest user-visible differentiators, and it
must be designed as first-class behavior rather than later optimization folklore.

Phase 4 gate:

- no later phase begins until partial updates can be certified as narrower than
  broad whole-line replacement under real diagnostics and history evidence

### Phase 5: Resource Diagnostics, History, Branch, And Restore Surface

Make the resource product auditable.

This phase must ship:

- line-scoped diagnostics summaries
- line-scoped history/replay/restore surfaces or equivalent graph-integrated
  access
- branch-aware resource inspection
- explicit denial/retention posture when rich diagnostics are unavailable
- export/import or package-boundary posture as applicable for resource-backed
  graph products

The goal is that a resource line is not just usable but explainable.

Phase 5 gate:

- no later phase begins until replay/restore/branch history can explain a
  resource line honestly instead of merely reproducing its current visible
  value

### Phase 6: Binary Descriptor, Download, And Live-Delivery Surface

Own the nontrivial read-side boundary for files and pushed updates now rather
than leaving it as vague later glue.

This phase must ship:

- typed binary/asset descriptor vocabulary for resource values that reference
  files, exports, or media
- explicit download-descriptor or equivalent surface that separates structured
  truth from byte transport
- resource-line ingestion of live delivered structured patches and invalidation
  packets through a transport-neutral compatibility boundary
- diagnostics that explain whether a visible change came from local refresh,
  pushed delivery, or structured patch reconciliation
- resource-side reconciliation contracts that future multipart upload mutations
  must target

This phase does not make the resource layer the websocket owner or the upload
owner. It makes those adjacent systems safe to plug in later without changing
resource semantics.

Phase 6 gate:

- no later phase begins until binary descriptors, downloads, and live delivered
  patches can be hosted by one resource-line model without semantic drift

### Phase 7: Query And Server Compatibility Surface

Reserve and certify the seam that lets this milestone grow into the larger
Forge stack cleanly.

This phase must ship:

- a resource-definition posture that can later accept `forge-query` result
  shapes without semantic drift
- a compatibility contract for server-delivered patches/bases/basis refresh
- evidence that signals-first and query/server-backed resource lines can
  converge on one local materialization model
- docs that state clearly what is already native and what becomes authoritative
  only when query/server delivery is present

This phase is not full server integration. It is the architectural lock that
keeps the initial product from becoming a dead end.

## Must Ship

- first-class detail, collection, and paged-collection family authoring in
  TypeScript
- typed parameter normalization and stable family-member identity
- a canonical resource-line facade with first-class line-scoped derived views
- typed request/auth/header/context posture that meaningfully simplifies common
  frontend API integration
- typed continuation posture for redirects, callback completion, webhook
  correlation, and deferred processing outcomes
- typed upload-transport posture for direct multipart and signed-upload flows
- runtime-lowered refresh, retry, timeout, revalidation, supersession, and
  continuity behavior surfaced as product API
- named continuity/freshness/retry policy postures that lower to runtime policy
  truth rather than ad hoc option bags
- automatic item-/aspect-aware partial patch reconciliation where declared
  family structure proves narrower scope honestly
- explicit reconciliation-oriented resource patch APIs kept distinct from
  mutation/optimistic write intent
- explicit broad-replace or denial fallback where declared family structure does
  not justify narrow patch legality
- collection-capable resource behavior suitable for serious application lists
  and detail views
- typed binary/asset descriptor support plus download-facing resource posture
- live-delivery ingestion compatibility for pushed structured updates
- resource-scoped diagnostics/history/branch/replay/restore truth
- docs and examples that teach one obvious resource story
- a compatibility seam for future `forge-query`/`forge-server` backing without
  semantic drift

## Must Preserve

- `forge-signal` remains the owner of lifecycle, policy, replay, and temporal
  truth
- `forge-query` remains the owner of typed query expression, result shapes, and
  live read semantics
- `forge-server` remains the owner of network delivery, resume, basis
  negotiation, and durable subscription semantics
- transport lowering may carry headers/auth/method/body details, but the API
  surface owns the typed semantic declaration that drives them
- wasm resources do not become a second query engine, second async runtime, or
  second authority store
- forms and resources share substrate truth rather than carrying separate async
  or freshness models
- mutation/optimistic write intent remains the neighboring mutation milestone's
  responsibility even when resources participate in reconciliation

## Acceptance Evidence

This milestone is complete only when the package can prove:

- same parameters produce one stable resource family line under canonical
  normalization
- changed parameters produce explicit replacement or new-line behavior rather
  than ambiguous reuse
- runtime-owned refresh/retry/timeout/supersession/continuity truth is visible
  through the product API without semantic drift
- one canonical line facade and view vocabulary can be used across family kinds
  without semantic drift
- auth/context/continuation declarations lower consistently into request
  behavior without turning the public API back into ad hoc transport folklore
- narrow item/aspect patch reconciliation produces the same committed truth as
  broad replacement while touching less declared semantic surface
- resource patch APIs remain clearly reconciliation-oriented instead of becoming
  an accidental second mutation surface
- resource diagnostics explain freshness, retry, timeout, supersession,
  invalidation, and continuity honestly
- branch restore and replay reconstruct the same local resource truth and the
  same lifecycle explanation artifacts
- the signals-first resource surface can be shown to converge structurally with
  the later query/server-backed posture instead of requiring a second resource
  product

Required named proof families:

- `The Resource Family Identity Equivalence Test`
- `The Canonical Parameter Normalization Test`
- `The Resource Family Shape Compile-Time Separation Test`
- `The Canonical Line Facade Stability Test`
- `The Resource Refresh And Continuity Policy Parity Test`
- `The Resource Policy Profile Lowering Test`
- `The Resource Line View Narrowing Test`
- `The Auth And Request Context Lowering Test`
- `The Redirect And Callback Continuation Parity Test`
- `The Deferred Processing Job Lifecycle Test`
- `The Signed Upload Prepare Transfer Finalize Parity Test`
- `The Narrow Patch Versus Broad Refresh Equivalence Test`
- `The Automatic Narrow Patch Admission Test`
- `The Broad Replace Honest Fallback Test`
- `The Resource Reconciliation Versus Mutation Intent Boundary Test`
- `The Binary Descriptor Does Not Collapse Structured Truth Test`
- `The Live Delivery And Local Refresh Convergence Test`
- `The Resource Diagnostics And History Honesty Test`
- `The Branch Restore Resource Parity Test`
- `The Signals First And Query Backed Resource Convergence Test`

## Architectural Notes

- The strongest implementation shape is probably family-first rather than
  instance-first. That keeps identity, reuse, and continuity laws centralized.
- The resource surface should prefer typed parameter objects or equivalent
  canonical input bundles over ad hoc variadic key arrays.
- Collection and detail resources should likely share one family substrate with
  different result-shape/materialization policies rather than diverging into
  separate lifecycle engines.
- A later mutation milestone should integrate through explicit resource
  reconciliation hooks rather than letting mutations patch resource lines by
  ambient side effects.
- A later forms milestone should consume resource lines as source truth rather
  than copying them into an unrelated local store.

## Sequencing Notes

This milestone belongs after opaque ergonomic authoring because:

- resource authoring should inherit id-less local handles, linked writable
  state, and graph-owned lifecycle instead of compensating for older ceremony

This milestone still belongs after forms in the current roadmap sequence unless
you deliberately want to reorder the product line.

Why the current order can still be defended:

- forms can ship first as local/runtime-native draft productization on top of
  existing graph and async substrate
- the resource milestone can then focus on external/read lifecycle without
  diluting the forms story

Why reordering might become reasonable later:

- `forge-query` is already far enough along that query-backed read language is
  becoming a stronger near-term dependency than originally assumed
- if the product direction shifts toward query-backed resources as the default
  app read lane before forms, the roadmap should be revised explicitly rather
  than letting implementation drift silently

Current judgment:

- write this milestone now
- preserve the seam to `forge-query` and `forge-server`
- keep roadmap sequencing judgment explicit instead of accidental
