use crate::basis_lifecycle::BasisOperationLane;

pub(crate) trait WorthQueryProjectionLifecycleSource<D, O, F, L: BasisOperationLane> {
    fn identity(&self) -> &str;
    fn bound_operation(
        &self,
    ) -> &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>;
    fn consumer_contract(
        &self,
    ) -> &crate::domain_installation::WorthQueryConsumerProjectionContract<D, O, F, L>;
    fn installed_read(
        &self,
        checks: &mut usize,
    ) -> Option<crate::ordinary::read::WorthQueryReadDeclaration>;
    fn workflow_run_identity(&self) -> Option<&str>;
    fn publication_stage_identity(&self) -> Option<&str>;
    fn operation_resources(
        &self,
    ) -> &crate::domain_installation::WorthQueryAdmittedExecutionResourcePlan;
    fn operation_resource_evidence(
        &self,
    ) -> &crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence;
    fn stage_resources(
        &self,
        stage_identity: &str,
    ) -> Option<&crate::domain_installation::WorthQueryAdmittedExecutionResourcePlan>;
    fn stage_resource_evidence(
        &self,
        stage_identity: &str,
    ) -> Option<&crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence>;
    fn native_access_layout(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryNativeAccessLayout>;
    fn projection_authority_contract(
        &self,
    ) -> crate::projection_consumption::ProjectionAuthorityContract;
    fn semantic_dependency_closure(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure>;

    fn admits_conditional_location(
        &self,
        location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    ) -> bool {
        match location {
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::Operation {
                ..
            } => true,
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::WorkflowStage {
                stage_identity,
                ..
            } => self.publication_stage_identity() == Some(stage_identity.as_str()),
        }
    }
}

pub(crate) fn validate_live_source_authority<D: 'static, O, F, L, S>(
    source: &S,
    workspace: &crate::runtime::WorthQueryWorkspace,
) -> Result<(), crate::domain_installation::WorthQueryDomainHandleDenial>
where
    L: BasisOperationLane,
    S: WorthQueryProjectionLifecycleSource<D, O, F, L>,
{
    let witness =
        crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness::from_authority(
            std::sync::Arc::clone(source.bound_operation().operation().domain_authority()),
        );
    workspace.validate_installed_domain_witness::<D>(&witness)
}

impl<D, O, F, L: BasisOperationLane> WorthQueryProjectionLifecycleSource<D, O, F, L>
    for crate::domain_installation::WorthQuerySettledDomainProjection<D, O, F, L>
{
    fn identity(&self) -> &str {
        self.identity()
    }

    fn bound_operation(
        &self,
    ) -> &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L> {
        self.bound_operation()
    }

    fn consumer_contract(
        &self,
    ) -> &crate::domain_installation::WorthQueryConsumerProjectionContract<D, O, F, L> {
        self.consumer_contract()
    }

    fn installed_read(
        &self,
        checks: &mut usize,
    ) -> Option<crate::ordinary::read::WorthQueryReadDeclaration> {
        installed_read(
            self.bound_operation().executor()?.installed_read.as_ref(),
            self.consumer_contract(),
            checks,
        )
    }

    fn workflow_run_identity(&self) -> Option<&str> {
        None
    }

    fn publication_stage_identity(&self) -> Option<&str> {
        None
    }

    fn operation_resources(
        &self,
    ) -> &crate::domain_installation::WorthQueryAdmittedExecutionResourcePlan {
        self.resources()
    }

    fn operation_resource_evidence(
        &self,
    ) -> &crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence {
        self.execution_receipt().execution_resources()
    }

    fn stage_resources(
        &self,
        _stage_identity: &str,
    ) -> Option<&crate::domain_installation::WorthQueryAdmittedExecutionResourcePlan> {
        None
    }

    fn stage_resource_evidence(
        &self,
        _stage_identity: &str,
    ) -> Option<&crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence> {
        None
    }

    fn native_access_layout(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryNativeAccessLayout> {
        self.native_access_layout()
    }

    fn projection_authority_contract(
        &self,
    ) -> crate::projection_consumption::ProjectionAuthorityContract {
        self.authority().consumer_contract().clone()
    }

    fn semantic_dependency_closure(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure>
    {
        Some(self.semantic_aspect_dependency_closure())
    }
}

impl<D, O, F, L: BasisOperationLane> WorthQueryProjectionLifecycleSource<D, O, F, L>
    for crate::domain_installation::WorthQuerySettledWorkflowProjection<D, O, F, L>
{
    fn identity(&self) -> &str {
        self.identity()
    }

    fn bound_operation(
        &self,
    ) -> &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L> {
        self.bound_operation()
    }

    fn consumer_contract(
        &self,
    ) -> &crate::domain_installation::WorthQueryConsumerProjectionContract<D, O, F, L> {
        self.consumer_contract()
    }

    fn installed_read(
        &self,
        checks: &mut usize,
    ) -> Option<crate::ordinary::read::WorthQueryReadDeclaration> {
        installed_read(
            self.installed_workflow_read(),
            self.consumer_contract(),
            checks,
        )
    }

    fn workflow_run_identity(&self) -> Option<&str> {
        Some(self.workflow_run_identity())
    }

    fn publication_stage_identity(&self) -> Option<&str> {
        Some(self.publication_stage_identity())
    }

    fn operation_resources(
        &self,
    ) -> &crate::domain_installation::WorthQueryAdmittedExecutionResourcePlan {
        self.trace().resources().operation()
    }

    fn operation_resource_evidence(
        &self,
    ) -> &crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence {
        self.trace().operation_resource_evidence()
    }

    fn stage_resources(
        &self,
        stage_identity: &str,
    ) -> Option<&crate::domain_installation::WorthQueryAdmittedExecutionResourcePlan> {
        self.trace().resources().stage(stage_identity)
    }

    fn stage_resource_evidence(
        &self,
        stage_identity: &str,
    ) -> Option<&crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence> {
        self.trace()
            .stage_receipts()
            .iter()
            .find(|receipt| receipt.stage_identity() == stage_identity)
            .map(crate::domain_installation::WorthQueryWorkflowStageReceipt::execution_resources)
    }

    fn native_access_layout(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryNativeAccessLayout> {
        self.native_access_layout()
    }

    fn projection_authority_contract(
        &self,
    ) -> crate::projection_consumption::ProjectionAuthorityContract {
        self.authority().consumer_contract().clone()
    }

    fn semantic_dependency_closure(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure>
    {
        self.trace().semantic_aspect_dependency_closure()
    }
}

fn installed_read<D, O, F, L: BasisOperationLane>(
    read: Option<&crate::ordinary::read::WorthQueryReadDeclaration>,
    consumer: &crate::domain_installation::WorthQueryConsumerProjectionContract<D, O, F, L>,
    checks: &mut usize,
) -> Option<crate::ordinary::read::WorthQueryReadDeclaration> {
    *checks += 1;
    let read = read?;
    let canonical = consumer.canonical_projection();
    *checks += 1;
    if read.identity().canonical_query_digest() != canonical.query().digest().as_str() {
        return None;
    }
    *checks += 1;
    if read.identity().canonical_result_shape_digest() != canonical.result_shape().digest().as_str()
    {
        return None;
    }
    Some(read.clone_for_installed_execution())
}
