# Diagnostics Surfaces

Router diagnostics answer different questions at different layers. Start with
the smallest surface that can answer yours; scale up to retained history and
auditability only when the question actually crosses navigation events.

## One Admission Decision

```ts
const outcome = await routes.admit("/projects/p7", admissionFacts);

console.log(outcome.diagnostics().outcomeKind);
console.log(outcome.provenance().terminalSource);
```

`diagnostics()` summarizes what happened. `provenance()` explains the attempted
and resolved route, prerequisite decisions, consumed source declarations, and
recovery trail.

## One Browser Boundary Crossing

```ts
const ingress = signals.router.browserHistory.external("/projects/p7");
const report = await routes.admitBrowserHistoryIngress(ingress);

console.log(report.diagnostics().boundaryArtifact);
```

The report explains whether route truth converged, drifted, or failed to admit.
It does not imply the host performed a navigation.

## A Retained Navigation Story

```ts
const story = signals.router.browserHistory.story(report);
const inspection = story.inspection();
const auditability = story.auditability();

console.log(inspection.summary());
console.log(auditability.summary());
```

Use `inspection()` to ask what entries, breadcrumbs, outlets, and restore
postures are retained. Use `auditability()` to ask why the current route is
visible now. Passing a hydration report to `auditability(hydrationReport)` adds
that boundary evidence to the explanation.

## Verification Is A Different Tool

Most router artifacts also expose `verification()`. Verification packages carry
stable digests for equivalence and proof checks. They are not a replacement for
human-readable diagnostics and should not be the first thing logged during an
ordinary admission failure.

Avoid rebuilding summaries from `story.events()`. That throws away the router's
distinction between the latest boundary event and the event that last advanced
route truth.

Next: [History Inspection](../history/history_inspection.md),
[Navigation Auditability](../history/navigation_auditability.md), and
[Verification Packages](./verification_packages.md).
