# Lineage, Provenance, Receipts, And Support Truth

This folder documents the Milestone 7 boundary-evidence surface in
`forge-foundational`.

Use these docs when you need to answer questions like:

- Did this identity survive, split, merge, replay, restore, or break?
- Under what basis and freshness posture was this conclusion produced?
- What actually executed, and what was only planned, blocked, or denied?
- What support-grade truth exists when recovery is stale, reduced, replayed, or
  quarantined?
- How do these descriptive surfaces attach to boundary artifacts and
  participate in canonical and digest identity?
- Which public lane is the supported first-contact API, and which stronger
  seams are reserved for readmission and readiness?

Read the docs in this order if you are new to the surface:

1. [Primitive Categories, Locality, And Role Postures](./primitive-categories-locality-and-role-postures.md)
2. [Provenance Layering And Freshness](./provenance-layering-and-freshness.md)
3. [Receipts And Closeout Truth](./receipts-and-closeout-truth.md)
4. [Lineage, Continuity, Divergence, And Promotion](./lineage-continuity-divergence-and-promotion.md)
5. [Support Truth, Recovery, And Degraded Operation](./support-truth-recovery-and-degraded-operation.md)
6. [Attachment Materialization, Canonical Participation, And Readmission](./attachment-materialization-canonical-participation-and-readmission.md)
7. [Grouped Public Lanes And Stronger Readiness](./grouped-public-lanes-and-stronger-readiness.md)
8. [Boundary Evidence Production Readiness](./boundary-evidence-production-readiness.md)

Capability order matters.

- Start with primitives so category, locality, freshness, and
  planned-versus-executed posture mean one thing.
- Build provenance before you claim continuity or execution truth.
- Build receipts before you claim what actually happened.
- Build lineage after provenance and executed-boundary truth are already typed.
- Build support truth after the stronger execution and continuity lanes are
  stable.
- Attach and materialize only after the families themselves are settled.
- Use the grouped public lanes when you want the supported first-contact API.
- Use the readiness artifact when you need the exact machine-checkable closure
  contract for the milestone.

These docs are feature-first on purpose. They are not milestone notes or
closeout notes. If a capability shipped, it has a home here.

The crate-facing API surface these docs describe lives under:

- `forge_foundational::boundary_evidence_api::common_path`
- `forge_foundational::boundary_evidence_api::lower_lane`
- `forge_foundational::boundary_evidence_api::stronger_lane`
- `forge_foundational::boundary_evidence_api::boundary_evidence_public_surface_inventory()`
