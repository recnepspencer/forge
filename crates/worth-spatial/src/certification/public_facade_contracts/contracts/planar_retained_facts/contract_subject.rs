use worth_spatial::facade::planar_contract_bundle::{
    PlanarContractBundleValidationContracts, PlanarContractBundleValidationReceipt,
    PlanarContractBundleValidator,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarMotionPostureReceipt, PlanarReorientation,
};
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFacts, RetainedPlanarFactsContracts, RetainedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts, PlanarStructuralIdentityReceipt,
};

use crate::public_api_planar_contract_bundle::complete_bundle::complete_bundle;
use crate::public_api_planar_contract_bundle::proof_fixture::{
    complete_bundle_parts, CompleteBundleParts, NEIGHBORHOOD,
};
use crate::public_api_planar_contract_bundle::runtime_handles::bundle_handle;

use super::runtime_handles::{
    motion_posture_handle, retained_planar_handle, structural_identity_handle,
};

pub(crate) struct RetainedPlanarParts {
    pub(crate) readiness: PlanarContractBundleValidationReceipt,
    pub(crate) motion: PlanarMotionPostureReceipt,
    pub(crate) structural: PlanarStructuralIdentityReceipt,
    pub(crate) bundle_parts: CompleteBundleParts,
}

pub(crate) fn retained_planar_parts(world: &'static str) -> RetainedPlanarParts {
    let bundle_parts = complete_bundle_parts(world);
    let readiness =
        PlanarContractBundleValidator::for_boolean_readiness(complete_bundle(&bundle_parts))
            .within_planar_neighborhood(NEIGHBORHOOD)
            .compile(&PlanarContractBundleValidationContracts::new(
                bundle_handle(world),
            ))
            .expect("retained planar bundle plan")
            .certify()
            .expect("retained planar bundle receipt");
    let motion = PlanarMotionPosture::from_boolean_readiness(readiness.clone())
        .after_exact_translation("motion:retained-translate")
        .after_exact_rotation("motion:retained-quarter-turn")
        .after_exact_rotation("motion:retained-quarter-turn-inverse")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
            world,
        )))
        .expect("retained planar motion plan")
        .certify()
        .expect("retained planar motion receipt");
    let structural = PlanarStructuralIdentity::from_boolean_readiness(readiness.clone())
        .with_motion_posture(motion.clone())
        .with_topology_identity("topology:retained-planar")
        .with_persistent_name("name:retained-planar")
        .with_binding_identity("binding:retained-planar")
        .with_lineage_identity("lineage:retained-planar")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(world),
        ))
        .expect("retained planar structural plan")
        .certify()
        .expect("retained planar structural receipt");
    RetainedPlanarParts {
        readiness,
        motion,
        structural,
        bundle_parts,
    }
}

pub(crate) fn retained_planar_receipt(world: &'static str) -> RetainedPlanarFactsReceipt {
    let parts = retained_planar_parts(world);
    RetainedPlanarFacts::from_boolean_readiness(parts.readiness)
        .retain_planar_classification()
        .retain_structural_identity(parts.structural)
        .retain_motion_posture(parts.motion)
        .retain_topology_contract(parts.bundle_parts.topology_contract)
        .compile(&RetainedPlanarFactsContracts::new(retained_planar_handle(
            world,
        )))
        .expect("retained planar facts plan")
        .retain()
        .expect("retained planar fact receipt")
}
