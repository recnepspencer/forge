# Map Server Truth Back Into Local Truth

Use this page when you are deciding what a response should update after a
write.

## Pick The Narrowest Truth The Response Really Proves

Common target shapes:

- detail `replace`
- detail `field`
- detail `region`
- detail `jsonPath`
- collection `item`
- collection `insert`
- collection `delete`
- summary `summary`
- identity migration

The route declaration should say what the response actually proved, not what
the app hopes happened.

## Example

```ts
const saveTask = api.url("/tasks/:taskId")
  .response(signals.resource.response.detail()({
    id: "id",
    status: "status",
    version: "version",
  }))
  .update({
    reconciles: [{
      family: taskDetail,
      params: ({ taskId }) => ({ taskId }),
      fallback: "refetchRequired",
      detail: { kind: "field", field: "status" },
    }, {
      family: taskList,
      params: () => ({}),
      fallback: "refetchRequired",
      collection: { kind: "item" },
    }, {
      family: taskList,
      params: () => ({}),
      fallback: "partialReconciliation",
      summary: { kind: "summary", summary: "version" },
    }],
    load: ({ taskId }) => ({
      id: taskId,
      status: "reviewed",
      version: 7,
    }),
  });
```

## Rule Of Thumb

- If the response proved one field, declare one field.
- If it proved one visible item, declare one item.
- If a remove response should keep a visible deleted record, use collection
  `item` and let the canonical item value become a tombstone instead of
  pretending the item disappeared.
- If a create response changes draft identity, declare identity migration
  explicitly instead of remapping ids in feature code.
- If it proved collection metadata, declare a summary target.
- If it did not prove exact local truth, declare the fallback honestly.

## Related Docs

- [Understand Mutation Responses](./understand-mutation-responses.md)
- [Handle Partial Canonical Truth](./handle-partial-canonical-truth.md)
- [Collections And Delivery](../collections-and-delivery.md)
