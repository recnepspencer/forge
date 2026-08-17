#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiMountedLogicalDamage {
    bounds: crate::UiMountedCanonicalBox,
}

impl UiMountedLogicalDamage {
    #[doc(hidden)]
    pub const fn from_runtime_mounting(bounds: crate::UiMountedCanonicalBox) -> Self {
        Self { bounds }
    }

    pub const fn bounds(self) -> crate::UiMountedCanonicalBox {
        self.bounds
    }
}
