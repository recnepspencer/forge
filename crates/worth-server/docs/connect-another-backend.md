# Connect another backend

## What this feature is

This guide is for a service that wants to call `worth-server` as a downstream
system.

That service might be:

- an app backend
- a worker
- a gateway
- a sidecar
- another Rust service

## Why you use it

A backend should use `worth-server` when it wants a strict execution boundary
without reimplementing:

- validator rules
- basis admission
- idempotent write replay
- multipart upload validation
- binary resume and integrity logic

## Stable entry points

Same-process Rust integration:

- `WorthServer::compat_http()`

HTTP-shaped integration:

- the five `/compat/...` route families

If you are already in the same Rust process, use the facade directly. If you
are crossing a network boundary, model requests around the normalized route
families and preserve the same headers and preconditions.

## Core mental model

Another backend should treat `worth-server` as an authority boundary, not as a
thin helper library.

That means:

- do not normalize away denial semantics
- do not rewrite validator or basis rules locally
- do not invent your own resume protocol
- do not silently map upload failures into plain mutation retries

## How it executes

The simplest service-to-service pattern is:

1. Resolve the target operation.
2. Build a read, mutation, upload, stream, or download request in the correct
   route family.
3. Preserve the response semantics exactly.
4. Retry only with the server's supported preconditions and replay keys.

## Small example

```text
Backend A wants to create a task attachment.

1. POST /compat/uploads/files.attachment.upload
2. Include structured command metadata in the manifest.
3. Include the file part separately.
4. If the upload is interrupted, use cleanup or retry honestly instead of
   assuming partial commit succeeded.
```

## Real example

If your backend needs to bridge from OpenAI tools, browser actions, or another
app server into Worth truth:

- map "fetch current state" to `/compat/reads/...`
- map "perform authoritative change" to `/compat/mutations/...`
- map "large result delivery" to `/compat/streams/...`
- map "binary ingress" to `/compat/uploads/...`
- map "binary egress" to `/compat/downloads/...`

The payoff is that your backend does not have to become its own half-correct
query handoff layer.

## Inspection and debugging

Log and preserve:

- operation name
- route family
- validator and basis when present
- idempotency-key on writes
- integrity and retry posture on downloads
- exact denial code and detail

Those are the fields that explain behavior. Most generic request logs are too
thin to debug this surface well.

## Anti-patterns

- Flattening every denial into a generic 400 or 500 with no semantic code.
- Mixing binary transfer concerns into ordinary JSON writes.
- Replacing explicit server preconditions with "best effort" local retries.
- Assuming a generic HTTP client abstraction is enough by itself.

## Current limits

- These docs focus on the stable execution surface, not a generated language
  SDK.
- A non-Rust backend will need its own thin request builder layer.
- Durable resume claims remain narrower than ordinary runtime-backed retries.

## Related docs

- [Connect an agent](./connect-an-agent.md)
- [Write data](./write-data.md)
- [Upload files](./upload-files.md)
- [Runtime-backed vs durable](./runtime-backed-vs-durable.md)
