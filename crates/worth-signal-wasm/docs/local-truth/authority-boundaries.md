# Standalone And Platform Authority Boundaries

## What This Feature Is

Worth has two intentionally different application-value authority lanes. A
browser-only app may use process-local TypeScript local truth. A full Rust
platform application uses Query and Relational for authoritative state. Both
lanes can feed Signal, which evaluates derived work.

## Why You Use It

- Choose the standalone lane for an in-memory browser workflow or demo.
- Choose the platform lane for durable, shared, regulated production state.
- Use this guide when deciding where branch history or merge policy belongs.

## Stable Entry Points

- Standalone: `localTruthSchema(...)` and `signals.localTruth(...)`.
- Platform: the public Worth Query workflow backed by Worth Relational.
- Derived execution in both lanes: Worth Signal.

## Core Mental Model

Standalone:

```text
TypeScript Local Truth -> Signal
```

Platform:

```text
Query -> Relational -> Bridge -> Signal
```

The arrow is one-way authority flow. A downstream Signal proof can explain
invalidation, recomputation, branch lifecycle, or replay. It cannot authorize
an upstream application-value commit.

## How It Executes

In standalone mode, TypeScript publishes one local-truth commit and then asks
Signal to derive. In platform mode, Relational commits authoritative state and
the bridge transports committed change to Signal. Signal failure never rewrites
the upstream commit in either lane.

## Small Example

```ts
const local = signals.localTruth({
  authorityId: "browser-workspace",
  schema,
  initialEntities,
  bindings,
});
```

The returned authority is explicitly `inMemoryProcessLocal`.

## Real Example

A regulated service that needs retention, cross-process concurrency, and
durable audit uses Query -> Relational. Its browser may still render Signal
outputs, but it does not replace platform truth with a local journal.

## How It Relates To Other Features

- Resource effects use disposable Signal branches for pending intent.
- Forms and router projections may consume state; they do not become a second
  truth store.
- Local truth is useful for standalone branch editing, not platform MVCC.

## Inspection And Debugging

Standalone inspection reports `supportPosture: "inMemoryProcessLocal"`.
Platform inspection and durability evidence come from Query and Relational,
not from this package.

## Anti-Patterns

- Do not add a local-truth journal to native Signal.
- Do not call process-local history durable or restart-stable.
- Do not let a UI cache or form draft become a hidden competing authority.

## Current Limits

- This npm package does not embed Worth Relational.
- Standalone local truth does not provide MVCC, persistence, replication, or
  authenticated audit identity.
- Platform merge guidance belongs to the Query and Relational documentation.

## Related Docs

- [Branch Merge And Manual Resolution](./branch-merge.md)
- [App Surface Overview](../app-surface/overview.md)
