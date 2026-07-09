# Back Provenance

## What This Feature Is

Back provenance is the dedicated artifact for the previous admitted
route-history entry.

## Why You Use It

- treat "back" as retained navigation truth, not just `history.back()`
- restore or replay the previous route exactly when that authority exists

## Stable Entry Points

- `story.backProvenance()`
- `back.available`
- `back.restoreBoundary()`
- `back.restore(...)`
- `back.replay(...)`

## Core Mental Model

Back provenance is not an instruction to the browser. It is the router-owned
evidence for the previous retained route entry.

## How It Executes

1. story derives the current and previous admitted entries
2. `backProvenance()` wraps the previous entry as a dedicated artifact
3. restore and replay authority delegate to that previous entry

## Small Example

```ts
const back = story.backProvenance();

console.log(back.available);
```

## Real Example

```ts
const back = story.backProvenance();

if (back.available) {
  console.log(back.previous?.routeId);
  console.log(back.restoreBoundary());
}
```

## How It Relates To Other Features

- use [Route History Entries](./route_history_entries.md) when you need the
  whole retained list
- use [Navigation Auditability](./navigation_auditability.md) when you need the
  higher-level explanation surface

## Inspection And Debugging

- `back.previous`
- `back.verification()`

## Anti-Patterns

- using raw browser back semantics as if they guaranteed route restore truth

## Current Limits

- no previous admitted entry means no back provenance authority

## Related Docs

- [Route History Entries](./route_history_entries.md)
- [Navigation Auditability](./navigation_auditability.md)
