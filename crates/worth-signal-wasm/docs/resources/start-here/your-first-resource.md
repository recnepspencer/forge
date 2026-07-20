# Your First Resource

Use a resource when a value comes from an external system and your application
needs more than the resolved JSON. A resource line keeps the request identity,
visible value, loading state, freshness, and debugging evidence together.

This tutorial builds one detail resource backed by `fetch`. It is the normal
path, not a toy API you will have to discard later.

## 1. Create The Runtime And API

```ts
import { createSignals } from "worth-signals-wasm";

const signals = await createSignals();
const api = signals.api({ baseUrl: "/api" });
```

`createSignals()` uses the worker-first deployment by default. `signals.api(...)`
declares defaults shared by a group of routes; it does not create a network
client or issue a request.

## 2. Declare One Detail Family

```ts
interface Project {
  id: string;
  name: string;
  revision: number;
}

const projectDetail = api.url("/projects/:projectId").detail<Project>({
  async load({ projectId }, request) {
    if (!request.target.url) throw new Error("project URL was not admitted");

    const response = await fetch(request.target.url);
    if (!response.ok) {
      throw new Error(`project ${projectId} failed: ${response.status}`);
    }
    return response.json() as Promise<Project>;
  },
});
```

The route owns how `projectId` becomes request identity. The `detail` finalizer
says each parameter set represents one logical record. Your `load` function
owns the I/O and returns the observed server value.

The second `load` argument is the admitted request descriptor. Prefer its URL,
method, auth posture, and request context over rebuilding those decisions inside
the loader.

## 3. Materialize A Line

```ts
const project = projectDetail.line({ projectId: "project-42" });
```

Calling `line(...)` starts materialization. Calling it again with canonically
equivalent params reuses the same logical resource member rather than creating a
second cache entry.

For an asynchronous first load, the line can be pending with `null` visible
value. Read the lifecycle explicitly:

```ts
const status = project.status();

if (status.kind === "pending") {
  renderProjectSkeleton();
}
```

## 4. Wait When Your Task Requires Settled Truth

```ts
const settlement = await project.awaitSettlement({ timeoutMs: 5_000 });

if (
  settlement.resultKind === "fulfilled" ||
  settlement.resultKind === "partial"
) {
  const value = project.value();
  if (value) console.log(value);
} else {
  console.error(project.summary());
}
```

Use `awaitSettlement()` in loaders, tests, or workflows that cannot proceed
without a result. The public line type remains nullable, so check the value even
after settlement when `null` is not valid for your domain. In a UI, subscribe
to `project.signal()` and render pending, fulfilled, rejected, and timed-out
state instead of blocking the screen.

## 5. Inspect Before You Guess

```ts
const summary = project.summary();

console.log(summary.current.status.kind);
console.log(summary.current.freshness.kind);
console.log(project.request().target.url);
console.log(project.diagnosticsSummary().latest);
```

`summary()` is the best first debugging read. It groups current lifecycle,
request, processing, transfer, diagnostic, and history-availability truth. It is
not a second cache and it does not copy the complete retained history.

## Refresh, Revalidate, Or Invalidate

```ts
project.invalidate(); // keep the visible value, mark it stale
project.refresh();    // start a new load
project.revalidate(); // apply the family's revalidation posture
```

These operations act on the same line identity. Do not replace the family with
a new instance to force a request; that throws away the identity and evidence
you wanted the resource runtime to preserve.

## Choose The Right Shape

- Use `.detail(...)` for one logical record.
- Use `.list(...)` when the value contains identifiable items.
- Use `.paged(...)` when later pages accumulate and the family can declare how.

Collections need stable item identity. Paged resources need stable item
identity plus an accumulation rule. If the server returns an array but the
application treats it as one indivisible document, a detail family can still be
the honest shape.

## Common Mistakes

- Do not call `fetch` outside the family and copy its result into a separate
  component cache.
- Do not create canonical keys from labels, formatted dates, or other unstable
  display values.
- Do not hide auth, headers, or retry policy inside `load(...)` when they belong
  in the API or family declaration.
- Do not assume `value()` is populated before initial settlement.
- Do not use the raw `signals.resource.*(...)` lane until you need manual
  identity or compatibility-oriented authoring.

## Where To Go Next

- [Reading And Caching](../caching/README.md)
- [Writing And Server Reconciliation](../updating/README.md)
- [Collections And Partial Updates](../partial-updates/README.md)
- [Resource Family Reference](../../api-reference/resource-family-authoring.md)
- [Resource Line Reference](../../api-reference/resource-line.md)
