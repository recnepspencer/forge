use super::super::request::PrimitiveConstructionFamily;
use topology::facade::{
    TopologyPrimitiveConstructionBirthComposeEvidence,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
};
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[derive(Clone, Debug)]
pub(crate) struct PreparedPrimitiveConstructionAdmittedArtifact {
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    topology_compose_evidence: Option<TopologyPrimitiveConstructionBirthComposeEvidence>,
    birth_consequence_digest: String,
    birth_mapping_digest: String,
    conditioning_witness: PrimitiveConditioningWitness,
    realization_strategy: PrimitiveRealizationStrategy,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    realization_digest: String,
    realization_geometry_digest: String,
}

impl PreparedPrimitiveConstructionAdmittedArtifact {
    pub(super) fn from_topology_query_admitted_handoff(
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        birth_consequence_digest: String,
        birth_mapping_digest: String,
        conditioning_witness: PrimitiveConditioningWitness,
        realization_strategy: PrimitiveRealizationStrategy,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: PrimitiveStabilityClass,
        realization_digest: String,
        realization_geometry_digest: String,
    ) -> Self {
        Self::new(
            topology_query_admitted_handoff,
            None,
            birth_consequence_digest,
            birth_mapping_digest,
            conditioning_witness,
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            realization_digest,
            realization_geometry_digest,
        )
    }

    pub(crate) fn new(
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        topology_compose_evidence: Option<TopologyPrimitiveConstructionBirthComposeEvidence>,
        birth_consequence_digest: String,
        birth_mapping_digest: String,
        conditioning_witness: PrimitiveConditioningWitness,
        realization_strategy: PrimitiveRealizationStrategy,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: PrimitiveStabilityClass,
        realization_digest: String,
        realization_geometry_digest: String,
    ) -> Self {
        Self {
            topology_query_admitted_handoff,
            topology_compose_evidence,
            birth_consequence_digest,
            birth_mapping_digest,
            conditioning_witness,
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            realization_digest,
            realization_geometry_digest,
        }
    }

    pub(super) fn with_topology_compose_evidence(
        mut self,
        topology_compose_evidence: TopologyPrimitiveConstructionBirthComposeEvidence,
    ) -> Self {
        self.topology_compose_evidence = Some(topology_compose_evidence);
        self
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        kernel_family_from_topology_family(
            self.topology_query_admitted_handoff
                .topology_query_handoff()
                .family(),
        )
    }

    pub(crate) fn scaffold_digest(&self) -> &str {
        self.topology_query_admitted_handoff
            .topology_query_handoff()
            .scaffold_digest()
    }

    pub(crate) fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub(crate) fn realization_strategy(&self) -> PrimitiveRealizationStrategy {
        self.realization_strategy
    }

    pub(crate) fn attempted_realization_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_realization_strategies
    }

    pub(crate) fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub(crate) fn realization_digest(&self) -> &str {
        &self.realization_digest
    }

    pub(crate) fn realization_geometry_digest(&self) -> &str {
        &self.realization_geometry_digest
    }

    pub(crate) fn birth_consequence_digest(&self) -> &str {
        &self.birth_consequence_digest
    }

    pub(crate) fn birth_mapping_digest(&self) -> &str {
        &self.birth_mapping_digest
    }

    pub(crate) fn topology_query_admitted_handoff(
        &self,
    ) -> &TopologyPrimitiveConstructionQueryAdmittedHandoff {
        &self.topology_query_admitted_handoff
    }

    pub(crate) fn topology_compose_evidence(
        &self,
    ) -> Option<&TopologyPrimitiveConstructionBirthComposeEvidence> {
        self.topology_compose_evidence.as_ref()
    }

    pub(crate) fn admitted_handoff_digest(&self) -> &str {
        self.topology_query_admitted_handoff
            .admitted_handoff_digest()
    }
}

fn kernel_family_from_topology_family(
    family: topology::facade::TopologyPrimitiveConstructionBirthFamily,
) -> PrimitiveConstructionFamily {
    match family {
        topology::facade::TopologyPrimitiveConstructionBirthFamily::SimplexSolid => {
            PrimitiveConstructionFamily::SimplexSolid
        }
        topology::facade::TopologyPrimitiveConstructionBirthFamily::Orthotope => {
            PrimitiveConstructionFamily::Orthotope
        }
        topology::facade::TopologyPrimitiveConstructionBirthFamily::RegularPrism => {
            PrimitiveConstructionFamily::RegularPrism
        }
        topology::facade::TopologyPrimitiveConstructionBirthFamily::RegularPyramid => {
            PrimitiveConstructionFamily::RegularPyramid
        }
        topology::facade::TopologyPrimitiveConstructionBirthFamily::WireBody => {
            PrimitiveConstructionFamily::WireBody
        }
        topology::facade::TopologyPrimitiveConstructionBirthFamily::ShellWithHole => {
            PrimitiveConstructionFamily::ShellWithHole
        }
    }
}
