# Carry Query Facts Into A Downstream Runtime

## What This Recipe Covers

Use this recipe when a downstream runtime must act on facts produced by Query.
The handoff is one sealed authority object, not a tuple of digests and facts.

## When To Use It

Use it for allocation, invalidation, rendering, or another downstream decision
whose correctness depends on the exact Query basis and source lineage.

## The Smallest Useful Path

```rust
let outcome = write_receipt.consume_projection_authority(
    result_shape_digest,
    &authorized_projection,
    ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
        .require_target_identity()
        .require_source_references(),
)?;

let (authority, warnings) = outcome
    .into_admitted()
    .map_err(|non_admitted| format!("cannot hand off: {non_admitted:?}"))?;

downstream_runtime.accept_query_authority(authority, warnings)?;
```

Declare only the guarantees the consumer needs. Handle a non-admitted outcome
as the terminal result; do not retry by extracting raw IDs or comparing basis
digests locally.

## Durable Contract Replay

Persist the declaration, not the authority:

```rust
let document = contract.to_terminal_json_document()?;
let replayed = load_projection_authority_contract_document(&document.to_external())?;
```

The loaded contract enters the same canonical transition. Unknown schemas,
requirements, facts, and malformed field paths fail closed.

## What To Retain

- Retain `WorthQueryConsumedProjectionAuthority` while downstream work depends
  on the authoritative relationship.
- Use `source_identity()`, `receipt()`, and `evidence()` for inspection or
  indexing only.
- Use `consume_projection_facts(...)` only for immediate inspection that does
  not become downstream authority.

## Related Docs

- [Projection Consumption And Downstream Authority](../../capabilities/projection-consumption.md)
- [Downstream Runtime Integration](../../foundations/downstream-runtime-integration.md)
- [Support Matrix And Admission](../../foundations/support-matrix-and-admission.md)
