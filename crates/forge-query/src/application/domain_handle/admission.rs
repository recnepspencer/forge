use super::admitted_handle::ForgeQueryAdmittedConfiguredDomainHandle;
use super::checked_outcome::{
    ForgeQueryConfiguredDomainHandleAdmissionError, ForgeQueryConfiguredDomainHandleChecked,
    ForgeQueryConfiguredDomainHandleDeferred, ForgeQueryConfiguredDomainHandleInvalidContext,
    ForgeQueryConfiguredDomainHandleUnsupported,
};
use super::operating_context::ForgeQueryDomainOperatingContext;
use super::validated_handle::ForgeQueryValidatedConfiguredDomainHandle;
use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus, ForgeQueryConfigSectionFamily,
    ForgeQueryDomainEntryMarker,
};

pub(crate) fn admit_configured_domain_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    validated_handle: ForgeQueryValidatedConfiguredDomainHandle<D, C>,
) -> Result<
    ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    ForgeQueryConfiguredDomainHandleAdmissionError<D, C>,
> {
    match checked_from_validated_handle(validated_handle) {
        ForgeQueryConfiguredDomainHandleChecked::Admitted(handle) => Ok(handle),
        ForgeQueryConfiguredDomainHandleChecked::Deferred(denial) => Err(
            ForgeQueryConfiguredDomainHandleAdmissionError::Deferred(denial),
        ),
        ForgeQueryConfiguredDomainHandleChecked::Unsupported(denial) => Err(
            ForgeQueryConfiguredDomainHandleAdmissionError::Unsupported(denial),
        ),
        ForgeQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
            Err(ForgeQueryConfiguredDomainHandleAdmissionError::InvalidContext(denial))
        }
    }
}

pub(crate) fn checked_from_validated_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    validated_handle: ForgeQueryValidatedConfiguredDomainHandle<D, C>,
) -> ForgeQueryConfiguredDomainHandleChecked<D, C> {
    let deferred =
        blocking_capability_families(&validated_handle, ForgeQueryCapabilityStatus::DeferredDebt);
    if !deferred.is_empty() {
        return ForgeQueryConfiguredDomainHandleChecked::Deferred(
            ForgeQueryConfiguredDomainHandleDeferred::new(validated_handle, deferred),
        );
    }

    let unsupported =
        blocking_capability_families(&validated_handle, ForgeQueryCapabilityStatus::Unsupported);
    if !unsupported.is_empty() {
        return ForgeQueryConfiguredDomainHandleChecked::Unsupported(
            ForgeQueryConfiguredDomainHandleUnsupported::new(validated_handle, unsupported),
        );
    }

    let disabled_sections = disabled_required_config_sections(&validated_handle);
    if !disabled_sections.is_empty() {
        return ForgeQueryConfiguredDomainHandleChecked::InvalidContext(
            ForgeQueryConfiguredDomainHandleInvalidContext::new(
                validated_handle.marker(),
                validated_handle.operating_context().clone(),
                validated_handle.support_snapshot().clone(),
                disabled_sections,
                "required config sections must be enabled before handle admission",
            ),
        );
    }

    let (
        marker,
        operating_context,
        support_snapshot,
        required_capability_families,
        required_config_sections,
        operating_context_identity_digest,
        handle_identity_digest,
    ) = validated_handle.into_parts();
    ForgeQueryConfiguredDomainHandleChecked::Admitted(
        ForgeQueryAdmittedConfiguredDomainHandle::new(
            marker,
            operating_context,
            support_snapshot,
            required_capability_families,
            required_config_sections,
            operating_context_identity_digest,
            handle_identity_digest,
        ),
    )
}

fn blocking_capability_families<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    validated_handle: &ForgeQueryValidatedConfiguredDomainHandle<D, C>,
    target: ForgeQueryCapabilityStatus,
) -> Vec<ForgeQueryCapabilityFamily> {
    validated_handle
        .required_capability_families()
        .iter()
        .copied()
        .filter(|family| {
            validated_handle
                .support_snapshot()
                .capability_status(*family)
                == Some(target)
        })
        .collect()
}

fn disabled_required_config_sections<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    validated_handle: &ForgeQueryValidatedConfiguredDomainHandle<D, C>,
) -> Vec<ForgeQueryConfigSectionFamily> {
    validated_handle
        .required_config_sections()
        .iter()
        .copied()
        .filter(|section| {
            validated_handle
                .support_snapshot()
                .section_postures()
                .iter()
                .find(|posture| posture.section() == *section)
                .is_some_and(|posture| !posture.enabled())
        })
        .collect()
}
