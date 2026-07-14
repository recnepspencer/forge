use crate::{
    FoundationalQuarantineReceiptBasis, QuarantineReceipt, QuarantineRecord, QuarantineSealOutcome,
    QuarantineSealRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalQuarantineAuthority;

impl PhysicalQuarantineAuthority {
    pub fn seal(request: QuarantineSealRequest) -> QuarantineSealOutcome {
        let (finding, lifecycle_posture, handoff_posture) = match request.into_checked_parts() {
            Ok(parts) => parts,
            Err(denial) => return QuarantineSealOutcome::denied(denial),
        };
        let locality = finding.locality();
        let damage_classification = finding.damage_classification().clone();
        let receipt_basis = FoundationalQuarantineReceiptBasis::from_parts(
            locality,
            &damage_classification,
            lifecycle_posture,
        );
        QuarantineSealOutcome::sealed(QuarantineRecord::new(
            locality,
            damage_classification,
            QuarantineReceipt::new(receipt_basis),
            lifecycle_posture,
            handoff_posture,
        ))
    }
}
