use worth_query_installation::facade::ApplicationSchema;

use super::{
    provider_recomparison::recover_equivalent_commit_evidence, WorthQueryApplicationCommitReceipt,
    WorthQueryApplicationIdempotencyBinding,
};
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::primary_graph::{
    WorthQueryAdmittedApplicationOperation, WorthQueryOperationAuthorizationDenial,
    WorthQueryPrimaryGraphApplicationRuntime,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationIdempotencyResolution {
    Unseen,
    AlreadyCommitted(WorthQueryApplicationCommitReceipt),
    IntentDrift,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationIdempotencyResolutionDenialKind {
    Authorization,
    ForeignAdmission,
    ProviderUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationIdempotencyResolutionDenial {
    kind: WorthQueryApplicationIdempotencyResolutionDenialKind,
    authorization: Option<WorthQueryOperationAuthorizationDenial>,
}

impl WorthQueryApplicationIdempotencyResolutionDenial {
    pub const fn kind(&self) -> WorthQueryApplicationIdempotencyResolutionDenialKind {
        self.kind
    }

    pub const fn authorization(&self) -> Option<&WorthQueryOperationAuthorizationDenial> {
        self.authorization.as_ref()
    }

    fn from_authorization(denial: WorthQueryOperationAuthorizationDenial) -> Self {
        Self {
            kind: WorthQueryApplicationIdempotencyResolutionDenialKind::Authorization,
            authorization: Some(denial),
        }
    }

    const fn foreign_admission() -> Self {
        Self {
            kind: WorthQueryApplicationIdempotencyResolutionDenialKind::ForeignAdmission,
            authorization: None,
        }
    }

    const fn provider_unavailable() -> Self {
        Self {
            kind: WorthQueryApplicationIdempotencyResolutionDenialKind::ProviderUnavailable,
            authorization: None,
        }
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn resolve_admitted_application_idempotency<Operation, Input, Scope>(
        &self,
        admission: &WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
        binding: WorthQueryApplicationIdempotencyBinding,
    ) -> Result<
        WorthQueryApplicationIdempotencyResolution,
        WorthQueryApplicationIdempotencyResolutionDenial,
    > {
        admission
            .validate_current_authority()
            .map_err(WorthQueryApplicationIdempotencyResolutionDenial::from_authorization)?;
        if !admission.belongs_to(
            self.runtime.authority_identity(),
            &self.installed_schema.binding_identity(),
        ) {
            return Err(WorthQueryApplicationIdempotencyResolutionDenial::foreign_admission());
        }
        let binding = binding.bind_preconditions(admission.mutation_preconditions().identity());
        let serialization = self.primary_provider.serialize_application_commit();
        let proof = self
            .authorize_idempotency_inspection(admission, &serialization)
            .map_err(WorthQueryApplicationIdempotencyResolutionDenial::from_authorization)?;
        let resolution = proof
            .govern((), |()| {
                self.primary_provider.resolve_idempotency_binding(binding)
            })
            .map_err(|()| {
                WorthQueryApplicationIdempotencyResolutionDenial::from_authorization(
                    WorthQueryOperationAuthorizationDenial::inconsistent(admission.operation()),
                )
            })?;
        match resolution {
            Ok(WorthQueryProviderIdempotencyResolution::Absent) => {
                Ok(WorthQueryApplicationIdempotencyResolution::Unseen)
            }
            Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => Ok(
                WorthQueryApplicationIdempotencyResolution::AlreadyCommitted(
                    WorthQueryApplicationCommitReceipt::from_provider(
                        receipt,
                        recover_equivalent_commit_evidence(admission.mutation_preconditions()),
                        admission.canonical_work(),
                    ),
                ),
            ),
            Ok(WorthQueryProviderIdempotencyResolution::Drift) => {
                Ok(WorthQueryApplicationIdempotencyResolution::IntentDrift)
            }
            Err(_) => Err(WorthQueryApplicationIdempotencyResolutionDenial::provider_unavailable()),
        }
    }
}

impl std::fmt::Display for WorthQueryApplicationIdempotencyResolutionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application idempotency resolution denied: {:?}",
            self.kind
        )
    }
}

impl std::error::Error for WorthQueryApplicationIdempotencyResolutionDenial {}
