use crate::{
    FoundationalQuarantineReceiptBasis, QuarantineReceipt, QuarantineRecord, QuarantineSealDenial,
    QuarantineSealRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalQuarantineAuthority;

impl PhysicalQuarantineAuthority {
    pub fn seal(request: QuarantineSealRequest) -> Result<QuarantineRecord, QuarantineSealDenial> {
        let (finding, lifecycle_posture, handoff_posture) = request.into_checked_parts()?;
        let locality = finding.locality();
        let damage_classification = finding.damage_classification().clone();
        let receipt_basis = FoundationalQuarantineReceiptBasis::from_parts(
            locality,
            &damage_classification,
            lifecycle_posture,
        );
        Ok(QuarantineRecord::new(
            locality,
            damage_classification,
            QuarantineReceipt::new(receipt_basis),
            lifecycle_posture,
            handoff_posture,
        ))
    }
}
