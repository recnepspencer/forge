use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::mapping::{AspectKeySelector, MappingSelector, TruthPatchTargetSelector};

use super::super::registration::BridgeAspectRegistration;
use super::super::types::SliceWideningPolicy;

pub(crate) fn validate_registration_values(
    registrations: &[BridgeAspectRegistration],
) -> Result<(), BridgeBuildError> {
    for registration in registrations {
        validate_non_empty(
            "aspect registration id",
            registration.registration_id().as_str(),
        )?;
        validate_entity_selector(
            "aspect registration entity selector",
            registration.truth_scope().entity_selector(),
        )?;
        validate_aspect_selector(
            "aspect registration aspect selector",
            registration.truth_scope().aspect_selector(),
        )?;
        validate_read_contract_scope(registration)?;
        validate_target_selector(registration.truth_scope().target_selector())?;

        let scope = registration.truth_scope();
        if matches!(scope.entity_selector(), MappingSelector::Any)
            && matches!(scope.aspect_selector(), AspectKeySelector::Any)
            && matches!(scope.target_selector(), TruthPatchTargetSelector::Any)
        {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::MissingMappingRegistrations,
                "aspect registration scope cannot be fully wildcarded.",
            ));
        }

        match registration.widening_policy() {
            SliceWideningPolicy::Disallow => {}
            SliceWideningPolicy::RegisteredEntityCoarseWidening => {
                if *registration.subscription_slice_kind()
                    != crate::mapping::SubscriptionSliceKind::RegisteredCoarseWidening
                {
                    return Err(BridgeBuildError::new(
                        BridgeBuildErrorKind::InvalidFineGrainedWideningPolicy,
                        format!(
                            "Aspect registration `{}` uses entity coarse widening without targeting the registered coarse widening slice kind.",
                            registration.registration_id().as_str()
                        ),
                    ));
                }
            }
            SliceWideningPolicy::RegisteredPartitionWidening => {
                if *registration.subscription_slice_kind()
                    != crate::mapping::SubscriptionSliceKind::SignalPartition
                {
                    return Err(BridgeBuildError::new(
                        BridgeBuildErrorKind::InvalidFineGrainedWideningPolicy,
                        format!(
                            "Aspect registration `{}` uses partition widening without targeting the signal partition slice kind.",
                            registration.registration_id().as_str()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_read_contract_scope(
    registration: &BridgeAspectRegistration,
) -> Result<(), BridgeBuildError> {
    match registration.truth_scope().aspect_selector() {
        AspectKeySelector::Exact(aspect_key)
            if aspect_key == registration.snapshot_read_contract().aspect_key() =>
        {
            Ok(())
        }
        AspectKeySelector::Exact(aspect_key) => Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingMappingRegistrations,
            format!(
                "Aspect registration `{}` declares read contract `{}` for truth aspect `{}`.",
                registration.registration_id().as_str(),
                registration.snapshot_read_contract().aspect_key().as_str(),
                aspect_key.as_str()
            ),
        )),
        AspectKeySelector::Any => Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingMappingRegistrations,
            format!(
                "Aspect registration `{}` must declare an exact truth aspect for its snapshot read contract.",
                registration.registration_id().as_str()
            ),
        )),
    }
}

fn validate_entity_selector(
    label: &str,
    selector: &MappingSelector,
) -> Result<(), BridgeBuildError> {
    if matches!(selector, MappingSelector::Exact(value) if value.as_ref().is_empty()) {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingMappingRegistrations,
            format!("{label} cannot be empty."),
        ));
    }

    Ok(())
}

fn validate_aspect_selector(
    label: &str,
    selector: &AspectKeySelector,
) -> Result<(), BridgeBuildError> {
    if matches!(selector, AspectKeySelector::Exact(value) if value.as_str().is_empty()) {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingMappingRegistrations,
            format!("{label} cannot be empty."),
        ));
    }

    Ok(())
}

fn validate_target_selector(selector: &TruthPatchTargetSelector) -> Result<(), BridgeBuildError> {
    if matches!(selector, TruthPatchTargetSelector::EntityField(path) if path.fields().is_empty()) {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingMappingRegistrations,
            "aspect registration field target selector cannot be empty.",
        ));
    }

    Ok(())
}

fn validate_non_empty(label: &str, value: &str) -> Result<(), BridgeBuildError> {
    if value.is_empty() {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingMappingRegistrations,
            format!("{label} cannot be empty."),
        ));
    }

    Ok(())
}
