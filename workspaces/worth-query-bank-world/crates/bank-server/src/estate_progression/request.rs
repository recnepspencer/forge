use bank_domain::estate::EstateAction;
use bank_domain::schema::{
    BankSchema, EstateCaseIdentityField, RequestEstateEmergencyAccessCapability,
    RequestEstateEmergencyAccessOperation,
};
use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::declaration::application_schema::TypedMutationPreconditions;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationIdempotencyBinding, WorthQueryElevationRequestOutcome,
};

use super::BankEstateProgressionDenial;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub fn request_estate_emergency_access(
        &self,
        principal: &BankAuthenticatedPrincipal,
        action: EstateAction,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryElevationRequestOutcome, BankEstateProgressionDenial> {
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                RequestEstateEmergencyAccessCapability::reference(),
                RequestEstateEmergencyAccessOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::CapabilityInstallation)?;
        let access = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::Authorization)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(RequestEstateEmergencyAccessOperation::reference())
            .map_err(BankEstateProgressionDenial::OperationInstallation)?;
        let admission = self
            .application_runtime()
            .authorize_elevation_request(
                access,
                &operation,
                TypedMutationPreconditions::<
                    BankSchema,
                    RequestEstateEmergencyAccessOperation,
                    bank_domain::schema::EstateCase,
                >::default(),
            )
            .map_err(BankEstateProgressionDenial::Authorization)?;
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                reader.require_decision_field(estate, EstateCaseIdentityField::reference())
            })
            .map_err(BankEstateProgressionDenial::Projection)?;
        let (field_result, projection, _) = projected.into_parts();
        field_result.map_err(BankEstateProgressionDenial::DecisionProjection)?;
        let program = self
            .application_runtime()
            .begin_projected_application_read_attempt(admission, projection)
            .map_err(BankEstateProgressionDenial::Attempt)?
            .complete_projected_dependencies()
            .map_err(BankEstateProgressionDenial::Attempt)?
            .materialize_elevation_request_program()
            .map_err(BankEstateProgressionDenial::Attempt)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_elevation_request(program, idempotency))
    }
}
