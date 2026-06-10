# Download files

## What this feature is

Use the download surface when you want binary delivery with explicit integrity,
range handling, and resume planning.

Stable entry points:

- `ForgeServerCompatibilityFacade::download(...)`
- `ForgeServerCompatibilityFacade::plan_binary_resume(...)`
- `ForgeServerCompatibilityFacade::plan_binary_integrity(...)`

Normalized route family:

```text
GET|HEAD /compat/downloads/{operation}
```

## Why you use it

Use this when you need:

- full or ranged binary delivery
- integrity truth for the selected representation
- explicit resume planning
- honest distinction between runtime-backed retry and durable restart stability

## Core mental model

A download is not just "send bytes."

It is:

- a canonical read-shaped admission
- a selected representation
- explicit integrity for that selection
- an optional retry or resume posture

The server is careful not to overclaim:

- runtime-backed resume can be admitted
- restart stability is still a separate claim
- `HEAD` still carries integrity truth even when no bytes are transferred

## How it executes

1. Prepare a request for the download route family.
2. Build a `ForgeServerBinaryDownloadRequest`.
3. Call `download(...)`.
4. If needed, call `plan_binary_resume(session)` and build a follow-up request
   with `ForgeServerBinaryResumeRequest`.

Important behavior:

- range requests preserve canonical metadata parity with full delivery
- selected representation digests change when the selected byte range changes
- `HEAD` returns no body but still returns integrity shape
- resumed requests can verify expected integrity
- runtime-backed resume does not imply durable restart stability

## Small example

```rust
use forge_server::{
    ForgeServerBinaryDownloadRequest, ForgeServerBinaryResumeRequest,
};

let first = compat_download_success(server.compat_http().download(
    download_execution_input(
        ForgeServerBinaryDownloadRequest::new(body.clone()),
    ),
));

let resume = server
    .compat_http()
    .plan_binary_resume(first.session())
    .expect("resume planning should succeed");

let resumed = compat_download_success(server.compat_http().download(
    resumed_download_execution_input(
        ForgeServerBinaryDownloadRequest::new(body).with_resume_request(
            ForgeServerBinaryResumeRequest::resume_from(resume)
                .with_expected_integrity(first.integrity_digest().clone()),
        ),
    ),
));

assert!(resumed.session().retry_posture().is_resume());
assert!(!resumed.session().retry_posture().restart_stable());
```

## Real example

A good client flow is:

- issue an initial full or range request
- persist the integrity and retry posture you were actually given
- if transfer stops, ask the server to plan a resume from the admitted session
- send the resume request with the expected integrity digest

That keeps retries honest. The caller never has to guess whether a resumed
request is really continuing the same admitted delivery contract.

## Inspection and debugging

Inspect:

- `integrity_digest()`
- `session().retry_posture()`
- selected start and end offsets
- performance counters for resume admission and integrity verification

If two deliveries have the same canonical source but different selected ranges,
their selected representation digests should differ. That is expected.

## Anti-patterns

- Treating runtime-backed resume as if it were durable restart stability.
- Ignoring expected integrity on resumed requests.
- Assuming `HEAD` has no integrity meaning because it has no payload.
- Reconstructing your own resume offset rules without using the planned session.

## Current limits

- Runtime-backed resume is supported where admitted.
- Durable restart-stable delivery is a separate capability and should not be
  inferred automatically.
- Integrity is explicit for full, range, and head delivery.

## Related docs

- [Stream results](./stream-results.md)
- [Upload files](./upload-files.md)
- [Runtime-backed vs durable](./runtime-backed-vs-durable.md)
