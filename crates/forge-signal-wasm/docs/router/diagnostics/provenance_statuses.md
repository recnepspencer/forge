# Provenance Statuses

## What This Feature Is

Provenance statuses are the router's public labels for where breadcrumb truth
came from, alongside the route-outcome provenance surfaces that explain
admission and recovery decisions. They let you inspect derived navigation truth
without pretending everything is just "resolved."

## Why You Use It

- tell route declaration breadcrumbs apart from carried or restored ancestry
- inspect whether an outcome was admitted directly, blocked by prerequisites,
  or recovered
- keep fallback and recovered truth explicit instead of hiding it in UI labels

## Stable Entry Points

- `breadcrumbEntry.status`
- `breadcrumbEntry.sourceKind`
- `breadcrumbEntry.provenance()`
- `outcome.provenance()`
- `inspection.breadcrumbProvenance()`

## Core Mental Model

The router distinguishes two related but different ideas:

- **status**: what state this breadcrumb entry is in now, such as `resolved`,
  `carried`, or `fallback`
- **source kind**: which truth lane produced it, such as `routeDeclaration`,
  `carriedProvenance`, `restoredProvenance`, or `historyFallback`

For route outcomes, the equivalent explanation is not a status field. It lives
in `terminalSource`, `prerequisiteDecisions`, and `recoveryTrail`.

## How It Executes

1. route authoring, carry, restore, or history fallback produces breadcrumb
   truth
2. the router assigns a public status and source kind
3. route admission records whether truth came from direct admission,
   prerequisite artifacts, or recovered outcome
4. provenance surfaces retain that classification for later inspection

## Small Example

```ts
if (outcome.kind === "admitted") {
  const trail = outcome.route().breadcrumbTrail();

  console.log(trail?.entries[0].status);
  console.log(trail?.entries[0].sourceKind);
}
```

This is the smallest honest example because breadcrumb provenance is one of the
most common places where status meaning matters immediately.

## Real Example

```ts
const report = await routes.admitBrowserHistoryIngress(
  signals.router.browserHistory.load("/search/results/r17", {
    carriedBreadcrumbs,
  }),
);

console.log(report.outcome().provenance().terminalSource);
console.log(report.outcome().provenance().recoveryTrail);

if (report.outcome().kind === "admitted") {
  console.log(
    report
      .outcome()
      .route()
      .breadcrumbTrail()
      ?.entries.map((entry) => ({
        id: entry.crumbId,
        status: entry.status,
        sourceKind: entry.sourceKind,
      })),
  );
}
```

In this lane, one route outcome can still be directly admitted while one parent
breadcrumb is carried provenance and another is a route declaration. The router
keeps those meanings separate for you.

## How It Relates To Other Features

- Read [Carried Provenance](../breadcrumbs/carried_provenance.md) and
  [Restored Provenance](../breadcrumbs/restored_provenance.md) for how those
  breadcrumb statuses are produced.
- Read [Recovery Provenance](../recovery/recovery_provenance.md) when the
  important provenance question is nearest-valid route recovery.
- Read [Diagnostics Surfaces](./diagnostics_surfaces.md) when you need the
  surrounding explanation surfaces, not just the vocabulary.

## Inspection And Debugging

- check `status` when you care about present breadcrumb posture
- check `sourceKind` when you care about the truth lane that produced it
- check `restoreAvailability` and `replayAvailability` on breadcrumb
  provenance when the crumb should support exact return
- check `terminalSource`, `prerequisiteDecisions`, and `recoveryTrail` on
  route outcomes when admission truth is the real question

## Anti-Patterns

- collapsing `carried`, `restored`, and `fallback` into one generic
  "non-standard" bucket
- showing recovered ancestry as if it came from the current URL shape
- treating breadcrumb status as if it explained route-outcome provenance too

## Current Limits

- breadcrumb status vocabulary is intentionally small; richer explanation stays
  in provenance artifacts
- route-outcome provenance explains router decisions, not app-level business
  analytics

## Related Docs

- [Breadcrumb Entries](../breadcrumbs/breadcrumb_entries.md)
- [Carried Provenance](../breadcrumbs/carried_provenance.md)
- [Restored Provenance](../breadcrumbs/restored_provenance.md)
- [Recovery Provenance](../recovery/recovery_provenance.md)
