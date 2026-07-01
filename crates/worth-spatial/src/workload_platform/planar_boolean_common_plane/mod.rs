mod local_frame_selection;
mod operand_projection_consumption;
mod operand_side;
mod plane_agreement;
mod posture_agreement;
mod precision_agreement;
mod reduced_operand_pair;
mod shared_plane_identity;

#[cfg(test)]
pub(crate) use local_frame_selection::{
    readiness_receipt as common_plane_readiness_receipt_for_tests,
    shared_plane_identity_receipt as common_plane_shared_plane_identity_receipt_for_tests,
};
pub use local_frame_selection::{
    PlanarBooleanCommonPlaneLocalFrameSelectionDenial,
    PlanarBooleanCommonPlaneLocalFrameSelectionDenialKind,
    PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
};
pub use operand_projection_consumption::{
    PlanarBooleanCommonPlaneOperandProjectionConsumptionDenial,
    PlanarBooleanCommonPlaneOperandProjectionConsumptionDenialKind,
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
};
pub use operand_side::PlanarBooleanCommonPlaneOperandSide;
pub use plane_agreement::{
    PlanarBooleanCommonPlaneAgreementDenial, PlanarBooleanCommonPlaneAgreementDenialKind,
    PlanarBooleanCommonPlaneAgreementReceipt, PlanarBooleanCommonPlaneAgreementWorkload,
    PlanarBooleanCommonPlaneWitness,
};
pub use posture_agreement::{
    PlanarBooleanCommonPlanePostureAgreementDenial,
    PlanarBooleanCommonPlanePostureAgreementDenialKind,
    PlanarBooleanCommonPlanePostureAgreementReceipt,
    PlanarBooleanCommonPlanePostureAgreementWorkload, PlanarBooleanCommonPlanePostureWitness,
};
pub use precision_agreement::PlanarBooleanCommonPlanePrecisionAgreementReceipt;
pub use reduced_operand_pair::{
    PlanarBooleanCommonPlaneReducedOperandPairDenial,
    PlanarBooleanCommonPlaneReducedOperandPairDenialKind,
    PlanarBooleanCommonPlaneReducedOperandPairOrderingContract,
    PlanarBooleanCommonPlaneReducedOperandPairReceipt,
};
pub use shared_plane_identity::PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt;
