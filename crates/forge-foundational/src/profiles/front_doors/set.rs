use super::super::{
    request_foundational_profile_set, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile, FoundationalProfileSet,
    FoundationalProfileSetInput, RequestedFoundationalProfileArtifact, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use super::vocabulary::{
    FoundationalProfileFrontDoorConstructionDenial, FoundationalProfileFrontDoorFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalProfileSetFrontDoor {
    diagnostic_richness: Option<DiagnosticRichnessProfile>,
    support_posture: Option<SupportPostureProfile>,
    compatibility_posture: Option<CompatibilityPostureProfile>,
    admission_readiness: Option<AdmissionReadinessProfile>,
    retention_delivery: Option<RetentionDeliveryProfile>,
    certification_posture: Option<CertificationPostureProfile>,
    duplicate_family: Option<FoundationalProfileFrontDoorFamily>,
}

impl FoundationalProfileSetFrontDoor {
    pub const fn new() -> Self {
        Self {
            diagnostic_richness: None,
            support_posture: None,
            compatibility_posture: None,
            admission_readiness: None,
            retention_delivery: None,
            certification_posture: None,
            duplicate_family: None,
        }
    }

    pub fn diagnostic_richness(mut self, profile: DiagnosticRichnessProfile) -> Self {
        let duplicate = self.diagnostic_richness.replace(profile).is_some();
        self.record_assignment(
            FoundationalProfileFrontDoorFamily::DiagnosticRichness,
            duplicate,
        );
        self
    }

    pub fn support_posture(mut self, profile: SupportPostureProfile) -> Self {
        let duplicate = self.support_posture.replace(profile).is_some();
        self.record_assignment(
            FoundationalProfileFrontDoorFamily::SupportPosture,
            duplicate,
        );
        self
    }

    pub fn compatibility_posture(mut self, profile: CompatibilityPostureProfile) -> Self {
        let duplicate = self.compatibility_posture.replace(profile).is_some();
        self.record_assignment(
            FoundationalProfileFrontDoorFamily::CompatibilityPosture,
            duplicate,
        );
        self
    }

    pub fn admission_readiness(mut self, profile: AdmissionReadinessProfile) -> Self {
        let duplicate = self.admission_readiness.replace(profile).is_some();
        self.record_assignment(
            FoundationalProfileFrontDoorFamily::AdmissionReadiness,
            duplicate,
        );
        self
    }

    pub fn retention_delivery(mut self, profile: RetentionDeliveryProfile) -> Self {
        let duplicate = self.retention_delivery.replace(profile).is_some();
        self.record_assignment(
            FoundationalProfileFrontDoorFamily::RetentionDelivery,
            duplicate,
        );
        self
    }

    pub fn certification_posture(mut self, profile: CertificationPostureProfile) -> Self {
        let duplicate = self.certification_posture.replace(profile).is_some();
        self.record_assignment(
            FoundationalProfileFrontDoorFamily::CertificationPosture,
            duplicate,
        );
        self
    }

    pub fn compose(
        self,
    ) -> Result<FoundationalProfileSet, FoundationalProfileFrontDoorConstructionDenial> {
        if let Some(family) = self.duplicate_family {
            return Err(
                FoundationalProfileFrontDoorConstructionDenial::DuplicateFamilyAssignment(family),
            );
        }

        FoundationalProfileSet::new(FoundationalProfileSetInput {
            diagnostic_richness: Self::require(
                self.diagnostic_richness,
                FoundationalProfileFrontDoorFamily::DiagnosticRichness,
            )?,
            support_posture: Self::require(
                self.support_posture,
                FoundationalProfileFrontDoorFamily::SupportPosture,
            )?,
            compatibility_posture: Self::require(
                self.compatibility_posture,
                FoundationalProfileFrontDoorFamily::CompatibilityPosture,
            )?,
            admission_readiness: Self::require(
                self.admission_readiness,
                FoundationalProfileFrontDoorFamily::AdmissionReadiness,
            )?,
            retention_delivery: Self::require(
                self.retention_delivery,
                FoundationalProfileFrontDoorFamily::RetentionDelivery,
            )?,
            certification_posture: Self::require(
                self.certification_posture,
                FoundationalProfileFrontDoorFamily::CertificationPosture,
            )?,
        })
        .map_err(FoundationalProfileFrontDoorConstructionDenial::IllegalComposition)
    }

    pub fn request(
        self,
    ) -> Result<RequestedFoundationalProfileArtifact, FoundationalProfileFrontDoorConstructionDenial>
    {
        self.compose().map(request_foundational_profile_set)
    }

    fn record_assignment(&mut self, family: FoundationalProfileFrontDoorFamily, duplicate: bool) {
        if duplicate && self.duplicate_family.is_none() {
            self.duplicate_family = Some(family);
        }
    }

    fn require<T>(
        value: Option<T>,
        family: FoundationalProfileFrontDoorFamily,
    ) -> Result<T, FoundationalProfileFrontDoorConstructionDenial> {
        value.ok_or(FoundationalProfileFrontDoorConstructionDenial::MissingFamily(family))
    }
}
