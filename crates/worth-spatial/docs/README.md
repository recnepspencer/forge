<!-- worth-doc
crate: worth-spatial
kind: crate_readme
id: worth-spatial-docs
doc_style: semantic-first,authority-first
neighbor_crates: worth-kernel, worth-topo, worth-geom, forge-query
categories: foundations, features, boundaries
-->

# worth-spatial Docs

`worth-spatial` is the semantic-first crate in the Worth stack.

It owns:

- authored spatial vocabulary
- construction-time birth semantics
- spatial completeness, impossibility, and retained spatial truth surfaces

It does not own:

- topology truth authority; that belongs to `worth-topo`
- runtime lifecycle or support posture; that belongs to `forge-query`
- kernel orchestration; that belongs to `worth-kernel`

## Reading Style

These docs are semantic-first and authority-first.

- Start here if you need to know what a spatial artifact means.
- Jump to a boundary doc when you need to know where topology or Query takes
  over.
- Jump to `worth-kernel` when you need workflow orchestration or the common
  construction path.
- Jump to `worth-topo` when you need topology authority or runtime-support
  posture for topology-domain families.
- Jump to `worth-geom` when you need analytic geometry primitives instead of
  spatial meaning.

## Map

- Foundations
  - [Spatial Overview](./foundations/spatial-overview.md)
- Features
  - [Construction-Time Birth Bindings](./features/construction-time-birth-bindings.md)
  - [Birth Completeness And Impossibility](./features/birth-completeness-and-impossibility.md)
  - [Birth Truth Artifacts](./features/birth-truth-artifacts.md)
- Boundaries
  - [Spatial To Topo](./boundaries/spatial-to-topo.md)
  - [Spatial Query Proof Posture](./boundaries/spatial-query-proof-posture.md)

## Neighboring Crates

- `worth-kernel` owns construction and workload orchestration.
- `worth-topo` owns topology truth authority.
- `worth-geom` owns pure geometry primitives and algorithms.
