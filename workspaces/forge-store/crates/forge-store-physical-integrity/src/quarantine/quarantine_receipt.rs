use crate::{DamageClassification, PhysicalLocalityReport, QuarantineLifecyclePosture};
use forge_foundational::boundary_evidence_api::lower_lane::receipts::FoundationalBoundaryEvidenceReceiptKind;
use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalQuarantineReceiptBasis {
    receipt_kind: FoundationalBoundaryEvidenceReceiptKind,
    digest: StableDigest,
}

impl FoundationalQuarantineReceiptBasis {
    pub(crate) fn from_parts(
        locality: PhysicalLocalityReport,
        damage_classification: &DamageClassification,
        posture: QuarantineLifecyclePosture,
    ) -> Self {
        let digest = StableDigest::new(format!(
            "new-quarantine:{:?}:{:?}:{:?}",
            locality, damage_classification, posture
        ))
        .expect("quarantine receipt basis is non-empty");
        Self {
            receipt_kind: FoundationalBoundaryEvidenceReceiptKind::Execution,
            digest,
        }
    }

    pub const fn receipt_kind(&self) -> FoundationalBoundaryEvidenceReceiptKind {
        self.receipt_kind
    }

    pub fn digest(&self) -> &StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuarantineReceipt {
    basis: FoundationalQuarantineReceiptBasis,
}

impl QuarantineReceipt {
    pub(crate) const fn new(basis: FoundationalQuarantineReceiptBasis) -> Self {
        Self { basis }
    }

    pub const fn foundational_basis(&self) -> &FoundationalQuarantineReceiptBasis {
        &self.basis
    }
}
