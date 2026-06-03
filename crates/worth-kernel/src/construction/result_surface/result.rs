use crate::construction::admitted_scaffold::prepare_primitive_construction_admitted_artifact;
use crate::construction::artifact::CanonicalPrimitiveConstructionArtifact;
use crate::construction::evidence::PrimitiveConstructionResultEvidence;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use crate::construction::PrimitiveConstructionIntent;
use topology::facade::{
    TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryMutationSurface,
    TopologyPrimitiveConstructionQueryHandoff,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};
use worth_spatial::facade::bindings::AdmittedPrimitiveConstructionBirthConsequence;

use super::digest::digest_owned_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedPrimitiveConstructionResult {
    canonical_artifact: CanonicalPrimitiveConstructionArtifact,
    evidence: PrimitiveConstructionResultEvidence,
    result_digest: String,
}

impl PreparedPrimitiveConstructionResult {
    fn new(
        canonical_artifact: CanonicalPrimitiveConstructionArtifact,
        evidence: PrimitiveConstructionResultEvidence,
    ) -> Self {
        let result_digest = digest_owned_parts(&[
            canonical_artifact.artifact_digest().to_string(),
            evidence
                .result_assembly_report()
                .report_digest()
                .to_string(),
            evidence
                .birth_consequence()
                .consequence_digest()
                .to_string(),
            evidence.birth_mapping_digest().to_string(),
            evidence
                .topology_query_admitted_handoff()
                .admitted_handoff_digest()
                .to_string(),
        ]);
        Self {
            canonical_artifact,
            evidence,
            result_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.canonical_artifact.family()
    }

    pub fn topology_birth_class(&self) -> &str {
        self.canonical_artifact.topology_birth_class()
    }

    pub fn realization_report(&self) -> &PrimitiveRealizationReport {
        self.canonical_artifact.realization_report()
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.canonical_artifact.realization_strategy()
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.canonical_artifact.attempted_realization_strategies()
    }

    pub fn attempted_realization_strategy_count(&self) -> usize {
        self.canonical_artifact
            .attempted_realization_strategy_count()
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.canonical_artifact.stability_class()
    }

    pub fn artifact_digest(&self) -> &str {
        self.canonical_artifact.artifact_digest()
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.canonical_artifact.feature_conditioning_class()
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.canonical_artifact.support_normal_class()
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.canonical_artifact.normalization_disposition()
    }

    pub fn birth_truth_digest(&self) -> &str {
        self.canonical_artifact.birth_truth_digest()
    }

    pub fn birth_completeness_digest(&self) -> &str {
        self.canonical_artifact.birth_completeness_digest()
    }

    pub fn topology_fact_digest(&self) -> &str {
        self.canonical_artifact.topology_fact_digest()
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.canonical_artifact.mutation_surface()
    }

    pub fn inspection_surface(&self) -> TopologyConstructionQueryInspectionSurface {
        self.canonical_artifact.inspection_surface()
    }

    pub fn supported_loop_count(&self) -> usize {
        self.canonical_artifact.supported_loop_count()
    }

    pub fn supported_body_count(&self) -> usize {
        self.canonical_artifact.supported_body_count()
    }

    pub(crate) fn birth_consequence(&self) -> &AdmittedPrimitiveConstructionBirthConsequence {
        self.evidence.birth_consequence()
    }

    pub(crate) fn topology_query_handoff(&self) -> &TopologyPrimitiveConstructionQueryHandoff {
        self.evidence.topology_query_handoff()
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionResultError {
    Phase(PrimitiveConstructionPhaseError),
}

impl std::fmt::Display for PrimitiveConstructionResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Phase(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionResultError {}

pub fn prepare_primitive_construction_result<I: Into<PrimitiveConstructionIntent>>(
    intent: I,
) -> Result<PreparedPrimitiveConstructionResult, PrimitiveConstructionResultError> {
    let intent = intent.into();
    let request = intent.request().clone();
    let admitted_artifact = prepare_primitive_construction_admitted_artifact(&request)
        .map_err(PrimitiveConstructionResultError::Phase)?;
    let canonical_artifact =
        CanonicalPrimitiveConstructionArtifact::from_admitted_artifact(&admitted_artifact);
    let evidence = PrimitiveConstructionResultEvidence::from_admitted_artifact(&admitted_artifact);
    Ok(PreparedPrimitiveConstructionResult::new(
        canonical_artifact,
        evidence,
    ))
}

#[cfg(test)]
mod tests {
    use crate::construction::{
        PrimitiveConstructionFamily, PrimitiveConstructionIntent, RegularPrismSpec,
    };
    use worth_spatial::facade::bindings::SpatialConstructionBirthMappingKind;

    use super::prepare_primitive_construction_result;

    #[test]
    fn prepared_result_surface_bundles_phase_chain_artifact_and_birth_mapping() {
        let result = prepare_primitive_construction_result(
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 6,
                radius: 1.0,
                height: 2.0,
            }),
        )
        .expect("prepared result");

        assert_eq!(result.family(), PrimitiveConstructionFamily::RegularPrism);
        assert_eq!(result.topology_birth_class(), "closed_regular_prism_body");
        assert_eq!(
            result
                .birth_consequence()
                .row_for(SpatialConstructionBirthMappingKind::Face)
                .expect("face row")
                .mapped_count(),
            8
        );
        assert_eq!(
            result.birth_completeness_digest(),
            result.birth_consequence().consequence_digest()
        );
        assert_eq!(
            result.topology_fact_digest(),
            result
                .topology_query_handoff()
                .topology_query_envelope()
                .fact_digest()
        );
        assert_eq!(result.family(), PrimitiveConstructionFamily::RegularPrism);
        assert!(!result.result_digest().is_empty());
    }
}
