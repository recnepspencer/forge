# Artifact Access

This is the shortest honest answer to "what can I expect to exist right now?"

`forge-signal` has three different things that are easy to blur together:

- the core runtime truth
- rich results kept around eagerly
- rich results rebuilt on demand

They are not the same thing.

## The hard guarantee

These are the authoritative runtime facts in every supported policy:

- stable task IDs
- stable segment IDs
- execution report summaries
- replay artifacts
- failure and rollback diagnostics
- enough compact state to check deterministic equivalence

If you need the answer to "what really happened?", start here.
