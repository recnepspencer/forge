# Handling Server Responses

Use this section when a write already ran and you need to understand what the
server response changed, what stayed explicit as fallback, and where that
evidence lives.

## Start Here

- [Understand Mutation Responses](./understand-mutation-responses.md)
  The normal mental model for response-owned reconciliation.
- [Handle Partial Canonical Truth](./handle-partial-canonical-truth.md)
  When the response proves some local truth, but not all of it.
- [Handle Fallback Reconciliation](./handle-fallback-reconciliation.md)
  When the runtime stays honest instead of pretending it updated everything.
- [Map Server Truth Back Into Local Truth](./map-server-truth-back-into-local-truth.md)
  Choosing detail, item, summary, delete, tombstone, and identity-migration
  targets.

## Common Questions

- "Why did this save update one view but not another?"
  [Handle Fallback Reconciliation](./handle-fallback-reconciliation.md)
- "Why is this still partial instead of exact?"
  [Handle Partial Canonical Truth](./handle-partial-canonical-truth.md)
- "Where do I inspect what the response actually did?"
  [Understand Mutation Responses](./understand-mutation-responses.md)
- "How do remove responses delete versus retain a tombstone?"
  [Map Server Truth Back Into Local Truth](./map-server-truth-back-into-local-truth.md)
- "How do creates migrate a draft id to a canonical id?"
  [Map Server Truth Back Into Local Truth](./map-server-truth-back-into-local-truth.md)

## Deeper History

The older deeper page is still available while this tree is being rewritten:

- [Mutation Response Reconciliation](../mutation-response-reconciliation.md)
