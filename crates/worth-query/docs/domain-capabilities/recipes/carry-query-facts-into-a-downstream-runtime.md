# Carry Query Facts Into A Downstream Runtime

## What This Recipe Covers

Use this recipe when a downstream runtime must act on facts produced by Query.
The handoff is one sealed authority object, not a tuple of digests and facts.

## When To Use It

Use it for allocation, invalidation, rendering, or another downstream decision
whose correctness depends on the exact Query basis and source lineage.

## The Smallest Useful Path

```rust
use worth_query::facade::{
    AuthorizedProjectionArtifact, ProjectionAuthorityContract,
    ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError,
    WorthQueryWriteReceipt,
};

fn query_authority_outcome(
    write_receipt: &WorthQueryWriteReceipt,
    authorized_projection: &AuthorizedProjectionArtifact,
    result_shape_digest: &str,
) -> Result<ProjectionAuthorityOutcome, ProjectionFactConsumptionPathError> {
    let outcome = write_receipt.consume_projection_authority(
        result_shape_digest,
        authorized_projection,
        ProjectionAuthorityContract::declare()
            .require_settled_consumption()
            .require_source_authority()
            .require_target_identity()
            .require_source_references(),
    )?;

    Ok(outcome)
}
```

Declare only the guarantees the consumer needs. Handle a non-admitted outcome
as the terminal result; do not retry by extracting raw IDs or comparing basis
digests locally.

When the downstream owner is ready to take the proof, call
`ProjectionAuthorityOutcome::into_admitted()`. It moves the non-cloneable
authority and its optional warnings together; an error returns the original
typed non-admitted outcome for matching.

## Durable Contract Replay

Persist the declaration, not the authority:

```rust
let document = contract.to_terminal_json_document()?;
let replayed = load_projection_authority_contract_document(&document.to_external())?;
```

The loaded contract enters the same canonical transition. Unknown schemas,
requirements, facts, and malformed field paths fail closed.

Do not serialize `WorthQueryConsumedProjectionAuthority`. It proves one
admitted consumption and must remain with the downstream operation that relies
on it.

## What To Retain

- Retain `WorthQueryConsumedProjectionAuthority` while downstream work depends
  on the authoritative relationship.
- Use `source_identity()`, `receipt()`, and `evidence()` for inspection or
  indexing only.
- Use admitted authority's `facts()` for immediate typed inspection. The old
  decomposed fact-consumption call is not a public fallback.

## Related Docs

- [Projection Consumption And Downstream Authority](../../capabilities/projection-consumption.md)
- [Downstream Runtime Integration](../../foundations/downstream-runtime-integration.md)
- [Support Matrix And Admission](../../foundations/support-matrix-and-admission.md)
