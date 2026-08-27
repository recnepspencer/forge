use serde::Deserialize;

const SOURCE: &str = include_str!("platform_pulse_control_points.json");
const SCHEMA: &str = "platform-pulse-control-points-v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct PlatformPulseControlPointManifest {
    schema: String,
    identity: String,
    logical_client_extent: [u32; 2],
    background_logical_point: [u32; 2],
    target_logical_point: [u32; 2],
    target_region_inset: [u32; 2],
    action_control_logical_point: [u32; 2],
    confirmation_control_logical_point: [u32; 2],
    blue_rgba: [u8; 4],
    green_rgba: [u8; 4],
    target_rgba: [u8; 4],
    confirmation_rgba: [u8; 4],
    overlay_rgba: [u8; 4],
    channel_tolerance: u8,
    maximum_capture_scale: u32,
    maximum_pixel_bytes: u64,
    visible_region_count: u64,
    hit_test_region_count: u64,
    target_authored_name: String,
    environment: ControlPointEnvironment,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct ControlPointEnvironment {
    dpi: u32,
    text_profile: String,
    font_asset_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlatformPulseControlPointManifestFailure {
    Decode,
    Incomplete,
}

pub(super) fn checked_in(
) -> Result<PlatformPulseControlPointManifest, PlatformPulseControlPointManifestFailure> {
    let manifest: PlatformPulseControlPointManifest = serde_json::from_str(SOURCE)
        .map_err(|_| PlatformPulseControlPointManifestFailure::Decode)?;
    let complete = manifest.schema == SCHEMA
        && !manifest.identity.is_empty()
        && manifest
            .logical_client_extent
            .iter()
            .all(|value| *value > 0)
        && inside(
            manifest.background_logical_point,
            manifest.logical_client_extent,
        )
        && inside(
            manifest.target_logical_point,
            manifest.logical_client_extent,
        )
        && inside(manifest.target_region_inset, manifest.logical_client_extent)
        && inside(
            manifest.action_control_logical_point,
            manifest.logical_client_extent,
        )
        && inside(
            manifest.confirmation_control_logical_point,
            manifest.logical_client_extent,
        )
        && manifest.blue_rgba[3] == 255
        && manifest.green_rgba[3] == 255
        && manifest.target_rgba[3] == 255
        && manifest.confirmation_rgba[3] == 255
        && manifest.overlay_rgba[3] == 255
        && manifest.channel_tolerance > 0
        && manifest.maximum_capture_scale > 0
        && manifest.maximum_pixel_bytes > 0
        && manifest.visible_region_count > 0
        && manifest.hit_test_region_count > 0
        && !manifest.target_authored_name.is_empty()
        && manifest.environment.dpi > 0
        && !manifest.environment.text_profile.is_empty()
        && manifest.environment.font_asset_sha256.len() == 64;
    complete
        .then_some(manifest)
        .ok_or(PlatformPulseControlPointManifestFailure::Incomplete)
}

fn inside(point: [u32; 2], extent: [u32; 2]) -> bool {
    point[0] < extent[0] && point[1] < extent[1]
}

macro_rules! accessors {
    ($($name:ident : $kind:ty),+ $(,)?) => {
        impl PlatformPulseControlPointManifest {
            $(pub(super) fn $name(&self) -> $kind { self.$name })+
        }
    };
}

accessors!(
    logical_client_extent: [u32; 2],
    background_logical_point: [u32; 2],
    target_logical_point: [u32; 2],
    target_region_inset: [u32; 2],
    action_control_logical_point: [u32; 2],
    confirmation_control_logical_point: [u32; 2],
    blue_rgba: [u8; 4],
    green_rgba: [u8; 4],
    target_rgba: [u8; 4],
    confirmation_rgba: [u8; 4],
    overlay_rgba: [u8; 4],
    channel_tolerance: u8,
    maximum_capture_scale: u32,
    maximum_pixel_bytes: u64,
    visible_region_count: u64,
    hit_test_region_count: u64,
);

impl PlatformPulseControlPointManifest {
    pub(super) fn target_authored_name(&self) -> &str {
        &self.target_authored_name
    }
}
