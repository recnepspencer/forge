# Recovery Provenance

## What This Feature Is

Recovery provenance is the explicit explanation of how the router moved
from a stale attempted route to a recovered admitted route.

## Why You Use It

- debug why a route changed after admission
- inspect the exact failed artifact that triggered fallback
- surface recovery behavior in logs or support tooling

## Stable Entry Points

- `outcome.provenance().recoveryTrail`
- `outcome.diagnostics().recovery`
- `outcome.recovery()`

## Core Mental Model

The recovery artifact tells you that recovery happened. Provenance tells you
how it happened.

Use the artifact when you want one simple answer. Use provenance when you need
the chain: failed route, failed artifact, fallback target, and final resolved
route.

## How It Executes

1. admission records the failing artifact
2. recovery emits a fallback artifact
3. the router admits the fallback target
4. the final outcome stores one recovery step in the provenance trail

## Small Example

```ts
const trail = outcome.provenance().recoveryTrail;

console.log(trail[0]?.fromRouteId);
console.log(trail[0]?.fromArtifactKind);
console.log(trail[0]?.toRouteId);
```

## Real Example

```ts
const outcome = await routes.admit("/projects/project-1", {
  workspaceState: "active",
  projectState: "deleted",
});

console.log(outcome.diagnostics().recovery);
console.log(outcome.provenance().terminalSource);
console.log(outcome.provenance().recoveryTrail.map((step) => ({
  recovery: step.recovery,
  fromRouteId: step.fromRouteId,
  fromHref: step.fromHref,
  fromArtifactKind: step.fromArtifactKind,
  toRouteId: step.toRouteId,
  toHref: step.toHref,
})));
```

## How It Relates To Other Features

- prerequisite decisions remain in the same outcome provenance object:
  [Admission Facts](../admission/admission_facts.md)
- browser-history story can later retain the recovered route as visible truth:
  [Browser History Story](../history/browser_history_story.md)

## Inspection And Debugging

- `outcome.diagnostics().recovery`
- `outcome.provenance().terminalSource`
- `outcome.provenance().recoveryTrail`
- `outcome.verification().routeOutcomeDigest`

## Anti-Patterns

- rebuilding recovery explanation from only `outcome.href`
- logging recovery as if it were a redirect
- dropping the failed route id when support or audit tooling needs it

## Current Limits

- the main public story is a linear recovery trail
- recovery provenance exists only on the final route outcome, not as a
  standalone story object
- when no recovery runs, the trail is empty and `outcome.recovery()` is `null`

## Related Docs

- [Stale Deep Link Recovery](./stale_deep_link_recovery.md)
- [Nearest Valid Truth](./nearest_valid_truth.md)
- [Route Outcomes](../admission/route_outcomes.md)
