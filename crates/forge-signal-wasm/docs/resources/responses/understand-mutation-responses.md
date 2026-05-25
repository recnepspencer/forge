# Understand Mutation Responses

Use this page when a `create(...)`, `update(...)`, or `remove(...)` response
should update related read lines for you.

## What This Is

A mutation response is the runtime's declared answer to:

- what the write response proved
- which related read lines were declared targets
- which targets updated exactly
- which targets stayed in fallback posture instead

The important part is that the runtime keeps this explicit. It does not hide
best-effort cache folklore behind a "successful save".

## Stable Entry Points

- `api.url(...).response(...).create(...)`
- `api.url(...).response(...).update(...)`
- `api.url(...).response(...).remove(...)`
- `line.mutationResponse()`
- `line.summary().diagnostics.latest`
- `line.history().verificationPackage()`

## Mental Model

The write line you hold after `create`, `update`, or `remove` is still the
write line. The response can also update declared read families such as:

- a detail line
- one item inside a collection or paged line
- a collection summary
- a migrated identity target

Forge builds one response plan and records the exact and fallback outcomes on
that plan.

## Small Example

```ts
const saveTask = api.url("/tasks/:taskId")
  .response(signals.resource.response.detail()({
    id: "id",
    status: "status",
  }))
  .update({
    reconciles: [{
      family: taskDetail,
      params: ({ taskId }) => ({ taskId }),
      fallback: "refetchRequired",
      detail: { kind: "field", field: "status" },
    }],
    load: ({ taskId }) => ({
      id: taskId,
      status: "reviewed",
    }),
  });

const line = saveTask.line({
  taskId: "task:1",
  body: {},
});

console.log(line.mutationResponse()?.confirmation.kind);
console.log(line.mutationResponse()?.targetCount);
```

## What To Inspect First

Start with:

- `line.mutationResponse()`
- `line.summary().diagnostics.latest.mutationResponseConfirmationKind`
- `line.summary().diagnostics.latest.mutationResponseTargetOutcomes`

Drop to `line.history().verificationPackage().mutationResponse.plan` when you
need the full plan artifact for tooling, tests, or a deeper debugging session.

## Related Docs

- [Handle Partial Canonical Truth](./handle-partial-canonical-truth.md)
- [Handle Fallback Reconciliation](./handle-fallback-reconciliation.md)
- [Mutation Response Reconciliation](../mutation-response-reconciliation.md)
