mod admission;
mod admitted_handle;
mod checked_outcome;
mod draft;
mod operating_context;
mod validated_handle;
mod validation;

pub use admitted_handle::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEntryProgressionError,
};
pub use checked_outcome::{
    ForgeQueryConfiguredDomainHandleAdmissionError, ForgeQueryConfiguredDomainHandleChecked,
    ForgeQueryConfiguredDomainHandleDeferred, ForgeQueryConfiguredDomainHandleInvalidContext,
    ForgeQueryConfiguredDomainHandleUnsupported,
};
pub use draft::ForgeQueryConfiguredDomainHandleDraft;
pub use operating_context::ForgeQueryDomainOperatingContext;
pub use validated_handle::ForgeQueryValidatedConfiguredDomainHandle;

pub(crate) use checked_outcome::forge_query_checked_configured_domain_handle;

#[cfg(test)]
mod tests;
