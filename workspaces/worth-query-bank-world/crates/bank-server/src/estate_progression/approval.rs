use bank_domain::{
    estate::EstateAction,
    schema::{
        ApproveEstateEmergencyAccessCapability, ApproveEstateEmergencyAccessOperation, BankSchema,
    },
};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    declaration::application_schema::TypedMutationPreconditions,
    primary_graph::{
        WorthQueryApplicationIdempotencyBinding, WorthQueryElevationApprovalOutcome,
        WorthQueryRequestedElevation,
    },
};

use super::{
    lifecycle_facts::{approval_lifecycle_identities, seal_approval_lifecycle_facts},
    BankEstateProgressionDenial,
};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub fn approve_estate_emergency_access(
        &self,
        principal: &BankAuthenticatedPrincipal,
        requested: WorthQueryRequestedElevation,
        action: EstateAction,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryElevationApprovalOutcome, BankEstateProgressionDenial> {
        let (access_identity, review_identity) = approval_lifecycle_identities(&requested)
            .map_err(BankEstateProgressionDenial::LifecycleProjection)?;
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                ApproveEstateEmergencyAccessCapability::reference(),
                ApproveEstateEmergencyAccessOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::from_capability_installation)?;
        let access = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::from_authorization)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(ApproveEstateEmergencyAccessOperation::reference())
            .map_err(BankEstateProgressionDenial::from_operation_installation)?;
        let admission = self
            .application_runtime()
            .authorize_elevation_approval(
                requested,
                access,
                &operation,
                TypedMutationPreconditions::<
                    BankSchema,
                    ApproveEstateEmergencyAccessOperation,
                    bank_domain::schema::EstateCase,
                >::default(),
            )
            .map_err(BankEstateProgressionDenial::from_approval_authorization)?;
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                seal_approval_lifecycle_facts(reader, access_identity, review_identity, estate)
            })
            .map_err(BankEstateProgressionDenial::from_projection)?;
        let (lifecycle_result, projection, _) = projected.into_parts();
        lifecycle_result.map_err(BankEstateProgressionDenial::LifecycleProjection)?;
        let program = self
            .application_runtime()
            .begin_projected_application_read_attempt(admission, projection)
            .map_err(BankEstateProgressionDenial::from_attempt)?
            .complete_projected_dependencies()
            .map_err(BankEstateProgressionDenial::from_attempt)?
            .materialize_elevation_approval_program()
            .map_err(BankEstateProgressionDenial::from_attempt)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_elevation_approval(program, idempotency))
    }
}
