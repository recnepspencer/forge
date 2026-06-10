use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarReorientation,
};
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsDenialKind, ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld,
};
use worth_spatial::facade::planar_retained_facts::{
    RetainedPlanarFacts, RetainedPlanarFactsContracts,
};
use worth_spatial::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts,
};

use super::super::bundle_closeout::contract_bundle::readiness_receipt;
use super::super::bundle_closeout::runtime_handles::{
    motion_posture_handle, retained_planar_handle, structural_identity_handle,
};

#[test]
fn kernel_consumes_projection_consumed_planar_facts_without_payload_spelunking() {
    let readiness = readiness_receipt();
    let retained = retained_planar_facts(readiness.clone());
    let contracts = ProjectionConsumedPlanarFactsContracts::new(projection_consumption_handle());

    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(readiness.basis().projection_receipts().to_vec())
        .compile(&contracts)
        .expect("projection-consumed planar facts plan")
        .consume()
        .expect("projection-consumed planar facts receipt");

    assert_eq!(
        projected.retained_planar_fact_digest(),
        retained.retained_fact_digest()
    );
    assert_eq!(
        projected.structural_identity_digest(),
        retained
            .basis()
            .structural_identity_receipt()
            .structural_identity_digest()
    );
    assert_eq!(
        projected.motion_posture_digest(),
        retained
            .basis()
            .motion_posture_receipt()
            .retained_motion_digest()
    );
    assert_eq!(
        projected.counters().projection_receipts_consumed(),
        readiness.basis().projection_receipts().len()
    );
}

#[test]
fn kernel_rejects_projection_consumed_planar_fact_summary_upgrade() {
    let readiness = readiness_receipt();
    let retained = retained_planar_facts(readiness);
    let contracts = ProjectionConsumedPlanarFactsContracts::new(projection_consumption_handle());

    let denial = match ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained)
        .consume_bundle_projection_receipts(Vec::new())
        .compile(&contracts)
    {
        Ok(_) => panic!("kernel summary-only projection consumption must deny"),
        Err(error) => error,
    };
    assert_eq!(
        denial.kind(),
        ProjectionConsumedPlanarFactsDenialKind::MissingProjectionReceipts
    );
}

pub(crate) fn retained_planar_facts(
    readiness: worth_spatial::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt,
) -> worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt {
    let topology_contract = readiness.basis().topology_contract_receipt().clone();
    let motion = PlanarMotionPosture::from_boolean_readiness(readiness.clone())
        .after_exact_translation("motion:kernel-projection-consumed-translate")
        .after_exact_rotation("motion:kernel-projection-consumed-rotation")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle()))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt");
    let structural = PlanarStructuralIdentity::from_boolean_readiness(readiness.clone())
        .with_motion_posture(motion.clone())
        .with_topology_identity("topology:kernel-projection-consumed")
        .with_persistent_name("name:kernel-projection-consumed")
        .with_binding_identity("binding:kernel-projection-consumed")
        .with_lineage_identity("lineage:kernel-projection-consumed")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt");
    RetainedPlanarFacts::from_boolean_readiness(readiness)
        .retain_planar_classification()
        .retain_structural_identity(structural)
        .retain_motion_posture(motion)
        .retain_topology_contract(topology_contract)
        .compile(&RetainedPlanarFactsContracts::new(retained_planar_handle()))
        .expect("retained planar facts plan")
        .retain()
        .expect("retained planar facts receipt")
}

pub(crate) fn projection_consumption_handle(
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ProjectionConsumedPlanarFactsQueryDomain)
        .with_operating_context(ProjectionConsumedPlanarFactsQueryWorld::new(
            "kernel-projection-consumed",
        ))
        .validate()
        .expect("validated projection consumption test domain")
        .admit()
        .expect("admitted projection consumption test domain")
}
