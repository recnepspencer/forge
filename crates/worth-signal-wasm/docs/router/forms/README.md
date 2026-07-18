# Router And Forms

The router decides route authority. A form owns source, draft, and effective
values. The seam is a typed handoff that tells a route-coupled form whether to
preserve, freeze, discard, or defer its draft.

Start with [Route Authority Handoff](./route_authority_handoff.md), then read:

- [Draft Continuity](./draft_continuity.md)
- [Route-Coupled Behavior](./route_coupled_behavior.md)
- [Continuity Audit](./continuity_audit.md)

Do not copy route ids into form state or let navigation silently erase a draft.
The handoff makes the decision visible and verifiable.
