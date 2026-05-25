# Delivery And Compatibility Digests

## What This Feature Is

This page covers the verification-package parts that explain delivery
provenance, compatibility kind, capability truth, and typed denials.

## Why You Use It

Use it when you need to compare or audit:

- delivery basis movement
- external compatibility posture
- what the line honestly supports
- typed replay/branch/restore denials

## Stable Entry Points

- `line.history().verificationPackage().deliveryProvenance`
- `line.history().verificationPackage().externalCompatibility`
- `line.history().verificationPackage().capabilities`
- `line.history().verificationPackage().typedDenials`

## Core Mental Model

These digests answer four different questions:

- how updates arrived
- what compatibility contract backs the line
- what the line can do
- which exact retained-history paths are denied, and why

## How It Executes

The package keeps:

- delivery kind, scope, packet id, basis ids, and last effect
- compatibility kind and contracts
- capability booleans for patch/deliver/reconciliation breadth
- typed denial artifacts for replay, replayExact, branch, and restoreExact

## Small Example

```ts
const verification = line.history().verificationPackage();

console.log(verification.externalCompatibility.kind);
console.log(verification.capabilities.patch);
```

## Real Example

```ts
const verification = line.history().verificationPackage();

console.log(verification.deliveryProvenance);
console.log(verification.externalCompatibility);
console.log(verification.capabilities);
console.log(verification.typedDenials);
```

## How It Relates To Other Features

- Use [Read Delivery And Compatibility](../debugging/read-delivery-and-compatibility.md)
  for the friendlier debugging view.
- Use [Raw Resource Lines](../advanced/raw-resource-lines.md) when capability
  parity between route-built and raw-built lines matters.

## Inspection And Debugging

These digests are the best lane when the question is "what was this line even
allowed to do?" or "why was exact recovery denied here?"

## Anti-Patterns

- Do not collapse typed denials into one generic unavailable message.
- Do not assume compatibility kind says anything about current patch/delivery
  capability on its own.

## Current Limits

Capability and denial digests are proof-bearing summaries. They do not replace
the underlying line, diagnostics, or history reads.

## Related Docs

- [Delivery And Compatibility Contract](../../resource-contracts/delivery-and-compatibility.md)
- [Verification Packages](./verification-packages.md)
