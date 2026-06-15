use worth_spatial::facade::planar_contract_bundle::{
    PlanarContractBundleValidationContracts, PlanarContractBundleValidationReceipt,
    PlanarContractBundleValidator,
};
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarMotionPostureReceipt, PlanarReorientation,
};
use worth_spatial::facade::planar_projection::ProjectPointToCertifiedPlane2DReceipt;
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFacts, RetainedPlanarFactsContracts, RetainedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts, PlanarStructuralIdentityReceipt,
};

use crate::public_api_planar_contract_bundle::complete_bundle::complete_bundle;
use crate::public_api_planar_contract_bundle::proof_fixture::{
    complete_bundle_parts, stray_projection_receipt, CompleteBundleParts, NEIGHBORHOOD,
};

use super::runtime_handles::{
    bundle_handle, motion_posture_handle, retained_planar_handle, structural_identity_handle,
};

pub(crate) struct ProjectionConsumedPlanarParts {
    pub(crate) readiness: PlanarContractBundleValidationReceipt,
    pub(crate) retained: RetainedPlanarFactsReceipt,
    pub(crate) projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    pub(crate) bundle_parts: CompleteBundleParts,
}

pub(crate) fn projection_consumed_planar_parts(
    world: &'static str,
) -> ProjectionConsumedPlanarParts {
    let bundle_parts = complete_bundle_parts(world);
    let readiness =
        PlanarContractBundleValidator::for_boolean_readiness(complete_bundle(&bundle_parts))
            .within_planar_neighborhood(NEIGHBORHOOD)
            .compile(&PlanarContractBundleValidationContracts::new(
                bundle_handle(world),
            ))
            .expect("projection-consumed bundle plan")
            .certify()
            .expect("projection-consumed bundle receipt");
    let motion = motion_receipt(world, readiness.clone());
    let structural = structural_receipt(world, readiness.clone(), motion.clone());
    let retained = RetainedPlanarFacts::from_boolean_readiness(readiness.clone())
        .retain_planar_classification()
        .retain_structural_identity(structural)
        .retain_motion_posture(motion)
        .retain_topology_contract(bundle_parts.topology_contract.clone())
        .compile(&RetainedPlanarFactsContracts::new(retained_planar_handle(
            world,
        )))
        .expect("retained planar facts plan")
        .retain()
        .expect("retained planar fact receipt");
    ProjectionConsumedPlanarParts {
        readiness,
        retained,
        projections: bundle_parts.projections.clone(),
        bundle_parts,
    }
}

pub(crate) fn stray_projection(
    world: &'static str,
    parts: &ProjectionConsumedPlanarParts,
) -> ProjectPointToCertifiedPlane2DReceipt {
    stray_projection_receipt(world, &parts.bundle_parts.frame)
}

fn motion_receipt(
    world: &'static str,
    readiness: PlanarContractBundleValidationReceipt,
) -> PlanarMotionPostureReceipt {
    PlanarMotionPosture::from_boolean_readiness(readiness)
        .after_exact_translation("motion:projection-consumed-translate")
        .after_exact_rotation("motion:projection-consumed-quarter-turn")
        .after_exact_rotation("motion:projection-consumed-quarter-turn-inverse")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle(
            world,
        )))
        .expect("projection-consumed motion plan")
        .certify()
        .expect("projection-consumed motion receipt")
}

fn structural_receipt(
    world: &'static str,
    readiness: PlanarContractBundleValidationReceipt,
    motion: PlanarMotionPostureReceipt,
) -> PlanarStructuralIdentityReceipt {
    PlanarStructuralIdentity::from_boolean_readiness(readiness)
        .with_motion_posture(motion)
        .with_topology_identity("topology:projection-consumed")
        .with_persistent_name("name:projection-consumed")
        .with_binding_identity("binding:projection-consumed")
        .with_lineage_identity("lineage:projection-consumed")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(world),
        ))
        .expect("projection-consumed structural plan")
        .certify()
        .expect("projection-consumed structural receipt")
}
