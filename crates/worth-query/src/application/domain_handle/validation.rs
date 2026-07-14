use super::admitted_world_basis::compose_admitted_configured_domain_handle_identity_parts;
use super::checked_outcome::WorthQueryConfiguredDomainHandleInvalidContext;
use super::draft::WorthQueryConfiguredDomainHandleDraft;
use super::operating_context::{
    WorthQueryDomainOperatingContext, WorthQueryDomainOperatingRequirement,
};
use super::validated_handle::WorthQueryValidatedConfiguredDomainHandle;
use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
};

pub(crate) fn validate_configured_domain_handle_draft<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
>(
    draft: WorthQueryConfiguredDomainHandleDraft<D, C>,
) -> Result<
    WorthQueryValidatedConfiguredDomainHandle<D, C>,
    WorthQueryConfiguredDomainHandleInvalidContext<D, C>,
> {
    let marker = draft.marker();
    let operating_context = draft.operating_context().clone();
    let support_snapshot = draft.support_snapshot().clone();
    let required_capability_families = canonical_capability_families(
        marker.required_capability_families(),
        operating_context.required_capability_families(),
    );
    let required_config_sections =
        canonical_config_sections(operating_context.required_config_sections());
    let required_operating_requirements =
        canonical_operating_requirements(operating_context.required_operating_requirements());
    let context_identity_digest = operating_context.context_identity_digest();

    if context_identity_digest.is_empty() {
        return Err(WorthQueryConfiguredDomainHandleInvalidContext::new(
            marker,
            operating_context,
            support_snapshot,
            Vec::new(),
            "operating context identity digest may not be empty",
        ));
    }

    let missing_sections =
        missing_required_sections(&required_capability_families, &required_config_sections);
    if !missing_sections.is_empty() {
        return Err(WorthQueryConfiguredDomainHandleInvalidContext::new(
            marker,
            operating_context,
            support_snapshot,
            missing_sections,
            "required capability families must map to declared config sections",
        ));
    }

    let handle_identity_digest = compose_admitted_configured_domain_handle_identity_parts(
        marker.domain_key(),
        marker.display_name(),
        &required_capability_families,
        &required_config_sections,
        &required_operating_requirements,
        &context_identity_digest,
        support_snapshot.validated_config_digest(),
    )
    .as_str()
    .to_string();

    Ok(WorthQueryValidatedConfiguredDomainHandle::new(
        marker,
        operating_context,
        support_snapshot,
        required_capability_families,
        required_config_sections,
        required_operating_requirements,
        context_identity_digest,
        handle_identity_digest,
    ))
}

fn canonical_capability_families(
    marker_capabilities: &'static [WorthQueryCapabilityFamily],
    context_capabilities: &'static [WorthQueryCapabilityFamily],
) -> Vec<WorthQueryCapabilityFamily> {
    let mut families = marker_capabilities
        .iter()
        .chain(context_capabilities.iter())
        .copied()
        .collect::<Vec<_>>();
    families.sort();
    families.dedup();
    families
}

fn canonical_config_sections(
    required_config_sections: &'static [WorthQueryConfigSectionFamily],
) -> Vec<WorthQueryConfigSectionFamily> {
    let mut sections = required_config_sections.to_vec();
    sections.sort();
    sections.dedup();
    sections
}

fn canonical_operating_requirements(
    required_operating_requirements: &'static [WorthQueryDomainOperatingRequirement],
) -> Vec<WorthQueryDomainOperatingRequirement> {
    let mut requirements = required_operating_requirements.to_vec();
    requirements.sort();
    requirements.dedup();
    requirements
}

fn missing_required_sections(
    required_capability_families: &[WorthQueryCapabilityFamily],
    required_config_sections: &[WorthQueryConfigSectionFamily],
) -> Vec<WorthQueryConfigSectionFamily> {
    let mut missing = required_capability_families
        .iter()
        .map(WorthQueryCapabilityFamily::config_section)
        .filter(|section| !required_config_sections.contains(section))
        .collect::<Vec<_>>();
    missing.sort();
    missing.dedup();
    missing
}
