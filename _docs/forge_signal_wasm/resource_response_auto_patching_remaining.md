# Resource Response Auto Patching Remaining Work

## Current Status

Typed response auto patching is implemented for the intended collection
resource surface.

The current API lets a route attach one response contract and have the family
derive item patching, item-aspect patching, and item-aspect delivery helpers
without repeating manual route plumbing:

```ts
const taskResponse = resource.response.array({
  itemId: (task) => task.id,
  aspects: resource.response.objectAspects<Task>()({
    title: "title",
    status: "status",
    assigneeId: "assigneeId",
  }),
});

const tasks = api.url("/tasks").response(taskResponse).list({ load });
```

The implemented response contract lanes are:

- `resource.response.array(...)` for direct array responses.
- `resource.response.objectItems<T>()(...)` for object/envelope responses with
  one array-valued item field.
- `resource.response.collection<T>()(...)` for arbitrary typed response shapes
  with explicit `items` and `replaceItems`.
- `resource.response.objectAspects<T>()(...)` for plain object-field aspects.

The route lane intentionally owns item identity, item extraction, item
replacement, and object-field aspect definitions after `.response(...)`.
Manual `.items(...)`, `.reconcile(...)`, `.aspect(...)`, `.summary(...)`, and
`.pageWindowSummary(...)` remain on the explicit reconciliation lane.

## Adversarial Constraint

The rule that must survive production use is:

> A typed collection response must be patchable by declaring its item identity,
> item extraction, item replacement, and ordinary item-field aspects once, at
> the response contract boundary. Every patch must either preserve the declared
> response shape or fail closed before corrupting resource truth. There must be
> no manual per-route patch plumbing for ordinary typed collection responses.

That means the remaining work is not about making the happy path exist. It is
about certifying that the abstraction cannot silently degrade back into manual
plumbing, broad replacement, or malformed response truth.

## Already Covered

- Direct-array response contracts lower into the same reconciliation truth as
  the explicit `.items(...).aspect(...)` route lane.
- Object/envelope response contracts patch the declared item field while
  preserving sibling response fields.
- Generic collection response contracts patch arbitrary item projections and
  reconstruct the outer response with `replaceItems`.
- Response contract routes hide the lower-level builder methods that would
  duplicate or contradict the contract.
- Runtime denials reject malformed declarations, non-array extracted items, bad
  replacement results, and object-aspect writes against non-object items.
- Type smoke tests reject unknown aspects, wrong aspect value types, non-array
  `objectItems` fields, response-shape-breaking `replaceItems`, detail
  finalizers, and manual route-owned reconciliation after `.response(...)`.
- Package-facing docs now describe the automatic typed response contract model.

## Left To Finish Before Calling This Fully Closed

### 1. Add Explicit Paged Runtime Certification

The type surface exposes `.response(responseContract).paged(...)`, and the
implementation lowers through the same route finalization path, but the current
focused response-contract runtime tests exercise `.list(...)`.

Add hostile paged tests for:

- `resource.response.array(...).paged(...)`
- `resource.response.objectItems<T>()(...).paged(...)`
- `resource.response.collection<T>()(...).paged(...)`
- item patching and item-aspect patching on paged lines
- page-window metadata preservation when the response contract replaces items
- malformed paged response extraction and replacement denials

Acceptance bar: paged response contracts prove parity with list response
contracts, including fail-closed malformed-shape behavior.

### 2. Broaden Custom-Shape Hostile Coverage

The current custom-shape coverage proves a GraphQL-style `edges -> node`
projection. That is representative, but not adversarial enough to certify the
generic contract shape.

Add tests where `collection<T>()(...)` must preserve:

- nested response metadata
- empty item lists
- item reorder from replacement
- duplicate response objects with distinct item identities
- readonly input arrays returning mutable replacement arrays
- replacement that accidentally drops non-item fields
- extraction functions that return array-like objects instead of real arrays

Acceptance bar: the generic lane proves that only real arrays are admitted and
that outer response preservation is the contract author's responsibility, with
bad replacement caught when it stops producing extractable item arrays.

### 3. Certify Mutation Atomicity For Failed Aspect Writes

There is coverage for object-aspect writes against non-object items preserving
the original line value. The next hardening step is to make this property
broader and explicit.

Add tests for:

- unknown item id does not partially mutate the response
- invalid object aspect write does not mutate sibling items
- failed `replaceItems` validation leaves the line value unchanged
- failed delivery follows the same no-partial-mutation rule as failed patching

Acceptance bar: every response-contract failure mode proves the authoritative
line value is unchanged.

### 4. Add Architecture-Level Regression Tests For Builder Boundaries

The type smoke tests and runtime checks already reject several manual-plumbing
paths after `.response(...)`. Add a small structural test that treats this as
an architectural invariant.

The invariant:

> `.response(...)` creates a response-owned collection lane. The route builder
> must not expose post-response methods that can redefine identity,
> reconciliation, aspects, summaries, or detail semantics.

Acceptance bar: the runtime builder shape and TypeScript surface both certify
that the wrong lane is unrepresentable or unavailable.

### 5. Decide Whether Summary Contracts Belong In Response Contracts

Current docs honestly say response contracts do not own summary patching yet.
That is acceptable only if it remains an explicit design boundary.

Decision needed:

- Keep summaries on `.items(...).reconcile(...).summary(...)` only.
- Or add a response-owned summary contract surface later.

If summaries move into response contracts, they need a separate design pass.
They should not be slipped into the current object-aspect mechanism.

Acceptance bar: either the denial stays mechanically enforced and documented,
or a new summary contract design lands with type/runtime/docs/tests together.

### 6. Decide Whether Detail Resources Need A Separate Auto-Patching Surface

Current response contracts are collection contracts. Detail resources are
intentionally denied on the response lane.

Decision needed:

- Keep detail resources out of this feature.
- Or design a separate detail response contract that owns object-field patching
  without pretending it is collection reconciliation.

Acceptance bar: collection response contracts must not become a disguised
detail patching API.

### 7. Add A Closeout Matrix For This Feature Slice

Before this gets merged as "done", add a short closeout matrix that names the
proof surfaces:

- API surface implemented
- type denials implemented
- runtime happy paths implemented
- runtime hostile paths implemented
- paged parity implemented
- docs updated
- limitations mechanically enforced

Acceptance bar: every row points to a concrete test or document path.

## Not Required For This Slice

The following are not blockers for the current collection response contract
work unless the product scope expands:

- response-owned summary contracts
- detail response contracts
- custom non-object aspect writers inside `objectAspects<T>()(...)`
- automatic inference of item identity without an explicit `itemId`
- automatic inference of arbitrary custom response replacement

Those would be new design surfaces, not missing plumbing.

## Recommended Next Batch

Do the remaining work in this order:

1. Add paged runtime certification.
2. Add no-partial-mutation tests for failed patch and delivery paths.
3. Add hostile custom-shape tests.
4. Add the builder-boundary architecture regression.
5. Write the closeout matrix and update the package docs only if the limits
   change.

After that, the feature can be called closed under the mentality standard:
the hard problem is specified, the normal path is ergonomic, the wrong paths
are mechanically denied, and the adversarial cases are certified rather than
left to convention.
