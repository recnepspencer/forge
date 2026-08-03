use worth_ui::facade::inspection::{
    UiGeometryOnly, UiVisualHitTestOutcome, UiVisualHitTestTarget, UiVisualOverlayDenial,
    UiVisualSnapshotReceipt,
};
use worth_ui_runtime::facade::mounted::{
    UiMountedDiagnosticInspection, UiMountedInspectionReceipt, UiMountedInspectionRequest,
};

pub(crate) fn assert_superseded_overlay_sources_are_denied(
    context: &egui::Context,
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let grant = session.visual_inspection_authority().issue_overlay_grant();
    let hit = selected_hit_target(&receipt);
    let first = receipt.overlay_target(&hit).unwrap();
    let prederived_stale = receipt.overlay_target(&hit).unwrap();
    let pending = session.show_identity_overlay(&grant, first).unwrap();
    let published = super::transition::publish(context, &mut session, pending, 20, 2);

    let registration_denial = session
        .show_identity_overlay(&grant, prederived_stale)
        .expect_err("a prederived predecessor target cannot register");
    assert_eq!(registration_denial, UiVisualOverlayDenial::Superseded);
    let derivation_denial = match receipt.overlay_target(&hit) {
        Err(denial) => denial,
        Ok(_) => panic!("a predecessor receipt cannot derive a fresh overlay target"),
    };
    assert_eq!(derivation_denial, UiVisualOverlayDenial::Superseded);

    super::transition::clear(context, &mut session, published, 30, 3);
    let _ = session.dispose_visual_snapshot(receipt);
    let _ = session.shutdown();
}

pub(crate) fn assert_expired_snapshot_cannot_derive_overlay_target(
    session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let hit = selected_hit_target(&receipt);
    let shutdown = session.shutdown();
    assert_eq!(shutdown.visual_overlay().disposed_published_count(), 0);
    let denial = match receipt.overlay_target(&hit) {
        Err(denial) => denial,
        Ok(_) => panic!("a receipt whose session authority is gone is expired"),
    };
    assert_eq!(denial, UiVisualOverlayDenial::Expired);
}

pub(crate) fn assert_overlay_registry_capacity_is_typed(
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let grant = session.visual_inspection_authority().issue_overlay_grant();
    let hit = selected_hit_target(&receipt);
    let first = receipt.overlay_target(&hit).unwrap();
    let second = receipt.overlay_target(&hit).unwrap();
    let pending = session.show_identity_overlay(&grant, first).unwrap();
    let denial = session
        .show_identity_overlay(&grant, second)
        .expect_err("the bounded registry admits only one active overlay");
    assert_eq!(denial, UiVisualOverlayDenial::CapacityExceeded);
    assert_eq!(super::overlay_leases(&session), 1);
    drop(pending);
    assert_eq!(super::overlay_leases(&session), 0);
    let _ = session.dispose_visual_snapshot(receipt);
    let _ = session.shutdown();
}

pub(crate) fn assert_published_drop_persists_until_shutdown(
    context: &egui::Context,
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let grant = session.visual_inspection_authority().issue_overlay_grant();
    let target = super::derive_overlay_target(&receipt);
    let _ = session.dispose_visual_snapshot(receipt);
    let pending = session.show_identity_overlay(&grant, target).unwrap();
    let published = super::transition::publish(context, &mut session, pending, 20, 2);
    let expected_identity = published.identity().diagnostic_value();
    let expected_frame = published.published_frame();
    drop(published);

    let mechanics = active_overlay_mechanics(&session);
    assert_eq!(mechanics.len(), 1);
    assert_eq!(mechanics[0].overlay_identity(), expected_identity);
    assert_eq!(mechanics[0].successor_frame(), expected_frame);
    assert_eq!(super::overlay_leases(&session), 1);
    let shutdown = session.shutdown();
    assert_eq!(shutdown.visual_overlay().disposed_published_count(), 1);
}

fn selected_hit_target(receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>) -> UiVisualHitTestTarget {
    receipt.with_coordinate_scope(|scope| {
        let point = scope
            .client_pixel(
                worth_ui::facade::inspection::UiClientPhysicalPixel::new(100, 50).unwrap(),
            )
            .unwrap();
        let adjudication = scope.adjudicate_point(point).unwrap();
        let UiVisualHitTestOutcome::Target(target) = adjudication.hit_test() else {
            panic!("the authored overlap has one exact hit target");
        };
        target.clone()
    })
}

fn active_overlay_mechanics(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> Vec<worth_ui_host_contract::UiMountedIdentityOverlayMechanic> {
    let frame = match session
        .inspect_mounted_frame(UiMountedInspectionRequest::current().with_diagnostics())
    {
        UiMountedInspectionReceipt::Available(frame) => frame,
        other => panic!("published overlay remains inspectable, got {other:?}"),
    };
    let UiMountedDiagnosticInspection::Available(diagnostics) = frame.diagnostics() else {
        panic!("published overlay retains diagnostics");
    };
    diagnostics
        .rows()
        .iter()
        .filter_map(|(_, _, diagnostic)| match diagnostic {
            worth_ui_host_contract::UiMountedDiagnosticProjection::IdentityOverlay(mechanic) => {
                Some(*mechanic)
            }
            _ => None,
        })
        .collect()
}
