use super::phase_twelve_semantic_source_registry::phase_twelve_semantic_source_coverages as phase_twelve_coverages;
pub(crate) use super::phase_twelve_semantic_source_registry::SemanticSourceCoverage;

pub(crate) fn phase_fifteen_semantic_source_coverages() -> Vec<SemanticSourceCoverage> {
    let mut coverages = phase_twelve_coverages().to_vec();
    coverages.extend(
        super::phase_fifteen_semantic_source_registry::phase_fifteen_semantic_source_coverages(),
    );
    coverages
}
