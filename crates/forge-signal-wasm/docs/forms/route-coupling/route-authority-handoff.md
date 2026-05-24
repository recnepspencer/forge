# Route Authority Handoff

## What This Feature Is

This page covers the public route-authority seam for forms:

- `form.reportRouteAuthority(...)`
- `form.clearRouteAuthority(...)`
- `form.routeAuthority()`

## Why You Use It

- bind a form to admitted route truth
- let the form react honestly when the route tells it to preserve, freeze,
  discard, or defer draft behavior
- inspect route-coupled state without digging into router internals

## Stable Entry Points

- `form.routeAuthority()`
- `form.reportRouteAuthority(authority)`
- `form.clearRouteAuthority(...)`
- `outcome.route().formsAuthority()`

## Core Mental Model

The router owns route authority. The form owns draft continuity once authority
is handed off.

The handoff tells the form:

- which route surface owns it
- whether route-coupled behavior is admitted, deferred, or cleared
- how draft continuity should move

## How It Executes

1. admit a route
2. read the route-owned forms authority from the admitted route
3. report that authority into the form
4. inspect the route-authority report, diagnostics summary, or verification

## Small Example

```ts
const outcome = routes.admit({ href: "/tasks/ship-docs" });

if (outcome.kind === "admitted") {
  form.reportRouteAuthority(outcome.route().formsAuthority());
}

console.log(form.routeAuthority().summary);
```

## Real Example

```ts
const outcome = routes.admit({ href: "/tasks/ship-docs/review" });

if (outcome.kind === "admitted") {
  form.reportRouteAuthority(outcome.route().formsAuthority());
}

const report = form.routeAuthority();

console.log(report.summary.handoff);
console.log(report.summary.draftContinuity);
console.log(form.diagnosticsSummary().routeAuthority);
```

## How It Relates To Other Features

- Read [Draft Continuity](./draft-continuity.md) for the draft side of the
  same seam.
- Read [Continuity Audit](./continuity-audit.md) for the aggregated public
  summary in diagnostics and verification.

## Inspection And Debugging

- `form.routeAuthority().summary` shows current route id, continuity, handoff,
  and transition kind
- `form.routeAuthority().history` retains prior route-authority updates
- `form.diagnosticsSummary().routeAuthority` gives the same seam in a compact
  summary shape

## Anti-Patterns

- calling `reportRouteAuthority(...)` with a test helper artifact in app code
- treating route authority as if the form owned it independently
- assuming every route outcome is admitted before reading `formsAuthority()`

## Current Limits

- the route-authority seam is meaningful only when a router route actually owns
  the form behavior

## Related Docs

- [Draft Continuity](./draft-continuity.md)
- [Continuity Audit](./continuity-audit.md)
