use super::topology_ready_birth::PreparedPrimitiveConstructionTopologyReadyBirth;
use topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use worth_geom::facade::PrimitiveRealizationReport;
use worth_spatial::facade::{
    SpatialConstructionBirthCompletenessReport, SpatialConstructionBirthMappingReport,
};

use crate::construction::request::PrimitiveConstructionFamily;

#[derive(Clone, Debug)]
pub(crate) struct PreparedPrimitiveConstructionAdmittedResultInput {
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    birth_completeness_report: SpatialConstructionBirthCompletenessReport,
    birth_mapping_report: SpatialConstructionBirthMappingReport,
    realization_report: PrimitiveRealizationReport,
}

impl PreparedPrimitiveConstructionAdmittedResultInput {
    pub(super) fn from_topology_ready_birth(
        topology_ready_birth: PreparedPrimitiveConstructionTopologyReadyBirth,
        realization_report: PrimitiveRealizationReport,
    ) -> Self {
        let (topology_query_admitted_handoff, birth_completeness_report, birth_mapping_report) =
            topology_ready_birth.into_parts();
        Self::new(
            topology_query_admitted_handoff,
            birth_completeness_report,
            birth_mapping_report,
            realization_report,
        )
    }

    pub(crate) fn new(
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        birth_completeness_report: SpatialConstructionBirthCompletenessReport,
        birth_mapping_report: SpatialConstructionBirthMappingReport,
        realization_report: PrimitiveRealizationReport,
    ) -> Self {
        Self {
            topology_query_admitted_handoff,
            birth_completeness_report,
            birth_mapping_report,
            realization_report,
        }
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

    pub(crate) fn realization_report(&self) -> &PrimitiveRealizationReport {
        &self.realization_report
    }

    pub(crate) fn birth_completeness_report(&self) -> &SpatialConstructionBirthCompletenessReport {
        &self.birth_completeness_report
    }

    pub(crate) fn birth_mapping_report(&self) -> &SpatialConstructionBirthMappingReport {
        &self.birth_mapping_report
    }

    pub(crate) fn topology_query_admitted_handoff(
        &self,
    ) -> &TopologyPrimitiveConstructionQueryAdmittedHandoff {
        &self.topology_query_admitted_handoff
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
