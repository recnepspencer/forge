//! R8.41 — Foundational material alone cannot authorize undo.
//!
//! Fails on the *types* of both arguments, not on how many there are. A
//! Foundational lineage outcome is descriptive material any caller can name;
//! `admit_undo` names a move-only recovery handle and a privately minted effect
//! authority, so no amount of Foundational evidence reaches this signature.

use worth_foundational::facade::FoundationalBoundaryEvidenceLineageOutcomeKind;
use worth_query_execution::facade::provisional_aftermath::admit_undo;

fn foundational_material_cannot_stand_in_for_a_handle() {
    let lineage = FoundationalBoundaryEvidenceLineageOutcomeKind::SingularContinuity;
    let _ = admit_undo(lineage, &lineage);
}

fn main() {}
