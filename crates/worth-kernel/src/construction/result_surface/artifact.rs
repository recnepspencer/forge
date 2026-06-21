use super::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact;
use super::digest::digest_owned_parts;
use super::request::PrimitiveConstructionFamily;
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryMutationSurface, TopologyConstructionQueryReadSurface,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};
use worth_geom::facade::{
    PrimitiveFeatureConditioningClass, PrimitiveNormalizationDisposition,
    PrimitiveSupportNormalClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CanonicalPrimitiveConstructionArtifact {
    family: PrimitiveConstructionFamily,
    topology_birth_class: String,
    conditioning_witness: PrimitiveConditioningWitness,
    realization_strategy: PrimitiveRealizationStrategy,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    realization_digest: String,
    realization_geometry_digest: String,
    birth_digest: String,
    birth_completeness_digest: String,
    topology_fact_digest: String,
    mutation_surface: TopologyConstructionQueryMutationSurface,
    read_surface: TopologyConstructionQueryReadSurface,
    inspection_surface: TopologyConstructionQueryInspectionSurface,
    fact_provenance: TopologyConstructionQueryFactProvenance,
    projection_receipt_digest: String,
    supported_loop_count: usize,
    supported_body_count: usize,
    artifact_digest: String,
}

impl CanonicalPrimitiveConstructionArtifact {
    fn new(
        family: PrimitiveConstructionFamily,
        scaffold_digest: &str,
        conditioning_witness: PrimitiveConditioningWitness,
        realization_strategy: PrimitiveRealizationStrategy,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: PrimitiveStabilityClass,
        realization_digest: &str,
        realization_geometry_digest: &str,
        topology_query_admitted_handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff,
        admitted_handoff_digest: &str,
    ) -> Self {
        let topology_query_handoff = topology_query_admitted_handoff.topology_query_handoff();
        let topology_query_envelope = topology_query_handoff.topology_query_envelope();
        let parts = [
            family.as_str().to_string(),
            scaffold_digest.to_string(),
            topology_query_handoff.source_birth_digest().to_string(),
            topology_query_admitted_handoff
                .birth_completeness_digest()
                .to_string(),
            topology_query_envelope.fact_digest().to_string(),
            topology_query_handoff.handoff_digest().to_string(),
            admitted_handoff_digest.to_string(),
            topology_query_envelope.receipt_digest().to_string(),
            realization_digest.to_string(),
            realization_geometry_digest.to_string(),
            topology_query_envelope
                .mutation_surface()
                .as_str()
                .to_string(),
            topology_query_envelope.read_surface().as_str().to_string(),
            topology_query_envelope
                .inspection_surface()
                .as_str()
                .to_string(),
            topology_query_envelope
                .fact_provenance()
                .as_str()
                .to_string(),
        ];
        Self {
            family,
            topology_birth_class: topology_query_envelope.topology_birth_class().to_string(),
            conditioning_witness,
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            realization_digest: realization_digest.to_string(),
            realization_geometry_digest: realization_geometry_digest.to_string(),
            birth_digest: topology_query_handoff.source_birth_digest().to_string(),
            birth_completeness_digest: topology_query_admitted_handoff
                .birth_completeness_digest()
                .to_string(),
            topology_fact_digest: topology_query_envelope.fact_digest().to_string(),
            mutation_surface: topology_query_envelope.mutation_surface(),
            read_surface: topology_query_envelope.read_surface(),
            inspection_surface: topology_query_envelope.inspection_surface(),
            fact_provenance: topology_query_envelope.fact_provenance(),
            projection_receipt_digest: topology_query_envelope.receipt_digest().to_string(),
            supported_loop_count: topology_query_admitted_handoff.supported_loop_count(),
            supported_body_count: topology_query_admitted_handoff.supported_body_count(),
            artifact_digest: digest_owned_parts(&parts),
        }
    }

    pub(crate) fn from_admitted_artifact(
        admitted_artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
    ) -> Self {
        Self::new(
            admitted_artifact.family(),
            admitted_artifact.scaffold_digest(),
            admitted_artifact.conditioning_witness().clone(),
            admitted_artifact.realization_strategy(),
            admitted_artifact
                .attempted_realization_strategies()
                .to_vec(),
            admitted_artifact.stability_class(),
            admitted_artifact.realization_digest(),
            admitted_artifact.realization_geometry_digest(),
            admitted_artifact.topology_query_admitted_handoff(),
            admitted_artifact.admitted_handoff_digest(),
        )
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization_strategy
    }

    pub fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub fn realization_digest(&self) -> &str {
        &self.realization_digest
    }

    pub fn realization_geometry_digest(&self) -> &str {
        &self.realization_geometry_digest
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.conditioning_witness().feature_conditioning_class()
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.conditioning_witness().support_normal_class()
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.conditioning_witness().normalization_disposition()
    }

    pub fn birth_truth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub fn birth_completeness_digest(&self) -> &str {
        &self.birth_completeness_digest
    }

    pub fn topology_fact_digest(&self) -> &str {
        &self.topology_fact_digest
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.mutation_surface
    }

    pub fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.read_surface
    }

    #[cfg(test)]
    pub fn inspection_surface(&self) -> TopologyConstructionQueryInspectionSurface {
        self.inspection_surface
    }

    pub fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.fact_provenance
    }

    pub fn projection_receipt_digest(&self) -> &str {
        &self.projection_receipt_digest
    }

    #[cfg(test)]
    pub fn supported_loop_count(&self) -> usize {
        self.supported_loop_count
    }

    #[cfg(test)]
    pub fn supported_body_count(&self) -> usize {
        self.supported_body_count
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

#[cfg(test)]
pub(crate) fn build_canonical_primitive_construction_artifact(
    admitted_artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
) -> CanonicalPrimitiveConstructionArtifact {
    CanonicalPrimitiveConstructionArtifact::from_admitted_artifact(admitted_artifact)
}

#[cfg(test)]
mod tests {
    use super::super::admitted_scaffold::prepare_primitive_construction_admitted_artifact;
    use super::super::intent::PrimitiveConstructionIntent;
    use super::super::request::PrimitiveConstructionFamily;
    use super::super::specs::ShellWithHoleSpec;
    use super::build_canonical_primitive_construction_artifact;
    use topology::facade::TopologyConstructionQueryInspectionSurface;

    #[test]
    fn canonical_artifact_binds_shell_birth_and_topology_truth() {
        let intent = PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
            outer_loop_edge_count: 6,
            hole_loop_edge_counts: vec![3, 4],
        });
        let request = intent.clone().into_request();
        let admitted_artifact =
            prepare_primitive_construction_admitted_artifact(&request).expect("admitted artifact");
        let artifact = build_canonical_primitive_construction_artifact(&admitted_artifact);

        assert_eq!(
            artifact.family(),
            PrimitiveConstructionFamily::ShellWithHole
        );
        assert_eq!(
            artifact.topology_birth_class(),
            "planar_shell_with_hole_body"
        );
        assert_eq!(
            artifact.birth_truth_digest(),
            admitted_artifact
                .topology_query_admitted_handoff()
                .topology_query_handoff()
                .source_birth_digest()
        );
        assert_eq!(artifact.supported_loop_count(), 3);
        assert_eq!(artifact.supported_body_count(), 1);
        assert!(!artifact.topology_fact_digest().is_empty());
        assert_eq!(
            artifact.inspection_surface(),
            TopologyConstructionQueryInspectionSurface::InspectReceipt
        );
        assert_eq!(
            artifact.read_surface(),
            topology::facade::TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
        );
        assert_eq!(
            artifact.fact_provenance(),
            topology::facade::TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
        );
        assert!(!artifact.birth_completeness_digest().is_empty());
        assert!(!artifact.projection_receipt_digest().is_empty());
        assert!(!artifact.artifact_digest().is_empty());
    }
}
