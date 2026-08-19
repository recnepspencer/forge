use super::FoundationalProfileSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileCompatibilityClass {
    Exact,
    RichnessOnlyChange,
    RetentionOnlyNarrowing,
    SupportPostureChange,
    CertificationPostureChange,
    ExecutionObjectiveChange,
    ObservationActivationChange,
    Incompatible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileDifferenceReport {
    compatibility_class: FoundationalProfileCompatibilityClass,
}

impl FoundationalProfileDifferenceReport {
    pub const fn compatibility_class(&self) -> FoundationalProfileCompatibilityClass {
        self.compatibility_class
    }
}

pub fn compare_foundational_profiles(
    left: FoundationalProfileSet,
    right: FoundationalProfileSet,
) -> FoundationalProfileDifferenceReport {
    FoundationalProfileDifferenceReport {
        compatibility_class: classify_profile_compatibility(left, right),
    }
}

fn classify_profile_compatibility(
    left: FoundationalProfileSet,
    right: FoundationalProfileSet,
) -> FoundationalProfileCompatibilityClass {
    if left == right {
        return FoundationalProfileCompatibilityClass::Exact;
    }

    let mut changed_family = None;

    if record_difference(
        left.diagnostic_richness(),
        right.diagnostic_richness(),
        FoundationalProfileCompatibilityClass::RichnessOnlyChange,
        &mut changed_family,
    ) {
        return FoundationalProfileCompatibilityClass::Incompatible;
    }
    if record_difference(
        left.retention_delivery(),
        right.retention_delivery(),
        FoundationalProfileCompatibilityClass::RetentionOnlyNarrowing,
        &mut changed_family,
    ) {
        return FoundationalProfileCompatibilityClass::Incompatible;
    }
    if record_difference(
        left.support_posture(),
        right.support_posture(),
        FoundationalProfileCompatibilityClass::SupportPostureChange,
        &mut changed_family,
    ) {
        return FoundationalProfileCompatibilityClass::Incompatible;
    }
    if record_difference(
        left.certification_posture(),
        right.certification_posture(),
        FoundationalProfileCompatibilityClass::CertificationPostureChange,
        &mut changed_family,
    ) {
        return FoundationalProfileCompatibilityClass::Incompatible;
    }
    if record_difference(
        left.execution_objective(),
        right.execution_objective(),
        FoundationalProfileCompatibilityClass::ExecutionObjectiveChange,
        &mut changed_family,
    ) {
        return FoundationalProfileCompatibilityClass::Incompatible;
    }
    if record_difference(
        left.observation_activation(),
        right.observation_activation(),
        FoundationalProfileCompatibilityClass::ObservationActivationChange,
        &mut changed_family,
    ) {
        return FoundationalProfileCompatibilityClass::Incompatible;
    }

    if left.compatibility_posture() != right.compatibility_posture()
        || left.admission_readiness() != right.admission_readiness()
    {
        return FoundationalProfileCompatibilityClass::Incompatible;
    }

    changed_family.unwrap_or(FoundationalProfileCompatibilityClass::Incompatible)
}

fn record_difference<T>(
    left: T,
    right: T,
    class: FoundationalProfileCompatibilityClass,
    changed_family: &mut Option<FoundationalProfileCompatibilityClass>,
) -> bool
where
    T: Copy + PartialEq,
{
    if left == right {
        return false;
    }

    changed_family.replace(class).is_some()
}
