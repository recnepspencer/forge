use worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt;
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarMotionPostureReceipt, PlanarReorientation,
};

use crate::public_api_planar_contract_bundle::complete_bundle::complete_bundle;
use crate::public_api_planar_contract_bundle::proof_fixture::{
    complete_bundle_parts, NEIGHBORHOOD,
};
use crate::public_api_planar_contract_bundle::runtime_handles::bundle_handle;

use super::runtime_handles::motion_posture_handle;

pub(crate) fn boolean_readiness_receipt(
    world: &'static str,
) -> PlanarContractBundleValidationReceipt {
    let parts = complete_bundle_parts(world);
    let bundle_contracts =
        worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidationContracts::new(
            bundle_handle(world),
        );
    worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidator::for_boolean_readiness(
        complete_bundle(&parts),
    )
    .within_planar_neighborhood(NEIGHBORHOOD)
    .compile(&bundle_contracts)
    .expect("bundle plan")
    .certify()
    .expect("bundle receipt")
}

pub(crate) fn cancellation_motion_receipt(world: &'static str) -> PlanarMotionPostureReceipt {
    PlanarMotionPosture::from_boolean_readiness(boolean_readiness_receipt(world))
        .after_exact_translation("motion:translate-out")
        .after_exact_rotation("motion:quarter-turn")
        .after_exact_rotation("motion:quarter-turn-inverse")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
            world,
        )))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt")
}
