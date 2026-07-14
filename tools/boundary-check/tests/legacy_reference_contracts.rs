//! Consolidated shrink-only legacy-reference contracts.

mod legacy_reference_fixture;

#[path = "legacy_reference_contracts/hostile.rs"]
mod hostile;
#[path = "legacy_reference_contracts/ratchet.rs"]
mod ratchet;
