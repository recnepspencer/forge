//! Estate aftermath declarations through the generic Query aftermath contract.
//!
//! Published law-14 posture names are derived at installation. This module does
//! not declare `Reversible`, `Compensatable`, `Reconcilable`, or `Irreversible`
//! directly, and reads carry no aftermath contract.

use worth_query_decl::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract, DeclaredCompensation,
    DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand,
    DeclaredReconciliationProcedure, DeclaredRecordedInverse,
};

use super::EstateCapabilityOperation;

/// The one correlation family estate death notices escape through.
///
/// The application operation definition's external-effect slot is the sole
/// declaration of it. The aftermath contract deliberately cannot name a rail,
/// so an aftermath that disagreed with the lane the outbox correlates to is
/// unrepresentable rather than merely discouraged (Q8.25-C1).
pub const ESTATE_DEATH_NOTICE_RAIL: &str = "estate-death-notice-rail";

/// Declared aftermath for one estate capability operation.
///
/// `None` means the operation is not a mutation and carries no aftermath.
pub fn declared_aftermath_for(
    operation: EstateCapabilityOperation,
) -> Option<DeclaredApplicationAftermathContract> {
    match operation {
        EstateCapabilityOperation::ViewRestrictedEstate => None,
        EstateCapabilityOperation::NotifyDeath
        | EstateCapabilityOperation::RetransmitDeathNotice => Some(
            DeclaredApplicationAftermathContract::runtime_with_external_owner(
                DeclaredCorrectionMechanism::Compensation(
                    DeclaredCompensation::new(
                        "confirm-death-notice-with-authority",
                        DeclaredAftermathPostcondition::BusinessPostcondition {
                            identity: "death-notice-confirmed".into(),
                        },
                    )
                    .expect("estate compensation is well-formed"),
                ),
                DeclaredReconciliationProcedure::new("confirm-death-notice-with-authority")
                    .expect("estate reconciliation is well-formed"),
            ),
        ),
        EstateCapabilityOperation::FreezeAccount => Some(runtime_alone_inverse(
            "unfreeze-account",
            "estate-freeze-inverse",
            "Status",
        )),
        // Activation and emergency-request create lanes: the grant/access record
        // does not exist yet, so ExactPriorTruth recorded-inverse is not an
        // honest correction. Correction is a separate revoke/approve operation.
        EstateCapabilityOperation::DelegateCapability
        | EstateCapabilityOperation::RequestEmergencyAccess => {
            Some(DeclaredApplicationAftermathContract::not_correctable())
        }
        EstateCapabilityOperation::RevokeCapability => Some(runtime_alone_inverse(
            "restore-revoked-capability",
            "estate-revoke-inverse",
            "CapabilityGrantStatusField",
        )),
        EstateCapabilityOperation::ApproveEmergencyAccess => Some(runtime_alone_inverse(
            "revoke-emergency-access",
            "estate-emergency-inverse",
            "EmergencyAccessStatusField",
        )),
        EstateCapabilityOperation::DisburseEstate => {
            Some(DeclaredApplicationAftermathContract::runtime_alone(
                DeclaredCorrectionMechanism::Compensation(
                    DeclaredCompensation::new(
                        "compensating-estate-journal",
                        DeclaredAftermathPostcondition::BusinessPostcondition {
                            identity: "estate-journal-settled".into(),
                        },
                    )
                    .expect("estate compensation is well-formed"),
                ),
            ))
        }
        EstateCapabilityOperation::OpenEstateCase
        | EstateCapabilityOperation::RecognizeExecutor
        | EstateCapabilityOperation::RevokeEmergencyAccess
        | EstateCapabilityOperation::CompleteMandatoryReview
        | EstateCapabilityOperation::ReleaseEstate => {
            Some(DeclaredApplicationAftermathContract::not_correctable())
        }
    }
}

fn runtime_alone_inverse(
    inverse_operation: &str,
    lowering: &str,
    preimage_field: &str,
) -> DeclaredApplicationAftermathContract {
    DeclaredApplicationAftermathContract::runtime_alone(
        DeclaredCorrectionMechanism::RecordedInverse(
            DeclaredRecordedInverse::new(
                inverse_operation,
                DeclaredLoweringCorrespondenceRef::new(lowering)
                    .expect("estate lowering correspondence is well-formed"),
                DeclaredAftermathPostcondition::ExactPriorTruth,
                DeclaredPreImageDemand::new([preimage_field], 256)
                    .expect("estate pre-image demand is well-formed"),
            )
            .expect("estate recorded inverse is well-formed"),
        ),
    )
}
