#[doc(hidden)]
pub struct UiGateDPinWorldEvidence {
    pub(super) mounted_bindings: u32,
    pub(super) pinned_layouts: u32,
    pub(super) expected_pin_count: u32,
    pub(super) native_committed_pin_count: u32,
    pub(super) native_peak_pin_count: u32,
    pub(super) physical_signal_runtimes: u32,
    pub(super) pressure_transactions: u32,
    pub(super) pressure_releases: u32,
    pub(super) evictions: u32,
    pub(super) atlas_transactions: u32,
    pub(super) local_owner_releases: u32,
    pub(super) native_final_releases: u32,
    pub(super) terminal_zero: bool,
    pub(super) rasterized_glyphs: u32,
}

impl UiGateDPinWorldEvidence {
    pub const fn mounted_bindings(&self) -> u32 {
        self.mounted_bindings
    }

    pub const fn pinned_layouts(&self) -> u32 {
        self.pinned_layouts
    }

    pub const fn expected_pin_count(&self) -> u32 {
        self.expected_pin_count
    }

    pub const fn native_committed_pin_count(&self) -> u32 {
        self.native_committed_pin_count
    }

    pub const fn native_peak_pin_count(&self) -> u32 {
        self.native_peak_pin_count
    }

    pub const fn physical_signal_runtimes(&self) -> u32 {
        self.physical_signal_runtimes
    }

    pub const fn pressure_transactions(&self) -> u32 {
        self.pressure_transactions
    }

    pub const fn pressure_releases(&self) -> u32 {
        self.pressure_releases
    }

    pub const fn evictions(&self) -> u32 {
        self.evictions
    }

    pub const fn local_owner_releases(&self) -> u32 {
        self.local_owner_releases
    }

    pub const fn atlas_transactions(&self) -> u32 {
        self.atlas_transactions
    }

    pub const fn native_final_releases(&self) -> u32 {
        self.native_final_releases
    }

    pub const fn terminal_zero(&self) -> bool {
        self.terminal_zero
    }

    pub const fn rasterized_glyphs(&self) -> u32 {
        self.rasterized_glyphs
    }
}
