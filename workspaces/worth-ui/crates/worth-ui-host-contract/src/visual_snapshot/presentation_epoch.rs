#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiHostPresentationEpoch(u64);

impl UiHostPresentationEpoch {
    #[doc(hidden)]
    pub const fn issued_by_host(value: u64) -> Self {
        Self(value)
    }

    pub const fn diagnostic_value(self) -> u64 {
        self.0
    }
}
