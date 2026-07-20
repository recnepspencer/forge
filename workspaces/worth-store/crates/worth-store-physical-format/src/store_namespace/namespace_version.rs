#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreNamespaceVersion(u16);

impl StoreNamespaceVersion {
    pub const CURRENT: Self = Self(1);

    pub const fn value(self) -> u16 {
        self.0
    }
}
