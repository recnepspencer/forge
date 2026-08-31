#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiMountedAppearanceColor([u8; 4]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiMountedAppearanceOpacity(u16);

impl UiMountedAppearanceColor {
    pub const fn from_straight_srgba(channels: [u8; 4]) -> Self {
        Self(channels)
    }
    pub const fn straight_srgba(self) -> [u8; 4] {
        self.0
    }
}

impl UiMountedAppearanceOpacity {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(u16::MAX);

    pub const fn from_units(units: u16) -> Self {
        Self(units)
    }
    pub const fn units(self) -> u16 {
        self.0
    }

    pub fn compose(self, other: Self) -> Self {
        Self(super::compositing::round_ratio_even(
            u128::from(self.0) * u128::from(other.0),
            u128::from(u16::MAX),
        ) as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::UiMountedAppearanceOpacity as Opacity;

    #[test]
    fn opacity_composition_has_exact_zero_and_one_identities() {
        let value = Opacity::from_units(23_417);
        assert_eq!(Opacity::ZERO.compose(value), Opacity::ZERO);
        assert_eq!(value.compose(Opacity::ZERO), Opacity::ZERO);
        assert_eq!(Opacity::ONE.compose(value), value);
        assert_eq!(value.compose(Opacity::ONE), value);
    }

    #[test]
    fn opacity_composition_rounds_deterministically_in_integer_space() {
        assert_eq!(
            Opacity::from_units(32_768)
                .compose(Opacity::from_units(32_768))
                .units(),
            16_384
        );
        let forward = Opacity::from_units(10_000).compose(Opacity::from_units(40_000));
        let reverse = Opacity::from_units(40_000).compose(Opacity::from_units(10_000));
        assert_eq!(forward, reverse);
        assert_eq!(forward.units(), 6_104);
    }
}
