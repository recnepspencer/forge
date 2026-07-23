use worth_ui::facade::{
    registry::{IconAccessibilityPosture, IconDescriptor, IconFamily, IconId, IconThemePosture},
};

fn main() {
    let _ = IconDescriptor {
        id: IconId::new("workspace.icon.save").unwrap(),
        family: IconFamily::command(),
        source: None,
        theme_posture: IconThemePosture::inherits_text_color(),
        accessibility_posture: IconAccessibilityPosture::labelled_by_consumer(),
        raw_asset_reference: None,
    };
}
