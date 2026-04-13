# Writeback And Promotion

This guide covers the bridge's authoritative handoff story.

Routing and evaluation tell you what changed and what the compute side sees.
Writeback and promotion answer the harder question:

- "when does preview or derived intent cross back into authoritative truth?"

## The Authority Boundary Matters

The bridge is not allowed to blur preview and commit into one thing.

It must keep these states mechanically distinct:

- non-authoritative preview work
- authoritative truth handoff
- no-op outcome
- committed outcome

This is especially important after Milestone 12 and Milestone 12b, where the
bridge gained a real writeback and family-aware promotion story.

## Promotion In The Standard Path

For ordinary preview flows, promotion is surfaced through the speculative
session:

```rust
use forge_runtime_bridge::facade::BridgeSpeculativePromotionRequest;

let promoted = session.promote(BridgeSpeculativePromotionRequest::new(
    "commit-boundary:pricing",
    "authoritative-artifact:pricing",
))?;
```

That is the everyday "make this real" path.

The important thing is not the literal strings in the example.
The important thing is the boundary:

- preview does not become authoritative accidentally

## No-Op Versus Commit

One of the bridge's important responsibilities is to distinguish:

- this request changed authoritative truth
- this request was semantically a no-op

That distinction matters for:

- diagnostics
- replay
- trust
- certification

The bridge should preserve that classification even when the underlying host or
authority path is more complicated.

## Family-Aware Promotion

Milestone 12b made family-aware writeback real bridge territory.

That means promotion is not just:

- "send something back somehow"

It is:

- an admitted, typed, bridge-native handoff

The bridge should keep family-aware semantics explicit without forcing them
into the ordinary happy path unless the job actually needs them.

## Promotion Diagnostics

Promotion should be explainable through the same diagnostics door as the rest
of the bridge:

```rust
let promoted = session.promote(promotion_request)?;

let diagnostics = bridge.diagnostics();
let last_promotion = diagnostics.explain_last_promotion();
let named_promotion = diagnostics.explain_promotion(promoted.id());
```

That is where users should confirm:

- what crossed the authority boundary
- whether it was accepted
- what authoritative identity was produced
- whether the result was a no-op or commit

## Promotion Under Interleave

The bridge must preserve this meaning even when:

- main keeps accepting live changes
- speculative work has diverged
- diagnostics tiers vary
- replay happens later

This is one of the reasons the pricing-shock reference workload is useful.
It forces the bridge to prove that branch-local promotion meaning does not
collapse under real concurrent pressure.

## When To Think About Raw Writeback Surfaces

Most readers should not start with low-level writeback protocol surfaces.

Start with:

- session promotion
- diagnostics explanation
- authoritative versus non-authoritative distinction

Reach deeper only when the job specifically requires:

- explicit authority integration
- writeback family specialization
- replay or parity proof
- advanced host adapter work

## Product Rule

If users can no longer answer these questions clearly, the writeback story is
not good enough:

- was this still preview work or already authoritative?
- did this become a no-op or a commit?
- what proves that result?
- can the same answer be recovered in replay?

The bridge should make those answers obvious rather than implicit.

## Common Pitfalls

- Talking about promotion as though it were just another preview action. It is
  an authority crossing.
- Treating no-op versus commit as unimportant bookkeeping. It is part of the
  trust surface.
- Assuming family-aware writeback only matters to adapters. It is now part of
  the bridge product shape.
- Letting diagnostics talk vaguely about promotion when the job needs an
  explicit authoritative story.
