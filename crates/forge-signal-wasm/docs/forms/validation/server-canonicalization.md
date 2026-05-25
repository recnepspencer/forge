# Server Canonicalization

## What This Feature Is

Server canonicalization is the lane where an action completion replaces source
truth with a canonical server value and updates draft posture accordingly.

## Why You Use It

- accept server-owned canonical values after submit
- explain why source and draft changed after action fulfillment
- inspect canonicalization history instead of guessing from state diffs

## Stable Entry Points

- `form.fulfillAction(operationId, { canonicalValue })`
- `form.verification()`
- `form.diagnosticsHistory()`

## Core Mental Model

Canonicalization is not just "submit succeeded." It is the specific transition
where the runtime accepts a canonical source value and reconciles draft truth
against it.

## How It Executes

1. an action runs
2. the action completion supplies a canonical value
3. the runtime updates source truth
4. draft truth is cleared or reconciled according to the canonicalization lane
5. verification and diagnostics history retain the canonicalization result

## Small Example

```ts
const pending = form.executeAction("submit");

form.fulfillAction(pending.operationId, {
  canonicalValue: { title: "Ship docs", status: "published" },
});
```

## Real Example

```ts
const pending = form.executeAction("submit");

form.fulfillAction(pending.operationId, {
  canonicalValue: { title: "Ship docs", status: "published" },
});

console.log(form.source());
console.log(form.effective());
console.log(form.verification().digests.canonicalizationDigest);
```

## How It Relates To Other Features

- Read [Async Validation](./async-validation.md) when the pending lifecycle is
  still validator-owned rather than action-owned.
- Read [Source Compatibility And Draft Migration](./source-compatibility-and-draft-migration.md)
  when the source changed because schema versions drifted instead of because a
  canonical action completed.

## Inspection And Debugging

- `verification().digests.canonicalizationDigest` is the stable proof read
- `diagnosticsHistory()` retains canonicalization-linked diagnostics history
- compare `source()` and `draft()` after fulfillment when canonical changes are
  surprising

## Anti-Patterns

- treating canonicalization like an arbitrary local state overwrite
- assuming canonical completion always preserves draft

## Current Limits

- resource-backed canonicalization and mutation-response posture are part of a
  later resource-backed forms section

## Related Docs

- [Async Validation](./async-validation.md)
- [Source Compatibility And Draft Migration](./source-compatibility-and-draft-migration.md)
