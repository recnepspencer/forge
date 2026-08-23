//! Geometry and subpixel quantization used by text-owned glyph demand.

use worth_ui_host_contract::{UiMountedLogicalDamage, UiTextOriginalRange, UiTextRect};

use super::placement::UiGlyphRasterPlacement;

pub(super) fn fractional_origin(
    x: i64,
    y: i64,
    dpi_milli: u32,
) -> Option<worth_ui_host_contract::UiGlyphRasterFractionalOrigin> {
    let x = quantize_origin(x, dpi_milli)?;
    let y = quantize_origin(y, dpi_milli)?;
    Some(worth_ui_host_contract::UiGlyphRasterFractionalOrigin::from_sixty_fourths(x, y))
}

fn quantize_origin(value_millipoints: i64, dpi_milli: u32) -> Option<i16> {
    let numerator = i128::from(value_millipoints).checked_mul(i128::from(dpi_milli))?;
    let denominator = 1_000_000_i128;
    let remainder = numerator.rem_euclid(denominator);
    let signed_remainder = if numerator < 0 && remainder != 0 {
        remainder - denominator
    } else {
        remainder
    };
    let quantized = round_ties_even(signed_remainder * 64, denominator);
    let quantized = quantized.clamp(-63, 63);
    i16::try_from(quantized).ok()
}

fn round_ties_even(numerator: i128, denominator: i128) -> i128 {
    let sign = if numerator < 0 { -1 } else { 1 };
    let magnitude = numerator.unsigned_abs();
    let denominator = denominator as u128;
    let quotient = magnitude / denominator;
    let remainder = magnitude % denominator;
    let rounded = if remainder * 2 < denominator {
        quotient
    } else if remainder * 2 > denominator || quotient % 2 == 1 {
        quotient + 1
    } else {
        quotient
    };
    i128::from(sign) * i128::try_from(rounded).unwrap_or(i128::MAX)
}

pub(super) fn damage_intersects(
    ink: UiTextRect,
    placement: UiGlyphRasterPlacement,
    damage: &[UiMountedLogicalDamage],
) -> bool {
    if ink.width_millipoints() <= 0 || ink.height_millipoints() <= 0 {
        return false;
    }
    let Some(ink_left) = ink
        .left_millipoints()
        .checked_add(placement.origin_x_millipoints())
    else {
        return false;
    };
    let Some(ink_right) = ink
        .right_millipoints()
        .checked_add(placement.origin_x_millipoints())
    else {
        return false;
    };
    let Some(ink_top) = ink
        .top_millipoints()
        .checked_add(placement.origin_y_millipoints())
    else {
        return false;
    };
    let Some(ink_bottom) = ink
        .bottom_millipoints()
        .checked_add(placement.origin_y_millipoints())
    else {
        return false;
    };
    damage.iter().any(|region| {
        let bounds = region.bounds();
        let left = f64::from(bounds.x()) * 1_000.0;
        let top = f64::from(bounds.y()) * 1_000.0;
        let right = f64::from(bounds.x() + bounds.width()) * 1_000.0;
        let bottom = f64::from(bounds.y() + bounds.height()) * 1_000.0;
        left < ink_right as f64
            && right > ink_left as f64
            && top < ink_bottom as f64
            && bottom > ink_top as f64
    })
}

pub(super) fn contains_range(outer: UiTextOriginalRange, inner: UiTextOriginalRange) -> bool {
    outer.start() <= inner.start() && outer.end() >= inner.end()
}

#[cfg(test)]
mod tests {
    use super::{fractional_origin, round_ties_even};

    #[test]
    fn origin_rounding_is_signed_ties_to_even() {
        assert_eq!(round_ties_even(32, 64), 0);
        assert_eq!(round_ties_even(96, 64), 2);
        assert_eq!(round_ties_even(-32, 64), 0);
        assert_eq!(round_ties_even(-96, 64), -2);
        assert_eq!(round_ties_even(160, 64), 2);
        assert_eq!(fractional_origin(-750, 0, 1_000).unwrap().x_over_64(), -48);
        assert_eq!(fractional_origin(250, 0, 1_000).unwrap().x_over_64(), 16);
    }
}
