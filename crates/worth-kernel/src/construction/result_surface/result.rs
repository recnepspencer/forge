use crate::construction::artifact::{
    build_canonical_primitive_construction_artifact_with_completeness,
    CanonicalPrimitiveConstructionArtifact, PrimitiveConstructionArtifactError,
};
use crate::construction::execution::{
    PreparedPrimitiveConstructionExecution, PrimitiveConstructionExecutionError,
};
use crate::construction::phase_report::PrimitiveConstructionPhaseChainReport;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use crate::construction::PrimitiveConstructionIntent;
use topology::facade::{build_topology_construction_fact_report, TopologyConstructionFactReport};
use worth_geom::facade::{
    PrimitiveRealizationReport, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
use worth_spatial::facade::{
    build_primitive_construction_birth_mapping_report,
    certify_primitive_construction_birth_completeness,
    impossible_primitive_construction_birth_attachment, SpatialConstructionBirthCompletenessReport,
    SpatialConstructionBirthError, SpatialConstructionBirthMappingReport,
    SpatialConstructionBirthRejectionRow,
};

use super::digest::digest_owned_parts;
use super::lower_scaffold_to_topology;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionResultEvidence {
    phase_chain_report: PrimitiveConstructionPhaseChainReport,
    birth_completeness_report: SpatialConstructionBirthCompletenessReport,
    birth_mapping_report: SpatialConstructionBirthMappingReport,
    topology_fact_report: TopologyConstructionFactReport,
}

impl PrimitiveConstructionResultEvidence {
    fn new(
        phase_chain_report: PrimitiveConstructionPhaseChainReport,
        birth_completeness_report: SpatialConstructionBirthCompletenessReport,
        birth_mapping_report: SpatialConstructionBirthMappingReport,
        topology_fact_report: TopologyConstructionFactReport,
    ) -> Self {
        Self {
            phase_chain_report,
            birth_completeness_report,
            birth_mapping_report,
            topology_fact_report,
        }
    }

    pub fn phase_chain_report(&self) -> &PrimitiveConstructionPhaseChainReport {
        &self.phase_chain_report
    }

    pub fn birth_completeness_report(&self) -> &SpatialConstructionBirthCompletenessReport {
        &self.birth_completeness_report
    }

    pub fn birth_mapping_report(&self) -> &SpatialConstructionBirthMappingReport {
        &self.birth_mapping_report
    }

    pub fn topology_fact_report(&self) -> &TopologyConstructionFactReport {
        &self.topology_fact_report
    }
}

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
            evidence.phase_chain_report().report_digest().to_string(),
            evidence
                .birth_completeness_report()
                .completeness_digest()
                .to_string(),
            evidence.birth_mapping_report().report_digest().to_string(),
            evidence.topology_fact_report().report_digest().to_string(),
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

    pub fn canonical_artifact(&self) -> &CanonicalPrimitiveConstructionArtifact {
        &self.canonical_artifact
    }

    pub fn evidence(&self) -> &PrimitiveConstructionResultEvidence {
        &self.evidence
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionResultError {
    Phase(PrimitiveConstructionPhaseError),
    Execution(PrimitiveConstructionExecutionError),
    BirthCompleteness(SpatialConstructionBirthError),
    ImpossibleBirthAttachment(SpatialConstructionBirthRejectionRow),
    Artifact(PrimitiveConstructionArtifactError),
}

impl std::fmt::Display for PrimitiveConstructionResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Phase(error) => write!(f, "{error}"),
            Self::Execution(error) => write!(f, "{error}"),
            Self::BirthCompleteness(error) => write!(f, "{error}"),
            Self::ImpossibleBirthAttachment(row) => write!(f, "{}", row.reason()),
            Self::Artifact(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionResultError {}

pub fn prepare_primitive_construction_result<I: Into<PrimitiveConstructionIntent>>(
    intent: I,
) -> Result<PreparedPrimitiveConstructionResult, PrimitiveConstructionResultError> {
    let intent = intent.into();
    let request = intent.request().clone();
    let request_for_chain = request.clone();
    let admitted = intent
        .admit()
        .map_err(PrimitiveConstructionResultError::Phase)?;
    let scaffold = admitted
        .build_scaffold()
        .map_err(PrimitiveConstructionResultError::Phase)?;
    let birth_input = scaffold.birth_input();
    let (birth_plan, lowering_plan) =
        lower_scaffold_to_topology(&scaffold).map_err(PrimitiveConstructionResultError::Phase)?;
    if let Some(row) = impossible_primitive_construction_birth_attachment(&birth_input, &birth_plan)
    {
        return Err(PrimitiveConstructionResultError::ImpossibleBirthAttachment(
            row,
        ));
    }
    let birth_completeness_report =
        certify_primitive_construction_birth_completeness(&birth_input, &birth_plan)
            .map_err(PrimitiveConstructionResultError::BirthCompleteness)?;
    let birth_mapping_report =
        build_primitive_construction_birth_mapping_report(&birth_completeness_report);
    let execution = PreparedPrimitiveConstructionExecution::from_phase_chain(
        &request_for_chain,
        &admitted,
        &scaffold,
        &birth_plan,
        &lowering_plan,
    )
    .map_err(PrimitiveConstructionResultError::Execution)?;
    let certification = execution.plan_topology_certification();
    let topology_fact_report =
        build_topology_construction_fact_report(&lowering_plan, &certification);
    let phase_chain_report = PrimitiveConstructionPhaseChainReport::from_phase_chain(
        &request_for_chain,
        &admitted,
        &scaffold,
        &birth_plan,
        &lowering_plan,
        &execution,
        &certification,
    );
    let canonical_artifact = build_canonical_primitive_construction_artifact_with_completeness(
        &request_for_chain,
        &admitted,
        &scaffold,
        &birth_plan,
        &birth_completeness_report,
        &topology_fact_report,
        &lowering_plan,
        &execution,
        &certification,
    )
    .map_err(PrimitiveConstructionResultError::Artifact)?;
    let evidence = PrimitiveConstructionResultEvidence::new(
        phase_chain_report,
        birth_completeness_report,
        birth_mapping_report,
        topology_fact_report,
    );
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
    use worth_spatial::facade::SpatialConstructionBirthMappingKind;

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
                .evidence()
                .birth_mapping_report()
                .row_for(SpatialConstructionBirthMappingKind::Face)
                .expect("face row")
                .mapped_count(),
            8
        );
        assert_eq!(
            result.canonical_artifact().birth_completeness_digest(),
            result
                .evidence()
                .birth_completeness_report()
                .completeness_digest()
        );
        assert_eq!(
            result.canonical_artifact().topology_fact_digest(),
            result.evidence().topology_fact_report().report_digest()
        );
        assert_eq!(
            result.canonical_artifact().family(),
            PrimitiveConstructionFamily::RegularPrism
        );
        assert!(!result.result_digest().is_empty());
    }
}
