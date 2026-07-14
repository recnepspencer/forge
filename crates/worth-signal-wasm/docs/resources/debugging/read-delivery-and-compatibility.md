# Read Delivery And Compatibility

## What This Feature Is

This page covers the reads that explain externally delivered packets, basis
refresh, and compatibility-owned resource lines.

## Why You Use It

Use it when you need to answer:

- did this line update from normal runtime ownership or external delivery?
- is this a native resource line or an external definition?
- did basis drift or a packet refresh change what the line is showing?

## Stable Entry Points

- `line.diagnosticsSummary().latest`
- `line.history().verificationPackage().deliveryProvenance`
- `line.history().verificationPackage().externalCompatibility`

## Core Mental Model

Delivery and compatibility are separate truths:

- delivery says how new value reached the line
- compatibility says what kind of line contract is backing the surface

You often need both when a pushed packet or external definition is involved.

## How It Executes

The verification package keeps:

- the latest delivery kind and scope
- packet and basis ids
- basis advance counts
- compatibility kind, version, and reconciliation contract

## Small Example

```ts
const verification = line.history().verificationPackage();

console.log(verification.deliveryProvenance.lastDeliveryKind);
console.log(verification.externalCompatibility.kind);
```

## Real Example

```ts
const latest = line.diagnosticsSummary().latest;
const verification = line.history().verificationPackage();

console.log(latest.deliveryKind);
console.log(latest.deliveryBasisId);
console.log(verification.deliveryProvenance.basisAdvanceCount);
console.log(verification.externalCompatibility.kind);
console.log(verification.externalCompatibility);
```

## How It Relates To Other Features

- Use [External Delivery And Compatibility](../external-delivery-and-compatibility.md)
  for the older deeper contract page.
- Use [Caching And Refresh](../caching/README.md) when basis drift becomes a
  stale/fresh question.
- Use [Forms Integration](../forms/README.md) when this same line is the source
  of a resource-backed form.

## Inspection And Debugging

If a line looks "mysteriously updated," inspect:

- latest delivery kind
- delivery scope
- basis ids
- external compatibility kind

## Anti-Patterns

- Do not treat external definitions as if they were native internal lines.
- Do not read packet delivery and compatibility from request posture alone.

## Current Limits

Compatibility explains contract shape. It does not replace the ordinary line
inspection surfaces.

## Related Docs

- [External Delivery And Compatibility](../external-delivery-and-compatibility.md)
- [Forms](../forms/README.md)
