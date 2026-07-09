mod admission;
mod admitted_handle;
mod admitted_world_basis;
mod checked_outcome;
mod draft;
mod operating_context;
mod validated_handle;
mod validation;

pub use admitted_handle::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationEntryProgressionError,
};
pub(crate) use admitted_world_basis::compose_admitted_configured_domain_handle_identity;
pub use admitted_world_basis::WorthQueryAdmittedWorldBasis;
pub use checked_outcome::{
    WorthQueryConfiguredDomainHandleAdmissionError, WorthQueryConfiguredDomainHandleChecked,
    WorthQueryConfiguredDomainHandleDeferred, WorthQueryConfiguredDomainHandleInvalidContext,
    WorthQueryConfiguredDomainHandleUnsupported,
};
pub use draft::WorthQueryConfiguredDomainHandleDraft;
pub use operating_context::{
    WorthQueryContinuationExecutionReadmissionObservation, WorthQueryDomainOperatingContext,
    WorthQueryDomainOperatingRequirement,
};
pub use validated_handle::WorthQueryValidatedConfiguredDomainHandle;

pub(crate) use admitted_handle::checked_route_plan_from_progressed_with_profile;
pub(crate) use checked_outcome::worth_query_checked_configured_domain_handle;

#[cfg(test)]
mod admitted_world_basis_tests;
#[cfg(test)]
mod tests;
