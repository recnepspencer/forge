use crate::native::UiNativeGraphics;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RasterRect {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    pub(super) physical_width: u32,
    pub(super) physical_height: u32,
}

pub(super) fn raster_rect(
    mechanic: worth_ui_host_contract::UiMountedFilledRectMechanic,
    graphics: &UiNativeGraphics,
) -> Result<RasterRect, ()> {
    let bounds = mechanic.bounds();
    let clip = mechanic.clip_bounds();
    raster_from_basis(
        [bounds.x(), bounds.y(), bounds.width(), bounds.height()],
        [clip.x(), clip.y(), clip.width(), clip.height()],
        graphics.extent(),
        graphics.scale_factor as f32,
    )
}

pub(super) fn raster_damage(
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    graphics: &UiNativeGraphics,
) -> Result<Option<RasterRect>, ()> {
    raster_from_basis_optional(
        [bounds.x(), bounds.y(), bounds.width(), bounds.height()],
        [bounds.x(), bounds.y(), bounds.width(), bounds.height()],
        graphics.extent(),
        graphics.scale_factor as f32,
    )
}

fn raster_from_basis(
    bounds: [f32; 4],
    clip: [f32; 4],
    extent: [u32; 2],
    scale: f32,
) -> Result<RasterRect, ()> {
    raster_from_basis_optional(bounds, clip, extent, scale)?.ok_or(())
}

fn raster_from_basis_optional(
    bounds: [f32; 4],
    clip: [f32; 4],
    extent: [u32; 2],
    scale: f32,
) -> Result<Option<RasterRect>, ()> {
    if !scale.is_finite()
        || scale <= 0.0
        || bounds
            .iter()
            .chain(clip.iter())
            .any(|value| !value.is_finite())
        || extent.contains(&0)
    {
        return Err(());
    }
    let viewport = [extent[0] as f32 / scale, extent[1] as f32 / scale];
    let left = bounds[0].max(clip[0]).max(0.0);
    let top = bounds[1].max(clip[1]).max(0.0);
    let right = (bounds[0] + bounds[2])
        .min(clip[0] + clip[2])
        .min(viewport[0]);
    let bottom = (bounds[1] + bounds[3])
        .min(clip[1] + clip[3])
        .min(viewport[1]);
    if right <= left || bottom <= top {
        return Ok(None);
    }
    let (physical_left, physical_right) = snap_axis(left, right, scale, extent[0])?;
    let (physical_top, physical_bottom) = snap_axis(top, bottom, scale, extent[1])?;
    Ok(Some(RasterRect {
        left: physical_left as f32 * 2.0 / extent[0] as f32 - 1.0,
        top: 1.0 - physical_top as f32 * 2.0 / extent[1] as f32,
        right: physical_right as f32 * 2.0 / extent[0] as f32 - 1.0,
        bottom: 1.0 - physical_bottom as f32 * 2.0 / extent[1] as f32,
        physical_width: physical_right - physical_left,
        physical_height: physical_bottom - physical_top,
    }))
}

fn snap_axis(
    logical_min: f32,
    logical_max: f32,
    scale: f32,
    physical_limit: u32,
) -> Result<(u32, u32), ()> {
    let minimum = (logical_min * scale)
        .floor()
        .clamp(0.0, physical_limit as f32) as u32;
    let maximum = (logical_max * scale)
        .ceil()
        .clamp(0.0, physical_limit as f32) as u32;
    (maximum > minimum).then_some((minimum, maximum)).ok_or(())
}

#[cfg(test)]
mod tests {
    use super::{raster_from_basis, rectangle_shader};

    #[test]
    fn geometry_and_color_are_derived_from_the_admitted_command() {
        let rect = raster_from_basis(
            [10.0, 20.0, 1.0, 1.0],
            [10.25, 20.25, 0.5, 0.5],
            [200, 100],
            2.0,
        )
        .unwrap();
        assert_eq!(rect.physical_width, 2);
        assert_eq!(rect.physical_height, 2);
        for (observed, expected) in [
            (rect.left, -0.8),
            (rect.right, -0.78),
            (rect.top, 0.2),
            (rect.bottom, 0.16),
        ] {
            assert!((observed - expected).abs() < 0.000_001);
        }
        let red = rectangle_shader(rect, [255, 0, 0, 128]);
        let blue = rectangle_shader(rect, [0, 0, 255, 128]);
        let shifted = rectangle_shader(
            raster_from_basis([1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0], [20, 20], 1.0).unwrap(),
            [255, 0, 0, 128],
        );
        assert_ne!(red, blue);
        assert_ne!(red, shifted);
        for coordinate in
            [rect.left, rect.right, rect.top, rect.bottom].map(|value| value.to_string())
        {
            assert!(red.contains(&coordinate), "shader omits {coordinate}");
        }
        assert!(red.contains("vec3<f32>(1, 0, 0)"));
        assert!(blue.contains("vec3<f32>(0, 0, 1)"));
    }

    #[test]
    fn physical_edges_floor_minima_and_ceil_maxima_after_clipping() {
        let clipped_negative = raster_from_basis(
            [-0.25, -0.25, 0.5, 0.5],
            [-1.0, -1.0, 2.0, 2.0],
            [20, 20],
            2.0,
        )
        .unwrap();
        assert_eq!(clipped_negative.physical_width, 1);
        assert_eq!(clipped_negative.physical_height, 1);
        let fractional = raster_from_basis(
            [1.2, 2.4, 1.1, 1.1],
            [1.25, 2.45, 0.75, 0.75],
            [30, 30],
            1.5,
        )
        .unwrap();
        assert_eq!(fractional.physical_width, 2);
        assert_eq!(fractional.physical_height, 2);
    }

    #[test]
    fn empty_clipped_geometry_is_denied_before_effects() {
        assert!(raster_from_basis(
            [10.0, 10.0, 4.0, 4.0],
            [20.0, 20.0, 1.0, 1.0],
            [100, 100],
            1.0,
        )
        .is_err());
    }
}

pub(super) fn rectangle_shader(rect: RasterRect, rgba: [u8; 4]) -> String {
    let encoded = rgba.map(|channel| f32::from(channel) / 255.0);
    format!(
        r#"@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {{
    var positions = array<vec2<f32>, 6>(
        vec2<f32>({left}, {bottom}), vec2<f32>({right}, {bottom}),
        vec2<f32>({left}, {top}), vec2<f32>({left}, {top}),
        vec2<f32>({right}, {bottom}), vec2<f32>({right}, {top})
    );
    return vec4<f32>(positions[index], 0.0, 1.0);
}}
@fragment
fn fs_main() -> @location(0) vec4<f32> {{
    let encoded = vec3<f32>({red}, {green}, {blue});
    let low = encoded / vec3<f32>(12.92);
    let high = pow((encoded + vec3<f32>(0.055)) / vec3<f32>(1.055), vec3<f32>(2.4));
    let straight = select(high, low, encoded <= vec3<f32>(0.04045));
    return vec4<f32>(straight * {alpha}, {alpha});
}}"#,
        left = rect.left,
        right = rect.right,
        top = rect.top,
        bottom = rect.bottom,
        red = encoded[0],
        green = encoded[1],
        blue = encoded[2],
        alpha = encoded[3],
    )
}
