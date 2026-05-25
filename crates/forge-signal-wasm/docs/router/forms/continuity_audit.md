# Continuity Audit

## What This Feature Is

Continuity audit is the public diagnostics and verification story for the
router-to-forms authority seam.

## Why You Use It

- inspect route-authority continuity from diagnostics summaries
- verify the same continuity story through the verification package
- keep route-authority blockers distinct from unrelated blockers

## Stable Entry Points

- `form.diagnosticsSummary().routeAuthority.continuityAudit`
- `form.diagnosticsHistory()`
- `form.verification().routeAuthorityContinuity`

## Core Mental Model

The audit is the public explanation surface. It summarizes:

- handoff posture
- route-coupled behavior
- draft resolution
- route-coupled step and action counts
- the route-authority blocking reason

## How It Executes

1. forms derive route authority, handoff, and draft continuity
2. diagnostics summary and history retain that state
3. verification exposes the same continuity audit as proof of the public seam

## Small Example

```ts
const audit = form.diagnosticsSummary().routeAuthority.continuityAudit;
console.log(audit.blockingReason);
```

## Real Example

```ts
const summary = form.diagnosticsSummary();
const verification = form.verification();

console.log(summary.routeAuthority.continuityAudit);
console.log(verification.routeAuthorityContinuity);
```

## How It Relates To Other Features

- the underlying seam starts in [Route Authority Handoff](./route_authority_handoff.md)
- draft-specific meaning is in [Draft Continuity](./draft_continuity.md)

## Inspection And Debugging

- `diagnosticsSummary().routeAuthority.continuityAudit`
- `diagnosticsHistory()`
- `verification().routeAuthorityContinuity`

## Anti-Patterns

- surfacing a host or offline blocker as if it explained route authority
- auditing low-level route authority without also checking diagnostics or
  verification surfaces

## Current Limits

- the audit is scoped to the router-to-forms seam, not every form readiness
  rule

## Related Docs

- [Route Authority Handoff](./route_authority_handoff.md)
- [Draft Continuity](./draft_continuity.md)
