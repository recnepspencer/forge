use std::num::NonZeroU32;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default,
)]
pub(crate) struct PendingCauseSetId {
    pub(super) index: Option<NonZeroU32>,
    pub(super) generation: u32,
}

impl PendingCauseSetId {
    pub(crate) const EMPTY: Self = Self {
        index: None,
        generation: 0,
    };

    #[cfg(test)]
    pub(crate) const fn generation(self) -> u32 {
        self.generation
    }
}
