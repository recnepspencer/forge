use worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt;
use worth_spatial::facade::planar_structural_identity::{
    CanonicalPlanarTransformBasis, PlanarOrientationPolicy, PlanarStructuralIdentity,
    PlanarStructuralIdentityContracts, PlanarStructuralIdentityReceipt,
};

use crate::public_api_planar_contract_bundle::complete_bundle::complete_bundle;
use crate::public_api_planar_contract_bundle::proof_fixture::{
    complete_bundle_parts, MOVEMENT, NEIGHBORHOOD,
};
use crate::public_api_planar_contract_bundle::runtime_handles::bundle_handle;

use super::runtime_handles::structural_identity_handle;

pub(crate) fn structural_identity_receipt(
    world: &'static str,
    topology_identity: &'static str,
) -> PlanarStructuralIdentityReceipt {
    PlanarStructuralIdentity::from_boolean_readiness(boolean_readiness_receipt(world))
        .with_topology_identity(topology_identity)
        .with_persistent_name("name:stable")
        .with_binding_identity("binding:stable")
        .with_lineage_identity("lineage:stable")
        .with_canonical_transform_basis(bundle_transform_basis())
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(world),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt")
}

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

pub(crate) fn bundle_transform_basis() -> CanonicalPlanarTransformBasis {
    CanonicalPlanarTransformBasis::builder()
        .local_frame("frame:bundle")
        .movement_rotation_posture(MOVEMENT)
        .transform_chain_digest("transform:bundle")
        .orientation_policy(PlanarOrientationPolicy::Preserve)
        .build()
        .expect("canonical transform basis")
}
