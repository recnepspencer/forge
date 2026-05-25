# Access Policy

## What This Feature Is

Access policy is the route-level admission model built from prerequisites,
admission sources, facts, and recovery declarations.

## Why You Use It

- keep route access semantics explicit
- distinguish missing authority, forbidden authority, and unavailable authority
- let the router own redirect and denial semantics

## Stable Entry Points

- `route(..., { admission: [...] })`
- `signals.router.prerequisite(...)`
- `signals.router.recovery(...)`
- `routes.admit(...)`

## Core Mental Model

Access policy in this router is not one separate middleware phase. It is the
combination of declared prerequisite checks plus optional recovery.

## How It Executes

1. route declarations name prerequisite and recovery steps
2. admission evaluates those steps against route context and facts
3. the first terminal artifact or recovered route truth becomes the outcome

## Small Example

```ts
const adminOnly = signals.router.prerequisite("adminOnly", {
  evaluate: ({ facts }) => facts.role === "admin" ? true : { kind: "forbidden", reason: "adminOnly" },
});
```

## Real Example

```ts
const routes = signals.router.define({
  settings: signals.router.route("/settings", {
    admission: [adminOnly],
  }),
});

const outcome = await routes.admit("/settings", { role: "viewer" });

console.log(outcome.kind);
console.log(outcome.diagnostics());
```

## How It Relates To Other Features

- use [Prerequisites](./prerequisites.md) for the declaration surface
- use [Forbidden, Unavailable, And Denied](./forbidden_unavailable_denied.md)
  for the outcome distinctions

## Inspection And Debugging

- `outcome.diagnostics()`
- `outcome.provenance()`
- recovery trail and prerequisite decisions

## Anti-Patterns

- treating all route policy as one untyped "auth guard"
- hiding redirects or denials in arbitrary app-side effects

## Current Limits

- access policy only covers semantics you declare through the router surface

## Related Docs

- [Prerequisites](./prerequisites.md)
- [Forbidden, Unavailable, And Denied](./forbidden_unavailable_denied.md)
