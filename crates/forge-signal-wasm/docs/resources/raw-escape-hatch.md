# Raw Escape Hatch

Use this page when the pleasant route-first lane is not enough.

The raw lane is still fully supported. It is just not the default teaching
path.

## Reach For The Raw Lane When You Need

- manual `normalizeParams(...)`
- manual `resourceParamIdentity(...)`
- custom family declaration details that the route lane does not expose
- direct use of lower-level compatibility or raw request surfaces

## Happy Path

```ts
import {
  createSignals,
  resourceParamIdentity,
  resourceParams,
} from "forge-signal-wasm";

const signals = await createSignals();

const userDetail = signals.resource.detail({
  params: resourceParams(),
  normalizeParams: ({ userId }) =>
    resourceParamIdentity({ userId }, `/users/${userId}`),
  load: ({ userId }) => ({ id: userId, name: `User ${userId}` }),
});

const line = userDetail.line({ userId: "u1" });

console.log(line.value());
console.log(line.summary());
```

## Relationship To The Pleasant Lane

- the pleasant lane lowers into this one
- lifecycle, diagnostics, reconciliation, delivery, upload, download, and
  history truth still come from the same runtime
- choosing the raw lane is about authoring control, not a different cache or a
  different line model

## Where To Go Next

- route-first default lane:
  [Fetch And Write Resources](./fetch-and-write.md)
- low-level family reference:
  [Resource Family Authoring Reference](../api-reference/resource-family-authoring.md)
