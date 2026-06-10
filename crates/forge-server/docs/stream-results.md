# Stream results

## What this feature is

Use the streaming surface when the result is still one canonical read, but you
want delivery to happen incrementally instead of only as one buffered payload.

Stable entry points:

- `ForgeServerCompatibilityFacade::stream(...)`
- `ForgeServerStreamSelection`
- `ForgeServerStreamingResponse`

Normalized route family:

```text
GET|HEAD /compat/streams/{operation}
```

## Why you use it

Use this when you need:

- the same read semantics as a buffered compatibility read
- chunked delivery for larger payloads
- explicit fallback when synchronous delivery would be dishonest
- cancellation accounting instead of silent transport loss

## Core mental model

Streaming changes transport, not truth.

The server guarantees:

- chunk boundaries do not change the canonical read artifact
- the first chunk can arrive without full buffering
- `HEAD` is explicitly buffered with zero payload transfer
- if synchronous streaming is not honest, the server can return a background
  export outcome instead

## How it executes

1. Prepare the request for the streaming route family.
2. Call `stream(...)` with a `ForgeServerStreamSelection`.
3. Handle one of three outcomes:
   - incremental stream
   - buffered export
   - background export fallback

Cancellation is first-class:

- client disconnect
- downstream backpressure
- caller cancellation

All three produce explicit cancellation receipts instead of being hidden as
"someone probably closed the socket."

## Small example

```rust
use forge_proof::TransitionOutcome;
use forge_server::{ForgeServerStreamSelection, ForgeServerStreamingResponse};

let response = match server.compat_http().stream(
    prepared_stream_execution_input,
    ForgeServerStreamSelection::incremental().with_chunk_bytes(16 * 1024),
) {
    TransitionOutcome::Success(value) => value,
    other => panic!("expected streaming success, got {other:?}"),
};

match response {
    ForgeServerStreamingResponse::Stream(mut stream) => {
        while let Some(chunk) = stream.next_chunk().expect("chunk should serialize") {
            consume(chunk.bytes());
        }
        let export = stream.finish().expect("stream should finish");
        inspect(export.read().validator().canonical_digest());
    }
    ForgeServerStreamingResponse::Buffered(export) => {
        consume(export.payload_bytes());
    }
    ForgeServerStreamingResponse::BackgroundExport(export) => {
        schedule_follow_up(export);
    }
}
```

## Real example

The honest production pattern is:

- try incremental streaming for normal interactive reads
- configure chunk sizing for transport efficiency, not semantics
- set a background export threshold when you would rather switch modes than
  pretend giant synchronous delivery is cheap

If the result is too large for your sync contract, the server can admit a
background export path instead of forcing a misleading slow path.

## Inspection and debugging

Useful counters and fields:

- emitted chunks
- emitted bytes
- first-chunk-without-full-buffer evidence
- background export fallback counters
- cancellation kind receipts

If `finish()` fails with `StreamNotFullyConsumed`, that is a caller bug or an
intentional early stop, not a valid completed stream.

## Anti-patterns

- Treating chunk size as part of the data contract.
- Assuming `HEAD` should stream empty chunks.
- Ignoring cancellation receipts.
- Forcing synchronous delivery after the server already signaled background
  export would be more honest.

## Current limits

- The compatibility surface preserves read truth but does not promise durable
  stream resumption by itself.
- Background export is explicit fallback behavior, not hidden buffering.

## Related docs

- [Read data](./read-data.md)
- [Download files](./download-files.md)
- [Runtime-backed vs durable](./runtime-backed-vs-durable.md)
