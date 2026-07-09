# Mutation-Response Closeout Matrix

## What This Feature Is

This page covers the support matrix for response-owned reconciliation after
create, update, and remove operations.

## Why You Use It

Use it when you need to answer:

- is this a happy exact target path or an honest fallback?
- what kind of denial-only lane is this?
- what does the runtime claim to support for this mutation-response shape?

## Stable Entry Points

- `signals.resource.mutationResponses.closeoutMatrix()`

## Core Mental Model

The closeout matrix is not one execution artifact. It is the support contract
for the response-owned reconciliation feature family.

## How It Executes

The matrix groups support rows for:

- exact target placement
- fallback reconciliation
- stale target handling
- delivery-awaited and refetch-required posture
- partial reconciliation
- unsupported target posture

## Small Example

```ts
const matrix = signals.resource.mutationResponses.closeoutMatrix();
console.log(matrix.rows.length);
```

## Real Example

```ts
const matrix = signals.resource.mutationResponses.closeoutMatrix();

console.log(matrix.rows.map((row) => ({
  lane: row.lane,
  category: row.category,
  summary: row.summary,
})));
```

## How It Relates To Other Features

- Use [Handling Server Responses](../responses/README.md) for the task-first
  response docs.
- Use [Read Mutation Responses In Forms](../forms/read-mutation-responses-in-forms.md)
  when the response truth is surfacing through a resource-backed form.

## Inspection And Debugging

Use the matrix when someone asks "does the product support this response shape
as an exact path, or only as a fallback?"

## Anti-Patterns

- Do not confuse the closeout matrix with the runtime plan on one line.
- Do not flatten honest fallback into "supported exactly."

## Current Limits

The matrix is a support statement, not an excuse to skip checking the concrete
mutation-response plan or digests on a real line.

## Related Docs

- [Mutation Response Closeout Matrix](../../resource-contracts/mutation-response-closeout-matrix.md)
- [Understand Mutation Responses](../responses/understand-mutation-responses.md)
