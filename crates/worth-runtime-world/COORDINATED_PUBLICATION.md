# Coordinated publication contract

The frozen compiler-visible progression is:

```text
ProductBranchIntent
  -> ResolvedExpectedProductHead
  -> AdmittedCompositeRuntimeWorldBasis
  -> LoweredOwnerComponentPlan
  -> ReservedCompositePublicationAttempt
  -> OwnerExecutionSettlement
  -> CompositePublicationReady
  -> RuntimeWorldPublicationOutcome
```

The terminal outcome has exactly three cases:

- `PerformedCompositePublication` proves the reserved commit won the exact
  product-head compare-and-publish.
- `NoEffectCompositePublication` proves that no component owner and no product
  reference moved.
- `ProductUnpublishedOwnerEffects` records named owner movement when the
  product reference did not move and retains the recovery obligation.

Every `ReservedCompositePublicationAttempt` carries the complete expected
observation, predecessor basis, specialized Relational and Signal plans,
reserved commit/history/recovery/pin capacity, cancellation/deadline policy,
canonical owner order, and exact progress. It is linear and cannot be cloned.

The order is fixed as Relational preparation, exact product-head recheck,
Relational publication and settlement, exact recheck, Signal operation, exact
recheck, and final product publication. No Runtime World lock may be held over
a component-owner call. Unchanged components are represented by `RetainExact`
and do not imply a latest lookup or owner contact.

The Phase 1 service traits are internal seams for the later managed owner. They
do not implement bootstrap or publication and do not create an adapter around
either component owner. The Phase 1 retention root likewise freezes its
dependency classes, independent exact-component keys, opaque RAII obligations,
and obligation-transfer destinations. The later retention lane owns the full
unique-pin registry and component-owner lease calls.

All module declarations and integration-test submodules needed by the next
wave are installed here. A lane adds implementation and focused tests only
under its listed subtree; it does not edit workspace assembly, `lib.rs`, a
module root, the sole facade, or shared phase/outcome definitions.

## Exclusive Phase 2 ownership

The following paths are reserved for the next parallel wave. They may consume
the Phase 1 contracts but may not edit workspace assembly, module roots, the
sole facade, or shared phase/outcome definitions.

| Lane | Exclusive implementation/evidence paths |
| --- | --- |
| Bridge admission | `crates/worth-runtime-bridge/src/correspondence/runtime_world_admission/{admission.rs,denial.rs,tests.rs}` and `crates/worth-runtime-world/tests/runtime_world_certification/bridge.rs` |
| Basis/history | `crates/worth-runtime-world/src/identity/`, `src/basis/`, and `src/history/{catalog.rs,retention.rs,reclamation.rs}`, plus `tests/runtime_world_certification/basis_history.rs` |
| Retention | `crates/worth-runtime-world/src/retention/{component_obligation.rs,registry.rs,unique_component_pin.rs,obligation_transfer.rs}`, plus `tests/runtime_world_certification/retention.rs` |
| Reference | `crates/worth-runtime-world/src/branch/{reference_cell.rs,observation.rs}`, plus `tests/runtime_world_certification/reference.rs` |

Focused commands for those lanes are respectively:

```text
cargo test -p worth-runtime-bridge runtime_world_admission
cargo test -p worth-runtime-world --test runtime_world_certification basis_history
cargo test -p worth-runtime-world --test runtime_world_certification retention
cargo test -p worth-runtime-world --test runtime_world_certification reference
```

The named filters are the expected focused test families; a lane adds its
tests under its exclusive path before using the command. The preinstalled
contract tests ensure each filter already selects evidence in Phase 1.
