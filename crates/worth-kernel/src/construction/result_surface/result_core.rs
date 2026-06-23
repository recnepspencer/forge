use super::super::artifact::CanonicalPrimitiveConstructionArtifact;
use super::super::digest::digest_owned_parts;
use super::super::request::PrimitiveConstructionFamily;
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface, TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryHandoff,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PrimitiveConstructionResultCore {
    canonical_artifact: CanonicalPrimitiveConstructionArtifact,
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    birth_consequence_digest: String,
    birth_mapping_digest: String,
    result_digest: String,
}

impl PrimitiveConstructionResultCore {
    pub(super) fn new(
        canonical_artifact: CanonicalPrimitiveConstructionArtifact,
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        graph_authority_digest: String,
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
            graph_authority_digest,
        ]);
        Self {
            canonical_artifact,
            topology_query_admitted_handoff,
            birth_consequence_digest,
            birth_mapping_digest,
            result_digest,
        }
    }

    pub(super) fn family(&self) -> PrimitiveConstructionFamily {
        self.canonical_artifact.family()
    }

    pub(super) fn topology_birth_class(&self) -> &str {
        self.canonical_artifact.topology_birth_class()
    }

    pub(super) fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        self.canonical_artifact.conditioning_witness()
    }

    pub(super) fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.canonical_artifact.realization_strategy()
    }

    pub(super) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        self.canonical_artifact.attempted_realization_strategies()
    }

    pub(super) fn stability_class(&self) -> PrimitiveStabilityClass {
        self.canonical_artifact.stability_class()
    }

    pub(super) fn realization_digest(&self) -> &str {
        self.canonical_artifact.realization_digest()
    }

    pub(super) fn realization_geometry_digest(&self) -> &str {
        self.canonical_artifact.realization_geometry_digest()
    }

    pub(super) fn artifact_digest(&self) -> &str {
        self.canonical_artifact.artifact_digest()
    }

    pub(super) fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.canonical_artifact.feature_conditioning_class()
    }

    pub(super) fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.canonical_artifact.support_normal_class()
    }

    pub(super) fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.canonical_artifact.normalization_disposition()
    }

    pub(super) fn birth_truth_digest(&self) -> &str {
        self.canonical_artifact.birth_truth_digest()
    }

    pub(super) fn birth_completeness_digest(&self) -> &str {
        self.canonical_artifact.birth_completeness_digest()
    }

    pub(super) fn topology_fact_digest(&self) -> &str {
        self.canonical_artifact.topology_fact_digest()
    }

    pub(super) fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.canonical_artifact.mutation_surface()
    }

    pub(super) fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.canonical_artifact.read_surface()
    }

    pub(super) fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.canonical_artifact.fact_provenance()
    }

    pub(super) fn projection_receipt_digest(&self) -> &str {
        self.canonical_artifact.projection_receipt_digest()
    }

    pub(super) fn topology_query_handoff(&self) -> &TopologyPrimitiveConstructionQueryHandoff {
        self.topology_query_admitted_handoff
            .topology_query_handoff()
    }

    pub(super) fn result_digest(&self) -> &str {
        &self.result_digest
    }
}
