# Continuity Audit

## What This Feature Is

This page covers the named public summary of route-authority continuity across
diagnostics and verification.

## Why You Use It

- get one compact explanation of route-coupled behavior
- inspect handoff posture, draft resolution, and route-coupled step/action
  availability together
- avoid rebuilding this summary from several lower-level reports

## Stable Entry Points

- `form.diagnosticsSummary().routeAuthority.continuityAudit`
- `form.verification().routeAuthorityContinuity`

## Core Mental Model

The continuity audit is the compact public answer to:

"What is route authority doing to this form right now?"

## How It Executes

1. route authority summary is derived
2. route-coupled step/action posture is scanned
3. the continuity audit is materialized into diagnostics and verification

## Small Example

```ts
const audit = form.diagnosticsSummary().routeAuthority.continuityAudit;

console.log(audit.handoffPosture);
console.log(audit.draftResolution);
console.log(audit.blockingReason);
```

## Real Example

```ts
const audit = form.verification().routeAuthorityContinuity;

console.log({
  handoff: audit.handoffPosture,
  routeCoupledBehavior: audit.routeCoupledBehavior,
  draftDisposition: audit.draftDisposition,
  draftResolution: audit.draftResolution,
  routeCoupledActions: audit.routeCoupledActions,
  blockingReason: audit.blockingReason,
});
```

## How It Relates To Other Features

- Read [Diagnostics Summary](../diagnostics/diagnostics-summary.md) for the
  broader diagnostics surface.
- Read [Route Authority Verification](../verification/route-authority-verification.md)
  for the same continuity story inside the verification package.

## Inspection And Debugging

- diagnostics summary is the lighter everyday read
- verification is the package meant for testing and deeper consistency checks
- both expose the same continuity story on purpose

## Anti-Patterns

- rebuilding the continuity audit manually from actions, steps, and route
  authority history
- treating the continuity audit as a substitute for full route-authority
  history when you need chronological detail

## Current Limits

- the continuity audit is a summary, not a full transition log

## Related Docs

- [Diagnostics Summary](../diagnostics/diagnostics-summary.md)
- [Route Authority Verification](../verification/route-authority-verification.md)
