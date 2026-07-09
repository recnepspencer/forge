# Write A Resource

## What This Feature Is

The standard route-first write lane:

- `.create(...)`
- `.update(...)`
- `.remove(...)`

## Why You Use It

- use the common path for server-backed writes
- keep request method, body shape, and line behavior together
- get a normal line back instead of inventing a separate write state machine

## Stable Entry Points

- `.create(...)`
- `.update(...)`
- `.remove(...)`
- `family.line(...)`
- `line.request()`

## Core Mental Model

A write is still a resource family and then a line.

The main difference is that line params include a `body` for create/update
lanes, and the admitted request method comes from the finalizer.

## How It Executes

1. declare the route
2. finish it with `.create(...)`, `.update(...)`, or `.remove(...)`
3. materialize a line with path params, request params, and body as needed
4. inspect the result with ordinary line reads

## Small Example

```ts
const createUser = api.url("/users").create({
  load: ({ body }) => ({ id: body.userId, name: body.name }),
});

const line = createUser.line({
  body: { userId: "u1", name: "Ada" },
});
```

## Real Example

```ts
const updateTask = signals.api({
  baseUrl: "/api",
  effects: signals.resource.effects.branchNative(),
}).url("/workspaces/:workspaceId/tasks/:taskId")
  .update({
    load: ({ taskId, body }) => ({
      id: taskId,
      title: body.title,
      status: "saved",
    }),
  });

const line = updateTask.line({
  workspaceId: "demo",
  taskId: "t1",
  body: { title: "Ship docs" },
});

console.log(line.request().method);
console.log(line.summary());
```

## How It Relates To Other Features

- Use [Submit Patches And Replacements](./submit-patches-and-replacements.md)
  for local line changes after a value is already loaded.
- Use [Choose An Effect Profile](./choose-an-effect-profile.md) when the write
  behavior itself is the main decision.

## Inspection And Debugging

Start with:

- `line.request().method`
- `line.summary()`
- `line.diagnostics()`
- `line.mutationResponse()`

`line.mutationResponse()` is especially useful once the route declares
mutation-response reconciliation.

## Anti-Patterns

- inventing a separate client-side write abstraction when the route finalizers
  already fit
- using `.verb(...)` for ordinary create/update/remove flows

## Current Limits

- this page covers the standard write lane
- richer mutation-response mapping still lives in the older flat docs for now

## Related Docs

- [Choose An Effect Profile](./choose-an-effect-profile.md)
- [What Happens After A Write](./what-happens-after-a-write.md)
- [Older Fetch And Write Resources](../fetch-and-write.md)
