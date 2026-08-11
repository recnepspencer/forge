use bank_domain::{
    estate::EstateAction,
    schema::{
        BankSchema, CompleteEstateMandatoryReviewCapability, CompleteEstateMandatoryReviewOperation,
    },
};
use worth_query_host::facade::{
    admission::authenticated_principal::WorthQueryRequestScope,
    declaration::application_schema::TypedMutationPreconditions,
    primary_graph::{
        WorthQueryApplicationIdempotencyBinding, WorthQueryMandatoryReview,
        WorthQueryMandatoryReviewOutcome,
    },
};

use super::{lifecycle_facts::seal_review_lifecycle_facts, BankEstateProgressionDenial};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

impl BankIdentityRuntime {
    pub fn complete_estate_mandatory_review(
        &self,
        principal: &BankAuthenticatedPrincipal,
        mandatory: WorthQueryMandatoryReview,
        action: EstateAction,
        idempotency: WorthQueryApplicationIdempotencyBinding,
        request: &WorthQueryRequestScope,
    ) -> Result<WorthQueryMandatoryReviewOutcome, BankEstateProgressionDenial> {
        let EstateAction::CompleteMandatoryReview { access, review, .. } = action else {
            return Err(BankEstateProgressionDenial::CommandInput(
                "CompleteEstateMandatoryReviewOperation",
            ));
        };
        let capability = self
            .application_runtime()
            .installed_schema()
            .capability(
                CompleteEstateMandatoryReviewCapability::reference(),
                CompleteEstateMandatoryReviewOperation::reference(),
            )
            .map_err(BankEstateProgressionDenial::from_capability_installation)?;
        let access_authority = self
            .application_runtime()
            .admit_capability_access(principal.query(), &capability, action, request)
            .map_err(BankEstateProgressionDenial::from_authorization)?;
        let operation = self
            .application_runtime()
            .installed_schema()
            .installed_operation(CompleteEstateMandatoryReviewOperation::reference())
            .map_err(BankEstateProgressionDenial::from_operation_installation)?;
        let admission = self
            .application_runtime()
            .authorize_mandatory_review(
                mandatory,
                access_authority,
                &operation,
                TypedMutationPreconditions::<
                    BankSchema,
                    CompleteEstateMandatoryReviewOperation,
                    bank_domain::schema::EstateCase,
                >::default(),
            )
            .map_err(BankEstateProgressionDenial::from_review_authorization)?;
        let projected = self
            .invariant_projection()
            .project_admitted_operation(&admission, |reader, estate| {
                seal_review_lifecycle_facts(reader, access, review, estate)
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
            .materialize_mandatory_review_program()
            .map_err(BankEstateProgressionDenial::from_attempt)?;
        Ok(self
            .application_runtime()
            .compare_and_commit_mandatory_review(program, idempotency))
    }
}
