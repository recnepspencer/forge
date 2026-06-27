mod inspection_receipt;

pub use inspection_receipt::UiInspectionReceipt;

use crate::{
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionQuery,
};

pub fn phase3_unsupported_receipt(query: UiInspectionQuery) -> UiInspectionReceipt {
    UiInspectionReceipt::new(
        query,
        UiInspectionPosture::Unsupported {
            expected_in: UiInspectionMilestoneExpectation::Milestone31,
        },
    )
}
