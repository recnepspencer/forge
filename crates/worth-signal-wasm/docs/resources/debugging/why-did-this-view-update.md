# Why Did This View Update?

## What This Feature Is

This page is about reading the last patch, delivery, visible-selection, and
effect evidence that explains a visible change.

## Why You Use It

Use it when a user asks:

- why did this row change?
- why did this nested field update?
- why did the line switch from speculative to confirmed?
- was this change local, delivered, restored, or merged?

## Stable Entry Points

- `line.diagnosticsSummary().latest`
- `line.summary().diagnostics.latest`
- `line.diagnostics().lastEffect`
- `line.history().verificationPackage().deliveryProvenance`
- `line.history().verificationPackage().reconciliation`

## Core Mental Model

There are two common sources of visible change:

- a local or delivered patch/effect
- a server-owned delivery or confirmation change

The runtime keeps both the compact latest-change read and the deeper proof
package so you can explain either one without inventing your own side channel.

## How It Executes

The latest diagnostics summary tells you the newest visible change in compact
form. The verification package tells you the exact delivery scope, patch scope,
effect envelope, basis id, and reconciliation breadth behind it.

## Small Example

```ts
const summary = line.summary();
const latest = summary.diagnostics.latest;

console.log(latest.deliveryKind);
console.log(latest.deliveryScope);
console.log(summary.current.visibleSelection.kind);
```

## Real Example

```ts
const latest = line.diagnosticsSummary().latest;
const verification = line.history().verificationPackage();

console.log(latest.deliveryKind);
console.log(latest.deliveryScope);
console.log(verification.deliveryProvenance.lastDeliveryScope);
console.log(verification.reconciliation.lastPatchKind);
console.log(line.diagnostics().lastEffect?.provenance);
```

## How It Relates To Other Features

- Use [Partial Updates And Derived Views](../partial-updates/README.md) when
  the important question is how a narrow patch was declared.
- Use [Effects](../effects/README.md) when the visible change was
  branch-speculative or merge-aware.
- Use [Working With Lists](../lists/README.md) when the question is item or
  summary updates inside a collection.

## Inspection And Debugging

Start with `diagnosticsSummary().latest`. If that is not enough, inspect:

- `line.diagnostics().lastEffect`
- `line.history().verificationPackage().deliveryProvenance`
- `line.history().verificationPackage().reconciliation`

## Anti-Patterns

- Do not infer a visible update only from `value()`.
- Do not treat `lastEffect` as the only reason a view changes. Delivery and
  basis refresh can move visible truth too.

## Current Limits

The compact latest read explains the newest change, not the entire lifecycle of
the line. Use history when the sequence matters.

## Related Docs

- [Why Didn't This View Update?](./why-didnt-this-view-update.md)
- [Branch-Native Effects](../effects/branch-native-effects.md)
