# Application Authorization And Emergency Elevation

## What This Feature Is

Application authorization lets a domain describe a product permission once,
install it into Query, and obtain authority only from current application-graph
facts. Emergency elevation adds a short-lived, reviewable way to use one exact
permission that ordinary access does not grant.

Use this feature when the question is not merely "can this query run?", but
"may this authenticated principal perform this product action, for this
resource, purpose, field, and request context, right now?"

## Why You Use It

- protect a restricted query field with installed product policy
- authorize a domain command from current roles and relationships
- grant emergency use without widening the ordinary capability
- preserve an auditable request, approval, close, and mandatory-review chain
- keep policy evaluation in Query instead of recreating it in an application
  service

## Stable Entry Points

Host applications import the entry-audience facade:

```rust
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    declaration::application_capability,
    primary_graph,
};
```

The stable runtime entry points are methods on
`primary_graph::WorthQueryPrimaryGraphApplicationRuntime`:

- `admit_capability_access(...)`
- `admit_approved_elevation_access(...)`
- `authorize_elevation_request(...)`
- `authorize_elevation_approval(...)`
- `authorize_elevation_close(...)`
- `authorize_mandatory_review(...)`
- `compare_and_commit_elevation_request(...)`
- `compare_and_commit_elevation_approval(...)`
- `compare_and_commit_elevation_close(...)`
- `compare_and_commit_mandatory_review(...)`

Most product code should call a domain-native wrapper over that progression.
For example, Bank exposes `request_estate_emergency_access(...)` and
`approve_estate_emergency_access(...)`. The wrapper gives the operation a
product name; it does not become a second authorization owner.

The supported emergency-elevation path is runtime-backed and primary-graph
based. Certification replay is a separate cert-only audience and cannot mint
ordinary application authority.

## Core Mental Model

Four meanings stay separate:

1. A **lower-runtime ability** says what infrastructure can do, such as observe
   a relation or evaluate an installed boolean composition. It is not product
   permission.
2. An **application capability** is the installed product permission, including
   its action, resource, relation, purpose, field, context, validity, and
   composition rules.
3. A **lifecycle command** is the operation being authorized now, such as
   `ApproveEmergencyAccess`. Its authority target is the request, elevation,
   or review object that command changes.
4. The **governed upper bound** is the maximum product permission the lifecycle
   may activate. It remains bound to the original resource, action, purpose,
   field, grant, and requester.

Command authority and the governed upper bound are orthogonal. Approval
authority over an emergency request does not make approval itself a restricted
data view. Conversely, holding a valid restricted-view grant does not authorize
the approval command.

Query combines evidence owned by lower runtimes without transferring product
authority to them:

- Relational observes exact current graph facts.
- Signal evaluates the installed boolean composition.
- Runtime Bridge binds that evaluation to the installed correspondence.
- Query binds the complete result to the authenticated request and mints the
  move-only progression value.

Application code should not import those lower runtimes to rebuild the answer.

## How It Executes

The elevation path is a small typestate graph:

```text
request -> requested -> approved -> active use -> revoked/expired
                                      |                 |
                                      +------ close ----+
                                                |
                                         review required
                                                |
                                             reviewed
```

The public receipts make the lawful next transition compiler-visible:

- `WorthQueryRequestedElevation` is consumed by approval.
- `WorthQueryApprovedElevation` is borrowed for governed reads and consumed by
  close.
- `WorthQueryMandatoryReview` is consumed by mandatory review.
- `WorthQueryReviewedElevation` is terminal descriptive evidence.

Request, approval, and active use freshly revalidate the exact supporting
authority. Close and mandatory review intentionally do not require that
support to remain positive: revocation or expiry must not strand cleanup or a
legal review obligation.

Every commit method returns a typed outcome enum. The enum is the next-action
contract. Do not assume success or flatten it into a boolean.

## Small Example

Match the outcome and move the receipt only from a lawful success variant:

```rust
use worth_query_host::facade::primary_graph::{
    WorthQueryElevationRequestOutcome,
    WorthQueryRequestedElevation,
};

fn requested_receipt(
    outcome: WorthQueryElevationRequestOutcome,
) -> Result<WorthQueryRequestedElevation, WorthQueryElevationRequestOutcome> {
    match outcome {
        WorthQueryElevationRequestOutcome::Requested(receipt)
        | WorthQueryElevationRequestOutcome::AlreadyRequested(receipt) => Ok(receipt),
        stop => Err(stop),
    }
}
```

This helper does not manufacture authority. It consumes the complete outcome,
returns the move-only receipt on either committed posture, and preserves every
stop for the caller to handle or publish.

## Real Example

A domain-native emergency flow keeps command inputs separate from the
restricted query that the approval may later open:

```rust
use worth_query_host::facade::primary_graph::{
    WorthQueryElevationApprovalOutcome,
    WorthQueryElevationRequestOutcome,
};

let request_outcome = bank.request_estate_emergency_access(
    &requester,
    request_command,
    request_idempotency,
    &scope,
)?;

let requested = match request_outcome {
    WorthQueryElevationRequestOutcome::Requested(receipt)
    | WorthQueryElevationRequestOutcome::AlreadyRequested(receipt) => receipt,
    stop => return publish_request_stop(stop),
};

let approval_outcome = bank.approve_estate_emergency_access(
    &approver,
    requested,
    approval_command,
    approval_idempotency,
    &scope,
)?;

let approved = match approval_outcome {
    WorthQueryElevationApprovalOutcome::Approved(receipt)
    | WorthQueryElevationApprovalOutcome::AlreadyApproved(receipt) => receipt,
    stop => return publish_approval_stop(stop),
};

let disclosed = bank
    .query(estate_emergency_account_details(estate))
    .as_principal(&requester)
    .controls(read_controls)
    .execute_with_approved_elevation(&approved)?;
```

`request_command` and `approval_command` authorize changes to lifecycle
objects. The receipt independently retains the exact restricted-view upper
bound. `execute_with_approved_elevation` borrows that receipt, so a successful
read neither consumes close authority nor creates a reusable access token.

The `publish_*_stop` functions above represent application-owned response
handling. They must preserve the typed Query outcome; they must not infer a
policy explanation from strings.

## How It Relates To Other Features

- Pair application capability admission with an installed application query or
  operation. A capability does not replace query planning or effect programs.
- Disclosure is a second decision after internal computation. Approved
  elevation may admit protected computation while the result still follows its
  installed disclosure contract.
- One-shot, continuation, history, preview, and live may share one installed
  governed query. Each supported lane must re-admit the same capability and
  elevation meaning before another consumer-visible payload.
- Typed authorization and lifecycle outcomes can be lowered into publication
  diagnostics. Published explanations are descriptive and cannot be promoted
  back into authority.
- Lower-runtime capability routing is infrastructure composition, not an
  alternate application authorization API.

## Inspection And Debugging

Start with the exact typed outcome and denial kind. For successful governed
queries, inspect the public authorization work evidence and disclosure receipt.
For lifecycle work, retain the commit receipt and terminal outcome identity.

Useful questions are:

- Was the command capability missing, or did the governed upper bound fail?
- Did the exact grant, relationship path, principal, purpose, field, or trusted
  time sample become stale?
- Did the request reach `Requested`, `Approved`, a terminal close state, or
  `Reviewed` in authoritative graph readback?
- Did authorization work remain bounded to the declared touched evidence?

Do not diagnose by parsing `Display` text. Query publication exposes typed
authorization, disclosure, elevation, expiry, and review-required meaning.

## Anti-Patterns

- using the governed restricted-view target as every lifecycle command target
- treating a lower-runtime ability or Signal decision as product authority
- accepting a receipt from a different runtime, branch, schema generation,
  grant, request, or review
- cloning, wrapping, or serializing a lifecycle receipt as a reusable token
- extracting only the success payload and discarding stale, denied, partial,
  or indeterminate outcomes
- requiring positive governed support to close an expired or revoked elevation
- reading protected values and redacting them after the query
- building a bank-local or service-local policy evaluator beside Query

## Current Limits

- Product domains still provide typed schema declarations, effect programs,
  invariant projections, and domain-native wrapper names.
- The application-capability runtime is exposed through the host facade's
  primary-graph surface, not the ordinary workspace capability namespaces.
- Outcome enums intentionally require explicit matching; there is no unchecked
  `requested()?` or `approved()?` success extractor.
- Cross-lane parity is a property of an installed query that declares and
  proves those lanes. It is not implied for every capability automatically.
- Replay and reconstruction remain certification-only and cannot replace fresh
  ordinary admission.

## Related Docs

- [Policy, Tenant, And Relationship-Proof Narrowing](../foundations/policy-tenant-and-relationship-proof-narrowing.md)
- [Support Matrix And Admission](../foundations/support-matrix-and-admission.md)
- [Lower-Runtime Capability Routing](../domain-capabilities/lower-runtime-capability-routing.md)
- [Provider Sessions And Decision Read-Sets](../domain-capabilities/provider-sessions-and-decision-read-sets.md)
- [Provisional State And Invariant Execution](../domain-capabilities/provisional-state-and-invariant-execution.md)
- [Inspection](./inspection.md)
