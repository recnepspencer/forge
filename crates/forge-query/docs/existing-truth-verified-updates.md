# Existing-Truth Verified Updates

Use `workspace.update_existing_verified(...)` when an existing-target update must
prove current authoritative values immediately before the mutation executes.

## Shape

```rust
let binding = workspace.bind_existing_entity(
    ForgeQueryExistingEntityTarget::new("authority:task-1", resolved_identity)?
        .in_target_collection("Task")?,
)?;

let receipt = workspace.update_existing_verified(
    binding,
    |verify| verify.aspect("status.value", "open"),
    |update| update.aspect("status.value", "closed"),
)?;
```

The first closure declares the backend-verified precondition. The second closure
declares the actual update-family mutation.

Check support rows before treating this as an ordinary bridge-backed production
lane:

```rust
let support = workspace.public_authoritative_mutation_evidence_support();
let update_row = support
    .bridge_backed_verification_support_rows()
    .iter()
    .find(|row| {
        row.operation_family() == "update_existing_verified"
            && row.target_binding_family() == "direct_entity_identity"
    })
    .unwrap();

assert!(update_row.scaffold_profile_supported());
if update_row.primary_bridge_backed_runtime_supported() {
    assert_eq!(
        update_row.current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
    );
} else {
    assert_eq!(
        update_row.denial_class_when_unsupported(),
        Some("backend_verification_unsupported")
    );
}
```

## Contract

- the receipt remains an ordinary `update` mutation-family receipt
- existing-truth binding evidence stays attached
- backend-verified assertion evidence stays attached
- backend-verified assertion evidence now carries a verified assumption set,
  including the assumption snapshot token, assumption snapshot digest,
  verified precondition digest, and verification read-set breadth
- declared aspect operations describe only the update side
- the declared aspect value digest includes both the verified preconditions and
  the update values, so semantically different verified updates do not collapse

That means callers can distinguish:

- target binding evidence
- verified old-truth assumptions
- snapshot basis for those assumptions
- update result evidence

without rebuilding that story from local bridge glue.

## Denials

This lane remains fail-closed when:

- the backend does not admit backend verification
- the asserted current value is missing
- the asserted current value does not match authoritative truth
- preview tries to use the lane instead of the authoritative runtime
