use forge_store_physical_integrity::QuarantineRecord;
use forge_store_recovery_physics::RecoveryLayoutReadmissionIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutQuarantineWitness {
    family: crate::PhysicalArtifactFamily,
    record: Option<QuarantineRecord>,
    coverage: Option<crate::S8LayoutCoverageWitness>,
}

impl S8LayoutQuarantineWitness {
    pub(crate) fn new(family: crate::PhysicalArtifactFamily, record: QuarantineRecord) -> Self {
        Self {
            family,
            record: Some(record),
            coverage: None,
        }
    }

    pub(crate) const fn from_materialization(coverage: crate::S8LayoutCoverageWitness) -> Self {
        Self {
            family: coverage.family(),
            record: None,
            coverage: Some(coverage),
        }
    }

    pub(crate) const fn for_authoritative_family(family: crate::PhysicalArtifactFamily) -> Self {
        Self {
            family,
            record: None,
            coverage: None,
        }
    }

    pub const fn family(&self) -> crate::PhysicalArtifactFamily {
        self.family
    }

    pub const fn record(&self) -> Option<&QuarantineRecord> {
        self.record.as_ref()
    }

    pub const fn coverage(&self) -> Option<crate::S8LayoutCoverageWitness> {
        self.coverage
    }

    pub fn readmission_identity(&self) -> Option<RecoveryLayoutReadmissionIdentity> {
        self.record.as_ref().map(|record| {
            RecoveryLayoutReadmissionIdentity::QuarantineReceipt(
                record.receipt().foundational_basis().digest().clone(),
            )
        })
    }
}
