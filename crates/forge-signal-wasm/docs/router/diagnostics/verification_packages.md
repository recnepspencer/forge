# Verification Packages

## What This Feature Is

Verification packages are the router's stable verification digests. They give you a
stable way to certify that a canonical route, route outcome, breadcrumb trail,
history story, or auditability artifact is the thing you think it is without
making you parse private runtime structure.

## Why You Use It

- certify route and navigation behavior in hostile tests
- compare stable digests without snapshotting private internals
- expose stable verification boundaries to higher-level tooling

## Stable Entry Points

- `canonicalArtifact.verification()`
- `admissionPlan.verification()`
- `outcome.verification()`
- `breadcrumbEntry.verification()`
- `story.verification()`
- `inspection.verification()`
- `auditability.verification()`

## Core Mental Model

A verification package is not a general-purpose diagnostics object. It is an
opaque, stable verification artifact.

Use diagnostics when a human needs an explanation. Use verification packages
when a test or tool needs a durable digest.

## How It Executes

1. the router builds a public artifact such as a canonical route or route
   outcome
2. that artifact exposes `verification()`
3. the verification package returns digests for the feature boundary that
   artifact owns
4. callers compare or retain those digests without depending on internal shape

## Small Example

```ts
const detail = routes.detail.to({ params: { projectId: "p7" } });

console.log(detail.canonical().verification().canonicalUrlDigest);
```

This is the smallest honest example because it shows verification at the same
boundary where canonical route truth already exists.

## Real Example

```ts
const report = await routes.admitBrowserHistoryIngress(
  signals.router.browserHistory.load("/projects/p7", {
    routeIdentity: "project",
  }),
);

const story = signals.router.browserHistory.story(report);
const auditability = story.auditability();

console.log(report.outcome().verification());
console.log(story.inspection().verification());
console.log(auditability.verification());
```

The route outcome proves admission. Inspection proves the retained story
summary. Auditability proves the final visible-route explanation. Each package
stays at its own authority boundary.

## How It Relates To Other Features

- Read [Diagnostics Surfaces](./diagnostics_surfaces.md) when you need the
  human-readable explanation that pairs with these digests.
- Read [Projection Verification](../projection/projection_verification.md) for
  the route-shape side of the same proof vocabulary.
- Read [Provenance Statuses](./provenance_statuses.md) when the issue is status
  meaning rather than digest proof.

## Inspection And Debugging

- log the whole verification package in tests when you need an explainable
  failure
- compare the smallest digest that proves your claim
- prefer package-level proof over brittle string snapshots of full diagnostics

## Anti-Patterns

- treating verification packages as user-facing diagnostics
- parsing digests or depending on their string format
- comparing a higher-level package when a lower-level one already proves the
  claim

## Current Limits

- verification packages certify public boundaries; they do not expose private
  runtime state
- digest shape is opaque on purpose, so these packages are for comparison and
  certification, not human explanation

## Related Docs

- [Diagnostics Surfaces](./diagnostics_surfaces.md)
- [Projection Verification](../projection/projection_verification.md)
- [History Inspection](../history/history_inspection.md)
- [Navigation Auditability](../history/navigation_auditability.md)
