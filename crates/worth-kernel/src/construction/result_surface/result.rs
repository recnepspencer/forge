use super::admitted_scaffold::{
    prepare_primitive_construction_admitted_artifact,
    prepare_primitive_construction_executed_admitted_artifact,
};
use super::artifact::CanonicalPrimitiveConstructionArtifact;
use super::intent::PrimitiveConstructionIntent;
use super::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
#[path = "result_core.rs"]
mod result_core;
use result_core::PrimitiveConstructionResultCore;
use topology::facade::TopologyPrimitiveConstructionBirthComposeEvidence;
use topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use topology::facade::TopologyPrimitiveConstructionQueryHandoff;
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[derive(Debug)]
pub enum PrimitiveConstructionResultError {
    Phase(PrimitiveConstructionPhaseError),
    MissingExecutedGraphAuthorityEvidence,
}

impl std::fmt::Display for PrimitiveConstructionResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Phase(error) => write!(f, "{error}"),
            Self::MissingExecutedGraphAuthorityEvidence => {
                write!(
                    f,
                    "executed primitive construction result is missing Query graph authority evidence"
                )
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionResultError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedPrimitiveConstructionHandoffResult {
    core: PrimitiveConstructionResultCore,
}

impl PreparedPrimitiveConstructionHandoffResult {
    fn new(
        canonical_artifact: CanonicalPrimitiveConstructionArtifact,
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        birth_consequence_digest: String,
        birth_mapping_digest: String,
    ) -> Self {
        Self {
            core: PrimitiveConstructionResultCore::new(
                canonical_artifact,
                topology_query_admitted_handoff,
                "handoff-only-no-compose-evidence".to_string(),
                birth_consequence_digest,
                birth_mapping_digest,
            ),
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.core.family()
    }

    pub(crate) fn topology_birth_class(&self) -> &str {
        self.core.topology_birth_class()
    }

    pub(crate) fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        self.core.conditioning_witness()
    }

    pub(crate) fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.core.realization_strategy()
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.core.attempted_realization_strategies()
    }

    pub(crate) fn stability_class(&self) -> PrimitiveStabilityClass {
        self.core.stability_class()
    }

    pub(crate) fn realization_digest(&self) -> &str {
        self.core.realization_digest()
    }

    pub(crate) fn realization_geometry_digest(&self) -> &str {
        self.core.realization_geometry_digest()
    }

    pub(crate) fn artifact_digest(&self) -> &str {
        self.core.artifact_digest()
    }

    pub(crate) fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.core.feature_conditioning_class()
    }

    pub(crate) fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.core.support_normal_class()
    }

    pub(crate) fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.core.normalization_disposition()
    }

    pub(crate) fn birth_truth_digest(&self) -> &str {
        self.core.birth_truth_digest()
    }

    pub(crate) fn birth_completeness_digest(&self) -> &str {
        self.core.birth_completeness_digest()
    }

    pub(crate) fn topology_fact_digest(&self) -> &str {
        self.core.topology_fact_digest()
    }

    pub(crate) fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.core.mutation_surface()
    }

    pub(crate) fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.core.read_surface()
    }

    pub(crate) fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.core.fact_provenance()
    }

    pub(crate) fn projection_receipt_digest(&self) -> &str {
        self.core.projection_receipt_digest()
    }

    pub(crate) fn topology_query_handoff(&self) -> &TopologyPrimitiveConstructionQueryHandoff {
        self.core.topology_query_handoff()
    }

    pub(crate) fn result_digest(&self) -> &str {
        self.core.result_digest()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExecutedPrimitiveConstructionGraphAuthorityResult {
    core: PrimitiveConstructionResultCore,
    topology_compose_evidence: TopologyPrimitiveConstructionBirthComposeEvidence,
}

impl ExecutedPrimitiveConstructionGraphAuthorityResult {
    fn new(
        canonical_artifact: CanonicalPrimitiveConstructionArtifact,
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        topology_compose_evidence: TopologyPrimitiveConstructionBirthComposeEvidence,
        birth_consequence_digest: String,
        birth_mapping_digest: String,
    ) -> Self {
        let graph_authority_digest = topology_compose_evidence.evidence_digest().to_string();
        Self {
            core: PrimitiveConstructionResultCore::new(
                canonical_artifact,
                topology_query_admitted_handoff,
                graph_authority_digest,
                birth_consequence_digest,
                birth_mapping_digest,
            ),
            topology_compose_evidence,
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.core.family()
    }

    pub(crate) fn topology_birth_class(&self) -> &str {
        self.core.topology_birth_class()
    }

    pub(crate) fn artifact_digest(&self) -> &str {
        self.core.artifact_digest()
    }

    pub(crate) fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.core.realization_strategy()
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.core.attempted_realization_strategies()
    }

    pub(crate) fn stability_class(&self) -> PrimitiveStabilityClass {
        self.core.stability_class()
    }

    pub(crate) fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.core.feature_conditioning_class()
    }

    pub(crate) fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.core.support_normal_class()
    }

    pub(crate) fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.core.normalization_disposition()
    }

    pub(crate) fn topology_compose_evidence(
        &self,
    ) -> &TopologyPrimitiveConstructionBirthComposeEvidence {
        &self.topology_compose_evidence
    }

    pub(crate) fn result_digest(&self) -> &str {
        self.core.result_digest()
    }
}

pub(crate) fn prepare_primitive_construction_result<I: Into<PrimitiveConstructionIntent>>(
    intent: I,
) -> Result<PreparedPrimitiveConstructionHandoffResult, PrimitiveConstructionResultError> {
    let intent = intent.into();
    let request = intent.request().clone();
    let admitted_artifact = prepare_primitive_construction_admitted_artifact(&request)
        .map_err(PrimitiveConstructionResultError::Phase)?;
    let canonical_artifact =
        CanonicalPrimitiveConstructionArtifact::from_admitted_artifact(&admitted_artifact);
    let topology_query_admitted_handoff =
        admitted_artifact.topology_query_admitted_handoff().clone();
    let birth_consequence_digest = admitted_artifact.birth_consequence_digest().to_string();
    let birth_mapping_digest = admitted_artifact.birth_mapping_digest().to_string();
    Ok(PreparedPrimitiveConstructionHandoffResult::new(
        canonical_artifact,
        topology_query_admitted_handoff,
        birth_consequence_digest,
        birth_mapping_digest,
    ))
}

pub(crate) fn prepare_primitive_construction_executed_result<
    I: Into<PrimitiveConstructionIntent>,
>(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    intent: I,
) -> Result<ExecutedPrimitiveConstructionGraphAuthorityResult, PrimitiveConstructionResultError> {
    let intent = intent.into();
    let request = intent.request().clone();
    let admitted_artifact =
        prepare_primitive_construction_executed_admitted_artifact(workspace, &request)
            .map_err(PrimitiveConstructionResultError::Phase)?;
    let canonical_artifact =
        CanonicalPrimitiveConstructionArtifact::from_admitted_artifact(&admitted_artifact);
    let topology_query_admitted_handoff =
        admitted_artifact.topology_query_admitted_handoff().clone();
    let topology_compose_evidence = admitted_artifact
        .topology_compose_evidence()
        .cloned()
        .ok_or(PrimitiveConstructionResultError::MissingExecutedGraphAuthorityEvidence)?;
    let birth_consequence_digest = admitted_artifact.birth_consequence_digest().to_string();
    let birth_mapping_digest = admitted_artifact.birth_mapping_digest().to_string();
    Ok(ExecutedPrimitiveConstructionGraphAuthorityResult::new(
        canonical_artifact,
        topology_query_admitted_handoff,
        topology_compose_evidence,
        birth_consequence_digest,
        birth_mapping_digest,
    ))
}
