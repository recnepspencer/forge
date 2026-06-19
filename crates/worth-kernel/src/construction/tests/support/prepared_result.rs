use super::super::super::admitted_scaffold::{
    prepare_primitive_construction_admitted_artifact,
    prepare_primitive_construction_executed_admitted_artifact,
};
use super::super::super::artifact::CanonicalPrimitiveConstructionArtifact;
use super::super::super::digest::digest_owned_parts;
use super::super::super::intent::PrimitiveConstructionIntent;
use super::super::super::request::PrimitiveConstructionFamily;
use super::super::super::result::PrimitiveConstructionResultError;
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedPrimitiveConstructionResult {
    canonical_artifact: CanonicalPrimitiveConstructionArtifact,
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    topology_compose_evidence: Option<TopologyPrimitiveConstructionBirthComposeEvidence>,
    birth_consequence_digest: String,
    birth_mapping_digest: String,
    result_digest: String,
}

impl PreparedPrimitiveConstructionResult {
    fn new(
        canonical_artifact: CanonicalPrimitiveConstructionArtifact,
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        topology_compose_evidence: Option<TopologyPrimitiveConstructionBirthComposeEvidence>,
        birth_consequence_digest: String,
        birth_mapping_digest: String,
    ) -> Self {
        let result_digest = digest_owned_parts(&[
            canonical_artifact.artifact_digest().to_string(),
            birth_consequence_digest.clone(),
            birth_mapping_digest.clone(),
            topology_query_admitted_handoff
                .admitted_handoff_digest()
                .to_string(),
            topology_compose_evidence
                .as_ref()
                .map(|evidence| evidence.evidence_digest().to_string())
                .unwrap_or_else(|| "handoff-only-no-compose-evidence".to_string()),
        ]);
        Self {
            canonical_artifact,
            topology_query_admitted_handoff,
            topology_compose_evidence,
            birth_consequence_digest,
            birth_mapping_digest,
            result_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.canonical_artifact.family()
    }

    pub fn topology_birth_class(&self) -> &str {
        self.canonical_artifact.topology_birth_class()
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        self.canonical_artifact.conditioning_witness()
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.canonical_artifact.realization_strategy()
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.canonical_artifact.attempted_realization_strategies()
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.canonical_artifact.stability_class()
    }

    pub fn realization_digest(&self) -> &str {
        self.canonical_artifact.realization_digest()
    }

    pub fn realization_geometry_digest(&self) -> &str {
        self.canonical_artifact.realization_geometry_digest()
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

    pub fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.canonical_artifact.read_surface()
    }

    pub fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.canonical_artifact.fact_provenance()
    }

    pub fn projection_receipt_digest(&self) -> &str {
        self.canonical_artifact.projection_receipt_digest()
    }

    pub(crate) fn topology_query_handoff(&self) -> &TopologyPrimitiveConstructionQueryHandoff {
        self.topology_query_admitted_handoff
            .topology_query_handoff()
    }

    pub fn topology_compose_evidence(
        &self,
    ) -> Option<&TopologyPrimitiveConstructionBirthComposeEvidence> {
        self.topology_compose_evidence.as_ref()
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }
}

pub fn prepare_primitive_construction_result<I: Into<PrimitiveConstructionIntent>>(
    intent: I,
) -> Result<PreparedPrimitiveConstructionResult, PrimitiveConstructionResultError> {
    let intent = intent.into();
    let request = intent.request().clone();
    let admitted_artifact = prepare_primitive_construction_admitted_artifact(&request)
        .map_err(PrimitiveConstructionResultError::Phase)?;
    let canonical_artifact =
        CanonicalPrimitiveConstructionArtifact::from_admitted_artifact(&admitted_artifact);
    let topology_query_admitted_handoff =
        admitted_artifact.topology_query_admitted_handoff().clone();
    let topology_compose_evidence = admitted_artifact.topology_compose_evidence().cloned();
    let birth_consequence_digest = admitted_artifact.birth_consequence_digest().to_string();
    let birth_mapping_digest = admitted_artifact.birth_mapping_digest().to_string();
    Ok(PreparedPrimitiveConstructionResult::new(
        canonical_artifact,
        topology_query_admitted_handoff,
        topology_compose_evidence,
        birth_consequence_digest,
        birth_mapping_digest,
    ))
}

pub fn prepare_primitive_construction_executed_result<I: Into<PrimitiveConstructionIntent>>(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    intent: I,
) -> Result<PreparedPrimitiveConstructionResult, PrimitiveConstructionResultError> {
    let intent = intent.into();
    let request = intent.request().clone();
    let admitted_artifact =
        prepare_primitive_construction_executed_admitted_artifact(workspace, &request)
            .map_err(PrimitiveConstructionResultError::Phase)?;
    let canonical_artifact =
        CanonicalPrimitiveConstructionArtifact::from_admitted_artifact(&admitted_artifact);
    let topology_query_admitted_handoff =
        admitted_artifact.topology_query_admitted_handoff().clone();
    let topology_compose_evidence = admitted_artifact.topology_compose_evidence().cloned();
    let birth_consequence_digest = admitted_artifact.birth_consequence_digest().to_string();
    let birth_mapping_digest = admitted_artifact.birth_mapping_digest().to_string();
    Ok(PreparedPrimitiveConstructionResult::new(
        canonical_artifact,
        topology_query_admitted_handoff,
        topology_compose_evidence,
        birth_consequence_digest,
        birth_mapping_digest,
    ))
}
