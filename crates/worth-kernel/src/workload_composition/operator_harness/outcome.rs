use worth_spatial::facade::planar_boolean_overlap_region_extraction::{
    CoplanarOverlapOperatorDenial, CoplanarOverlapOperatorReceipt,
};

use super::declaration::WorkloadOperatorFamily;
use super::receipt_set::OperatorReceiptSet;
use super::run::OperatorRun;
use super::support::OperatorWorkloadError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorOutcome {
    kind: OperatorOutcomeKind,
    receipts: OperatorReceiptSet,
}

impl OperatorOutcome {
    pub fn admitted(receipts: OperatorReceiptSet) -> Self {
        Self {
            kind: OperatorOutcomeKind::Admitted,
            receipts,
        }
    }

    pub fn from_coplanar_overlap_receipt(
        run: OperatorRun,
        receipt: CoplanarOverlapOperatorReceipt,
    ) -> Result<Self, OperatorWorkloadError> {
        if run.family() != WorkloadOperatorFamily::CoplanarOverlap {
            return Err(OperatorWorkloadError::WrongOperatorFamily {
                expected: WorkloadOperatorFamily::CoplanarOverlap,
                actual: run.family(),
            });
        }
        Ok(Self::admitted(
            OperatorReceiptSet::from_coplanar_overlap_receipt(&run, &receipt),
        ))
    }

    pub fn kind(&self) -> OperatorOutcomeKind {
        self.kind
    }

    pub fn receipts(&self) -> &OperatorReceiptSet {
        &self.receipts
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OperatorOutcomeKind {
    Admitted,
}

impl From<CoplanarOverlapOperatorDenial> for OperatorWorkloadError {
    fn from(denial: CoplanarOverlapOperatorDenial) -> Self {
        Self::SpatialOperatorDenied(denial.human_reason().to_string())
    }
}
