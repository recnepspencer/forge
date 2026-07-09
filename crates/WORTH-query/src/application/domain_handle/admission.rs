use super::admitted_handle::WorthQueryAdmittedConfiguredDomainHandle;
use super::checked_outcome::{
    WorthQueryConfiguredDomainHandleAdmissionError, WorthQueryConfiguredDomainHandleChecked,
    WorthQueryConfiguredDomainHandleDeferred, WorthQueryConfiguredDomainHandleInvalidContext,
    WorthQueryConfiguredDomainHandleUnsupported,
};
use super::operating_context::{
    WorthQueryDomainOperatingContext, WorthQueryDomainOperatingRequirement,
};
use super::validated_handle::WorthQueryValidatedConfiguredDomainHandle;
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryCapabilityStatus, WorthQueryConfigSectionFamily,
    WorthQueryDomainEntryMarker,
};
use crate::runtime::WorthQueryRuntimeFamilySupportStatus;

pub(crate) fn admit_configured_domain_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    validated_handle: WorthQueryValidatedConfiguredDomainHandle<D, C>,
) -> Result<
    WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    WorthQueryConfiguredDomainHandleAdmissionError<D, C>,
> {
    match checked_from_validated_handle(validated_handle) {
        WorthQueryConfiguredDomainHandleChecked::Admitted(handle) => Ok(handle),
        WorthQueryConfiguredDomainHandleChecked::Deferred(denial) => Err(
            WorthQueryConfiguredDomainHandleAdmissionError::Deferred(denial),
        ),
        WorthQueryConfiguredDomainHandleChecked::Unsupported(denial) => Err(
            WorthQueryConfiguredDomainHandleAdmissionError::Unsupported(denial),
        ),
        WorthQueryConfiguredDomainHandleChecked::InvalidContext(denial) => {
            Err(WorthQueryConfiguredDomainHandleAdmissionError::InvalidContext(denial))
        }
    }
}

pub(crate) fn checked_from_validated_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    validated_handle: WorthQueryValidatedConfiguredDomainHandle<D, C>,
) -> WorthQueryConfiguredDomainHandleChecked<D, C> {
    let deferred_operating_requirements = blocking_operating_requirements(
        &validated_handle,
        WorthQueryRuntimeFamilySupportStatus::DeferredDebt,
    );
    let deferred =
        blocking_capability_families(&validated_handle, WorthQueryCapabilityStatus::DeferredDebt);
    if !deferred.is_empty() || !deferred_operating_requirements.is_empty() {
        return WorthQueryConfiguredDomainHandleChecked::Deferred(
            WorthQueryConfiguredDomainHandleDeferred::new(
                validated_handle,
                deferred,
                deferred_operating_requirements,
            ),
        );
    }

    let unsupported_operating_requirements = blocking_operating_requirements(
        &validated_handle,
        WorthQueryRuntimeFamilySupportStatus::Unsupported,
    );
    let unsupported =
        blocking_capability_families(&validated_handle, WorthQueryCapabilityStatus::Unsupported);
    if !unsupported.is_empty() || !unsupported_operating_requirements.is_empty() {
        return WorthQueryConfiguredDomainHandleChecked::Unsupported(
            WorthQueryConfiguredDomainHandleUnsupported::new(
                validated_handle,
                unsupported,
                unsupported_operating_requirements,
            ),
        );
    }

    let disabled_sections = disabled_required_config_sections(&validated_handle);
    if !disabled_sections.is_empty() {
        return WorthQueryConfiguredDomainHandleChecked::InvalidContext(
            WorthQueryConfiguredDomainHandleInvalidContext::new(
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
        required_operating_requirements,
        operating_context_identity_digest,
        handle_identity_digest,
    ) = validated_handle.into_parts();
    WorthQueryConfiguredDomainHandleChecked::Admitted(
        WorthQueryAdmittedConfiguredDomainHandle::new(
            marker,
            operating_context,
            support_snapshot,
            required_capability_families,
            required_config_sections,
            required_operating_requirements,
            operating_context_identity_digest,
            handle_identity_digest,
        ),
    )
}

fn blocking_capability_families<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    validated_handle: &WorthQueryValidatedConfiguredDomainHandle<D, C>,
    target: WorthQueryCapabilityStatus,
) -> Vec<WorthQueryCapabilityFamily> {
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
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    validated_handle: &WorthQueryValidatedConfiguredDomainHandle<D, C>,
) -> Vec<WorthQueryConfigSectionFamily> {
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

fn blocking_operating_requirements<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    validated_handle: &WorthQueryValidatedConfiguredDomainHandle<D, C>,
    target: WorthQueryRuntimeFamilySupportStatus,
) -> Vec<WorthQueryDomainOperatingRequirement> {
    validated_handle
        .required_operating_requirements()
        .iter()
        .copied()
        .filter(|requirement| {
            validated_handle
                .support_snapshot()
                .operating_requirement_status(*requirement)
                == Some(target)
        })
        .collect()
}
