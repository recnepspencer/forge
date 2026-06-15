use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneOperandSide, PlanarBooleanCommonPlanePostureAgreementDenial,
    PlanarBooleanCommonPlanePostureWitness,
};
use crate::workload_platform::transform_workload::TransformReceiptSet;

pub struct PlanarBooleanCommonPlanePostureAgreementWorkload {
    left_transform_receipts: TransformReceiptSet,
    right_transform_receipts: TransformReceiptSet,
    declaration: String,
}

impl PlanarBooleanCommonPlanePostureAgreementWorkload {
    pub fn for_transform_receipt_pair(
        left_transform_receipts: TransformReceiptSet,
        right_transform_receipts: TransformReceiptSet,
    ) -> Self {
        Self {
            left_transform_receipts,
            right_transform_receipts,
            declaration: "planar boolean common-plane posture agreement".to_string(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn certify(
        self,
    ) -> Result<
        crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementReceipt,
        PlanarBooleanCommonPlanePostureAgreementDenial,
    >{
        require_declaration(&self.declaration)?;

        let left_witness = certify_operand_posture_witness(
            &self.left_transform_receipts,
            PlanarBooleanCommonPlaneOperandSide::Left,
        )?;
        let right_witness = certify_operand_posture_witness(
            &self.right_transform_receipts,
            PlanarBooleanCommonPlaneOperandSide::Right,
        )?;

        require_matching_posture_identity(&left_witness, &right_witness)?;

        Ok(
            crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementReceipt::new(
                &self.declaration,
                left_witness,
                right_witness,
            ),
        )
    }
}

fn require_declaration(
    declaration: &str,
) -> Result<(), PlanarBooleanCommonPlanePostureAgreementDenial> {
    if declaration.trim().is_empty() {
        Err(PlanarBooleanCommonPlanePostureAgreementDenial::MissingDeclaration)
    } else {
        Ok(())
    }
}

fn certify_operand_posture_witness(
    transform_receipts: &TransformReceiptSet,
    side: PlanarBooleanCommonPlaneOperandSide,
) -> Result<PlanarBooleanCommonPlanePostureWitness, PlanarBooleanCommonPlanePostureAgreementDenial>
{
    let posture_receipt = transform_receipts.transform_posture_receipt();
    if posture_receipt.posture_identity().trim().is_empty() {
        return Err(
            PlanarBooleanCommonPlanePostureAgreementDenial::MissingMovementRotationPostureWitness {
                side,
                projected_workload_identity: transform_receipts
                    .projected_workload_identity()
                    .to_string(),
                transform_stage_identity: transform_receipts
                    .stage_identity()
                    .receipt_identity()
                    .to_string(),
            },
        );
    }

    Ok(PlanarBooleanCommonPlanePostureWitness::new(
        posture_receipt.posture_identity(),
        semantic_posture_identity(transform_receipts),
        transform_receipts.projected_workload_identity(),
        transform_receipts.stage_identity().receipt_identity(),
    ))
}

fn require_matching_posture_identity(
    left_witness: &PlanarBooleanCommonPlanePostureWitness,
    right_witness: &PlanarBooleanCommonPlanePostureWitness,
) -> Result<(), PlanarBooleanCommonPlanePostureAgreementDenial> {
    if left_witness.semantic_posture_identity() == right_witness.semantic_posture_identity() {
        Ok(())
    } else {
        Err(
            PlanarBooleanCommonPlanePostureAgreementDenial::DistinctMovementRotationPostures {
                left_projected_workload_identity: left_witness
                    .projected_workload_identity()
                    .to_string(),
                right_projected_workload_identity: right_witness
                    .projected_workload_identity()
                    .to_string(),
                left_transform_stage_identity: left_witness.transform_stage_identity().to_string(),
                right_transform_stage_identity: right_witness
                    .transform_stage_identity()
                    .to_string(),
                left_posture_identity: left_witness.posture_identity().to_string(),
                right_posture_identity: right_witness.posture_identity().to_string(),
            },
        )
    }
}

fn semantic_posture_identity(transform_receipts: &TransformReceiptSet) -> String {
    let posture_receipt = transform_receipts.transform_posture_receipt();
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-common-plane-semantic-posture".to_string(),
            format!("rotation:{}", posture_receipt.rotation_posture().as_str()),
            format!("reorientation:{}", posture_receipt.reorientation().as_str()),
            format!("cancellation:{}", posture_receipt.cancellation().as_str()),
        ],
    )
}
