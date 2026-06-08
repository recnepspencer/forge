use super::checked_outcome::ForgeQueryConfiguredDomainHandleInvalidContext;
use super::draft::ForgeQueryConfiguredDomainHandleDraft;
use super::operating_context::{
    ForgeQueryDomainOperatingContext, ForgeQueryDomainOperatingRequirement,
};
use super::validated_handle::ForgeQueryValidatedConfiguredDomainHandle;
use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDomainEntryMarker,
};
use crate::identity::hash_parts;

pub(crate) fn validate_configured_domain_handle_draft<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
>(
    draft: ForgeQueryConfiguredDomainHandleDraft<D, C>,
) -> Result<
    ForgeQueryValidatedConfiguredDomainHandle<D, C>,
    ForgeQueryConfiguredDomainHandleInvalidContext<D, C>,
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
        return Err(ForgeQueryConfiguredDomainHandleInvalidContext::new(
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
        return Err(ForgeQueryConfiguredDomainHandleInvalidContext::new(
            marker,
            operating_context,
            support_snapshot,
            missing_sections,
            "required capability families must map to declared config sections",
        ));
    }

    let handle_identity_digest = hash_parts(&[
        format!("domain:{}", marker.domain_key()),
        format!("display:{}", marker.display_name()),
        format!(
            "required_capabilities:{}",
            required_capability_families
                .iter()
                .map(ForgeQueryCapabilityFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "required_sections:{}",
            required_config_sections
                .iter()
                .map(ForgeQueryConfigSectionFamily::as_str)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "operating_requirements:{}",
            required_operating_requirements
                .iter()
                .map(|requirement| requirement.as_str())
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!("context:{context_identity_digest}"),
        format!(
            "validated_config:{}",
            support_snapshot.validated_config_digest()
        ),
    ]);

    Ok(ForgeQueryValidatedConfiguredDomainHandle::new(
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
    marker_capabilities: &'static [ForgeQueryCapabilityFamily],
    context_capabilities: &'static [ForgeQueryCapabilityFamily],
) -> Vec<ForgeQueryCapabilityFamily> {
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
    required_config_sections: &'static [ForgeQueryConfigSectionFamily],
) -> Vec<ForgeQueryConfigSectionFamily> {
    let mut sections = required_config_sections.to_vec();
    sections.sort();
    sections.dedup();
    sections
}

fn canonical_operating_requirements(
    required_operating_requirements: &'static [ForgeQueryDomainOperatingRequirement],
) -> Vec<ForgeQueryDomainOperatingRequirement> {
    let mut requirements = required_operating_requirements.to_vec();
    requirements.sort();
    requirements.dedup();
    requirements
}

fn missing_required_sections(
    required_capability_families: &[ForgeQueryCapabilityFamily],
    required_config_sections: &[ForgeQueryConfigSectionFamily],
) -> Vec<ForgeQueryConfigSectionFamily> {
    let mut missing = required_capability_families
        .iter()
        .map(ForgeQueryCapabilityFamily::config_section)
        .filter(|section| !required_config_sections.contains(section))
        .collect::<Vec<_>>();
    missing.sort();
    missing.dedup();
    missing
}
