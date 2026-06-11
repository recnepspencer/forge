use super::super::request::PrimitiveConstructionFamily;
use topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[derive(Clone, Debug)]
pub(crate) struct PreparedPrimitiveConstructionAdmittedArtifact {
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    #[cfg(test)]
    birth_consequence_digest: String,
    #[cfg(test)]
    birth_mapping_digest: String,
    conditioning_witness: PrimitiveConditioningWitness,
    realization_strategy: PrimitiveRealizationStrategy,
    attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    #[cfg(test)]
    realization_digest: String,
    #[cfg(test)]
    realization_geometry_digest: String,
}

impl PreparedPrimitiveConstructionAdmittedArtifact {
    pub(super) fn from_topology_query_admitted_handoff(
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        #[cfg(test)] birth_consequence_digest: String,
        #[cfg(test)] birth_mapping_digest: String,
        conditioning_witness: PrimitiveConditioningWitness,
        realization_strategy: PrimitiveRealizationStrategy,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: PrimitiveStabilityClass,
        #[cfg(test)] realization_digest: String,
        #[cfg(test)] realization_geometry_digest: String,
    ) -> Self {
        Self::new(
            topology_query_admitted_handoff,
            #[cfg(test)]
            birth_consequence_digest,
            #[cfg(test)]
            birth_mapping_digest,
            conditioning_witness,
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            #[cfg(test)]
            realization_digest,
            #[cfg(test)]
            realization_geometry_digest,
        )
    }

    pub(crate) fn new(
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        #[cfg(test)] birth_consequence_digest: String,
        #[cfg(test)] birth_mapping_digest: String,
        conditioning_witness: PrimitiveConditioningWitness,
        realization_strategy: PrimitiveRealizationStrategy,
        attempted_realization_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: PrimitiveStabilityClass,
        #[cfg(test)] realization_digest: String,
        #[cfg(test)] realization_geometry_digest: String,
    ) -> Self {
        Self {
            topology_query_admitted_handoff,
            #[cfg(test)]
            birth_consequence_digest,
            #[cfg(test)]
            birth_mapping_digest,
            conditioning_witness,
            realization_strategy,
            attempted_realization_strategies,
            stability_class,
            #[cfg(test)]
            realization_digest,
            #[cfg(test)]
            realization_geometry_digest,
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        kernel_family_from_topology_family(
            self.topology_query_admitted_handoff
                .topology_query_handoff()
                .family(),
        )
    }

    #[cfg(test)]
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

    #[cfg(test)]
    pub(crate) fn realization_digest(&self) -> &str {
        &self.realization_digest
    }

    #[cfg(test)]
    pub(crate) fn realization_geometry_digest(&self) -> &str {
        &self.realization_geometry_digest
    }

    #[cfg(test)]
    pub(crate) fn birth_consequence_digest(&self) -> &str {
        &self.birth_consequence_digest
    }

    #[cfg(test)]
    pub(crate) fn birth_mapping_digest(&self) -> &str {
        &self.birth_mapping_digest
    }

    #[cfg(test)]
    pub(crate) fn topology_query_admitted_handoff(
        &self,
    ) -> &TopologyPrimitiveConstructionQueryAdmittedHandoff {
        &self.topology_query_admitted_handoff
    }

    #[cfg(test)]
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
