use super::forbidden_surface::WorthTouchedGraphConflictForbiddenSurface as Forbidden;
use super::phase_fifteen_public_proof_semantic_source_registry::phase_fifteen_public_proof_semantic_source_coverages;
use super::phase_fifteen_semantic_source_registry::phase_fifteen_semantic_source_coverages;
use super::phase_twelve_semantic_source_registry::{
    phase_twelve_semantic_source_coverages, SemanticSourceCoverage,
};

pub(crate) fn phase_fourteen_raw_construction_semantic_source_coverages(
) -> Vec<SemanticSourceCoverage> {
    phase_twelve_semantic_source_coverages()
        .iter()
        .copied()
        .chain(phase_fifteen_semantic_source_coverages())
        .chain(phase_fifteen_public_proof_semantic_source_coverages())
        .filter(|coverage| {
            matches!(
                coverage.forbidden_surface(),
                Forbidden::DisplacedCacheKeyCarrier
                    | Forbidden::LocalComparatorFolklore
                    | Forbidden::CallerOwnedReuseDecision
                    | Forbidden::LocalPublicProofFabrication
                    | Forbidden::LocalDiagnosticAuthorityFabrication
            )
        })
        .collect()
}
