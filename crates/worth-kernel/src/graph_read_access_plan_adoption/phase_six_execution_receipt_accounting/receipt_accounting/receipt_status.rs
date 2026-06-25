#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessReceiptStatus {
    ExecutedThroughQueryReceipt,
    AdmittedPlanRequiresExecutionReceipt,
    RequiredQueryPostureNoReceipt,
    DeniedByQueryPostureNoReceipt,
    CarriedCapabilityGapNoReceipt,
}

impl WorthGraphReadAccessReceiptStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutedThroughQueryReceipt => "executed_through_query_receipt",
            Self::AdmittedPlanRequiresExecutionReceipt => {
                "admitted_plan_requires_execution_receipt"
            }
            Self::RequiredQueryPostureNoReceipt => "required_query_posture_no_receipt",
            Self::DeniedByQueryPostureNoReceipt => "denied_by_query_posture_no_receipt",
            Self::CarriedCapabilityGapNoReceipt => "carried_capability_gap_no_receipt",
        }
    }

    pub const fn claims_query_receipt(self) -> bool {
        matches!(self, Self::ExecutedThroughQueryReceipt)
    }

    pub const fn requires_future_receipt(self) -> bool {
        matches!(self, Self::AdmittedPlanRequiresExecutionReceipt)
    }
}
