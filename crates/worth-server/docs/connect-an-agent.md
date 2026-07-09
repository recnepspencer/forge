# Connect an agent

## What this feature is

This guide is for agents that need to call `worth-server` as an execution
backend.

An agent usually needs only a small set of actions:

- read data
- write data
- upload files
- download files
- optionally stream large read results

## Why you use it

Agents benefit from `worth-server` when they need:

- one strict boundary for reads and writes
- exact validator and basis handling
- idempotent retries for writes
- explicit file transfer lifecycle rules
- denials that are semantic and debuggable instead of vague transport failures

## Stable entry points

Inside Rust:

- `WorthServer::compat_http()`
- `prepare_request(...)`
- `read(...)`
- `mutate(...)`
- `stream(...)`
- `upload(...)`
- `download(...)`

Across a network boundary:

- `/compat/reads/{operation}`
- `/compat/mutations/{operation}`
- `/compat/streams/{operation}`
- `/compat/uploads/{operation}`
- `/compat/downloads/{operation}`

## Core mental model

Agents should think in terms of "declared operation plus explicit preconditions"
rather than "fire arbitrary JSON at an endpoint."

That means:

- name the operation directly
- keep basis and validator context explicit
- use `idempotency-key` for retried writes
- let the server deny dishonest resumes, stale validators, or malformed uploads

## How it executes

A practical agent loop looks like this:

1. Read with validator and basis capture.
2. Decide whether to write.
3. Write with `idempotency-key` and preconditions when appropriate.
4. Upload or download through the specialized route families instead of hiding
   binary data inside ordinary JSON mutation bodies.

## Small example

```text
1. GET /compat/reads/tasks.list
2. Save validator and basis from the result.
3. POST /compat/mutations/tasks.insert with:
   - idempotency-key: task-123-create
   - if-match: "current-validator"
4. If denied, inspect the semantic denial code before retrying.
```

## Real example

If you are wiring a coding agent or workflow agent:

- use reads to fetch canonical task or workspace state
- carry validator and basis forward when you need exact freshness
- use writes with idempotency for tool retries
- move large result delivery to streams
- move binary material to upload/download routes

The important behavior is that the server stays honest when the agent gets
messy. Duplicate retries, stale preconditions, malformed multipart bodies, and
fake resume claims all get denied before they become expensive truth drift.

## Inspection and debugging

Teach the agent to preserve and inspect:

- denial code
- denial detail
- validator
- basis
- idempotency-key
- retry posture for downloads

Do not reduce every failure to "HTTP request failed." That throws away the best
part of the surface.

## Anti-patterns

- Treating the compat routes as generic RPC with no precondition discipline.
- Sending file bytes inside ordinary mutation JSON.
- Retrying writes without idempotency.
- Dropping basis and validator context between agent steps.

## Current limits

- The stable docs are route-family-first, not SDK-first.
- Durable resume claims are narrower than runtime-backed retries.
- Public cacheability is intentionally not the default.

## Related docs

- [Read data](./read-data.md)
- [Write data](./write-data.md)
- [Upload files](./upload-files.md)
- [Download files](./download-files.md)
