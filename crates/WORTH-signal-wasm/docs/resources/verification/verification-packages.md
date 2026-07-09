# Verification Packages

## What This Feature Is

This page covers the retained verification artifact exposed by
`line.history().verificationPackage()`.

## Why You Use It

Use it when you need one stable digest package for:

- declaration truth
- request posture
- lifecycle and continuity
- reconciliation breadth
- diagnostics summary proof
- retained history and exact recovery availability

## Stable Entry Points

- `line.history().verificationPackage()`

## Core Mental Model

The verification package is the proof-bearing snapshot of the line boundary.
It is not a replacement for ordinary line reads, but it is the stable artifact
for comparison and audit.

## How It Executes

The package groups:

- `declaration`
- `requestPosture`
- `processing`
- `upload`
- `lifecycle`
- `continuity`
- `reconciliation`
- `diagnostics`
- `historyReplayRestore`
- `binaryDownload`
- `deliveryProvenance`
- `externalCompatibility`
- `boundaryPerformanceEnvelope`
- `capabilities`
- `typedDenials`
- optional `mutationResponse`

## Small Example

```ts
const verification = line.history().verificationPackage();

console.log(verification.declaration.familyKind);
console.log(verification.capabilities.summary);
```

## Real Example

```ts
const verification = line.history().verificationPackage();

console.log(verification.requestPosture);
console.log(verification.lifecycle);
console.log(verification.reconciliation);
console.log(verification.historyReplayRestore.availability);
console.log(verification.boundaryPerformanceEnvelope.summaryReadShape);
```

## How It Relates To Other Features

- Use [Inspecting And Debugging Resources](../debugging/README.md) for the
  friendlier grouped read.
- Use [Delivery And Compatibility Digests](./delivery-and-compatibility-digests.md)
  when that part of the package is the main question.

## Inspection And Debugging

Start with grouped reads for everyday debugging. Drop to the verification
package when you need one stable artifact to compare two runs or prove a line's
capabilities and denials.

## Anti-Patterns

- Do not treat the verification package as normal UI payload.
- Do not skip ordinary line reads when the question is just "what is happening
  right now?"

## Current Limits

The package is intentionally broad and retained. It is best for proof,
comparison, and audit, not for every ordinary render.

## Related Docs

- [Inspection And History Contract](../../resource-contracts/inspection-and-history.md)
- [Line Facade Stability Test](../../package/product/resource_runtime/authoring/line_facade_stability.test.mjs)
