use worth_store_physical_certification::{
    PhysicalFaultEvidenceClass, S6BackendSafetyQualificationDenial, S6IoPressureHarnessEvidence,
    S6PressureEvidenceMaturity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S6IoPressureHarnessCloseoutDenial {
    SimulatedBackendSafetyClaim(S6BackendSafetyQualificationDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6IoPressureHarnessCloseoutEvidence {
    harness_evidence: S6IoPressureHarnessEvidence,
    real_backend_safety: Option<PhysicalFaultEvidenceClass>,
}

impl S6IoPressureHarnessCloseoutEvidence {
    pub fn from_harness_evidence(harness_evidence: S6IoPressureHarnessEvidence) -> Self {
        let real_backend_safety = harness_evidence
            .require_real_backend_safety()
            .ok()
            .map(|qualification| qualification.evidence_class());
        Self {
            harness_evidence,
            real_backend_safety,
        }
    }

    pub fn require_real_backend_safety(
        harness_evidence: S6IoPressureHarnessEvidence,
    ) -> Result<Self, S6IoPressureHarnessCloseoutDenial> {
        let qualification = harness_evidence
            .require_real_backend_safety()
            .map_err(S6IoPressureHarnessCloseoutDenial::SimulatedBackendSafetyClaim)?;
        Ok(Self {
            harness_evidence,
            real_backend_safety: Some(qualification.evidence_class()),
        })
    }

    pub const fn harness_evidence(&self) -> &S6IoPressureHarnessEvidence {
        &self.harness_evidence
    }

    pub const fn real_backend_safety(&self) -> Option<PhysicalFaultEvidenceClass> {
        self.real_backend_safety
    }

    pub const fn evidence_maturity(&self) -> S6PressureEvidenceMaturity {
        self.harness_evidence.maturity()
    }
}
