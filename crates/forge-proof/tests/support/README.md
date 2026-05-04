# Forge Proof Test Support Topology

This support tree is organized by proof responsibility, not convenience.

Ownership split:

- `compile_fail/`
  - compile-fail case vocabulary
  - compile-fail bundle construction
  - compile-fail family execution helpers
- `type_shapes/`
  - type-shape report vocabulary
  - codegen-honesty report vocabulary
  - debt inventory vocabulary
- `proof_shapes/`
  - proof-shape digest vocabulary
  - basis digest vocabulary
- `milestone1/`
  - Milestone 1 evidence derivation
  - Milestone 1 compile-fail bundle definition
  - Milestone 1 type-shape derivation
  - Milestone 1 proof-shape and basis digest derivation
  - Milestone 1 codegen/debt note derivation

Rules:

- generic compile-fail mechanics belong under `compile_fail/`
- generic report types belong under `type_shapes/`
- generic digest vocabularies for proof-bearing surfaces belong under `proof_shapes/`
- milestone-local evidence derivation belongs under the milestone module
- no domain-specific migration pressure belongs here yet because Milestone 1
  does not ship migration suites
