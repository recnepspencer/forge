# Read data

## What this feature is

Use the compatibility read surface when you want a canonical read result from a
request that is shaped like ordinary HTTP.

Stable entry points:

- `WorthServer::compat_http()`
- `WorthServerCompatibilityFacade::prepare_request(...)`
- `WorthServerCompatibilityFacade::read(...)`
- `WorthServerCompatibilityFacade::state(...)`
- `WorthServerCompatibilityFacade::inspect(...)`

Normalized route family:

```text
GET|HEAD /compat/reads/{operation}
```

## Why you use it

Use this surface when you need:

- a strict external read boundary
- canonical validators and basis handling
- explicit state reads versus inspection reads
- predictable denial behavior for stale or malformed conditional requests

## Core mental model

A compatibility read is not "raw HTTP turned loose on the runtime."

It is:

1. A normalized request contract.
2. A route-family-specific admission step.
3. A canonical Worth query handoff.
4. A read artifact with explicit basis, validator, provenance, and cache
   posture.

`read`, `state`, and `inspect` are deliberately different products:

- `read(...)` returns the main live read artifact.
- `state(...)` returns runtime state projection.
- `inspect(...)` returns inspection-specific truth.

Do not collapse them into one concept in your caller.

## How it executes

The usual flow is:

1. Build a `WorthServerCompatibilityRequestInput`.
2. Call `prepare_request(...)` to normalize and admit request context.
3. Wrap the prepared request in `WorthServerCompatibilityExecutionInput`.
4. Call `read(...)`, `state(...)`, or `inspect(...)`.

The server enforces these read rules:

- at most one explicit `basis` query parameter
- no extra explicit basis on preview-targeted reads
- exact validator matching for `if-match` and `if-none-match`
- no simultaneous `if-match` and `if-none-match`
- `HEAD` preserves the same validator and basis truth as `GET`
- cache policy remains `private, no-store`

The default vary set is:

```text
authorization, x-Worth-branch, x-Worth-diagnostics
```

## Small example

```rust
use worth_proof::TransitionOutcome;
use worth_server::{
    WorthServer, WorthServerCompatHttpRouteFamily,
    WorthServerCompatibilityExecutionInput, WorthServerCompatibilityRequestInput,
};

let server: WorthServer = build_server();

let prepared = match server.compat_http().prepare_request(
    WorthServerCompatibilityRequestInput::builder()
        .with_authenticated_principal_id("principal-7")
        .with_tenant_id("tenant-a")
        .with_workspace_id("workspace-42")
        .with_branch_id("branch-9")
        .with_route_family(WorthServerCompatHttpRouteFamily::Read)
        .with_method("GET")
        .with_path("/compat/reads/users.profile")
        .with_header("accept", "application/json")
        .build()
        .expect("request should validate structurally"),
) {
    TransitionOutcome::Success(value) => value,
    other => panic!("expected prepared request, got {other:?}"),
};

let read = match server.compat_http().read(
    WorthServerCompatibilityExecutionInput::new(prepared, "users.profile"),
) {
    TransitionOutcome::Success(value) => value,
    other => panic!("expected compatibility read, got {other:?}"),
};

assert_eq!(read.cache_policy().cache_control(), "private, no-store");
assert!(read.validator().entity_tag().starts_with('"'));
```

## Real example

Use `read(...)` when your caller wants the canonical payload, `state(...)` when
it needs state-specific basis truth, and `inspect(...)` when it needs
inspection-specific truth.

That distinction matters because the canonical digests differ. A state result is
not just a renamed read result, and an inspection result is not just a verbose
read result.

If you are building a client cache:

- store the read validator exactly as returned
- reuse the returned basis only when the server admits it again
- treat `CompatibilityConditionalReadNotModified` and
  `CompatibilityConditionalReadPreconditionFailed` as semantic outcomes, not
  transport failures

## Inspection and debugging

Check these fields first:

- `validator().entity_tag()`
- `validator().canonical_digest()`
- `direct_context().basis_digest()`
- `response_envelope()`
- `cache_policy()`

If a read is denied, inspect the denial code before debugging your route:

- `CompatibilityBasisRequestInvalid`
- `CompatibilityBasisRequestUnsupported`
- `CompatibilityConditionalRequestInvalid`
- `CompatibilityConditionalReadNotModified`
- `CompatibilityConditionalReadPreconditionFailed`

## Anti-patterns

- Treating preview-targeted reads plus explicit `basis` as valid.
- Assuming `HEAD` means "different read semantics with no real validator."
- Reusing a stale validator after authoritative truth changed.
- Treating `inspect(...)` as a richer alias for `read(...)`.
- Relaxing cache policy at the caller because a payload "looks public."

## Current limits

- Compatibility reads are intentionally private and non-storeable.
- Basis reuse is exact, not fuzzy.
- Conditional request handling is strict and fail-closed.
- The docs here cover the stable Rust surface and the normalized route family,
  not a separate generated OpenAPI contract.

## Related docs

- [Write data](./write-data.md)
- [Stream results](./stream-results.md)
- [Runtime-backed vs durable](./runtime-backed-vs-durable.md)
