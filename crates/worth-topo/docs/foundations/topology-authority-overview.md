# Topology Authority Overview

## What This Feature Is

`worth-topo` owns topology truth semantics and topology-domain interpretation on
top of the Forge runtime stack.

## Why You Use It

Use this crate when the question is about topology authority, topology-domain
reads, topology-domain operators, or the admitted topology runtime support
surface.

## Stable Entry Points

- `worth_topo::facade`
- `worth_topo::runtime_support`
- `worth_topo::query_domain`

## Core Mental Model

`worth-topo` is not geometry and it is not a second runtime. It is topology
authority layered over Query-owned runtime mechanics.

## How It Executes

Query owns runtime lifecycle. `worth-topo` owns the topology-domain meaning of
the admitted read, edit, and certification surfaces on that runtime path.

## Small Example

If you need to know whether a topology-domain family is admitted, inspect the
runtime-support surface. If you need to know what an executed topology read
proved, inspect the domain-read surface.

## Real Example

This distinction matters because runtime admission and executed proof answer
different questions, and Phase 6 docs need to keep them separate.

## Related Docs

- [Domain Reads](../features/domain-reads.md)
- [Runtime Support](../features/runtime-support.md)
- [Topo Query Runtime Boundary](../boundaries/topo-query-runtime-boundary.md)
