use crate::error::{BridgeBuildError, BridgeBuildErrorKind};

use super::super::registration::BridgeAspectRegistration;
use super::super::types::{SliceFallbackPolicy, TruthDeltaSurfaceKind};

pub(crate) fn validate_registration_set(
    registrations: &[BridgeAspectRegistration],
) -> Result<(), BridgeBuildError> {
    for (index, left) in registrations.iter().enumerate() {
        for right in registrations.iter().skip(index + 1) {
            if left.registration_id() == right.registration_id() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateAspectRegistration,
                    format!(
                        "Aspect registration `{}` is registered more than once.",
                        left.registration_id().as_str()
                    ),
                ));
            }

            if left.truth_surface_kind() == right.truth_surface_kind()
                && left.truth_scope() == right.truth_scope()
                && left.semantic_duplicate_of(right)
            {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateAspectRegistration,
                    format!(
                        "Aspect registrations `{}` and `{}` are semantically identical.",
                        left.registration_id().as_str(),
                        right.registration_id().as_str()
                    ),
                ));
            }

            if left.truth_surface_kind() == right.truth_surface_kind()
                && left.truth_scope() == right.truth_scope()
            {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousAspectRegistration,
                    format!(
                        "Aspect registrations `{}` and `{}` overlap with the same truth scope.",
                        left.registration_id().as_str(),
                        right.registration_id().as_str()
                    ),
                ));
            }

            if left.truth_surface_kind() == right.truth_surface_kind()
                && registration_rank_group(left) == registration_rank_group(right)
                && left.truth_scope().overlaps(right.truth_scope())
            {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousAspectRegistration,
                    format!(
                        "Aspect registrations `{}` and `{}` overlap within the same fallback rank.",
                        left.registration_id().as_str(),
                        right.registration_id().as_str()
                    ),
                ));
            }

            if left.truth_surface_kind() == right.truth_surface_kind()
                && left.truth_scope().overlaps(right.truth_scope())
                && left.truth_scope().specificity_rank() == right.truth_scope().specificity_rank()
                && left.truth_scope() != right.truth_scope()
            {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousAspectRegistration,
                    format!(
                        "Aspect registrations `{}` and `{}` overlap on `{}` without a single most-specific registration.",
                        left.registration_id().as_str(),
                        right.registration_id().as_str(),
                        format!("{:?}", left.truth_scope())
                    ),
                ));
            }
        }
    }

    Ok(())
}

pub(crate) fn registration_rank_group(
    registration: &BridgeAspectRegistration,
) -> (TruthDeltaSurfaceKind, SliceFallbackPolicy) {
    (registration.truth_surface_kind(), registration.fallback_policy())
}
