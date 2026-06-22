# Upload files

## What this feature is

Use the upload surface when a request includes binary file parts plus structured
metadata that must lower through the same canonical mutation lane.

Stable entry points:

- `ForgeServerCompatibilityFacade::prepare_upload(...)`
- `ForgeServerCompatibilityFacade::begin_binary_ingress(...)`
- `ForgeServerCompatibilityFacade::upload(...)`
- `ForgeServerCompatibilityFacade::interrupt_binary_ingress(...)`
- cleanup helpers for expired, abandoned, and mismatched ingress sessions

Normalized route family:

```text
POST /compat/uploads/{operation}
```

## Why you use it

Use this when you need:

- multipart admission with real manifest validation
- staged ingress before authoritative metadata truth commits
- exact cleanup behavior for abandoned or mismatched transfers
- integrity verification before commit

## Core mental model

An upload has two truths that must stay aligned:

- binary ingress truth
- structured metadata truth

`forge-server` refuses to pretend those are the same thing automatically.

Instead it:

1. validates the multipart shape and manifest
2. stages ingress
3. verifies ownership, bounds, pacing, and integrity
4. lowers the admitted metadata through the same mutation lane as a plain write

Binary transport details do not get to leak into the structured mutation truth.

## How it executes

Use one of two patterns:

- one-shot `upload(...)` for straightforward calls
- staged `prepare_upload(...)` plus `begin_binary_ingress(...)` when you need
  explicit transfer lifecycle handling

Important behavior:

- manifest file-part names must match the part graph exactly
- metadata must contain a valid command shape
- duplicate part identities are denied
- oversized parts are denied early
- wrong content types are denied
- `expect: 100-continue` requests can be denied before body progression
- interrupted, expired, abandoned, and ownership-mismatched sessions produce
  explicit cleanup receipts
- integrity digest mismatches deny before metadata commit

## Small example

```rust
use forge_server::{
    ForgeServerMultipartUpload, ForgeServerUploadManifest, ForgeServerUploadPart,
};
use serde_json::json;

let upload = ForgeServerMultipartUpload::new(
    ForgeServerUploadManifest::new(json!({
        "command": {
            "family": "insert",
            "collection": "Task",
            "aspects": {
                "identity.id": "task-1",
                "title.value": "Avatar uploaded"
            }
        }
    }))
    .with_file_part("avatar"),
)
.with_part(
    ForgeServerUploadPart::file("avatar")
        .with_content_type("image/png")
        .with_declared_length(128)
        .with_body_bytes(vec![0; 128]),
);

let outcome = server.compat_http().upload(upload_execution_input(upload));
```

## Real example

The best production pattern is:

- keep metadata authoritative and explicit
- declare expected file parts in the manifest
- verify integrity at both manifest and part level when the caller can do it
- treat cleanup receipts as normal lifecycle outcomes, not weird exceptions

If a session is interrupted or abandoned, clean it up and retry honestly.
Do not try to "complete" a dead ingress session by reusing stale staged state.

## Inspection and debugging

Check:

- upload canonical digest
- mutation canonical digest
- ingress integrity receipt
- ingress performance counters
- cleanup reason when staged state is torn down

Useful cleanup reasons include:

- `Interrupted`
- `Expired`
- `Abandoned`
- `OwnershipMismatch`

## Anti-patterns

- Letting blob transport details become part of structured mutation identity.
- Accepting duplicate or missing file parts and hoping downstream code sorts it
  out.
- Committing metadata before ingress integrity is verified.
- Treating unknown-length or hostile compressed uploads as normal traffic.

## Current limits

- Multipart upload admission is intentionally strict.
- Unknown-length and compressed ingress are bounded aggressively.
- Cleanup paths are part of the contract, not optional hygiene.

## Related docs

- [Write data](./write-data.md)
- [Download files](./download-files.md)
- [Connect another backend](./connect-another-backend.md)
