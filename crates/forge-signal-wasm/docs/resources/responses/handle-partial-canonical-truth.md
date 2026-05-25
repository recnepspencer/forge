# Handle Partial Canonical Truth

Use this page when the server response proves some local truth exactly, but not
enough to replace everything.

## What Partial Means

`partialCanonicalTruth` means the runtime consumed real server truth, but only
for the declared parts the response actually proved.

That usually happens when:

- one detail field was present but other declared fields were not
- one related target updated exactly while another stayed fallback
- the response proved one summary but not another

## Stable Entry Points

- `line.mutationResponse()?.confirmation.kind`
- `line.mutationResponse()?.partialAdmission`
- `line.summary().diagnostics.latest.mutationResponseTargetOutcomes`

## Example

```ts
const saveTask = api.url("/tasks/:taskId")
  .response(signals.resource.response.detail()({
    status: "status",
  }))
  .update({
    reconciles: [{
      family: taskDetail,
      params: ({ taskId }) => ({ taskId }),
      fallback: "partialReconciliation",
      detail: { kind: "field", field: "status" },
    }, {
      family: taskList,
      params: () => ({}),
      fallback: "partialReconciliation",
      summary: { kind: "summary", summary: "version" },
    }],
    load: ({ taskId }) => ({
      id: taskId,
      status: "published",
    }),
  });

const line = saveTask.line({
  taskId: "task:1",
  body: {},
});

console.log(line.mutationResponse()?.confirmation.kind);
console.log(line.mutationResponse()?.partialAdmission);
```

The detail target can still update exactly while the summary target stays
explicitly partial.

## What Not To Do

Do not describe `partialCanonicalTruth` like a full exact cache update.

If the response omitted some declared proof, the runtime is doing the right
thing by keeping that gap visible.

## Related Docs

- [Handle Fallback Reconciliation](./handle-fallback-reconciliation.md)
- [Map Server Truth Back Into Local Truth](./map-server-truth-back-into-local-truth.md)
