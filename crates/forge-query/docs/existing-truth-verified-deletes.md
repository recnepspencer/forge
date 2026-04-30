# Existing-Truth Verified Deletes

Use `workspace.delete_existing_verified(...)` when an existing-target delete
must prove current authoritative values immediately before the mutation
executes.

## Shape

```rust
let binding = workspace.bind_existing_entity(
    ForgeQueryExistingEntityTarget::new("authority:task-1", resolved_identity)?
        .in_target_collection("Task")?,
)?;

let receipt = workspace.delete_existing_verified(
    binding,
    |verify| verify.aspect("status.value", "closed"),
    |delete| delete.touch("status.value"),
)?;
```

The first closure declares the backend-verified precondition. The second closure
declares the actual delete-family fallout meaning.

## Contract

- the receipt remains an ordinary `delete` mutation-family receipt
- existing-truth binding evidence stays attached
- backend-verified assertion evidence stays attached
- declared aspect operations describe only the delete fallout side
- the declared aspect value digest includes both the verified preconditions and
  the declared delete fallout, so semantically different verified deletes do
  not collapse

## Denials

This lane remains fail-closed when:

- the backend does not admit backend verification
- the asserted current value is missing
- the asserted current value does not match authoritative truth
- preview tries to use the lane instead of the authoritative runtime
