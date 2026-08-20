#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeClientDerivedStateLossClass {
    MountedLayouts,
    RasterCache,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeClientDerivedStateReconstructionObservation {
    class: UiNativeClientDerivedStateLossClass,
    loss_count: u64,
    reconstruction_count: u64,
    derived_items_lost: u64,
    derived_items_reconstructed: u64,
}

impl UiNativeClientDerivedStateReconstructionObservation {
    #[doc(hidden)]
    pub const fn reported(
        class: UiNativeClientDerivedStateLossClass,
        loss_count: u64,
        reconstruction_count: u64,
        derived_items_lost: u64,
        derived_items_reconstructed: u64,
    ) -> Self {
        Self {
            class,
            loss_count,
            reconstruction_count,
            derived_items_lost,
            derived_items_reconstructed,
        }
    }

    pub const fn class(self) -> UiNativeClientDerivedStateLossClass {
        self.class
    }

    pub const fn loss_count(self) -> u64 {
        self.loss_count
    }

    pub const fn reconstruction_count(self) -> u64 {
        self.reconstruction_count
    }

    pub const fn derived_items_lost(self) -> u64 {
        self.derived_items_lost
    }

    pub const fn derived_items_reconstructed(self) -> u64 {
        self.derived_items_reconstructed
    }
}
