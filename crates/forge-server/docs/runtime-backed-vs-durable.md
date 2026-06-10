# Runtime-backed vs durable

## What this feature is

This is the distinction that matters most for retries, resumes, and restart
promises.

`forge-server` makes a hard distinction between:

- runtime-backed continuation
- durable restart-stable continuation

## Why you use it

You need this distinction so you do not promise callers more than the server
actually admitted.

If you blur the two together, you will eventually:

- overclaim retry safety
- corrupt client expectations after restart
- hide genuine product gaps behind optimistic wording

## Stable entry points

The most visible place this shows up today is binary download resume planning:

- `ForgeServerCompatibilityFacade::plan_binary_resume(...)`
- download session `retry_posture()`
- integrity planning and verification

It also matters anywhere a caller wants to continue work from retained runtime
state instead of starting fresh.

## Core mental model

Runtime-backed means:

- the currently admitted runtime can continue the work
- the server can tell you what the next valid step is
- the continuation may rely on retained in-memory or current-process state

Durable means:

- the continuation contract survives the stronger boundary you care about
- restart stability is part of the claim
- another process or later runtime can still honor the same continuation

Those are not the same promise.

## How it executes

When a resumed binary download is admitted today:

- `is_resume()` can be true
- `restart_stable()` can still be false
- the server can still verify the expected integrity and next byte range

That is honest and useful. It lets the caller continue transfer without lying
about durability.

## Small example

```rust
let resumed = compat_download_success(/* ... */);

assert!(resumed.session().retry_posture().is_resume());
assert!(!resumed.session().retry_posture().restart_stable());
```

## Real example

A UI client or agent can safely resume an interrupted download during the same
runtime-backed continuity window.

What it must not do is advertise that resume token as a durable cross-restart
asset unless the server explicitly upgrades the contract to say so.

That difference is exactly how you avoid "it worked in dev" retry semantics that
turn into expensive corruption or support issues in production.

## Inspection and debugging

When debugging retry behavior, inspect:

- whether the request was a true resume
- whether restart stability was actually admitted
- expected next start offset
- integrity verification outcome

If a caller assumed durability and the posture only says runtime-backed, the
bug is in the caller contract, not in the server.

## Anti-patterns

- Treating every resume token as durable.
- Using "resume supported" language with no durability qualifier.
- Rebuilding resume ranges locally instead of reading retry posture from the
  admitted session.

## Current limits

- Runtime-backed continuation is stronger than blind retry but weaker than
  durable continuation.
- Durable restart-stable continuation should be documented only where the
  server explicitly admits it.

## Related docs

- [Download files](./download-files.md)
- [Stream results](./stream-results.md)
- [Connect another backend](./connect-another-backend.md)
