# Application Aftermath, External Effects, And Recovery

## What This Feature Is

This feature lets an application describe what must remain knowable after a
mutation commits, send one bounded typed effect to an external owner, recover
when the commit or delivery result is uncertain, and publish a safe description
of the result. Application code declares meaning; Query owns execution and
recovery; Relational owns commit history; the external service owns whether its
effect completed.

## Why You Use It

- Notify an external service without separating the local mutation from the
  dispatch record that makes delivery recoverable.
- Inspect or resolve an uncertain commit without accepting a caller-written
  recovery slot, posture, or boolean as authority.
- Retain an exact bounded pre-mutation field value when an installed contract
  requires it, while failing the commit if that exact truth is unavailable.
- Show callers a closed aftermath or delivery posture without exposing runtime
  handles, outbox bytes, protected decision facts, or execution internals.

## Stable Entry Points

Declaration consumers use `worth_query_decl::facade`:

- `application_schema::ApplicationExternalEffectPayload`
- `application_schema::ApplicationExternalEffectProtocol`
- `application_schema::ApplicationOperationDefinitionBuilder::external_effect`
- `application_aftermath::DeclaredApplicationAftermathContract`
- `application_aftermath::DeclaredPreImageDemand`

Hosts use `worth_query_host::facade`:

- `primary_graph` for sealed commit receipts and recovery progression
- `publication::domain_computation::publish_application_commit`
- `publication::application_aftermath::publish_application_aftermath`
- `publication::application_aftermath::publish_recovery_support`

Application hosts should normally wrap the generic host surface in domain-named
operations, as Bank does with `BankCommitReceipt::aftermath()` and its recovery
methods.

The `provisional_aftermath` facade contains the current undo/redo experiment.
It is not part of the stable feature described here.

## Core Mental Model

There are four independent owners:

1. Your declaration names the effect protocol and the operation's aftermath
   contract.
2. Query installs that declaration, executes the operation, co-commits the
   outbox data, and owns live recovery handles.
3. Relational assigns commit identity, parentage, branch head, and canonical
   history. Query does not keep a second lineage or history store.
4. The external service decides whether the external effect completed. Silence,
   a timeout, or a lost response is never treated as completion.

A recovery handle is a move-only runtime capability opened from a sealed commit
receipt. It is not a serialized token. An opaque wire identity can be logged or
transported, but must be re-admitted by the owning runtime before it can affect
anything.

Published aftermath is deliberately weaker. It describes accepted posture,
external observation, or recovery support, but cannot authorize a transition.

## How It Executes

1. The schema declares one typed emission and at most one external-effect
   contract for an operation.
2. The payload implementation supplies a stable version-free protocol family,
   an exact positive produced version, a maximum byte count, and the exact
   bytes to send.
3. Query installs the operation and its one aftermath contract.
4. During execution, Query derives the payload from the admitted typed emission.
   Callers cannot replace the effect name, protocol family, exact version,
   bound, or bytes.
5. Relational atomically commits the ordinary mutation and its outbox row.
6. Query reads that row through a fresh owner-sealed observation and dispatches
   it to the external owner.
7. The returned observation becomes `Completed`, `Acknowledged`, or
   `Unresolved(failure)`. Missing evidence never advances the posture.
8. If recovery is needed, the runtime opens one exact-handle lifecycle from the
   commit receipt. Inspection requires fresh disclosure authority; effectful
   progression requires fresh effect authority.
9. Publication projects the sealed commit or admitted inspection into closed
   consumer-facing values.

When an installed contract demands a pre-image, Query reads Relational's opaque
validated mutation footprint before commit. The exact record, aspect, and field
must be mutated by that attempt. A same-named field on another record, another
aspect, or merely-read field cannot satisfy the demand.

## Small Example

Define a stable external payload. The protocol family is application-owned,
version-free, and not a Rust module path; its exact produced version is a
separate typed value.

```rust
use worth_foundational::facade::{
    BoundaryProtocolIdentity, BoundaryProtocolVersion,
};
use worth_query_decl::facade::application_schema::{
    ApplicationEffectPayload, ApplicationExternalEffectPayload,
    ApplicationExternalEffectProtocol,
};

struct InvoiceReady {
    invoice_id: u64,
}

impl ApplicationEffectPayload for InvoiceReady {
    fn retained_bytes(&self) -> u64 {
        8
    }
}

impl ApplicationExternalEffectPayload for InvoiceReady {
    const PROTOCOL: ApplicationExternalEffectProtocol =
        ApplicationExternalEffectProtocol::new(
            BoundaryProtocolIdentity::new("billing.invoice-ready"),
            BoundaryProtocolVersion::new(1),
        );
    const MAX_EXTERNAL_BYTES: u64 = 8;

    fn external_effect_bytes(&self) -> Vec<u8> {
        self.invoice_id.to_be_bytes().to_vec()
    }
}
```

The schema must also declare the typed emission and bind that emission to an
external-effect correlation family with the operation definition's
`external_effect` transition. A payload implementation alone does not create a
dispatch lane.

## Real Example

Bank's death-notification operation declares one emission, one stable protocol
family with an exact version, and one rail:

```rust
schema
    .operation_emit(operation, EstateDeathNotificationEffect::reference())
```

The operation definition binds the escaping lane:

```rust
operation
    .definition()
    .external_effect(
        EstateDeathNotificationEffect::reference(),
        ESTATE_DEATH_NOTICE_RAIL,
    )
    .aftermath(declared_aftermath)
    .finish()
```

The payload encodes the exact estate, notice, and subject as three big-endian
`u64` values under family `bank.estate.death-notification`, version `1`. After a
real commit, an application consumer reads only the published posture:

```rust
use worth_query_host::facade::publication::application_aftermath::{
    WorthQueryPublishedExternalEffectPosture,
    WorthQueryPublishedExternalEffectFailure,
};

match receipt.aftermath().external_effect() {
    WorthQueryPublishedExternalEffectPosture::Completed => {
        // The rail independently decoded and completed this effect.
    }
    WorthQueryPublishedExternalEffectPosture::Acknowledged => {
        // Accepted by the rail, but completion is not established.
    }
    WorthQueryPublishedExternalEffectPosture::Unresolved(
        WorthQueryPublishedExternalEffectFailure::LostResponse,
    ) => {
        // Open recovery through the host's receipt-bound recovery API.
    }
    _ => {}
}
```

The Bank rail is a separate process. It independently checks the effect name,
protocol family, exact version, declared bound, and bytes before admitting the
notice to its ledger.
A lost response can therefore leave Query unresolved even when the rail has
completed the effect; safe retry relies on the rail's idempotent correlation,
not on Query guessing what happened.

## How It Relates To Other Features

- Pair external effects with ordinary Query mutation and idempotency. The outbox
  row and mutation share one Relational commit.
- Use recovery when the commit or external observation is unresolved. Do not
  use replay: reconstruction and historical replay remain certification-only.
- Use publication for consumer descriptions. Use execution recovery authority
  only inside the host runtime that owns the handle.
- Relational remains the authority for commit history, entity lineage, branch
  head, and ancestry. Runtime Bridge may transport an already-admitted portable
  description, but it does not decide any of those facts.
- Exact pre-image retention is stable infrastructure. What an eventual undo or
  redo product may lawfully do with retained truth is still deferred.

## Inspection And Debugging

For a committed operation, inspect:

- the host receipt's commit identity and ordinary canonical-work counters;
- `aftermath().posture()` for the installed accepted posture;
- `aftermath().external_effect()` for external observation state;
- the host's committed-outbox observation when diagnosing transport ownership;
- `publish_recovery_support(&inspection)` after disclosure-admitted recovery
  inspection.

`Unresolved` includes typed failures such as timeout, disconnect, lost response,
duplicate acknowledgement, payload rejection, unsupported protocol version, or
an unknown provider outcome. Unsupported versions retain the exact produced
version and distinguish `PredatesWindow`, `ExceedsWindow`, and `Retired`.
These values explain what Query observed; they do not overwrite the external
owner's ledger.

## Anti-Patterns

- Do not derive a protocol family or version from `std::any::type_name`.
- Do not accept payload bytes, effect names, completion flags, or recovery slots
  from the caller after operation admission.
- Do not treat acknowledgement, silence, timeout, or response loss as completion.
- Do not construct publication from copied enums or raw receipt fields.
- Do not serialize a recovery handle and treat the bytes as live authority.
- Do not add Query-local lineage, parent selection, branch heads, or publication
  order. Those are Relational responsibilities.
- Do not use `provisional_aftermath` as a stable product contract.

## Current Limits

- Recovery handles are process-local and not restart-durable. Published recovery
  support reports when a Store capability is still required.
- The accepted external-effect lane carries one declared typed effect per
  operation contract.
- Durable cross-runtime recovery, branch-aware reversal, merge interaction,
  rebase, and history navigation belong to the cross-runtime roadmap.
- Undo and redo behavior, eligibility, occurrence meaning, divergence policy,
  and public DX await the Query Undo/Redo Semantics milestone.

## Related Docs

- [Runtime Phase 8 Finish Plan](../milestone-9.16-runtime-phase-8-finish-plan.md)
- [Runtime Phase 8 Specification](../milestone-9.16-runtime-phase-8.md)
- [Milestone 9.16](../milestone-9.16.md)
- [Cross-Runtime Merging And Branching Roadmap](../../cross-runtime/merging-and-branching-roadmap.md)
