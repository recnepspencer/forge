# Route Declaration And Matching

Declare route grammar once, resolve it with `signals.router.define(...)`, and
use the returned references everywhere else. A **projection** is the structural
answer to “what matches this URL?” It includes layouts, outlets, controllers,
graphs, resources, and breadcrumbs, but it is not an access decision.

Read in this order:

1. [Route Schema Authoring](./route_schema_authoring.md)
2. [Projected Candidates](./projected_candidates.md)
3. [Layout Placement](./layout_placement.md)
4. [Outlet Contracts](./outlet_contracts.md)
5. [Route Capabilities](./route_capabilities.md)
6. [Projection Verification](./projection_verification.md)

The small route tree scales into the full system by adding declarations. You
do not replace it with a “serious” router when the app grows.
