use super::{UiMountedAppearanceColor, UiMountedAppearanceOpacity};

const FIXED_ONE: u64 = 1_u64 << 32;
pub const SRGB_LINEAR_THRESHOLD_NUMERATOR: u32 = 40_450;
pub const SRGB_LINEAR_THRESHOLD_DENOMINATOR: u32 = 1_000_000;
pub const SRGB_OFFSET_NUMERATOR: u32 = 55;
pub const SRGB_OFFSET_DENOMINATOR: u32 = 1_000;
pub const SRGB_SCALE_NUMERATOR: u32 = 1_055;
pub const SRGB_SCALE_DENOMINATOR: u32 = 1_000;
pub const SRGB_GAMMA_NUMERATOR: u32 = 12;
pub const SRGB_GAMMA_DENOMINATOR: u32 = 5;
pub const SRGB_LINEAR_SCALE_NUMERATOR: u32 = 1_292;
pub const SRGB_LINEAR_SCALE_DENOMINATOR: u32 = 100;

pub fn compose_source_over(
    bottom_to_top: impl IntoIterator<Item = (UiMountedAppearanceColor, UiMountedAppearanceOpacity)>,
) -> UiMountedAppearanceColor {
    let mut destination = [0_u16; 4];
    for (color, opacity) in bottom_to_top {
        let [red, green, blue, alpha] = color.straight_srgba();
        let color_alpha = round_ratio_even(u128::from(alpha) * 65_535, 255) as u16;
        let source_alpha = mul_unit(color_alpha, opacity.units());
        let source = [decode_srgb(red), decode_srgb(green), decode_srgb(blue)];
        let inverse = 65_535 - source_alpha;
        for channel in 0..3 {
            let source_premultiplied = mul_unit(source[channel], source_alpha);
            destination[channel] =
                source_premultiplied.saturating_add(mul_unit(destination[channel], inverse));
        }
        destination[3] = source_alpha.saturating_add(mul_unit(destination[3], inverse));
    }
    if destination[3] == 0 {
        return UiMountedAppearanceColor::from_straight_srgba([0, 0, 0, 0]);
    }
    let rgb = [0, 1, 2].map(|channel| {
        let straight = round_ratio_even(
            u128::from(destination[channel]) * 65_535,
            u128::from(destination[3]),
        )
        .min(65_535) as u16;
        encode_srgb(straight)
    });
    let alpha = round_ratio_even(u128::from(destination[3]) * 255, 65_535) as u8;
    UiMountedAppearanceColor::from_straight_srgba([rgb[0], rgb[1], rgb[2], alpha])
}

fn decode_srgb(channel: u8) -> u16 {
    let linear_threshold_channel = (u64::from(SRGB_LINEAR_THRESHOLD_NUMERATOR) * 255
        / u64::from(SRGB_LINEAR_THRESHOLD_DENOMINATOR)) as u8;
    if channel <= linear_threshold_channel {
        return round_ratio_even(
            u128::from(channel) * 65_535 * u128::from(SRGB_LINEAR_SCALE_DENOMINATOR),
            255 * u128::from(SRGB_LINEAR_SCALE_NUMERATOR),
        ) as u16;
    }
    let numerator = u128::from(channel) * u128::from(SRGB_OFFSET_DENOMINATOR)
        + 255 * u128::from(SRGB_OFFSET_NUMERATOR);
    let denominator = 255 * u128::from(SRGB_SCALE_NUMERATOR);
    let x = round_ratio_even(
        numerator * u128::from(SRGB_SCALE_DENOMINATOR) * u128::from(FIXED_ONE),
        denominator * u128::from(SRGB_OFFSET_DENOMINATOR),
    ) as u64;
    let rooted = fixed_nth_root(x, SRGB_GAMMA_DENOMINATOR);
    let decoded = fixed_pow(rooted, SRGB_GAMMA_NUMERATOR);
    round_ratio_even(u128::from(decoded) * 65_535, u128::from(FIXED_ONE)).min(65_535) as u16
}

fn encode_srgb(linear: u16) -> u8 {
    let mut low = 0_u16;
    let mut high = 255_u16;
    while low < high {
        let middle = (low + high) / 2;
        if decode_srgb(middle as u8) < linear {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    if low == 0 {
        return 0;
    }
    let upper = decode_srgb(low as u8).abs_diff(linear);
    let lower = decode_srgb((low - 1) as u8).abs_diff(linear);
    if lower < upper || (lower == upper && (low - 1) % 2 == 0) {
        (low - 1) as u8
    } else {
        low as u8
    }
}

fn fixed_pow(value: u64, exponent: u32) -> u64 {
    let mut result = FIXED_ONE;
    for _ in 0..exponent {
        result = fixed_mul(result, value);
    }
    result
}

fn fixed_nth_root(value: u64, degree: u32) -> u64 {
    let mut low = 0_u64;
    let mut high = FIXED_ONE;
    while low + 1 < high {
        let middle = low + (high - low) / 2;
        if fixed_pow(middle, degree) <= value {
            low = middle;
        } else {
            high = middle;
        }
    }
    let low_error = value.abs_diff(fixed_pow(low, degree));
    let high_error = value.abs_diff(fixed_pow(high, degree));
    if high_error < low_error || (high_error == low_error && high % 2 == 0) {
        high
    } else {
        low
    }
}

fn fixed_mul(left: u64, right: u64) -> u64 {
    let numerator = u128::from(left) * u128::from(right);
    let denominator = u128::from(FIXED_ONE);
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    match (remainder * 2).cmp(&denominator) {
        std::cmp::Ordering::Less => quotient as u64,
        std::cmp::Ordering::Greater => (quotient + 1) as u64,
        std::cmp::Ordering::Equal if quotient % 2 == 1 => (quotient + 1) as u64,
        std::cmp::Ordering::Equal => quotient as u64,
    }
}

fn mul_unit(left: u16, right: u16) -> u16 {
    round_ratio_even(u128::from(left) * u128::from(right), 65_535) as u16
}

pub(super) fn round_ratio_even(numerator: u128, denominator: u128) -> u128 {
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    match (remainder * 2).cmp(&denominator) {
        std::cmp::Ordering::Less => quotient,
        std::cmp::Ordering::Greater => quotient + 1,
        std::cmp::Ordering::Equal if quotient % 2 == 1 => quotient + 1,
        std::cmp::Ordering::Equal => quotient,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparent_and_opaque_source_over_are_exact() {
        let red = UiMountedAppearanceColor::from_straight_srgba([255, 0, 0, 255]);
        assert_eq!(
            compose_source_over([(red, UiMountedAppearanceOpacity::ZERO)]).straight_srgba(),
            [0, 0, 0, 0]
        );
        assert_eq!(
            compose_source_over([(red, UiMountedAppearanceOpacity::ONE)]),
            red
        );
    }

    #[test]
    fn repeated_black_layers_match_independent_alpha_oracle() {
        let black = UiMountedAppearanceColor::from_straight_srgba([0, 0, 0, 128]);
        let output = compose_source_over([(black, UiMountedAppearanceOpacity::ONE); 2]);
        assert_eq!(output.straight_srgba(), [0, 0, 0, 192]);
    }

    #[test]
    fn colored_layers_match_an_independent_linear_light_oracle() {
        let red = UiMountedAppearanceColor::from_straight_srgba([255, 0, 0, 128]);
        let green = UiMountedAppearanceColor::from_straight_srgba([0, 255, 0, 128]);
        let output = compose_source_over([
            (red, UiMountedAppearanceOpacity::ONE),
            (green, UiMountedAppearanceOpacity::ONE),
        ]);
        assert_eq!(output.straight_srgba(), [156, 213, 0, 192]);
    }

    #[test]
    fn every_opaque_srgb_channel_roundtrips_and_transfer_is_monotonic() {
        let mut prior = 0;
        for channel in 0_u8..=u8::MAX {
            let decoded = decode_srgb(channel);
            assert!(decoded >= prior, "channel {channel} regressed");
            assert_eq!(encode_srgb(decoded), channel, "channel {channel}");
            let gray =
                UiMountedAppearanceColor::from_straight_srgba([channel, channel, channel, 255]);
            assert_eq!(
                compose_source_over([(gray, UiMountedAppearanceOpacity::ONE)]),
                gray,
                "opaque channel {channel}",
            );
            prior = decoded;
        }
    }

    #[test]
    fn partial_opacity_nonblack_layers_have_locked_independent_oracle_values() {
        let blue = UiMountedAppearanceColor::from_straight_srgba([20, 40, 220, 173]);
        let amber = UiMountedAppearanceColor::from_straight_srgba([240, 130, 10, 119]);
        assert_eq!(
            compose_source_over([
                (blue, UiMountedAppearanceOpacity::from_units(51_337)),
                (amber, UiMountedAppearanceOpacity::from_units(42_001)),
            ])
            .straight_srgba(),
            [168, 94, 169, 171],
        );
    }
}
