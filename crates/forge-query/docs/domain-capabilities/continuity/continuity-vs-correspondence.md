# Continuity Vs Correspondence

## What This Feature Is

This doc explains the difference between authoritative continuity and weaker
correspondence so downstream domains do not collapse them into one meaning.

## Why You Use It

- geometry kernels often need both "this is the same truth continued" and
  "this probably corresponds to that older structure"
- the wrong choice here creates false identity guarantees
- the public Query continuity lane is intentionally stronger than loose
  correspondence

## Stable Entry Points

Ordinary continuity lane:

- `forge_query_domain(...).for_admitted_intent_plan(...).preserves_continuity(...).because(...).materialize()`

Sharper correspondence-oriented surfaces live below the ordinary common lane and
materialize through the continuity category's proof and canonical runtime
families.

## Core Mental Model

Continuity means Query can carry authoritative predecessor and successor truth
through an admitted runtime path.

Correspondence means the domain can argue that two things match or relate, but
not that Query has one authoritative continuation fact.

If your geometry operation truly preserved identity through the runtime lane,
use continuity.

If it only established a structural or historical match, use correspondence.

## How It Executes

Continuity executes as admitted-plan-bound authoritative evidence.

Correspondence executes as a weaker continuity-category posture and should not
be taught as if it were the same guarantee.

## Small Example

```rust
let continuity = forge_query_domain("worth.spatial")
    .for_admitted_intent_plan(&plan)
    .preserves_continuity("identity.edge_split", "edge:before", "edge:after")
    .because("the split continues one authoritative edge as one successor")
    .materialize()?;
```

## Real Example

Use continuity when a normalization or rebind step preserves one authoritative
target. Use correspondence when a replay, restore, or structural rematch only
gives you "these probably refer to the same semantic shape."

For geometry work:

- edge split with one canonical successor: continuity
- replay-restored topology matched back to a current edge chain: correspondence

## How It Relates To Other Features

- [Continuity Contributions And Authoritative Successors](./continuity-contributions-and-authoritative-successors.md)
  covers the ordinary continuity path
- [Cross-Runtime Fallback Vs Store-Backed Replay Gap](../explanation/cross-runtime-fallback-vs-store-backed-replay-gap.md)
  often explains why a replay path only supports correspondence-like reasoning

## Inspection And Debugging

- if the runtime can no longer prove the original binding basis, continuity may
  deny or require rebind instead of silently degrading
- do not relabel weaker correspondence as continuity in downstream logs or docs

## Anti-Patterns

- using continuity language when the domain only has structural match evidence
- teaching correspondence as a harmless synonym for continuity
- hiding replay ambiguity behind authoritative-successor vocabulary

## Current Limits

- the ordinary common lane emphasizes authoritative continuity
- correspondence remains a sharper category posture and should be taught more
  carefully

## Related Docs

- [Continuity Contributions And Authoritative Successors](./continuity-contributions-and-authoritative-successors.md)
- [Cross-Runtime Fallback Vs Store-Backed Replay Gap](../explanation/cross-runtime-fallback-vs-store-backed-replay-gap.md)
