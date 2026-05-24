# Route Authority Handoff

## What This Feature Is

Route authority handoff is the explicit forms artifact that says how admitted
route truth should hand off authority to a route-coupled form.

## Why You Use It

- keep preserve, freeze, discard, defer, and cleared semantics explicit
- prevent route-coupled forms from guessing whether authority is available
- expose route-coupled behavior in one stable surface

## Stable Entry Points

- `form.routeAuthority().summary.handoff`
- `FormRouteAuthorityHandoffArtifact`

## Core Mental Model

This seam belongs to forms, but the router supplies the route authority that
drives it. The handoff tells the form what kind of authority relationship it
has right now.

## How It Executes

1. a route authority artifact is reported into the form
2. forms derive handoff posture from that authority
3. route-coupled behavior becomes admitted, deferred, or cleared

## Small Example

```ts
const handoff = form.routeAuthority().summary.handoff;
console.log(handoff?.posture);
```

## Real Example

```ts
const routes = signals.router.define({
  review: signals.router.route("/review", {
    forms: signals.router.forms("review-surface", {
      continuity: "defer",
    }),
  }),
});

const outcome = await routes.admit("/review");

if (outcome.kind === "admitted") {
  form.reportRouteAuthority(outcome.route().formsAuthority());
  console.log(form.routeAuthority().summary.handoff?.routeCoupledBehavior);
}
```

## How It Relates To Other Features

- draft results are covered in [Draft Continuity](./draft_continuity.md)
- public audit surfaces are covered in [Continuity Audit](./continuity_audit.md)

## Inspection And Debugging

- `summary.handoff?.posture`
- `summary.handoff?.routeCoupledBehavior`
- `summary.transitionKind`

## Anti-Patterns

- collapsing cleared authority into deferred authority
- letting steps or actions infer authority from unrelated blockers

## Current Limits

- this is a route-coupled forms seam, not a general forms availability system

## Related Docs

- [Draft Continuity](./draft_continuity.md)
- [Continuity Audit](./continuity_audit.md)
