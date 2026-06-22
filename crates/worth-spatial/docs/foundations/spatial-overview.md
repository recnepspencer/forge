# Spatial Overview

## What This Feature Is

`worth-spatial` owns the semantic layer between analytic geometry and topology
authority. It gives Worth a place for binding, birth meaning, impossibility,
projection-facing spatial facts, and retained spatial interpretation.

## Why You Use It

Use `worth-spatial` when the question is about what a geometric or binding
result means, not how the runtime executes it and not how topology stores it.

## Stable Entry Points

- `worth_spatial::facade`

## Core Mental Model

Spatial is the meaning layer. It consumes Query-owned runtime lifecycle and
hands topology-safe outputs to `worth-topo`.

## How It Executes

Callers typically enter through one of the namespaced facade modules. The exact
runtime path still belongs to Query; spatial owns the semantics expressed on
that path.

## Small Example

If you need anchor binding, placement semantics, planar predicate meaning, or
birth completeness, start with `worth_spatial::facade`.

## Real Example

Milestone 4 and its nearby work widened spatial surfaces for:

- birth bindings
- retained spatial facts
- planar contract and predicate families
- rebinding and recovery posture

Those surfaces all share the same rule: spatial owns the semantics, not the
runtime lifecycle.

## How It Relates To Other Features

- [Construction-Time Birth Bindings](../features/construction-time-birth-bindings.md)
- [Spatial To Topo](../boundaries/spatial-to-topo.md)

## Inspection And Debugging

When a result is semantically wrong but the runtime path looks admitted, this
is usually the right crate to inspect next.

## Anti-Patterns

- treating spatial as topology storage
- treating spatial as the runtime owner
- flattening impossibility into generic construction failure

## Current Limits

These docs cover the shipped Milestone 4 and adjacent admitted surfaces only.

## Related Docs

- [Spatial Query Proof Posture](../boundaries/spatial-query-proof-posture.md)
