# Profiles And Policy Vocabulary

This folder documents the Milestone 3 profile surface in
`forge-foundational`.

Use these docs when you need to answer questions like:

- How do I declare what level of diagnostics, retention, compatibility, and
  certification a boundary surface should carry?
- How do I move from a requested profile to an admitted or materialized one
  without hiding narrowing?
- How do I attach profile meaning to a boundary, support, or proof-bearing
  target without flattening target legality?
- How do I plan descriptive surfaces honestly before I materialize or certify
  anything?
- What exactly is frozen by the Milestone 3 readiness artifact?

Read the docs in this order if you are new to the surface:

1. [Profile Families And Composed Profile Sets](./profile-families-and-composed-profile-sets.md)
2. [Requested, Admitted, And Materialized Profile Progression](./requested-admitted-and-materialized-profile-progression.md)
3. [Target-Aware Profile Attachment](./target-aware-profile-attachment.md)
4. [Descriptive Surface Materialization And Elision](./descriptive-surface-materialization-and-elision.md)
5. [Profile Identity, Difference, And Canonical Basis](./profile-identity-difference-and-canonical-basis.md)
6. [Proof-Bearing Profile Certification And Readmission](./proof-bearing-profile-certification-and-readmission.md)
7. [Profile Production Readiness](./profile-production-readiness.md)

The order matters.

- Start with profile families so a composed set means one thing.
- Move through requested, admitted, and materialized progression before you
  attach profile meaning to real targets.
- Plan descriptive surfaces before you talk about identity, certification, or
  readiness.
- Treat certification as a stronger lane, not a casual upgrade.
- Use readiness when you need the machine-checkable closure contract for the
  whole milestone.

The grouped public surface for this milestone is also part of what shipped:

- `forge_foundational::profiles_api::common_path`
- `forge_foundational::profiles_api::lower_lane::composition`
- `forge_foundational::profiles_api::lower_lane::progression`
- `forge_foundational::profiles_api::lower_lane::attachment`
- `forge_foundational::profiles_api::lower_lane::materialization`
- `forge_foundational::profiles_api::lower_lane::identity`
- `forge_foundational::profiles_api::lower_lane::certification`
- `forge_foundational::profiles_api::stronger_lane`
- `forge_foundational::profiles_api::stronger_lane::readiness`

These docs are capability-first on purpose. They are not milestone notes,
closeout notes, or test tours. If a profile capability shipped, it should have
one stable home in this folder.
