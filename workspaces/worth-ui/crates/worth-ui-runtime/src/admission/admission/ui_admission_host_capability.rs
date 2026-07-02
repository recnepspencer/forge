#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAdmissionHostCapability {
    Available,
    Missing,
    Ambiguous,
}

impl UiAdmissionHostCapability {
    pub const fn available() -> Self {
        Self::Available
    }
}
