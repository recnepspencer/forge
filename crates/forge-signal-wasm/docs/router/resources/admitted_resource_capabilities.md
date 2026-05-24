# Admitted Resource Capabilities

## What This Feature Is

Admitted resource capabilities are the route-resource handles available once a
route outcome is admitted.

## Why You Use It

- read the live resource line for the visible route
- inspect current status, freshness, and diagnostics
- preserve visible resource continuity across refresh-pending work

## Stable Entry Points

- `outcome.route().resource(name)`
- `resource.line()`
- `resource.current()`

## Core Mental Model

Once a route is admitted, its resource capability becomes part of admitted
route truth. It is the native resource line for that route, not a router-owned
copy.

## How It Executes

1. admit the route
2. materialize the route resource line
3. expose the same resource line through the admitted route capability

## Small Example

```ts
const outcome = await routes.admit("/users/user-1");
const detail = outcome.kind === "admitted"
  ? outcome.route().resource("detail")
  : null;
```

## Real Example

```ts
const outcome = await routes.admit("/users/user-2");

if (outcome.kind === "admitted") {
  const detail = outcome.route().resource("detail");
  console.log(detail.line().status());
  console.log(detail.current().freshness.kind);
  console.log(detail.current().diagnosticsSummary.activity.continuity);
}
```

## How It Relates To Other Features

- projected access comes earlier in
  [Projected Resource Capabilities](./projected_resource_capabilities.md)
- restore and replay still belong to history features, not to admitted route
  resources

## Inspection And Debugging

- `resource.line()`
- `resource.current()`
- `resource.current().diagnosticsSummary`
- `resource.verification()`

## Anti-Patterns

- copying admitted route resource truth into a second route cache
- reading the line from a projected candidate once route admission already
  happened

## Current Limits

- admitted capabilities only exist on admitted route outcomes

## Related Docs

- [Route Resource Declarations](./route_resource_declarations.md)
- [Projected Resource Capabilities](./projected_resource_capabilities.md)
