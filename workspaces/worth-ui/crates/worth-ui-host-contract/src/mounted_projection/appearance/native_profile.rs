#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiHostAppearanceMechanicFamily {
    SurfaceFill,
    SurfaceBorder,
    CornerRadii,
    Outline,
    TextRangeForeground,
    PortalSurface,
    Backdrop,
    OverlayOrder,
    PointerAffordance,
    Damage,
    Clip,
}

impl UiHostAppearanceMechanicFamily {
    pub const ALL: [Self; 11] = [
        Self::SurfaceFill,
        Self::SurfaceBorder,
        Self::CornerRadii,
        Self::Outline,
        Self::TextRangeForeground,
        Self::PortalSurface,
        Self::Backdrop,
        Self::OverlayOrder,
        Self::PointerAffordance,
        Self::Damage,
        Self::Clip,
    ];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiHostAppearanceProfileContract {
    identity: Box<str>,
    version: u16,
    mechanics: Box<[UiHostAppearanceMechanicFamily]>,
    primary_pointer: Option<super::UiHostPrimaryPointerKind>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostAppearanceProfileDenial {
    EmptyIdentity,
    DuplicateMechanic,
    MissingRequiredMechanic,
}

impl UiHostAppearanceProfileContract {
    pub fn admit(
        identity: impl Into<Box<str>>,
        version: u16,
        mechanics: impl IntoIterator<Item = UiHostAppearanceMechanicFamily>,
        primary_pointer: Option<super::UiHostPrimaryPointerKind>,
    ) -> Result<Self, UiHostAppearanceProfileDenial> {
        let identity = identity.into();
        if identity.is_empty() {
            return Err(UiHostAppearanceProfileDenial::EmptyIdentity);
        }
        let mut mechanics = mechanics.into_iter().collect::<Vec<_>>();
        mechanics.sort();
        if mechanics.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(UiHostAppearanceProfileDenial::DuplicateMechanic);
        }
        let mut required = UiHostAppearanceMechanicFamily::ALL.to_vec();
        required.sort();
        if mechanics != required {
            return Err(UiHostAppearanceProfileDenial::MissingRequiredMechanic);
        }
        Ok(Self {
            identity,
            version,
            mechanics: mechanics.into_boxed_slice(),
            primary_pointer,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub const fn version(&self) -> u16 {
        self.version
    }
    pub fn mechanics(&self) -> &[UiHostAppearanceMechanicFamily] {
        &self.mechanics
    }
    pub const fn primary_pointer(&self) -> Option<super::UiHostPrimaryPointerKind> {
        self.primary_pointer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_requires_the_exhaustive_typed_mechanic_family() {
        assert!(UiHostAppearanceProfileContract::admit(
            "worth-ui-windows-dx12-v2",
            2,
            UiHostAppearanceMechanicFamily::ALL,
            Some(super::super::UiHostPrimaryPointerKind::Mouse),
        )
        .is_ok());
        assert_eq!(
            UiHostAppearanceProfileContract::admit(
                "worth-ui-windows-dx12-v2",
                2,
                [UiHostAppearanceMechanicFamily::SurfaceFill],
                None,
            ),
            Err(UiHostAppearanceProfileDenial::MissingRequiredMechanic)
        );
    }
}
