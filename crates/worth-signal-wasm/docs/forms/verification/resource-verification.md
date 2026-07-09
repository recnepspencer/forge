# Resource Verification

## What This Feature Is

This page covers the resource-backed proof lanes inside `form.verification()`.

## Why You Use It

- inspect the digest set for resource source, merge, drift, replay/restore, and
  reset
- confirm retained operation counts for resource-backed forms
- connect form-level proof to resource-backed behavior without dropping into
  lower resource internals

## Stable Entry Points

- `form.verification().digests.resourceSourceDigest`
- `form.verification().digests.resourceMergeDigest`
- `form.verification().digests.resourceDriftDigest`
- `form.verification().replayRestoreHistory`
- `form.verification().resetHistory`

## Core Mental Model

Resource-backed verification is the form-side proof for resource-owned draft
behavior. It mirrors the major resource-backed form surfaces without replacing
them.

## How It Executes

1. resource-backed form reports are derived
2. digests and retained history counts are collected
3. the verification package exposes them together

## Small Example

```ts
const verification = form.verification();

console.log(verification.digests.resourceSourceDigest);
console.log(verification.replayRestoreHistory.operations);
```

## Real Example

```ts
const verification = form.verification();

console.log({
  resourceSourceDigest: verification.digests.resourceSourceDigest,
  resourceMergeDigest: verification.digests.resourceMergeDigest,
  resourceDriftDigest: verification.digests.resourceDriftDigest,
  replayRestoreOperations: verification.replayRestoreHistory.operations,
  resetOperations: verification.resetHistory.operations,
});
```

## How It Relates To Other Features

- Read [Resource-Backed Forms](../resource-backed/README.md) for the actual
  feature surfaces.
- Read [Resource History](../diagnostics/resource-history.md) for the retained
  runtime histories.

## Inspection And Debugging

- use resource-source, merge, and drift digests to see which lane changed
- use replay/restore and reset operation counts when debugging retained history

## Anti-Patterns

- treating resource verification as if it proved every deeper resource line
  contract outside the form facade
- expecting resource digests to be non-null on non-resource-backed forms

## Current Limits

- this is form-scoped verification, not the entire resource subsystem proof

## Related Docs

- [Resource-Backed Forms](../resource-backed/README.md)
- [Resource History](../diagnostics/resource-history.md)
