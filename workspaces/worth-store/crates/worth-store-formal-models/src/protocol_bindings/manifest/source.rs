use std::any::type_name;

use super::super::{OwnerCrashSurvivalPosture, OwnerEvidenceClass};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerSourcePolymorphism {
    ExactType,
    AcrossBackendDurabilityProfiles,
    AcrossOwnerScopeTypes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnerOutcomeSource {
    rust_type: &'static str,
    evidence_class: OwnerEvidenceClass,
    polymorphism: OwnerSourcePolymorphism,
}

impl OwnerOutcomeSource {
    pub(super) fn of<T>(evidence_class: OwnerEvidenceClass) -> Self {
        Self {
            rust_type: type_name::<T>(),
            evidence_class,
            polymorphism: OwnerSourcePolymorphism::ExactType,
        }
    }

    pub(super) fn polymorphic<T>(
        evidence_class: OwnerEvidenceClass,
        polymorphism: OwnerSourcePolymorphism,
    ) -> Self {
        Self {
            rust_type: type_name::<T>(),
            evidence_class,
            polymorphism,
        }
    }

    pub const fn rust_type(self) -> &'static str {
        self.rust_type
    }

    pub const fn evidence_class(self) -> OwnerEvidenceClass {
        self.evidence_class
    }

    pub const fn polymorphism(self) -> OwnerSourcePolymorphism {
        self.polymorphism
    }

    pub const fn crash_survival_posture(self) -> OwnerCrashSurvivalPosture {
        self.evidence_class.crash_survival_posture()
    }
}
