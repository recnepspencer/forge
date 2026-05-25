# Reflect Resource Settlement In A Form

## What This Feature Is

This page explains how a form reads pending, confirmed, failed, retry, and
visible-selection posture from its resource-backed source.

## Why You Use It

Use it when you need UI that reacts to:

- pending initial load
- confirmed delivery
- failed refresh or submit follow-up
- retry recommendation
- visible-selection changes such as `speculative`, `confirmed`, or `restored`

## Stable Entry Points

- `form.resourceSource()?.settlement`
- `form.resourceSource()?.lifecycle`
- `form.resourceSource()?.visibleSelection`
- `form.readiness()`

## Core Mental Model

Settlement is not just status. It combines:

- line status
- freshness
- confirmation posture
- visible selection
- retry recommendation

## How It Executes

The resource-backed report turns raw line status plus mutation-response and
visible-selection evidence into one form-friendly settlement artifact.

## Small Example

```ts
const settlement = form.resourceSource()?.settlement;

console.log(settlement?.kind);
console.log(settlement?.retryRecommended);
```

## Real Example

```ts
const source = form.resourceSource();

console.log(source.lifecycle.retry.kind);
console.log(source.lifecycle.deliveryBasis.kind);
console.log(source.settlement.confirmationKind);
console.log(source.visibleSelection.kind);
```

## How It Relates To Other Features

- Use [Use A Resource As Form Source](./use-a-resource-as-form-source.md) for
  the broader source report.
- Use [Read Mutation Responses In Forms](./read-mutation-responses-in-forms.md)
  when the important question is write completion detail.
- Use [Check Status, Freshness, And History](../debugging/check-status-settlement-and-history.md)
  when you want the raw resource-line read without the form-side settlement
  projection.

## Inspection And Debugging

If a form is unexpectedly blocked, inspect:

- `resourceSource().settlement`
- `resourceSource().lifecycle.retry`
- `form.readiness().blockers`

## Anti-Patterns

- Do not guess retry posture from status alone.
- Do not flatten speculative, confirmed, and restored visible truth into one
  generic "loaded" state.

## Current Limits

Settlement stays resource-owned. The form summarizes it, but does not invent
new resource lifecycle states.

## Related Docs

- [Resource Settlement](../../forms/resource-backed/resource-settlement.md)
- [Read Mutation Responses In Forms](./read-mutation-responses-in-forms.md)
