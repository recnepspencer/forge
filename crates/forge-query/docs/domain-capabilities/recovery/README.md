# Recovery Overview

Use the recovery docs when your app already has a typed Query stop and needs to
decide what to do next without flattening the cause.

Start here if you want the high-level map:

- [Recovery Boundary](../recovery-boundary.md): the main API surface
- [Aspect-Native Recovery](./aspect-native-recovery.md): how aspect truth
  changes repair guidance
- [Foundational Support And Evidence Strength](./foundational-support-and-evidence-strength.md):
  how support-grade, checked, and proof-visible answers differ
- [Recovery Requests And Next-Step Actions](./recovery-requests-and-next-step-actions.md):
  how to translate a brief into app behavior

The short version:

- `ForgeQueryRecoveryBrief` answers what stopped and what to do next
- `ForgeQueryRecoveryExplanation` answers why and how strong that answer is
- `ForgeQueryRecoveryRequest` gives you one typed next-step request shape
