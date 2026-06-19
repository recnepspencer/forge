<!-- worth-doc
crate: worth-geom
kind: crate_readme
id: worth-geom-docs
doc_style: pure-geometry-first
neighbor_crates: worth-kernel, worth-spatial, worth-topo
categories: foundations, features, boundaries
-->

# worth-geom Docs

`worth-geom` is the pure-geometry-first crate in the Worth stack.

It owns:

- analytic geometry primitives
- curve and surface representations
- geometry algorithms and measurement utilities

It does not own:

- topology truth
- Query runtime lifecycle
- kernel workflow orchestration

## Reading Style

These docs are pure-geometry-first.

Start here when you need the geometric substrate itself rather than runtime,
kernel, or topology behavior.

- Jump to `worth-spatial` when geometry results need spatial meaning.
- Jump to `worth-topo` when the question becomes topology authority.
- Jump to `worth-kernel` when the question is about end-to-end workflow
  orchestration.

## Map

- Foundations
  - [Geometry Overview](./foundations/geometry-overview.md)
- Features
  - [Analytic Primitives And Planes](./features/analytic-primitives-and-planes.md)
  - [Curve And Surface Schema](./features/curve-and-surface-schema.md)
  - [Spatial Acceleration And Matching](./features/spatial-acceleration-and-matching.md)
  - [Boundary Certification And Intersection](./features/boundary-certification-and-intersection.md)
  - [Primitive Realization Strategies](./features/primitive-realization-strategies.md)
- Boundaries
  - [Geom To Spatial Authority Boundary](./boundaries/geom-to-spatial-authority-boundary.md)

## Neighboring Crates

- `worth-spatial` owns spatial semantics layered over geometry.
- `worth-topo` owns topology truth authority.
- `worth-kernel` owns workflow orchestration.
