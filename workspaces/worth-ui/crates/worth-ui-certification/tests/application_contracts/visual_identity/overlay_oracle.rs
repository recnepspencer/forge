use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiGeometryOnly, UiVisualHitTestOutcome, UiVisualSnapshotReceipt,
};
use worth_ui_runtime::facade::mounted::{
    UiMountedDiagnosticInspection, UiMountedInspectionReceipt, UiMountedInspectionRequest,
    UiMountedRetentionClass,
};

#[path = "overlay_oracle/adversarial.rs"]
mod adversarial;
#[path = "overlay_oracle/host_consequence.rs"]
mod host_consequence;
#[path = "overlay_oracle/transition.rs"]
mod transition;

pub(super) use adversarial::{
    assert_expired_snapshot_cannot_derive_overlay_target,
    assert_overlay_registry_capacity_is_typed, assert_published_drop_persists_until_shutdown,
    assert_superseded_overlay_sources_are_denied,
};

pub(super) fn assert_managed_overlay_successors(
    context: &egui::Context,
    host: &worth_ui_host_egui::WorthUiHostEgui,
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let overlay_grant = session.visual_inspection_authority().issue_overlay_grant();
    let overlay_target = derive_overlay_target(&receipt);
    let base_snapshot = receipt.identity();
    let base_frame = overlay_target.base_frame();
    let target_region = overlay_target.target_region();
    let selected_instance = overlay_target.target().mounted_node().mounted_instance();
    let disposed = session.dispose_visual_snapshot(receipt);
    assert!(disposed.released_registered_resource());
    assert_eq!(overlay_leases(&session), 1);

    let pending = session
        .show_identity_overlay(&overlay_grant, overlay_target)
        .expect("the same-session overlay grant admits the retained target");
    assert_eq!(pending.base_snapshot(), base_snapshot);
    let (published, published_output) =
        transition::publish_with_output(context, &mut session, pending, 20, 2);
    assert_ne!(published.published_frame(), base_frame);
    assert_eq!(overlay_leases(&session), 1);
    assert_exact_published_cost(&session, &published);
    assert_exact_overlay_diagnostic(&session, &published);
    host_consequence::assert_published_shapes(
        &published_output,
        target_region,
        context.pixels_per_point(),
    );
    host_consequence::assert_retained_overlay_repaint(context, host, target_region);
    host_consequence::assert_overlay_is_not_indexed(context, &mut session, selected_instance);

    let published_frame = published.published_frame();
    let (cleared, cleared_output) =
        transition::clear_with_output(context, &mut session, published, 30, 4);
    assert_eq!(cleared.published_frame(), published_frame);
    assert_ne!(cleared.cleared_frame(), published_frame);
    assert_eq!(overlay_leases(&session), 0);
    assert_eq!(cleared.cost().counters(), [0; 11]);
    host_consequence::assert_no_overlay_shapes(&cleared_output);
    host_consequence::assert_retained_clear_repaint(context, host);

    let shutdown = session.shutdown();
    assert_eq!(shutdown.visual_overlay().cancelled_pending_count(), 0);
    assert_eq!(shutdown.visual_overlay().disposed_published_count(), 0);
    assert_eq!(shutdown.visual_overlay().disposed_clearing_count(), 0);
}

pub(super) fn assert_overlay_rollbacks_preserve_linear_authority(
    context: &egui::Context,
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let grant = session.visual_inspection_authority().issue_overlay_grant();
    let target = derive_overlay_target(&receipt);
    let _ = session.dispose_visual_snapshot(receipt);
    let pending = session.show_identity_overlay(&grant, target).unwrap();

    let publication_failure = transition::fail_publication(context, &mut session, pending, 4, 5);
    assert_eq!(
        publication_failure.denial(),
        worth_ui::facade::inspection::UiVisualOverlayDenial::Presentation
    );
    assert_eq!(overlay_leases(&session), 1);

    let published = transition::publish(
        context,
        &mut session,
        publication_failure.into_pending(),
        20,
        6,
    );
    let published_cost = published.cost();
    let clear_failure = transition::fail_clear(context, &mut session, published, 6, 7);
    assert_eq!(
        clear_failure.denial(),
        worth_ui::facade::inspection::UiVisualOverlayDenial::Presentation
    );
    assert_eq!(overlay_leases(&session), 1);

    let recovered = clear_failure.into_published();
    assert_eq!(recovered.cost(), published_cost);
    transition::clear(context, &mut session, recovered, 30, 8);
    assert_eq!(overlay_leases(&session), 0);
    let _ = session.shutdown();
}

pub(super) fn assert_pending_overlay_is_enumerated_at_shutdown(
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let grant = session.visual_inspection_authority().issue_overlay_grant();
    let target = derive_overlay_target(&receipt);
    let _ = session.dispose_visual_snapshot(receipt);
    let pending = session.show_identity_overlay(&grant, target).unwrap();
    assert_eq!(overlay_leases(&session), 1);

    let shutdown = session.shutdown();
    assert_eq!(shutdown.visual_overlay().cancelled_pending_count(), 1);
    assert_eq!(shutdown.visual_overlay().disposed_published_count(), 0);
    assert_eq!(shutdown.visual_overlay().disposed_clearing_count(), 0);
    drop(pending);
}

pub(super) fn assert_pending_drop_is_no_effect_cancellation(
    context: &egui::Context,
    host: &worth_ui_host_egui::WorthUiHostEgui,
    mut session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let grant = session.visual_inspection_authority().issue_overlay_grant();
    let target = derive_overlay_target(&receipt);
    let _ = session.dispose_visual_snapshot(receipt);
    let pending = session.show_identity_overlay(&grant, target).unwrap();
    assert_eq!(overlay_leases(&session), 1);
    drop(pending);
    assert_eq!(overlay_leases(&session), 0);
    host_consequence::assert_retained_clear_repaint(context, host);
    let shutdown = session.shutdown();
    assert_eq!(shutdown.visual_overlay().cancelled_pending_count(), 0);
}

pub(super) fn assert_foreign_session_rejects_overlay_before_registration(
    mut owner: worth_ui::facade::app::WorthUiActiveApplicationSession,
    owner_receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
    mut foreign: worth_ui::facade::app::WorthUiActiveApplicationSession,
    foreign_receipt: UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    let foreign_grant = foreign.visual_inspection_authority().issue_overlay_grant();
    let target = derive_overlay_target(&owner_receipt);
    let _ = owner.dispose_visual_snapshot(owner_receipt);
    let _ = foreign.dispose_visual_snapshot(foreign_receipt);
    assert_eq!(overlay_leases(&owner), 1);

    let denial = foreign
        .show_identity_overlay(&foreign_grant, target)
        .expect_err("a foreign session cannot register retained overlay authority");
    assert_eq!(
        denial,
        worth_ui::facade::inspection::UiVisualOverlayDenial::ForeignSession
    );
    assert_eq!(overlay_leases(&owner), 0);
    assert_eq!(overlay_leases(&foreign), 0);
    let _ = owner.shutdown();
    let _ = foreign.shutdown();
}

fn assert_exact_overlay_diagnostic(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    published: &worth_ui::facade::inspection::UiPublishedVisualOverlay,
) {
    let inspected = match session
        .inspect_mounted_frame(UiMountedInspectionRequest::current().with_diagnostics())
    {
        UiMountedInspectionReceipt::Available(frame) => frame,
        other => panic!("overlay successor remains inspectable, got {other:?}"),
    };
    let UiMountedDiagnosticInspection::Available(diagnostics) = inspected.diagnostics() else {
        panic!("overlay successor retains diagnostics");
    };
    let mechanics = diagnostics
        .rows()
        .iter()
        .filter_map(|(_, _, diagnostic)| match diagnostic {
            worth_ui_host_contract::UiMountedDiagnosticProjection::IdentityOverlay(mechanic) => {
                Some(*mechanic)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(mechanics.len(), 1);
    let mechanic = mechanics[0];
    assert_eq!(
        mechanic.overlay_identity(),
        published.identity().diagnostic_value()
    );
    assert_eq!(
        mechanic.base_snapshot(),
        published.base_snapshot().diagnostic_value()
    );
    assert_eq!(mechanic.base_frame(), published.base_frame());
    assert_eq!(
        mechanic.target_receipt().diagnostic_value(),
        published.target().mounted_node().node_receipt()
    );
    assert_eq!(mechanic.successor_frame(), published.published_frame());
    let region = published.target_region();
    assert_eq!(mechanic.target_region().left(), region.left());
    assert_eq!(mechanic.target_region().top(), region.top());
    assert_eq!(mechanic.target_region().right(), region.right());
    assert_eq!(mechanic.target_region().bottom(), region.bottom());
    assert_eq!(mechanic.border_width(), 2);
    assert_eq!(mechanic.color().channels(), [255, 0, 255, 255]);
}

fn overlay_leases(session: &worth_ui::facade::app::WorthUiActiveApplicationSession) -> usize {
    session
        .mounted_retention_report()
        .class(UiMountedRetentionClass::VisualOverlay)
        .active_leases()
}

fn assert_exact_published_cost(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
    published: &worth_ui::facade::inspection::UiPublishedVisualOverlay,
) {
    let structural_bytes = session
        .mounted_retention_report()
        .class(UiMountedRetentionClass::VisualOverlay)
        .lease_charged_structural_bytes();
    assert_eq!(
        published.cost().counters(),
        [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            4,
            1,
            u64::try_from(structural_bytes).expect("the public report fits the cost domain"),
        ]
    );
}

fn derive_overlay_target(
    receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>,
) -> worth_ui::facade::inspection::UiVisualOverlayTarget {
    receipt.with_coordinate_scope(|scope| {
        let point = scope
            .client_pixel(UiClientPhysicalPixel::new(100, 50).unwrap())
            .unwrap();
        let adjudication = scope.adjudicate_point(point).unwrap();
        let UiVisualHitTestOutcome::Target(target) = adjudication.hit_test() else {
            panic!("the authored overlap has one exact hit target");
        };
        receipt
            .overlay_target(target)
            .expect("a target from this receipt retains overlay authority")
    })
}
