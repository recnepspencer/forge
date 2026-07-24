#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UiHostObservationFamily {
    Viewport,
    DeviceScale,
    PointerMotion,
    PointerButton,
    Keyboard,
    Focus,
    ScrollDelta,
    Clock,
    Tick,
    TextComposition,
    ImeComposition,
}

impl UiHostObservationFamily {
    pub const fn permits_latest_value_coalescing(self) -> bool {
        matches!(
            self,
            Self::Viewport | Self::DeviceScale | Self::PointerMotion | Self::Clock | Self::Tick
        )
    }

    pub const fn requires_lossless_delivery(self) -> bool {
        !self.permits_latest_value_coalescing()
    }
}
