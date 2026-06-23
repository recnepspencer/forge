<!-- worth-doc
crate: worth-kernel
kind: crate_readme
id: worth-kernel-docs
doc_style: workflow-first,authority-first
neighbor_crates: worth-spatial, worth-topo, worth-geom, forge-query
categories: foundations, features, boundaries
-->

# worth-kernel Docs

`worth-kernel` is the workflow-first crate in the Worth stack.

It owns:

- primitive and workload-oriented orchestration
- canonical construction artifact policy
- the kernel-side Query adoption and proof posture used to certify that Worth
  consumes Query honestly

It does not own:

- topology truth authority; that belongs to `worth-topo`
- spatial birth truth; that belongs to `worth-spatial`
- runtime lifecycle semantics; those belong to `forge-query`
- a second public construction runtime facade

Read these docs when you need to understand how an admitted construction or
workload flow is assembled across Query, spatial birth truth, and topology
authority.

## Reading Style

These docs are workflow-first and authority-first.

- Start with the feature doc that matches the workflow you want.
- Drop to a boundary doc when you need to see where responsibility changes.
- Jump to neighboring crate docs when the next authority boundary is owned
  elsewhere.

## Map

- Foundations
  - [Kernel Overview](./foundations/kernel-overview.md)
  - [Execution Context And Artifact Policy](./foundations/execution-context-and-artifact-policy.md)
- Features
  - [Primitive Construction](./features/primitive-construction.md)
  - [Shell-With-Hole Construction](./features/shell-with-hole-construction.md)
  - [Wire-Body Construction](./features/wire-body-construction.md)
  - [Construction Simulation](./features/construction-simulation.md)
  - [Construction Replay](./features/construction-replay.md)
  - [Construction Results And Diagnostics](./features/construction-results-and-diagnostics.md)
- Boundaries
  - [Kernel To Spatial](./boundaries/kernel-to-spatial.md)
  - [Worth To Query](./boundaries/worth-to-query.md)

## Neighboring Crates

- Go to `worth-spatial` when you need the semantic meaning of birth truth,
  binding, or spatial impossibility.
- Go to `worth-topo` when you need topology authority, topology certification,
  or topology-domain runtime support.
- Go to `worth-geom` when you need analytic geometry primitives rather than
  kernel orchestration.
