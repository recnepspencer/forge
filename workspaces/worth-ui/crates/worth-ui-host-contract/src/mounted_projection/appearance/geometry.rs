#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAppearanceDamageRegion {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAppearanceEmptyRegion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAppearanceClip(UiAppearanceDamageRegion);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAppearancePhysicalRadii([u32; 4]);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAppearanceDamageAttribution {
    Surface,
    Outline,
    TextForeground,
    PortalSurface,
    Backdrop,
}

impl UiAppearanceClip {
    pub const fn new(region: UiAppearanceDamageRegion) -> Self {
        Self(region)
    }
    pub const fn region(self) -> UiAppearanceDamageRegion {
        self.0
    }
}

impl UiAppearanceDamageRegion {
    pub const fn new(
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<Self, UiAppearanceEmptyRegion> {
        if width == 0 || height == 0 {
            Err(UiAppearanceEmptyRegion)
        } else {
            Ok(Self {
                x,
                y,
                width,
                height,
            })
        }
    }

    pub const fn x(self) -> i32 {
        self.x
    }
    pub const fn y(self) -> i32 {
        self.y
    }
    pub const fn width(self) -> u32 {
        self.width
    }
    pub const fn height(self) -> u32 {
        self.height
    }
}

impl UiAppearancePhysicalRadii {
    pub fn normalize(bounds: UiAppearanceDamageRegion, authored: [u32; 4]) -> Self {
        let [top_left, top_right, bottom_right, bottom_left] = authored;
        let constraints = [
            (
                u64::from(bounds.width),
                u64::from(top_left) + u64::from(top_right),
            ),
            (
                u64::from(bounds.width),
                u64::from(bottom_left) + u64::from(bottom_right),
            ),
            (
                u64::from(bounds.height),
                u64::from(top_left) + u64::from(bottom_left),
            ),
            (
                u64::from(bounds.height),
                u64::from(top_right) + u64::from(bottom_right),
            ),
        ];
        let (numerator, denominator) = constraints.into_iter().filter(|(_, sum)| *sum != 0).fold(
            (1_u64, 1_u64),
            |best, candidate| {
                if u128::from(candidate.0) * u128::from(best.1)
                    < u128::from(best.0) * u128::from(candidate.1)
                {
                    candidate
                } else {
                    best
                }
            },
        );
        if numerator >= denominator {
            return Self(authored);
        }
        let rounded =
            authored.map(|radius| round_scaled_to_nearest_even(radius, numerator, denominator));
        // Canonical corner order resolves the only ambiguity left by exact
        // nearest-even rounding: two independently rounded half ties can
        // exceed their shared edge by one unit. Earlier corners retain their
        // rounded value and the later adjacent corner is constrained.
        let top_left = rounded[0];
        let top_right = rounded[1].min(bounds.width.saturating_sub(top_left));
        let bottom_right = rounded[2].min(bounds.height.saturating_sub(top_right));
        let bottom_left = rounded[3]
            .min(bounds.width.saturating_sub(bottom_right))
            .min(bounds.height.saturating_sub(top_left));
        Self([top_left, top_right, bottom_right, bottom_left])
    }

    pub const fn corners(self) -> [u32; 4] {
        self.0
    }
}

fn round_scaled_to_nearest_even(value: u32, numerator: u64, denominator: u64) -> u32 {
    let product = u128::from(value) * u128::from(numerator);
    let denominator = u128::from(denominator);
    let quotient = product / denominator;
    let remainder = product % denominator;
    let twice_remainder = remainder * 2;
    let increment =
        twice_remainder > denominator || (twice_remainder == denominator && quotient % 2 == 1);
    (quotient + u128::from(increment)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radii_normalize_proportionally_without_overflow() {
        let bounds = UiAppearanceDamageRegion::new(0, 0, 100, 50).unwrap();
        assert_eq!(
            UiAppearancePhysicalRadii::normalize(bounds, [80, 40, 40, 80]).corners(),
            [25, 12, 12, 25]
        );
        let huge = UiAppearanceDamageRegion::new(0, 0, u32::MAX, u32::MAX).unwrap();
        assert_eq!(
            UiAppearancePhysicalRadii::normalize(huge, [u32::MAX; 4]).corners(),
            [2_147_483_648, 2_147_483_647, 2_147_483_648, 2_147_483_647]
        );
    }

    #[test]
    fn normalization_never_rounds_a_tied_pair_beyond_its_edge() {
        let bounds = UiAppearanceDamageRegion::new(0, 0, 3, 100).unwrap();
        let normalized = UiAppearancePhysicalRadii::normalize(bounds, [3, 3, 0, 0]);
        let [top_left, top_right, _, _] = normalized.corners();
        assert_eq!([top_left, top_right], [2, 1]);
        assert!(top_left + top_right <= bounds.width());
    }

    #[test]
    fn exact_half_ties_use_even_then_canonical_edge_constraint() {
        assert_eq!(round_scaled_to_nearest_even(5, 1, 2), 2);
        assert_eq!(round_scaled_to_nearest_even(7, 1, 2), 4);
        let bounds = UiAppearanceDamageRegion::new(0, 0, 5, 5).unwrap();
        assert_eq!(
            UiAppearancePhysicalRadii::normalize(bounds, [5, 5, 5, 5]).corners(),
            [2, 2, 2, 2]
        );
    }

    #[test]
    fn empty_damage_and_clip_regions_cannot_be_constructed() {
        assert_eq!(
            UiAppearanceDamageRegion::new(0, 0, 0, 1),
            Err(UiAppearanceEmptyRegion)
        );
        assert!(UiAppearanceDamageRegion::new(i32::MIN, i32::MAX, 1, 1).is_ok());
    }
}
