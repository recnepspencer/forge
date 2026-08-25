use worth_ui_host_contract::UiMountedNodeReceiptIssuer;
use worth_ui_runtime::facade::mounted::{UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity};

use super::*;

pub(super) fn assert_receipt_coordinate_denials() {
    assert_out_of_range_instance();
    assert_fresh_issuer_cannot_forge_current_receipt();
}

fn assert_out_of_range_instance() {
    let mut world = published_observation_world("observation-foreign-instance");
    let raw_report = mounted_report(
        UiMountedInstanceIdentity::mint_unbound().unwrap(),
        UiMountedNodeReceiptIdentity::mint_unbound().unwrap(),
    );
    let raw = mounted_batch(&world, raw_report);
    assert_denial(
        &mut world,
        raw,
        UiHostObservationReportDenial::MountedInstanceNotPresented,
    );
}

fn assert_fresh_issuer_cannot_forge_current_receipt() {
    let mut world = published_observation_world("observation-forged-current-receipt");
    let forged = UiMountedNodeReceiptIssuer::mint_for(world.current.frame)
        .unwrap()
        .receipt_for(world.current.instance);
    let raw_report = mounted_report(world.current.instance, forged);
    let raw = mounted_batch(&world, raw_report);
    assert_denial(
        &mut world,
        raw,
        UiHostObservationReportDenial::NodeReceiptMismatch,
    );
}

fn mounted_report(
    instance: UiMountedInstanceIdentity,
    receipt: UiMountedNodeReceiptIdentity,
) -> UiHostObservationReport {
    UiHostObservationReport::new(
        UiHostObservationSequence::new(1),
        UiHostObservationTimeBasis::HostMonotonicMillis(1),
        UiHostObservationPayload::Focus { focused: true },
    )
    .with_mounted_basis(UiHostObservationMountedBasis::new(instance, receipt))
}

fn mounted_batch(
    world: &super::super::mounted_application_lifecycle::published_mounted_world::PublishedObservationWorld,
    report: UiHostObservationReport,
) -> UiHostObservationBatch {
    batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report],
    )
}
