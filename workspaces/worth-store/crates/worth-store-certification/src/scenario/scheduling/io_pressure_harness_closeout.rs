use worth_store_physical_certification::{
    IoPressureBackendSafetyQualificationDenial, IoPressureEvidenceMaturity,
    IoPressureHarnessEvidence, PhysicalFaultEvidenceClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoPressureHarnessCloseoutDenial {
    SimulatedBackendSafetyClaim(IoPressureBackendSafetyQualificationDenial),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoPressureHarnessCloseoutEvidence {
    harness_evidence: IoPressureHarnessEvidence,
    real_backend_safety: Option<PhysicalFaultEvidenceClass>,
}

impl IoPressureHarnessCloseoutEvidence {
    pub fn from_harness_evidence(harness_evidence: IoPressureHarnessEvidence) -> Self {
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
        harness_evidence: IoPressureHarnessEvidence,
    ) -> Result<Self, IoPressureHarnessCloseoutDenial> {
        let qualification = harness_evidence
            .require_real_backend_safety()
            .map_err(IoPressureHarnessCloseoutDenial::SimulatedBackendSafetyClaim)?;
        Ok(Self {
            harness_evidence,
            real_backend_safety: Some(qualification.evidence_class()),
        })
    }

    pub const fn harness_evidence(&self) -> &IoPressureHarnessEvidence {
        &self.harness_evidence
    }

    pub const fn real_backend_safety(&self) -> Option<PhysicalFaultEvidenceClass> {
        self.real_backend_safety
    }

    pub const fn evidence_maturity(&self) -> IoPressureEvidenceMaturity {
        self.harness_evidence.maturity()
    }
}
