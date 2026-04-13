use super::*;

pub(super) fn validate_registration_set(
    registrations: &[BridgeMappingRegistration],
) -> Result<(), BridgeBuildError> {
    for (index, left) in registrations.iter().enumerate() {
        for right in registrations.iter().skip(index + 1) {
            if left.mapping_id() == right.mapping_id() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateMappingRegistration,
                    format!(
                        "Duplicate bridge mapping id `{}` detected across multiple registrations.",
                        left.mapping_id().as_str()
                    ),
                ));
            }

            if !left.truth_scope().overlaps(right.truth_scope()) {
                continue;
            }

            if left.semantic_duplicate_of(right) {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::DuplicateMappingRegistration,
                    format!(
                        "Duplicate bridge mapping registration detected for truth scope between `{}` and `{}`.",
                        left.mapping_id().as_str(),
                        right.mapping_id().as_str()
                    ),
                ));
            }

            if left.truth_scope() == right.truth_scope() {
                continue;
            }

            if left.truth_scope().specificity_rank() == right.truth_scope().specificity_rank() {
                return Err(BridgeBuildError::new(
                    BridgeBuildErrorKind::AmbiguousMappingRegistration,
                    format!(
                        "Ambiguous bridge mapping registration overlap detected between `{}` and `{}`.",
                        left.mapping_id().as_str(),
                        right.mapping_id().as_str()
                    ),
                ));
            }
        }
    }

    Ok(())
}

pub(super) fn validate_registration_values(
    registrations: &[BridgeMappingRegistration],
) -> Result<(), BridgeBuildError> {
    for registration in registrations {
        validate_non_empty("mapping id", registration.mapping_id().as_str())?;
        validate_selector(
            "entity selector",
            registration.truth_scope().entity_selector(),
        )?;
        validate_selector(
            "aspect selector",
            registration.truth_scope().aspect_selector(),
        )?;
        validate_selector(
            "surface selector",
            registration.truth_scope().surface_selector(),
        )?;
        validate_non_empty("signal scope", registration.signal_scope().as_str())?;
    }

    Ok(())
}

fn validate_selector(label: &str, selector: &MappingSelector) -> Result<(), BridgeBuildError> {
    match selector {
        MappingSelector::Any => Ok(()),
        MappingSelector::Exact(value) => validate_non_empty(label, value),
    }
}

fn validate_non_empty(label: &str, value: &str) -> Result<(), BridgeBuildError> {
    if value.trim().is_empty() {
        return Err(BridgeBuildError::new(
            BridgeBuildErrorKind::AmbiguousMappingRegistration,
            format!("Bridge mapping {label} must be non-empty."),
        ));
    }

    Ok(())
}

impl crate::mapping::registration::TruthPatchScope {
    pub(crate) fn matches_key(&self, key: BridgeMappingLookupKey<'_>) -> bool {
        self.entity_selector().matches(key.entity_identity())
            && self.aspect_selector().matches(key.aspect_label())
            && self.surface_selector().matches(key.surface_label())
    }
}
