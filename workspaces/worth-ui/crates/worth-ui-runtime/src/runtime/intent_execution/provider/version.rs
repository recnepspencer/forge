#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiIntentProviderVersion(u16);

impl UiIntentProviderVersion {
    pub const fn stable(version: u16) -> Self {
        assert!(version > 0, "intent provider version must be nonzero");
        Self(version)
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}
