use crate::construction::admitted_scaffold::PreparedPrimitiveConstructionAdmittedResultInput;
use crate::construction::request::PrimitiveConstructionFamily;
use topology::facade::{
    TopologyConstructionQueryMutationSurface, TopologyPrimitiveConstructionQueryAdmittedHandoff,
};
use worth_spatial::facade::{
    SpatialConstructionBirthCompletenessReport, SpatialConstructionBirthMappingReport,
};

use super::digest::digest_owned_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionResultAssemblyReport {
    family: PrimitiveConstructionFamily,
    topology_query_admitted_handoff_digest: String,
    mutation_surface: TopologyConstructionQueryMutationSurface,
    report_digest: String,
}

impl PrimitiveConstructionResultAssemblyReport {
    pub(crate) fn from_admitted_result_input(
        result_input: &PreparedPrimitiveConstructionAdmittedResultInput,
    ) -> Self {
        let topology_query_handoff = result_input
            .topology_query_admitted_handoff()
            .topology_query_handoff();
        let topology_query_envelope = topology_query_handoff.topology_query_envelope();
        let parts = [
            result_input.family().as_str().to_string(),
            result_input.scaffold_digest().to_string(),
            topology_query_handoff.source_birth_digest().to_string(),
            result_input.admitted_handoff_digest().to_string(),
            topology_query_envelope
                .mutation_surface()
                .as_str()
                .to_string(),
        ];
        Self {
            family: result_input.family(),
            topology_query_admitted_handoff_digest: result_input
                .admitted_handoff_digest()
                .to_string(),
            mutation_surface: topology_query_envelope.mutation_surface(),
            report_digest: digest_owned_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.mutation_surface
    }

    pub fn topology_query_admitted_handoff_digest(&self) -> &str {
        &self.topology_query_admitted_handoff_digest
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionResultEvidence {
    result_assembly_report: PrimitiveConstructionResultAssemblyReport,
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    birth_completeness_report: SpatialConstructionBirthCompletenessReport,
    birth_mapping_report: SpatialConstructionBirthMappingReport,
}

impl PrimitiveConstructionResultEvidence {
    pub(crate) fn from_admitted_result_input(
        result_input: &PreparedPrimitiveConstructionAdmittedResultInput,
    ) -> Self {
        let topology_query_admitted_handoff =
            result_input.topology_query_admitted_handoff().clone();
        Self {
            result_assembly_report:
                PrimitiveConstructionResultAssemblyReport::from_admitted_result_input(result_input),
            topology_query_admitted_handoff,
            birth_completeness_report: result_input.birth_completeness_report().clone(),
            birth_mapping_report: result_input.birth_mapping_report().clone(),
        }
    }

    pub fn result_assembly_report(&self) -> &PrimitiveConstructionResultAssemblyReport {
        &self.result_assembly_report
    }

    pub fn topology_query_admitted_handoff(
        &self,
    ) -> &TopologyPrimitiveConstructionQueryAdmittedHandoff {
        &self.topology_query_admitted_handoff
    }

    pub fn birth_completeness_report(
        &self,
    ) -> &worth_spatial::facade::SpatialConstructionBirthCompletenessReport {
        &self.birth_completeness_report
    }

    pub fn birth_mapping_report(
        &self,
    ) -> &worth_spatial::facade::SpatialConstructionBirthMappingReport {
        &self.birth_mapping_report
    }

    pub fn topology_query_handoff(
        &self,
    ) -> &topology::facade::TopologyPrimitiveConstructionQueryHandoff {
        self.topology_query_admitted_handoff
            .topology_query_handoff()
    }
}
