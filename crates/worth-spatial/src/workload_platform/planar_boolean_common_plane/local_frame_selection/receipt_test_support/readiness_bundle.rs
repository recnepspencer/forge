use crate::facade::planar_contract_bundle::{
    PlanarBooleanReadinessBundle, PlanarContractBundleValidationContracts,
    PlanarContractBundleValidator, PlanarM7ReadinessBundle, PlanarM7ReadinessReceipt,
    PlanarM7ReadinessSupportPosture,
};
use crate::facade::planar_contracts::{
    admit_planar_contract_family, PlanarAdmissionFamily, PlanarRuntimeConcern,
};
use crate::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticSubject,
};
use crate::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarMotionPosture, PlanarMotionPostureContracts,
    PlanarReorientation,
};
use crate::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
};
use crate::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoverySource,
};
use crate::facade::planar_retained_facts::{RetainedPlanarFacts, RetainedPlanarFactsContracts};
use crate::facade::planar_structural_identity::{
    PlanarStructuralIdentity, PlanarStructuralIdentityContracts,
};

use super::geometry_receipts::{
    frame_receipt, overlap_receipt, precision_receipt, predicate_consumption_receipt,
    predicate_receipt, projected_points, segment_orientation_predicates, segment_receipt,
    signed_area_receipt, topology_contract_receipt, winding_receipt,
};
use super::handles::{
    bundle_handle, diagnostic_handle, motion_posture_handle, projection_consumption_handle,
    recovery_handle, retained_planar_handle, structural_identity_handle,
};
use super::{MOVEMENT, NEIGHBORHOOD, TOPOLOGY};

pub(crate) fn readiness_receipt() -> PlanarM7ReadinessReceipt {
    let bundle_receipt = boolean_readiness_bundle_receipt();
    let retained = RetainedPlanarFacts::from_boolean_readiness(bundle_receipt.clone())
        .retain_planar_classification()
        .retain_structural_identity(structural_identity_receipt(bundle_receipt.clone()))
        .retain_motion_posture(motion_posture_receipt(bundle_receipt.clone()))
        .retain_topology_contract(topology_contract_receipt())
        .compile(&RetainedPlanarFactsContracts::new(retained_planar_handle()))
        .expect("retained planar facts plan")
        .retain()
        .expect("retained planar facts receipt");
    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(bundle_receipt.basis().projection_receipts().to_vec())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(),
        ))
        .expect("projection-consumed plan")
        .consume()
        .expect("projection-consumed receipt");
    let recovery = PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_projection_denial("local-frame-test:projection-denial"),
    )
    .with_retained_planar_facts(retained.clone())
    .with_projection_consumed_facts(projected.clone())
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle()))
    .expect("recovery plan")
    .certify()
    .expect("recovery receipt");
    let diagnostics = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::binding_failure("local-frame-test:diagnostic"),
    )
    .with_retained_planar_facts(retained.clone())
    .with_projection_consumed_planar_facts(projected.clone())
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle()))
    .expect("diagnostic plan")
    .certify()
    .expect("diagnostic receipt");

    PlanarM7ReadinessBundle::from_certified_planar_bundle(bundle_receipt)
        .with_structural_identity(retained.basis().structural_identity_receipt().clone())
        .with_motion_posture(retained.basis().motion_posture_receipt().clone())
        .with_retained_planar_facts(retained)
        .with_projection_consumed_facts(projected)
        .with_recovery_posture(recovery)
        .with_diagnostics(diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "local-frame selection tests keep later boolean execution support-gated",
        ))
        .compile(&PlanarContractBundleValidationContracts::new(
            bundle_handle(),
        ))
        .expect("M7 readiness plan")
        .certify()
        .expect("M7 readiness receipt")
}

fn boolean_readiness_bundle_receipt(
) -> crate::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt {
    let predicate = predicate_receipt();
    let precision = precision_receipt(&predicate);
    let frame = frame_receipt(&precision);
    let left = projected_points(
        &frame,
        "left",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
    );
    let right = projected_points(
        &frame,
        "right",
        &[
            [2.0e-9, 0.0],
            [4.0e-9, 0.0],
            [4.0e-9, 2.0e-9],
            [2.0e-9, 2.0e-9],
        ],
    );
    let left_winding = winding_receipt("left", left.clone());
    let right_winding = winding_receipt("right", right.clone());
    let left_area = signed_area_receipt(left_winding.clone(), precision.clone());
    let right_area = signed_area_receipt(right_winding, precision.clone());
    let overlap = overlap_receipt(left_area.clone(), right_area);
    let segment = segment_receipt(
        left[1].clone(),
        left[2].clone(),
        right[3].clone(),
        right[0].clone(),
    );
    let segment_predicates = segment_orientation_predicates(&segment);
    let predicate_consumption =
        predicate_consumption_receipt(segment.clone(), segment_predicates.clone());
    let mut predicates = vec![predicate];
    predicates.extend(segment_predicates);
    let mut projections = left;
    projections.extend(right);
    let bundle = PlanarBooleanReadinessBundle::builder()
        .admission(
            admit_planar_contract_family(
                PlanarAdmissionFamily::PlanarContractBundle,
                PlanarRuntimeConcern::BooleanReadinessBundle,
            )
            .expect("bundle admission"),
        )
        .topology_contract(topology_contract_receipt())
        .precision(precision)
        .local_frame(frame)
        .projection_consumption(projections)
        .predicate_authority(predicates)
        .segment_contacts(vec![segment])
        .winding(left_winding)
        .signed_area(left_area)
        .coplanar_overlap(overlap)
        .predicate_consumption(predicate_consumption)
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:local-frame-test")
        .build()
        .expect("boolean readiness bundle");
    PlanarContractBundleValidator::for_boolean_readiness(bundle)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&PlanarContractBundleValidationContracts::new(
            bundle_handle(),
        ))
        .expect("bundle plan")
        .certify()
        .expect("bundle receipt")
}

fn motion_posture_receipt(
    bundle_receipt: crate::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt,
) -> crate::facade::planar_motion_posture::PlanarMotionPostureReceipt {
    PlanarMotionPosture::from_boolean_readiness(bundle_receipt)
        .after_exact_translation("motion:local-frame-test-translate")
        .after_exact_rotation("motion:local-frame-test-rotation")
        .after_reorientation(PlanarReorientation::PreservesHandedness)
        .with_cancellation_policy(PlanarMotionCancellation::ExactBasisReplay)
        .compile(&PlanarMotionPostureContracts::new(motion_posture_handle()))
        .expect("motion posture plan")
        .certify()
        .expect("motion posture receipt")
}

fn structural_identity_receipt(
    bundle_receipt: crate::facade::planar_contract_bundle::PlanarContractBundleValidationReceipt,
) -> crate::facade::planar_structural_identity::PlanarStructuralIdentityReceipt {
    let motion = motion_posture_receipt(bundle_receipt.clone());
    PlanarStructuralIdentity::from_boolean_readiness(bundle_receipt)
        .with_motion_posture(motion)
        .with_topology_identity("topology:local-frame-structural")
        .with_persistent_name("name:local-frame-structural")
        .with_binding_identity("binding:local-frame-structural")
        .with_lineage_identity("lineage:local-frame-structural")
        .compile(&PlanarStructuralIdentityContracts::new(
            structural_identity_handle(),
        ))
        .expect("structural identity plan")
        .certify()
        .expect("structural identity receipt")
}
