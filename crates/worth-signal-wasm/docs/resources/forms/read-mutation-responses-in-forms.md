# Read Mutation Responses In Forms

## What This Feature Is

This page explains the form-side digest of resource mutation-response work.

## Why You Use It

Use it when you need to show or debug:

- confirmation kind
- target counts
- fallback versus exact targets
- placement and tombstone summary
- multi-family mutation-response completion

## Stable Entry Points

- `form.resourceSource()?.mutationResponse`
- `form.verification().digests.resourceMutationResponseDigest`
- `form.verification().digests.resourceMutationResponseContractDigest`

## Core Mental Model

The form does not expose the raw mutation-response plan directly as the common
read. It exposes a compact report that preserves the important completion and
contract digests.

## How It Executes

The resource-backed source report includes:

- `confirmationKind`
- `targetOutcomes`
- fallback and exact counts
- contract digests
- completion digests
- identity-migration summary when relevant

## Small Example

```ts
const response = form.resourceSource()?.mutationResponse;

console.log(response?.confirmationKind);
console.log(response?.planCount);
```

## Real Example

```ts
const response = form.resourceSource()?.mutationResponse;

console.log(response?.completion.multiFamily);
console.log(response?.completion.placement.kind);
console.log(response?.completion.deletion.kind);
console.log(response?.contract.digest);
```

## How It Relates To Other Features

- Use [Handling Server Responses](../responses/README.md) for the resource-side
  meaning of mutation responses.
- Use [Mutation Response Readback](../../forms/resource-backed/mutation-response-readback.md)
  for the deeper form-side page.

## Inspection And Debugging

When a resource-backed submit feels surprising, inspect:

- confirmation kind
- exact versus fallback counts
- placement/deletion summary
- verification digests

## Anti-Patterns

- Do not assume every form-backed write is single-family or exact-only.
- Do not discard the contract digests if your debugging or audit flow needs to
  prove what was admitted.

## Current Limits

This is a summarized form read, not a replacement for the raw resource
mutation-response plan when you need topology-level detail.

## Related Docs

- [Understand Mutation Responses](../responses/understand-mutation-responses.md)
- [Mutation Response Readback](../../forms/resource-backed/mutation-response-readback.md)
