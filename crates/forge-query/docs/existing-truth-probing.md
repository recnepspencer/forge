# Existing-Truth Probing

Use `workspace.probe_existing(...)` when a caller needs the backend to return
current authoritative values for a bound existing target without executing a
mutation.

This is the target-first DX surface:

```rust
let binding = workspace.bind_existing_entity(
    ForgeQueryExistingEntityTarget::new(
        "authority:task-123",
        "Task:42",
    )?
    .in_target_collection("Task")?,
)?;

let probe = workspace.probe_existing(
    binding,
    ["identity.id", "title.value", "status.value"],
)?;
```

The result preserves:

- the existing-truth binding
- the backend-verified probe mode
- one typed field entry per requested aspect path
- a probe digest that changes when the returned authoritative values change

Before teaching this as an ordinary production flow on a bridge-backed runtime,
read the support rows:

```rust
let support = workspace.public_authoritative_mutation_evidence_support();
let probe_row = support
    .bridge_backed_verification_support_rows()
    .iter()
    .find(|row| {
        row.operation_family() == "probe_existing"
            && row.target_binding_family() == "direct_entity_identity"
    })
    .unwrap();

assert!(probe_row.compatibility_runtime_supported());
if probe_row.primary_bridge_backed_runtime_supported() {
    assert_eq!(
        probe_row.current_posture_status(),
        ForgeQueryBridgeBackedVerificationSupportStatus::Admitted
    );
} else {
    assert_eq!(
        probe_row.denial_class_when_unsupported(),
        Some("backend_probe_unsupported")
    );
}
```

Typed access stays straightforward:

```rust
assert_eq!(
    probe.field("title.value").unwrap().value_json(),
    "\"Ship authority probe\""
);
```

Fail-closed behavior matters here too:

- unsupported backends deny with
  `ForgeQueryExistingTruthProbeDenialKind::BackendProbeUnsupported`
- missing aspect paths deny with
  `ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect`
- unavailable resolved targets deny with
  `ForgeQueryExistingTruthProbeDenialKind::ResolvedTargetUnavailable`

Use `probe_existing(...)` when the domain needs current authoritative truth as
an input. Use `assert_existing(...)` or `verify_existing(...)` when the domain
needs a retained or backend-verified assertion receipt in the mutation lane.
