# Async Lifecycle And Canonicalization

## What This Feature Is

This feature owns async validation, action execution lifecycle, stale-safe
completions, server rejection mapping, schema compatibility posture, and
canonicalization after fulfillment.

## Why You Use It

- keep async completions from rewriting newer form truth
- map server field rejections onto the same message and summary lanes as local
  validation
- make canonical server values explicit instead of silently rewriting draft
  state

## Stable Entry Points

- async validation builders
- `startAsyncValidation(...)`
- `fulfillAsyncValidation(...)`
- `rejectAsyncValidation(...)`
- `cancelAsyncValidation(...)`
- `timeoutAsyncValidation(...)`
- `executeAction(actionId)`
- `fulfillAction(operationId, payload?)`
- `rejectAction(operationId, payload?)`
- `cancelAction(operationId, payload?)`
- `timeoutAction(operationId, payload?)`
- `retryAction(operationId)`
- `asyncValidationHistory()`
- `canonicalizationHistory()`
- `sourceAdmission()`
- `draftRestore()`
- `sourceCompatibility()`
- `sourceCompatibilityHistory()`

## Core Mental Model

Async lifecycle is runtime-owned. A completion only applies if it still
matches the current operation and basis truth. Canonicalization is explicit and
retained instead of silently rewriting the draft. Schema drift is also
explicit: current, compatible, migrated, or unavailable.

## How It Executes

The runtime records pending async artifacts, compares later completions against
current truth, rejects stale completions structurally, maps server results into
typed artifacts, then records canonicalization and compatibility evidence when
the result changes source or draft posture.

## Small Example

```ts
const pending = form.startAsyncValidation("title-remote-check");

const fulfilled = form.fulfillAsyncValidation(pending.operationId, {
  reason: "server accepted title",
});

console.log(pending.resultKind);
console.log(fulfilled.resultKind);
```

This is the smallest honest example because it shows the lifecycle artifacts
the runtime retains, not just the final boolean outcome.

## Real Example

```ts
const execution = form.executeAction("submit");

if (execution.resultKind === "pending") {
  form.fulfillAction(execution.operationId, {
    reason: "server canonicalized title",
    canonicalValue: { title: "Published docs", status: "done" },
  });
}

console.log(form.canonicalizationHistory());
console.log(form.sourceCompatibility());
```

The runtime records whether fulfillment updated canonical source truth, whether
the draft was preserved, and whether schema compatibility changed along the
way.

## How It Relates To Other Features

- Pair it with [Actions And Submit](./actions-and-submit.md) because action
  execution uses this lifecycle lane.
- Pair it with [Validation And Messages](./validation-and-messages.md) because
  server rejections land in the same message and validation surfaces.
- Pair it with [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
  when you need retained lifecycle evidence.

## Inspection And Debugging

- `asyncValidationHistory()` shows lifecycle result kinds such as fulfilled,
  cancelled, timed out, superseded, and stale completion.
- `canonicalizationHistory()` shows explicit source/draft/effective changes.
- `sourceAdmission()` and `draftRestore()` show whether the form is still
  entering or restoring before the visible lifecycle can settle.
- `sourceCompatibility()` and `sourceCompatibilityHistory()` show schema drift
  posture and migration evidence.

## Anti-Patterns

- treating stale async completions as harmless and applying them anyway
- mutating draft values directly when the server returns canonical truth
- collapsing source-compatibility posture into one generic "version mismatch"

## Current Limits

- route-coupled step transitions remain deferred to route authority
- compatibility posture is explicit even when migration is unavailable
- async lifecycle does not own resource authority; resource-line submit still
  composes with resource proof rather than replacing it

## Related Docs

- [Actions And Submit](./actions-and-submit.md)
- [Validation And Messages](./validation-and-messages.md)
- [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
