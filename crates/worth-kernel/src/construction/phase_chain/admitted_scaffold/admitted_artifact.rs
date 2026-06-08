use super::topology_ready_birth::PreparedPrimitiveConstructionTopologyReadyBirth;
use topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use worth_geom::facade::PrimitiveRealizationReport;
use worth_spatial::facade::birth::AdmittedPrimitiveConstructionBirthConsequence;

use crate::construction::request::PrimitiveConstructionFamily;

#[derive(Clone, Debug)]
pub(crate) struct PreparedPrimitiveConstructionAdmittedArtifact {
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    birth_consequence: AdmittedPrimitiveConstructionBirthConsequence,
    realization_report: PrimitiveRealizationReport,
}

impl PreparedPrimitiveConstructionAdmittedArtifact {
    pub(super) fn from_topology_ready_birth(
        topology_ready_birth: PreparedPrimitiveConstructionTopologyReadyBirth,
        realization_report: PrimitiveRealizationReport,
    ) -> Self {
        let (topology_query_admitted_handoff, birth_consequence) =
            topology_ready_birth.into_parts();
        Self::new(
            topology_query_admitted_handoff,
            birth_consequence,
            realization_report,
        )
    }

    pub(crate) fn new(
        topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
        birth_consequence: AdmittedPrimitiveConstructionBirthConsequence,
        realization_report: PrimitiveRealizationReport,
    ) -> Self {
        Self {
            topology_query_admitted_handoff,
            birth_consequence,
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

    pub(crate) fn birth_consequence(&self) -> &AdmittedPrimitiveConstructionBirthConsequence {
        &self.birth_consequence
    }

    pub(crate) fn birth_mapping_digest(&self) -> String {
        self.birth_consequence
            .rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>()
            .join("|")
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
