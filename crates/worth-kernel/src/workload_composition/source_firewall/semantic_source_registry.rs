use super::phase_twelve_semantic_source_registry::phase_twelve_semantic_source_coverages as phase_twelve_coverages;
pub(crate) use super::phase_twelve_semantic_source_registry::SemanticSourceCoverage;
#[cfg(test)]
pub(crate) use super::phase_fourteen_raw_construction_registry::phase_fourteen_raw_construction_semantic_source_coverages;
#[cfg(test)]
pub(crate) use super::phase_fourteen_reintroduction_registry::phase_fourteen_reintroduction_semantic_source_coverages;

pub(crate) fn phase_fifteen_semantic_source_coverages() -> Vec<SemanticSourceCoverage> {
    let mut coverages = phase_twelve_coverages().to_vec();
    coverages.extend(
        super::phase_fifteen_semantic_source_registry::phase_fifteen_semantic_source_coverages(),
    );
    coverages.extend(
        super::phase_fifteen_public_proof_semantic_source_registry::phase_fifteen_public_proof_semantic_source_coverages(),
    );
    coverages
}
