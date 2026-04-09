use crate::error::{BridgeBuildError, BridgeBuildErrorKind};
use crate::mapping::MappingSelector;

use super::super::registration::BridgeAspectRegistration;
use super::super::types::SliceFallbackPolicy;

pub(crate) fn validate_registration_values(
    registrations: &[BridgeAspectRegistration],
) -> Result<(), BridgeBuildError> {
    for registration in registrations {
        validate_non_empty(
            "aspect registration id",
            registration.registration_id().as_str(),
        )?;
        validate_selector(
            "aspect registration entity selector",
            registration.truth_scope().entity_selector(),
        )?;
        validate_selector(
            "aspect registration aspect selector",
            registration.truth_scope().aspect_selector(),
        )?;
        validate_selector(
            "aspect registration surface selector",
            registration.truth_scope().surface_selector(),
        )?;

        let scope = registration.truth_scope();
        if matches!(scope.entity_selector(), MappingSelector::Any)
            && matches!(scope.aspect_selector(), MappingSelector::Any)
            && matches!(scope.surface_selector(), MappingSelector::Any)
        {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::MissingMappingRegistrations,
                "aspect registration scope cannot be fully wildcarded.",
            ));
        }

        match registration.fallback_policy() {
            SliceFallbackPolicy::Disallow => {}
            SliceFallbackPolicy::RegisteredEntityCoarseFallback => {
                if *registration.subscription_slice_kind()
                    != crate::mapping::SubscriptionSliceKind::RegisteredCoarseFallback
                {
                    return Err(BridgeBuildError::new(
                        BridgeBuildErrorKind::InvalidFineGrainedFallbackPolicy,
                        format!(
                            "Aspect registration `{}` uses entity coarse fallback without targeting the registered coarse fallback slice kind.",
                            registration.registration_id().as_str()
                        ),
                    ));
                }
            }
            SliceFallbackPolicy::RegisteredPartitionFallback => {
                if *registration.subscription_slice_kind()
                    != crate::mapping::SubscriptionSliceKind::SignalPartition
                {
                    return Err(BridgeBuildError::new(
                        BridgeBuildErrorKind::InvalidFineGrainedFallbackPolicy,
                        format!(
                            "Aspect registration `{}` uses partition fallback without targeting the signal partition slice kind.",
                            registration.registration_id().as_str()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_selector(label: &str, selector: &MappingSelector) -> Result<(), BridgeBuildError> {
    if matches!(selector, MappingSelector::Exact(value) if value.as_ref().is_empty()) {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::MissingMappingRegistrations,
            format!("{label} cannot be empty."),
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
