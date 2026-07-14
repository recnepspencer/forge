# Router Glossary

This page defines the core router vocabulary used across the router docs.

- **Raw location authority**: a local URL-shaped input that has not yet been
  canonicalized.
- **Canonical route artifact**: the normalized route truth for one route plus
  typed search and hash state.
- **Route reference**: the stable public handle returned by
  `signals.router.define(...)` for one declared route.
- **Route location**: a built location from a route reference and concrete
  params/search/hash input.
- **Projected route candidate**: the matched route/layout/outlet structure
  before admission runs.
- **Admission**: the step that turns a projected candidate into an admitted
  route or a non-admitted route outcome.
- **Prerequisite**: a route admission check declared with
  `signals.router.prerequisite(...)`.
- **Recovery**: a route repair step declared with `signals.router.recovery(...)`
  that can turn a failed admission into a recovered route.
- **Route outcome**: the result of route admission. This is one of `admitted`,
  `redirect`, `notFound`, `forbidden`, `unavailable`, or `denied`.
- **Browser-history ingress**: an explicit host envelope for browser-driven URL
  changes.
- **Browser-history writeback**: an explicit host envelope for graph-issued
  history mutation.
- **Boundary artifact**: the typed explanation of what happened at a host
  boundary, such as `routeTruthConverged` or `routeOutcomeNotAdmitted`.
- **Route-history entry**: one retained admitted route truth entry inside a
  browser-history story.
- **Back provenance**: the previous admitted route-history entry, wrapped as a
  restore/replay-capable artifact.
- **Inspection**: the browser-history summary surface that aggregates current,
  back, breadcrumb, and coherence evidence.
- **Auditability**: the closeout surface that answers why the current route is
  visible and which boundary or restore truth explains it.

Related docs:

- [Router Overview](./index.md)
- [Authority](./authority/README.md)
- [Admission](./admission/README.md)
- [History](./history/README.md)
