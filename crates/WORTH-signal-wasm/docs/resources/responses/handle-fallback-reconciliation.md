# Handle Fallback Reconciliation

Use this page when a response could not update some declared target exactly and
you need to know why.

## The Main Idea

Fallback reconciliation is not a hidden failure path. It is part of the
product contract.

Common fallback kinds include:

- `refetchRequired`
- `deliveryAwaited`
- `partialReconciliation`
- `placementUnavailable`
- `deletionUnavailable`
- `identityMigrationUnavailable`

## Stable Entry Points

- `line.mutationResponse()?.confirmation`
- `line.mutationResponse()?.targets`
- `line.summary().diagnostics.latest.mutationResponseFallbackReasonDigest`
- `line.summary().diagnostics.latest.mutationResponseTargetOutcomes`

## What It Usually Means

- `refetchRequired`
  The response did not prove canonical local truth.
- `deliveryAwaited`
  Canonical truth is expected from a later delivery packet.
- `partialReconciliation`
  Some declared targets updated exactly and others did not.
- `placementUnavailable`
  The create response did not prove a canonical insertion position.
- `deletionUnavailable`
  The remove response did not prove exact topology removal.

## Example

```ts
const response = line.mutationResponse();

console.log(response?.confirmation.kind);
console.log(
  response?.targets.map((target) => ({
    targetId: target.targetId,
    fallback: target.fallback.kind,
  })),
);
```

## How To Debug

Start compact:

- `line.summary().diagnostics.latest`

Go deeper when needed:

- `line.mutationResponse()`
- `line.history().verificationPackage().mutationResponse.plan`

## Related Docs

- [Understand Mutation Responses](./understand-mutation-responses.md)
- [Mutation Response Reconciliation](../mutation-response-reconciliation.md)
