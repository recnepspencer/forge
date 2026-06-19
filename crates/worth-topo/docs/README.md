<!-- worth-doc
crate: worth-topo
kind: crate_readme
id: worth-topo-docs
doc_style: authority-first
neighbor_crates: worth-kernel, worth-spatial, worth-geom, forge-query
categories: foundations, features, boundaries
-->

# worth-topo Docs

`worth-topo` is the authority-first crate in the Worth stack.

It owns:

- topology truth semantics
- topology-domain read and edit surfaces
- topology runtime-support posture

It does not own:

- geometry semantics
- kernel workflow orchestration
- a second generic runtime outside `forge-query`

## Reading Style

These docs are authority-first.

- Start with the authority overview if you are unsure what topo owns.
- Use the feature docs for domain reads and runtime-support posture.
- Use the boundary doc when you need to see how topology consumes Query.
- Jump to `worth-spatial` when the question is about spatial meaning rather
  than topology authority.
- Jump to `worth-kernel` when the question is about workflow orchestration or
  canonical construction artifacts.
- Jump to `worth-geom` when the issue is geometric substrate rather than
  topology truth.

## Map

- Foundations
  - [Topology Authority Overview](./foundations/topology-authority-overview.md)
- Features
  - [Topology Graph Authority](./features/topology-graph-authority.md)
  - [Topology Certification And Parity](./features/topology-certification-and-parity.md)
  - [Topology Workloads And Seeds](./features/topology-workloads-and-seeds.md)
  - [Domain Reads](./features/domain-reads.md)
  - [Runtime Support](./features/runtime-support.md)
- Boundaries
  - [Topo Query Runtime Boundary](./boundaries/topo-query-runtime-boundary.md)

## Neighboring Crates

- `worth-kernel` owns workflow orchestration.
- `worth-spatial` owns spatial semantics and birth meaning.
- `worth-geom` owns analytic geometry primitives.
