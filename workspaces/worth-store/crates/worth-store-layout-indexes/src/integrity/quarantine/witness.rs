use crate::PhysicalArtifactFamily;
use worth_store_physical_integrity::QuarantineRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutQuarantineWitness {
    basis: LayoutQuarantineBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LayoutQuarantineBasis {
    RecordBacked {
        family: crate::AdmittedPhysicalArtifactFamily,
        record: Box<QuarantineRecord>,
    },
}

impl LayoutQuarantineWitness {
    pub(in crate::integrity) fn from_record(
        family: crate::AdmittedPhysicalArtifactFamily,
        record: QuarantineRecord,
    ) -> Self {
        Self {
            basis: LayoutQuarantineBasis::RecordBacked {
                family,
                record: Box::new(record),
            },
        }
    }

    pub const fn family(&self) -> PhysicalArtifactFamily {
        match &self.basis {
            LayoutQuarantineBasis::RecordBacked { family, .. } => {
                family.lifecycle().declaration().family()
            }
        }
    }

    pub const fn record(&self) -> &QuarantineRecord {
        match &self.basis {
            LayoutQuarantineBasis::RecordBacked { record, .. } => record,
        }
    }

    pub const fn admitted_family(&self) -> crate::AdmittedPhysicalArtifactFamily {
        match &self.basis {
            LayoutQuarantineBasis::RecordBacked { family, .. } => *family,
        }
    }

    pub fn readmission_identity(
        &self,
    ) -> worth_store_recovery_physics::RecoveryLayoutReadmissionIdentity {
        worth_store_recovery_physics::RecoveryLayoutReadmissionIdentity::QuarantineReceipt(
            self.record()
                .receipt()
                .foundational_basis()
                .digest()
                .clone(),
        )
    }
}
