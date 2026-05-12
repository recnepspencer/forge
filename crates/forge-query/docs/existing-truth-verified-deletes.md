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

Check support rows before treating this as an ordinary bridge-backed production
lane:

```rust
let support = workspace.public_authoritative_mutation_evidence_support();
let delete_row = support
    .bridge_backed_verification_support_rows()
    .iter()
    .find(|row| {
        row.operation_family() == "delete_existing_verified"
            && row.target_binding_family() == "direct_entity_identity"
    })
    .unwrap();

assert!(delete_row.scaffold_profile_supported());
if delete_row.primary_bridge_backed_runtime_supported() {
    assert_eq!(
        delete_row.current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
    );
} else {
    assert_eq!(
        delete_row.denial_class_when_unsupported(),
        Some("backend_verification_unsupported")
    );
}
```

## Contract

- the receipt remains an ordinary `delete` mutation-family receipt
- existing-truth binding evidence stays attached
- backend-verified assertion evidence stays attached
- backend-verified assertion evidence now carries a verified assumption set,
  including the assumption snapshot token, assumption snapshot digest,
  verified precondition digest, and verification read-set breadth
- declared aspect operations describe only the delete fallout side
- the declared aspect value digest includes both the verified preconditions and
  the declared delete fallout, so semantically different verified deletes do
  not collapse

That lets domains tell the difference between:

- which authoritative target was selected
- which old truths had to hold at the verification snapshot
- what read-set breadth the backend verification depended on
- what delete-family fallout the admitted mutation then produced

## Denials

This lane remains fail-closed when:

- the backend does not admit backend verification
- the asserted current value is missing
- the asserted current value does not match authoritative truth
- preview tries to use the lane instead of the authoritative runtime
