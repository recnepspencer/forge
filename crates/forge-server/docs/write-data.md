# Write data

## What this feature is

Use the compatibility mutation surface when you need an HTTP-shaped write
request to become one canonical Forge mutation.

Stable entry points:

- `ForgeServer::compat_http()`
- `ForgeServerCompatibilityFacade::prepare_request(...)`
- `ForgeServerCompatibilityFacade::mutate(...)`

Normalized route family:

```text
POST /compat/mutations/{operation}
```

## Why you use it

Use this when you want:

- a strict external write boundary
- precondition checks before any authoritative write executes
- idempotency-key replay for identical writes
- clear denials for unsupported or forbidden mutation families

## Core mental model

A compatibility mutation is admitted only if the whole external claim is honest.

That means the server checks:

- request shape
- route family
- basis and validator preconditions
- mutation family legality
- support posture for downstream query families the write depends on
- idempotency-key scope and replay compatibility

Only then does the authoritative write path run.

## How it executes

The flow is:

1. Normalize the request with `prepare_request(...)`.
2. Build `ForgeServerCompatibilityMutationExecutionInput`.
3. Call `mutate(...)`.
4. Inspect the returned mutation envelope, replay receipt, result digest, and
   inspection digest.

Important behavior:

- forbidden families are denied at the external boundary
- unsupported families are denied before execution
- stale basis or validator claims are denied before execution
- identical `idempotency-key` retries replay instead of writing twice
- conflicting reuse of an `idempotency-key` is denied
- idempotency scope is isolated by workspace target

## Small example

```rust
use forge_proof::TransitionOutcome;
use forge_server::{
    ForgeServerCompatibilityMutationExecutionInput,
    ForgeServerCompatibilityRequestInput, ForgeServerCompatHttpRouteFamily,
};
use serde_json::json;

let prepared = server.compat_http().prepare_request(
    ForgeServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(ForgeServerCompatHttpRouteFamily::Mutation)
        .with_method("POST")
        .with_path("/compat/mutations/tasks.insert")
        .with_header("accept", "application/json")
        .with_header("idempotency-key", "task-1-insert")
        .with_body_content_type("application/json")
        .with_body_present(true)
        .build()
        .expect("request should validate structurally"),
);

let prepared = match prepared {
    TransitionOutcome::Success(value) => value,
    other => panic!("expected prepared mutation request, got {other:?}"),
};

let outcome = server.compat_http().mutate(
    ForgeServerCompatibilityMutationExecutionInput::new(
        prepared,
        "tasks.insert",
        json!({
            "command": {
                "family": "insert",
                "collection": "Task",
                "aspects": {
                    "identity.id": "task-1",
                    "title.value": "Write docs"
                }
            }
        }),
    ),
);
```

## Real example

The most useful production pattern is "retry safely, but only honestly."

If your caller may retry after timeouts or transport failures:

- send an `idempotency-key`
- keep the request body stable across retries
- expect exact replay only for semantically identical requests

If the caller changes the body while reusing the same key, the server denies the
request instead of guessing which write you meant.

That is what preserves money and truth at the same time:

- no silent double-write
- no hidden merge
- no optimistic acceptance of conflicting retries

## Inspection and debugging

Look at:

- `envelope().replay_receipt()`
- `mutation_result().result_digest()`
- `mutation_result().inspection_digest()`
- `precondition()`
- denial code and detail on rejected outcomes

Important denial codes include:

- `CompatibilityMutationFamilyForbidden`
- `CompatibilityMutationFamilyUnsupported`
- `CompatibilityMutationPreconditionFailed`
- `CompatibilityIdempotencyConflict`
- `UnsupportedQueryFacadeFamily`

## Anti-patterns

- Reusing an `idempotency-key` with a different body.
- Treating a stale `if-match` validator as a best-effort hint.
- Exposing forbidden query families at the external boundary.
- Retrying writes without preserving exact precondition context.
- Assuming a failed write may have partially committed if the outcome was denied.

## Current limits

- The server is strict about mutation family admission.
- Replay is for identical retries, not semantic reconciliation.
- The compatibility docs describe the stable boundary behavior, not every
  internal mutation implementation detail.

## Related docs

- [Read data](./read-data.md)
- [Upload files](./upload-files.md)
- [Connect another backend](./connect-another-backend.md)
