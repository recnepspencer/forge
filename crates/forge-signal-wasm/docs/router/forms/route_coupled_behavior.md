# Route-Coupled Behavior

## What This Feature Is

Route-coupled behavior is the steps-and-actions posture that results from the
current route authority handoff.

## Why You Use It

- explain why route-coupled steps are active or unavailable
- explain why route-coupled actions are accepted or denied
- keep route-authority blockers distinct from unrelated blockers

## Stable Entry Points

- `summary.handoff?.routeCoupledBehavior`
- step artifacts on route-coupled forms
- action plans on route-coupled forms

## Core Mental Model

Route-coupled behavior is the operational side of route authority. It is where
handoff posture turns into active, deferred, or cleared form behavior.

## How It Executes

1. derive route authority handoff
2. map that handoff into route-coupled step posture
3. map that handoff into route-coupled action acceptance or denial

## Small Example

```ts
console.log(form.steps().artifacts[0].posture);
console.log(form.actionPlan("reviewRoute").status);
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
  console.log(form.steps().artifacts[0].posture);
  console.log(form.actionPlan("reviewRoute").status);
}
```

## How It Relates To Other Features

- the route-side source of this behavior is [Route Authority Handoff](./route_authority_handoff.md)
- whole-system audit is [Continuity Audit](./continuity_audit.md)

## Inspection And Debugging

- `steps().artifacts`
- `actionPlan(name)`
- `routeAuthority().summary.handoff`

## Anti-Patterns

- mixing route-authority denial with unrelated host denial
- telling users a route-coupled action is unavailable without saying whether it
  is deferred, frozen, or cleared

## Current Limits

- this surface is only about route-coupled steps and actions

## Related Docs

- [Route Authority Handoff](./route_authority_handoff.md)
- [Continuity Audit](./continuity_audit.md)
