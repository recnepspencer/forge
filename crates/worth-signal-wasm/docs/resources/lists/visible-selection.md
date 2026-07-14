# Visible Selection

Use this page when you need to know which list truth is currently visible:
loaded truth, speculative truth, confirmed delivery, or restored history.

## Stable Entry Points

- `line.diagnostics().visibleSelection`
- `line.diagnosticsSummary().current.visibleSelection`
- `line.history().lifecycle`
- `line.history().verificationPackage().continuity.visibleSelection`

## What It Names

Visible selection can be:

- `unavailable`
- `committed`
- `speculative`
- `confirmed`
- `restored`

That is the runtime's answer to "which version of this line is the user
currently looking at?"

## Example

```ts
const line = tasks.line({});

console.log(line.diagnostics().visibleSelection.kind);

line.patch(
  tasks.patch.itemAspect({
    itemId: "t1",
    aspect: "title",
    value: "Draft",
  }),
);

console.log(line.diagnostics().visibleSelection.kind);
```

## Why It Matters

Visible selection is how the resource runtime keeps optimistic list behavior
honest:

- speculative UI truth is named as speculative
- confirmed delivery is named as confirmed
- rollback and restore are named as restored

## Related Docs

- [Update One Item Without Replacing Everything](./update-one-item-without-replacing-everything.md)
- [Branch-Native Resource Effects](../branch-native-effects.md)
