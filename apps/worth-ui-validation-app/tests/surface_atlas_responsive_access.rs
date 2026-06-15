use egui::vec2;
use worth_ui_validation_app::pages::surface_atlas::{
    SurfaceAtlasFamily, SurfaceAtlasReachability, SurfaceAtlasViewport,
};

#[test]
fn surface_atlas_mobile_layout_preserves_surface_access() {
    let reachability = SurfaceAtlasReachability::for_viewport(SurfaceAtlasViewport::Narrow);

    for required_family in [
        SurfaceAtlasFamily::ScenarioList,
        SurfaceAtlasFamily::WorkbenchCanvas,
        SurfaceAtlasFamily::EvidenceInspector,
        SurfaceAtlasFamily::BottomTimeline,
        SurfaceAtlasFamily::OverlayPreview,
    ] {
        assert!(
            reachability.reaches(required_family),
            "narrow atlas viewport lost access to {}",
            required_family.label()
        );
    }
}

#[test]
fn surface_atlas_viewport_classification_has_stable_breakpoints() {
    assert_eq!(
        SurfaceAtlasViewport::from_available_size(vec2(480.0, 800.0)),
        SurfaceAtlasViewport::Narrow
    );
    assert_eq!(
        SurfaceAtlasViewport::from_available_size(vec2(900.0, 800.0)),
        SurfaceAtlasViewport::Standard
    );
    assert_eq!(
        SurfaceAtlasViewport::from_available_size(vec2(1440.0, 900.0)),
        SurfaceAtlasViewport::Wide
    );
}
