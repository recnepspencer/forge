use worth_query::facade::{domain, read};

use super::super::GeometryDomain;
use super::material::{evidence_material, EvidenceScenario};
use super::{EvidenceFamily, EvidenceRead};

#[derive(Clone, Copy)]
pub(super) struct EvidenceDirectExecutor {
    scenario: EvidenceScenario,
}

impl EvidenceDirectExecutor {
    pub(super) const fn new(scenario: EvidenceScenario) -> Self {
        Self { scenario }
    }
}

impl domain::WorthQueryDomainOperationExecutor<GeometryDomain, EvidenceRead, EvidenceFamily>
    for EvidenceDirectExecutor
{
    const LOWERING_FAMILY: &'static str = "domain-evidence-read-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn installed_read_declaration(&self) -> Option<&read::WorthQueryReadDeclaration> {
        Some(super::super::executors::installed_read_declaration())
    }

    fn execute(
        &self,
        _: (),
        context: &domain::WorthQueryOperationExecutionContext<'_>,
        workspace: &mut domain::WorthQueryOperationWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryOperationExecutionMaterial<read::WorthQueryReadCompletion>,
        domain::WorthQueryOperationExecutorFailure,
    > {
        let completion = context.execute_installed_read(workspace)?;
        let output_identity =
            domain::WorthQueryOperationOutput::operation_output_identity(&completion);
        Ok(domain::WorthQueryOperationExecutionMaterial::new(
            completion,
            domain::WorthQueryOperationResultState::Ready,
        )
        .with_domain_evidence(evidence_material(&output_identity, self.scenario)))
    }
}
