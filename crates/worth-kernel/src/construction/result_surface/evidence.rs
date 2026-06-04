use crate::construction::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact;
use topology::facade::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use worth_spatial::facade::bindings::AdmittedPrimitiveConstructionBirthConsequence;

use super::digest::digest_owned_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionResultAssemblyReport {
    report_digest: String,
}

impl PrimitiveConstructionResultAssemblyReport {
    pub(crate) fn from_admitted_artifact(
        admitted_artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
    ) -> Self {
        let topology_query_handoff = admitted_artifact
            .topology_query_admitted_handoff()
            .topology_query_handoff();
        let topology_query_envelope = topology_query_handoff.topology_query_envelope();
        let parts = [
            admitted_artifact.family().as_str().to_string(),
            admitted_artifact.scaffold_digest().to_string(),
            topology_query_handoff.source_birth_digest().to_string(),
            admitted_artifact.admitted_handoff_digest().to_string(),
            topology_query_envelope
                .mutation_surface()
                .as_str()
                .to_string(),
        ];
        Self {
            report_digest: digest_owned_parts(&parts),
        }
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionResultEvidence {
    result_assembly_report: PrimitiveConstructionResultAssemblyReport,
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    birth_consequence: AdmittedPrimitiveConstructionBirthConsequence,
    birth_mapping_digest: String,
}

impl PrimitiveConstructionResultEvidence {
    pub(crate) fn from_admitted_artifact(
        admitted_artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
    ) -> Self {
        let topology_query_admitted_handoff =
            admitted_artifact.topology_query_admitted_handoff().clone();
        Self {
            result_assembly_report:
                PrimitiveConstructionResultAssemblyReport::from_admitted_artifact(admitted_artifact),
            topology_query_admitted_handoff,
            birth_consequence: admitted_artifact.birth_consequence().clone(),
            birth_mapping_digest: admitted_artifact.birth_mapping_digest(),
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

    pub fn birth_consequence(&self) -> &AdmittedPrimitiveConstructionBirthConsequence {
        &self.birth_consequence
    }

    pub fn birth_mapping_digest(&self) -> &str {
        &self.birth_mapping_digest
    }

    #[cfg(test)]
    pub fn topology_query_handoff(
        &self,
    ) -> &topology::facade::TopologyPrimitiveConstructionQueryHandoff {
        self.topology_query_admitted_handoff
            .topology_query_handoff()
    }
}
