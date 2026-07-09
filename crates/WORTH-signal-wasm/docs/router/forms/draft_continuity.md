# Draft Continuity

## What This Feature Is

Draft continuity is the forms artifact that explains what happened to a form
draft across route authority changes.

## Why You Use It

- distinguish preserved, frozen, replaced, deferred, and cleared draft results
- debug why effective form value changed after route authority changed
- keep route-driven draft semantics explicit

## Stable Entry Points

- `form.routeAuthority().summary.draftContinuity`
- `FormRouteAuthorityDraftContinuityArtifact`

## Core Mental Model

Route authority changes do not only affect whether the form may act. They can
also affect what happens to the current draft. Draft continuity is the typed
answer to that question.

## How It Executes

1. forms compare the previous authority and current draft
2. apply preserve, freeze, discard, defer, or clear semantics
3. emit one draft continuity artifact

## Small Example

```ts
const draftContinuity = form.routeAuthority().summary.draftContinuity;
console.log(draftContinuity?.draftResolution);
```

## Real Example

```ts
form.fields.title.set("Keep me");
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
  console.log(form.routeAuthority().summary.draftContinuity?.draftResolution);
}
```

## How It Relates To Other Features

- handoff posture is in [Route Authority Handoff](./route_authority_handoff.md)
- public diagnostics and verification audit it in [Continuity Audit](./continuity_audit.md)

## Inspection And Debugging

- `draftContinuity.posture`
- `draftContinuity.draftResolution`
- `draftContinuity.authorityChange`
- `draftContinuity.draftChanged`

## Anti-Patterns

- treating preserve and freeze as equivalent
- reading effective form value changes without checking draft continuity

## Current Limits

- this surface explains route-authority-driven continuity, not every form draft
  mutation

## Related Docs

- [Route Authority Handoff](./route_authority_handoff.md)
- [Route-Coupled Behavior](./route_coupled_behavior.md)
