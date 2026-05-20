use topology::facade::{
    prepare_primitive_construction_certification, prepare_primitive_construction_execution,
    TopologyConstructionCertificationPlan, TopologyConstructionExecutionError,
    TopologyConstructionExecutionPlan, TopologyConstructionLoweringPlan,
};
use worth_spatial::facade::SpatialConstructionBirthPlan;

use crate::construction::admission::AdmittedPrimitiveConstructionIntent;
use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionRequest};
use crate::construction::scaffold::PrimitiveConstructionScaffold;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedPrimitiveConstructionExecution {
    family: PrimitiveConstructionFamily,
    request_digest: String,
    intent_digest: String,
    scaffold_digest: String,
    birth_digest: String,
    lowering_digest: String,
    execution_plan: TopologyConstructionExecutionPlan,
    execution_digest: String,
}

impl PreparedPrimitiveConstructionExecution {
    pub fn from_phase_chain(
        request: &PrimitiveConstructionRequest,
        intent: &AdmittedPrimitiveConstructionIntent,
        scaffold: &PrimitiveConstructionScaffold,
        birth_plan: &SpatialConstructionBirthPlan,
        lowering_plan: &TopologyConstructionLoweringPlan,
    ) -> Result<Self, PrimitiveConstructionExecutionError> {
        let execution_plan = prepare_primitive_construction_execution(lowering_plan)
            .map_err(PrimitiveConstructionExecutionError::TopologyExecution)?;
        Ok(Self {
            family: request.family(),
            request_digest: request.request_digest().to_string(),
            intent_digest: intent.intent_digest().to_string(),
            scaffold_digest: scaffold.scaffold_digest().to_string(),
            birth_digest: birth_plan.birth_digest().to_string(),
            lowering_digest: lowering_plan.lowering_digest().to_string(),
            execution_digest: execution_plan.execution_digest().to_string(),
            execution_plan,
        })
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn execution_plan(&self) -> &TopologyConstructionExecutionPlan {
        &self.execution_plan
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }

    pub fn plan_topology_certification(&self) -> TopologyConstructionCertificationPlan {
        prepare_primitive_construction_certification(&self.execution_plan)
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionExecutionError {
    TopologyExecution(TopologyConstructionExecutionError),
}

impl std::fmt::Display for PrimitiveConstructionExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TopologyExecution(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionExecutionError {}

#[cfg(test)]
mod tests {
    use super::PreparedPrimitiveConstructionExecution;
    use crate::construction::{
        lower_scaffold_to_topology, PrimitiveConstructionFamily, PrimitiveConstructionIntent,
        RegularPrismSpec,
    };
    use topology::facade::{
        TopologyConstructionCertificationReadSurface, TopologyConstructionInspectionSurface,
    };

    #[test]
    fn prism_execution_and_certification_plans_bind_query_surfaces() {
        let intent = PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
            sides: 6,
            radius: 1.0,
            height: 2.0,
        });
        let request = intent.clone().into_request();
        let admitted = request.clone().admit().expect("admitted intent");
        let scaffold = admitted.build_scaffold().expect("scaffold");
        let (birth_plan, lowering_plan) = lower_scaffold_to_topology(&scaffold).expect("lowering");
        let execution = PreparedPrimitiveConstructionExecution::from_phase_chain(
            &request,
            &admitted,
            &scaffold,
            &birth_plan,
            &lowering_plan,
        )
        .expect("execution plan");
        let certification = execution.plan_topology_certification();

        assert_eq!(
            execution.family(),
            PrimitiveConstructionFamily::RegularPrism
        );
        assert_eq!(
            certification.read_surface(),
            TopologyConstructionCertificationReadSurface::ProjectionConsumptionFromInspectionReceipt
        );
        assert_eq!(
            certification.inspection_surface(),
            TopologyConstructionInspectionSurface::InspectReceipt
        );
        assert!(!execution.execution_digest().is_empty());
        assert!(!certification.certification_digest().is_empty());
    }
}
