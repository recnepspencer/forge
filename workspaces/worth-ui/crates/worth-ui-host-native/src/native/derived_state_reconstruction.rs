//! Owner-issued evidence for native derived-state loss and reconstruction.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiNativeDerivedStateLossClass {
    TextAtlasPagesAndIndex,
    TextAtlasPins,
    RetainedDrawList,
    RetainedTarget,
    PresentationAffinity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeDerivedStateReconstructionObservation {
    class: UiNativeDerivedStateLossClass,
    loss_count: u64,
    reconstruction_count: u64,
}

impl UiNativeDerivedStateReconstructionObservation {
    pub(crate) const fn observed(
        class: UiNativeDerivedStateLossClass,
        loss_count: u64,
        reconstruction_count: u64,
    ) -> Self {
        Self {
            class,
            loss_count,
            reconstruction_count,
        }
    }

    pub const fn class(self) -> UiNativeDerivedStateLossClass {
        self.class
    }

    pub const fn loss_count(self) -> u64 {
        self.loss_count
    }

    pub const fn reconstruction_count(self) -> u64 {
        self.reconstruction_count
    }
}
