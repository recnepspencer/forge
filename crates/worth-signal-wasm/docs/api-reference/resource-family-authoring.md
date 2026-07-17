# Resource Family Authoring Reference

A resource family is the reusable definition for one class of server-backed
value. It owns canonical param identity, request posture, value shape, loading,
and any declared structure used by patches, delivery, or response mapping.

## Recommended Entry Points

```ts
const api = signals.api({ baseUrl: "/api" });
const scoped = api.scope({ requestContext, policy });
const route = scoped.url("/projects/:projectId");
```

Finish a read route with:

- `.detail(...)` — one logical value per param set;
- `.list(...)` — a collection with stable item identity;
- `.paged(...)` — identifiable items plus an honest accumulation rule.

Finish a write route with:

- `.create(...)`;
- `.update(...)`;
- `.remove(...)`;
- `.command(...)` for a non-CRUD operation.

The route-first lane derives canonical identity and request target from route
params. `api.url(...)` does not send HTTP; `load(params, request)` performs the
I/O.

## Small Example

```ts
interface Project {
  id: string;
  name: string;
}

const projectDetail = signals.api({
  baseUrl: "/api",
}).url("/projects/:projectId").detail<Project>({
  async load({ projectId }, request) {
    if (!request.target.url) throw new Error("project URL was not admitted");
    const response = await fetch(request.target.url);
    if (!response.ok) throw new Error(`project ${projectId}: ${response.status}`);
    return response.json() as Promise<Project>;
  },
});
```

The family, not the component, now owns the relationship between `projectId`,
request identity, and the loaded value.

## Collection And Paged Requirements

Route builders declare item identity with `.items(...)`:

```ts
const tasks = api.url("/tasks")
  .items((task: { id: string; title: string }) => task.id)
  .list({ load: () => client.listTasks() });
```

A list without stable item identity cannot honestly support item-local patches,
effects, or delivery. A paged family additionally declares how an incoming page
combines with the existing accumulated value.

## Raw Family Entry Points

Use these when route-derived identity is not enough or an external
compatibility definition requires direct authoring:

- `signals.resource.detail(...)`
- `signals.resource.collection(...)`
- `signals.resource.paged(...)`
- `resourceParams<TParams>()`
- `resourceParamIdentity(params, canonicalKey)`

```ts
const invoiceDetail = signals.resource.detail({
  params: resourceParams<{ tenantId: string; invoiceId: string }>(),
  normalizeParams: ({ tenantId, invoiceId }) =>
    resourceParamIdentity(
      { tenantId, invoiceId },
      `${tenantId}:${invoiceId}`,
    ),
  load: ({ invoiceId }) => client.loadInvoice(invoiceId),
});
```

In the raw lane, `normalizeParams(...)` is authoritative for canonical identity.
Equivalent logical params must produce the same key. The raw lane uses the same
line runtime; it is not a more authoritative store.

## Shared Request Posture

Families can declare auth, request context, policy, continuation, processing,
upload transport, download posture, and effect profile. Prefer named helpers
such as `resourceAuth.*()`, `resourceRequestContext(...)`, and
`resourcePolicyProfiles.*()` over anonymous convention objects.

Named `apiScope` admits stable static defaults. Use `api.scope({...})` when a
default depends on route params or the current request.

## Declared Update Structure

A family can progressively declare:

- collection item identity and reconciliation;
- detail fields, regions, and JSON paths;
- item aspects and value summaries;
- mutation-response targets;
- branch-native effect posture;
- delivery, transfer, download, form, and route integration.

These declarations are proof for narrower operations. The runtime does not
infer semantic identity or merge rules from object shape.

## Family Operations

All family shapes expose:

- `line(params)`;
- `optionalLine(params | null | undefined | { enabled: false })`;
- `execute(params, options?)`;
- `invalidate(params)`;
- `invalidateAll()`.

Patch and delivery helpers depend on the declared family shape.

## Anti-Patterns And Limits

- Do not build canonical keys from display text, array position, or unstable
  serialization.
- Do not use `detail` for an identifiable collection merely to avoid declaring
  item identity.
- Do not hide auth, headers, method, or retry policy in `load(...)`.
- Do not declare narrow patch or merge behavior without the structure that
  proves it.
- A family owns browser-local resource behavior; it is not the durable server
  database.

## Related Docs

- [Resources Overview](../resources/index.md)
- [Your First Resource](../resources/start-here/your-first-resource.md)
- [Resource Line Reference](./resource-line.md)
- [Resource Request And Policy](./resource-request-and-policy.md)
- [Advanced Resource Capabilities](../resources/advanced/README.md)
