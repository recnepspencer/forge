# Stale Deep Link Recovery

## What This Feature Is

Stale deep link recovery is the router feature that lets a route declare an
explicit nearest-valid fallback when admission reaches a terminal non-admitted
artifact such as `notFound`.

## Why You Use It

- keep deleted or moved links from dead-ending the user
- recover to a declared route instead of guessing from raw URL shape
- keep recovery decisions visible in diagnostics and provenance

## Stable Entry Points

- `signals.router.recovery(...)`
- `route(..., { recovery: [...] })`
- `outcome.recovery()`

## Core Mental Model

Recovery is not a second matcher and it is not a redirect shortcut. Admission
runs first. If a route ends in a terminal artifact, a declared recovery can
convert that failure into a different admitted route.

The key rule is: the attempted route still matters. Recovery does not pretend
the original route was valid. It records that the router tried one route, hit a
terminal artifact, and then admitted a declared fallback.

## How It Executes

1. project and admit the attempted route
2. collect the terminal non-admitted artifact
3. run declared recovery handlers in order
4. project the fallback target
5. admit the fallback route as a new route outcome
6. retain the recovery trail on the final outcome

## Small Example

```ts
const staleProjectRecovery = signals.router.recovery(
  "stale-project",
  ({ terminalArtifact, fallback }) => {
    if (terminalArtifact.kind !== "notFound") {
      return null;
    }

    return fallback({
      href: "/projects",
      reason: "projectMissing",
    });
  },
);
```

This is the smallest honest example because it shows the declared recovery
shape without mixing in unrelated prerequisites.

## Real Example

```ts
const projectAvailable = signals.router.prerequisite(
  "project-available",
  ({ facts, allow, notFound }) => (
    facts.projectState === "deleted"
      ? notFound({ reason: "projectMissing" })
      : allow({ reason: "projectAvailable" })
  ),
);

const staleProjectRecovery = signals.router.recovery(
  "stale-project",
  ({ terminalArtifact, fallback }) => {
    if (terminalArtifact.kind !== "notFound") {
      return null;
    }

    return fallback({
      href: "/projects",
      reason: "staleProject",
      detail: "Recover to the project index when a requested project is gone.",
    });
  },
);

const routes = signals.router.define({
  projects: signals.router.layout("/projects", {
    index: signals.router.route("/projects"),
    detail: signals.router.route("/projects/:projectId", {
      admission: [projectAvailable],
      recovery: [staleProjectRecovery],
    }),
  }),
});

const outcome = await routes.admit("/projects/project-1", {
  projectState: "deleted",
});

console.log(outcome.kind);
console.log(outcome.routeId);
console.log(outcome.recovery()?.recovery);
```

In this lane the final visible route is `/projects`, but the outcome still
remembers that `/projects/project-1` was the attempted truth.

## How It Relates To Other Features

- recovery only runs after [Admit](../admission/admit.md)
- the recovered result is still a normal [Route Outcome](../admission/route_outcomes.md)
- browser-history docs matter when the stale link arrived from a host boundary:
  [Browser History Ingress](../history/browser_history_ingress.md)

## Inspection And Debugging

- `outcome.recovery()`
- `outcome.diagnostics().recovery`
- `outcome.provenance().recoveryTrail`
- `outcome.provenance().attemptedRouteId`
- `outcome.provenance().resolvedRouteId`

## Anti-Patterns

- using recovery as a replacement for ordinary redirects
- returning fallback targets that do not project a declared route
- assuming recovery hides the original failed route from diagnostics

## Current Limits

- recovery only helps once a route has already projected and admitted into a
  terminal artifact
- recovery targets must still project declared route candidates
- redirect outcomes dominate recovery; the router does not run recovery after a
  redirect artifact

## Related Docs

- [Admit](../admission/admit.md)
- [Route Outcomes](../admission/route_outcomes.md)
- [Nearest Valid Truth](./nearest_valid_truth.md)
- [Recovery Provenance](./recovery_provenance.md)
