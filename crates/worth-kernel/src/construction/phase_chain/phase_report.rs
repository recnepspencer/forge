use crate::construction::admission::AdmittedPrimitiveConstructionIntent;
use crate::construction::execution::PreparedPrimitiveConstructionExecution;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionRequest};
use crate::construction::scaffold::PrimitiveConstructionScaffold;
use topology::facade::{
    TopologyConstructionCertificationPlan, TopologyConstructionLoweringPlan,
    TopologyConstructionMutationSurface,
};
use worth_spatial::facade::SpatialConstructionBirthPlan;

use super::digest::digest_owned_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionPhaseChainReport {
    family: PrimitiveConstructionFamily,
    request_digest: String,
    intent_digest: String,
    scaffold_digest: String,
    birth_digest: String,
    lowering_digest: String,
    execution_digest: String,
    certification_digest: String,
    mutation_surface: TopologyConstructionMutationSurface,
    report_digest: String,
}

impl PrimitiveConstructionPhaseChainReport {
    pub fn from_phase_chain(
        request: &PrimitiveConstructionRequest,
        intent: &AdmittedPrimitiveConstructionIntent,
        scaffold: &PrimitiveConstructionScaffold,
        birth_plan: &SpatialConstructionBirthPlan,
        lowering_plan: &TopologyConstructionLoweringPlan,
        execution: &PreparedPrimitiveConstructionExecution,
        certification: &TopologyConstructionCertificationPlan,
    ) -> Self {
        let parts = [
            request.request_digest().to_string(),
            intent.intent_digest().to_string(),
            scaffold.scaffold_digest().to_string(),
            birth_plan.birth_digest().to_string(),
            lowering_plan.lowering_digest().to_string(),
            execution.execution_digest().to_string(),
            certification.certification_digest().to_string(),
            lowering_plan.mutation_surface().as_str().to_string(),
        ];
        Self {
            family: request.family(),
            request_digest: request.request_digest().to_string(),
            intent_digest: intent.intent_digest().to_string(),
            scaffold_digest: scaffold.scaffold_digest().to_string(),
            birth_digest: birth_plan.birth_digest().to_string(),
            lowering_digest: lowering_plan.lowering_digest().to_string(),
            execution_digest: execution.execution_digest().to_string(),
            certification_digest: certification.certification_digest().to_string(),
            mutation_surface: lowering_plan.mutation_surface(),
            report_digest: digest_owned_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn mutation_surface(&self) -> TopologyConstructionMutationSurface {
        self.mutation_surface
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
