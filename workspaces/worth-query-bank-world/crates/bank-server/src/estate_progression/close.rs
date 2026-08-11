use bank_domain::{
    estate::EstateAction,
    schema::{
        BankSchema, RevokeEstateEmergencyAccessCapability, RevokeEstateEmergencyAccessOperation,
    },
};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    declaration::application_schema::TypedMutationPreconditions,
    primary_graph::{
        WorthQueryApplicationIdempotencyBinding, WorthQueryApprovedElevation,
        WorthQueryElevationCloseOutcome,
    },
};

use super::{lifecycle_facts::seal_close_lifecycle_facts, BankEstateProgressionDenial};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub fn revoke_estate_emergency_access(
        &self,
        principal: &BankAuthenticatedPrincipal,
        approved: WorthQueryApprovedElevation,
        action: EstateAction,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryElevationCloseOutcome, BankEstateProgressionDenial> {
        let EstateAction::RevokeEmergencyAccess { access, .. } = action else {
            return Err(BankEstateProgressionDenial::CommandInput(
                "RevokeEstateEmergencyAccessOperation",
            ));
        };
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                RevokeEstateEmergencyAccessCapability::reference(),
                RevokeEstateEmergencyAccessOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::from_capability_installation)?;
        let access_authority = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::from_authorization)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(RevokeEstateEmergencyAccessOperation::reference())
            .map_err(BankEstateProgressionDenial::from_operation_installation)?;
        let admission = self
            .application_runtime()
            .authorize_elevation_close(
                approved,
                access_authority,
                &operation,
                TypedMutationPreconditions::<
                    BankSchema,
                    RevokeEstateEmergencyAccessOperation,
                    bank_domain::schema::EstateCase,
                >::default(),
            )
            .map_err(BankEstateProgressionDenial::from_close_authorization)?;
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                seal_close_lifecycle_facts(reader, access, estate)
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
            .materialize_elevation_close_program()
            .map_err(BankEstateProgressionDenial::from_attempt)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_elevation_close(program, idempotency))
    }
}
