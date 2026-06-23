# Geometry Overview

## What This Feature Is

`worth-geom` is the analytic geometry substrate for Worth: planes, points,
surfaces, curves, measurement, intersection, triangulation, and related
geometry algorithms.

## Why You Use It

Use this crate when you need geometry primitives or geometry algorithms, not
topology authority and not runtime-facing workflow behavior.

## Stable Entry Points

- `worth_geom::facade`

## Core Mental Model

Geometry is a binding layer with bounded error and explicit geometric meaning.
It is not topology truth authority.

## How It Executes

External callers enter through the facade. Downstream crates consume geometry
results and then attach their own semantics or authority boundaries.

## Small Example

Use `worth_geom::facade` when you need an analytic plane, point relation,
polygon calculation, or geometric measurement primitive.

## Real Example

`worth-spatial` and higher layers rely on these geometry primitives, but they
must not treat geometry outputs as if they were already topology truth or
runtime proof.

## How It Relates To Other Features

- [Geom To Spatial Authority Boundary](../boundaries/geom-to-spatial-authority-boundary.md)

## Inspection And Debugging

If the problem is numerical or geometric, inspect `worth-geom`. If the problem
is runtime posture, birth semantics, or topology truth, move to the owning
crate instead.

## Anti-Patterns

- treating geometry outputs as topology authority
- importing deep internal modules instead of the public facade

## Current Limits

This Phase 6 doc only establishes the crate map and the boundary to spatial
semantics.

## Related Docs

- [Geom To Spatial Authority Boundary](../boundaries/geom-to-spatial-authority-boundary.md)
