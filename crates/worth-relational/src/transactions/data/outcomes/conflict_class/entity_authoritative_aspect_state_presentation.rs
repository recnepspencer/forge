use worth_foundational::facade::BoundarySourceLocator;

use super::authoritative_aspect_source_locator::{
    aspect_field_locator_field_label, source_locator_aspect_label, source_locator_field_label,
};
use super::EntityAuthoritativeAspectStateDenial;

pub(super) fn entity_authoritative_aspect_state_denial_detail(
    denial: &EntityAuthoritativeAspectStateDenial,
) -> String {
    match denial {
        EntityAuthoritativeAspectStateDenial::MissingAspectPlan { kind_id } => {
            format!(
                "entity aspect creation requires a lowered aspect plan for kind {}",
                kind_id.0
            )
        }
        EntityAuthoritativeAspectStateDenial::ContractValidationDenied {
            source_locator,
            denial,
        } => format!(
            "declared field '{}' failed contract validation for aspect {}: {:?}",
            source_field_label(source_locator),
            source_locator_aspect_label(source_locator),
            denial
        ),
        EntityAuthoritativeAspectStateDenial::PatchConstructionDenied { denial } => format!(
            "authoritative aspect admission could not construct foundational patch: {:?}",
            denial
        ),
        EntityAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget { target, reason } => {
            format!(
                "declared field '{}' / {} is not a supported entity aspect target: {}",
                aspect_field_locator_field_label(target),
                target.aspect().aspect_key().as_str(),
                reason.label()
            )
        }
        EntityAuthoritativeAspectStateDenial::StructValueConstructionDenied { source_locator } => {
            format!(
                "field declarations for {} at '{}' cannot construct the declared struct aspect value",
                source_locator_aspect_label(source_locator),
                source_field_label(source_locator)
            )
        }
        EntityAuthoritativeAspectStateDenial::StructContractValidationDenied {
            source_locator,
            denial,
        } => format!(
            "declared struct aspect for {} at '{}' failed contract validation: {:?}",
            source_locator_aspect_label(source_locator),
            source_field_label(source_locator),
            denial
        ),
        EntityAuthoritativeAspectStateDenial::StateAdmissionDenied { denial } => format!(
            "declared aspects could not be admitted as authoritative state: {:?}",
            denial
        ),
    }
}

fn source_field_label(source_locator: &BoundarySourceLocator) -> String {
    source_locator_field_label(source_locator).unwrap_or_else(|| "whole_aspect".to_string())
}
