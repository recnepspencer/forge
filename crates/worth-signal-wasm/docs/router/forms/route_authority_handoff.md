# Route Authority Handoff

The router decides whether a route is admitted. The form owns its draft. Route
authority handoff is the explicit seam between those two responsibilities.

Start by declaring which form surface belongs to the route and what should
happen to its draft when route authority changes:

```ts
const routes = signals.router.define({
  review: signals.router.route("/review", {
    forms: signals.router.forms("review-form", {
      continuity: "defer",
      reason: "Keep the draft while review authority is temporarily absent.",
    }),
  }),
});
```

Then bind only admitted route authority:

```ts
const outcome = await routes.admit("/review", admissionFacts);

if (outcome.kind === "admitted") {
  const authority = outcome.route().formsAuthority();
  if (authority) {
    form.reportRouteAuthority(authority);
  } else {
    form.clearRouteAuthority({ reason: "Route has no form authority" });
  }
} else {
  form.clearRouteAuthority({ reason: `Route outcome: ${outcome.kind}` });
}
```

Narrowing `formsAuthority()` is important: not every admitted route declares a
form surface. `bindRouteAuthority(...)` is a convenience when the admitted
route type is compatible with the form controller; reporting the explicit
artifact is the most precise handoff for a typed route tree.

## Continuity Is Policy, Not Storage

The declaration's `preserve`, `freeze`, `discard`, or `defer` posture tells the
form how to interpret route authority changes. It does not move draft truth
into the router. Forms still own edits, validation, submission, and merge state.

```ts
const report = form.routeAuthority();
console.log(report.summary.handoff?.posture);
console.log(report.summary.handoff?.routeCoupledBehavior);
```

Cleared and deferred are intentionally different. Cleared says authority was
explicitly removed. Deferred says the relationship is temporarily unresolved
under a declared continuity policy. Do not collapse either into a generic
disabled flag.

Next: [Draft Continuity](./draft_continuity.md),
[Route-Coupled Behavior](./route_coupled_behavior.md), and
[Continuity Audit](./continuity_audit.md).
