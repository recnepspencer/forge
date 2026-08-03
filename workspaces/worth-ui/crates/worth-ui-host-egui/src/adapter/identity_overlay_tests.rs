use super::{UiEguiPreparedIdentityOverlay, UiHostSurfacePresentationDenial};
use worth_ui_host_contract::UiMountedProjectionView;
use worth_ui_test_support::{
    identity_overlay_projection_for_certification,
    UiIdentityOverlayProjectionCertificationMutation as ProjectionMutation,
};

#[test]
fn exact_mechanic_translates_to_four_foreground_physical_pixel_strips() {
    let context = egui::Context::default();
    let projection = overlay_projection(ProjectionMutation::Exact);
    let Ok(prepared) = prepare_in_frame(&context, &projection) else {
        panic!("the exact mounted mechanic must translate");
    };
    assert_eq!(prepared.layer.order, egui::Order::Foreground);
    assert_eq!(prepared.strips.len(), 4);
    assert_eq!(
        prepared.color,
        egui::Color32::from_rgba_unmultiplied(255, 0, 255, 255)
    );
    assert_eq!(
        prepared.strips,
        vec![
            egui::Rect::from_min_max(egui::pos2(32.0, 20.0), egui::pos2(128.0, 22.0)),
            egui::Rect::from_min_max(egui::pos2(32.0, 74.0), egui::pos2(128.0, 76.0)),
            egui::Rect::from_min_max(egui::pos2(32.0, 20.0), egui::pos2(34.0, 76.0)),
            egui::Rect::from_min_max(egui::pos2(126.0, 20.0), egui::pos2(128.0, 76.0)),
        ]
    );
    assert_eq!(
        super::super::mounted_effect_support::unsupported_projection_effect(&projection),
        None
    );
}

#[test]
fn foreign_affinity_and_out_of_client_bounds_reject_before_effects() {
    for mutation in [
        ProjectionMutation::ForeignSurface,
        ProjectionMutation::OffscreenBounds,
    ] {
        let context = egui::Context::default();
        let projection = overlay_projection(mutation);
        assert!(matches!(
            prepare_in_frame(&context, &projection),
            Err(UiHostSurfacePresentationDenial::MalformedProjection)
        ));
    }
}

#[test]
fn changed_dpi_client_extent_or_translation_rejects_snapshot_space_bounds() {
    let projection = overlay_projection(ProjectionMutation::Exact);
    for (screen, pixels_per_point) in [
        (
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0)),
            2.0,
        ),
        (
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(161.0, 96.0)),
            1.0,
        ),
        (
            egui::Rect::from_min_size(egui::pos2(4.0, 3.0), egui::vec2(160.0, 96.0)),
            1.0,
        ),
    ] {
        let context = egui::Context::default();
        assert!(matches!(
            prepare_with_geometry(&context, &projection, screen, pixels_per_point),
            Err(UiHostSurfacePresentationDenial::MalformedProjection)
        ));
    }
}

fn prepare_in_frame(
    context: &egui::Context,
    projection: &UiMountedProjectionView,
) -> Result<UiEguiPreparedIdentityOverlay, UiHostSurfacePresentationDenial> {
    prepare_with_geometry(
        context,
        projection,
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0)),
        1.0,
    )
}

fn prepare_with_geometry(
    context: &egui::Context,
    projection: &UiMountedProjectionView,
    screen: egui::Rect,
    pixels_per_point: f32,
) -> Result<UiEguiPreparedIdentityOverlay, UiHostSurfacePresentationDenial> {
    context.set_pixels_per_point(pixels_per_point);
    let mut prepared = None;
    let _ = context.run_ui(raw_input(screen), |_| {
        prepared = Some(UiEguiPreparedIdentityOverlay::prepare(context, projection));
    });
    prepared.expect("egui callback prepares the mounted overlay")
}

fn overlay_projection(mutation: ProjectionMutation) -> UiMountedProjectionView {
    identity_overlay_projection_for_certification(mutation)
}

fn raw_input(screen_rect: egui::Rect) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(screen_rect),
        ..Default::default()
    }
}
