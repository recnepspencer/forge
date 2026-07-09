use worth_foundational::facade::BoundarySourceLocator;

use super::authoritative_aspect_source_locator::{
    aspect_field_locator_field_label, source_locator_aspect_label, source_locator_field_label,
};
use super::RelationAuthoritativeAspectStateDenial;

pub(super) fn relation_authoritative_aspect_state_denial_detail(
    denial: &RelationAuthoritativeAspectStateDenial,
) -> String {
    match denial {
        RelationAuthoritativeAspectStateDenial::MissingAspectPlan { kind_id } => format!(
            "relation kind {:?} has no executable aspect plan for declared relation fields",
            kind_id
        ),
        RelationAuthoritativeAspectStateDenial::ContractValidationDenied {
            source_locator,
            denial,
        } => format!(
            "relation field '{}' failed contract validation for aspect {}: {:?}",
            source_field_label(source_locator),
            source_locator_aspect_label(source_locator),
            denial
        ),
        RelationAuthoritativeAspectStateDenial::PatchConstructionDenied { denial } => format!(
            "relation aspect admission could not construct foundational patch: {:?}",
            denial
        ),
        RelationAuthoritativeAspectStateDenial::PatchApplicationDenied { denial } => format!(
            "relation endpoint aspect update could not apply foundational patch: {:?}",
            denial
        ),
        RelationAuthoritativeAspectStateDenial::UnsupportedAspectFieldTarget { target, reason } => {
            format!(
                "relation field '{}' / {} is not a supported relation aspect target: {}",
                aspect_field_locator_field_label(target),
                target.aspect().aspect_key().as_str(),
                reason.label()
            )
        }
        RelationAuthoritativeAspectStateDenial::StructValueConstructionDenied {
            source_locator,
        } => format!(
            "relation field set for {} at '{}' cannot construct the declared struct aspect value",
            source_locator_aspect_label(source_locator),
            source_field_label(source_locator)
        ),
        RelationAuthoritativeAspectStateDenial::StructContractValidationDenied {
            source_locator,
            denial,
        } => format!(
            "relation struct for {} at '{}' failed contract validation: {:?}",
            source_locator_aspect_label(source_locator),
            source_field_label(source_locator),
            denial
        ),
        RelationAuthoritativeAspectStateDenial::StateAdmissionDenied { denial } => format!(
            "relation aspects could not be admitted as authoritative state: {:?}",
            denial
        ),
    }
}

fn source_field_label(source_locator: &BoundarySourceLocator) -> String {
    source_locator_field_label(source_locator).unwrap_or_else(|| "whole_aspect".to_string())
}
