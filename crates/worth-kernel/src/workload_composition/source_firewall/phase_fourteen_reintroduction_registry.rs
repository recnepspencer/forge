#[cfg(test)]
use super::forbidden_surface::WorthTouchedGraphConflictForbiddenSurface as Forbidden;
#[cfg(test)]
use super::phase_fifteen_public_proof_semantic_source_registry::phase_fifteen_public_proof_semantic_source_coverages;
#[cfg(test)]
use super::phase_fifteen_semantic_source_registry::phase_fifteen_semantic_source_coverages;
#[cfg(test)]
use super::phase_twelve_semantic_source_registry::{
    phase_twelve_semantic_source_coverages, SemanticSourceCoverage,
};

#[cfg(test)]
pub(crate) fn phase_fourteen_reintroduction_semantic_source_coverages(
) -> Vec<SemanticSourceCoverage> {
    phase_twelve_semantic_source_coverages()
        .iter()
        .copied()
        .chain(phase_fifteen_semantic_source_coverages())
        .chain(phase_fifteen_public_proof_semantic_source_coverages())
        .filter(|coverage| {
            matches!(
                coverage.forbidden_surface(),
                Forbidden::EntityOnlyOverlapHelper
                    | Forbidden::GenericOverlapSecondAuthorityLane
                    | Forbidden::PlannerRouteConstruction
                    | Forbidden::SupportWrapperShortcut
                    | Forbidden::LegacyExplainerImport
            )
        })
        .collect()
}
