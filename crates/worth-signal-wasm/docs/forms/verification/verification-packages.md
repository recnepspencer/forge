# Verification Packages

## What This Feature Is

This page covers `form.verification()`, the stable verification package for the
full form surface.

## Why You Use It

- certify that summary, history, and adjacent reports agree
- read the digest set for integration tests or debugging tools
- inspect performance-envelope counters without digging through internal
  runtime code

## Stable Entry Points

- `form.verification()`
- `form.diagnostics().verification`

## Core Mental Model

The verification package is the compact "everything lines up" surface. It does
not replace the feature reports. It ties them together and gives you one place
to check their digests and retained counts.

## How It Executes

1. feature reports are derived
2. digests and counters are collected
3. the verification package exposes the resulting proof set

## Small Example

```ts
const verification = form.verification();

console.log(verification.kind);
console.log(verification.packageDigest);
```

## Real Example

```ts
const verification = form.verification();

console.log({
  routeAuthority: verification.routeAuthorityContinuity,
  actionHistory: verification.actionHistory,
  diagnosticsHistory: verification.diagnosticsHistory,
  performanceEnvelope: verification.performanceEnvelope,
});
```

## How It Relates To Other Features

- Read [Diagnostics Summary](../diagnostics/diagnostics-summary.md) for the
  everyday summary view.
- Read the focused verification docs in this folder when you only need one part
  of the package.

## Inspection And Debugging

- `verification.digests` maps the major form feature families
- the history counters tell you retained-operation counts by family
- the performance envelope shows what the runtime had to inspect to produce the
  package

## Anti-Patterns

- using verification as your only normal UI read
- treating digest changes as if they were user-facing messages

## Current Limits

- verification is designed for debugging, testing, and consistency checks, not
  for rendering every screen state directly

## Related Docs

- [Action Verification](./action-verification.md)
- [Resource Verification](./resource-verification.md)
- [Route Authority Verification](./route-authority-verification.md)
