#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UiMeasurementRoundingPosture {
    ExactFloat,
    HostRounded,
    RuntimeRounded,
    DeferredToAllocation,
}

impl UiMeasurementRoundingPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactFloat => "exact_float",
            Self::HostRounded => "host_rounded",
            Self::RuntimeRounded => "runtime_rounded",
            Self::DeferredToAllocation => "deferred_to_allocation",
        }
    }
}
