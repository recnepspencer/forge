# What Happens After A Write

## What This Feature Is

This page is the quick answer for the practical question:

"After I run a write, what can I inspect?"

## Why You Use It

- understand line status after the write
- inspect request posture and effect posture
- inspect mutation-response planning when the route owns it
- know where to debug next

## Stable Entry Points

- `line.status()`
- `line.summary()`
- `line.diagnostics()`
- `line.history()`
- `line.mutationResponse()`

## Core Mental Model

A write still yields an ordinary line. The difference is that more write-specific
state may now be visible through:

- request method and body
- last effect
- mutation response plan
- history and replay / restore availability

## How It Executes

After the line runs, the runtime exposes:

- current status and freshness
- admitted request posture
- last effect, if the write created one
- mutation-response plan, if the route declared one
- history, replay, and restore availability

## Small Example

```ts
const line = createUser.line({
  body: { userId: "u1", name: "Ada" },
});

console.log(line.status());
console.log(line.summary());
console.log(line.request().method);
```

## Real Example

```ts
const updateTask = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/tasks/:taskId")
  .update({
    load: ({ taskId, body }) => ({
      id: taskId,
      title: body.title,
      status: "saved",
    }),
  });

const line = updateTask.line({
  taskId: "t1",
  body: { title: "Ship docs" },
});

console.log(line.summary());
console.log(line.diagnostics().lastEffect);
console.log(line.mutationResponse());
console.log(line.history().availability);
```

## How It Relates To Other Features

- Use [Choose An Effect Profile](./choose-an-effect-profile.md) when the main
  decision is how the write should behave.
- Use [Mutation Response Reconciliation](../mutation-response-reconciliation.md)
  when you need the deeper response-owned write mapping lane.
- Use [Line Inspection](../line-inspection.md) for the broader retained reads
  and diagnostics surface.

## Inspection And Debugging

The usual first reads are:

- `line.summary()`
- `line.diagnostics()`
- `line.mutationResponse()`
- `line.history()`

If the route declared a mutation-response plan, `line.mutationResponse()` is
usually the fastest way to understand what exact vs fallback work the runtime
planned.

## Anti-Patterns

- treating create/update/remove as fire-and-forget work with no line reads
- debugging write behavior only from server logs when the line already has
  request, effect, and history reads

## Current Limits

- this page is the short readback guide
- deeper mutation-response, history, and verification material still lives in
  the older flat docs for now

## Related Docs

- [Choose An Effect Profile](./choose-an-effect-profile.md)
- [Mutation Response Reconciliation](../mutation-response-reconciliation.md)
- [Line Inspection](../line-inspection.md)
