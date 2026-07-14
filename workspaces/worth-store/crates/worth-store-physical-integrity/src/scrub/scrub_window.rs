use crate::ProtectedPhysicalByteView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubMode {
    Online,
    Offline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrubWindowSource {
    OnlineProtectedRead,
    OfflineDeclaredVerifierInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScrubWindowOrdinal(u64);

impl ScrubWindowOrdinal {
    pub const fn from_zero_based(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubLocalitySummary {
    first_ordinal: ScrubWindowOrdinal,
    last_ordinal: ScrubWindowOrdinal,
    window_count: u64,
    byte_count: u64,
}

impl ScrubLocalitySummary {
    pub(crate) const fn single(window: &ScrubWindow<'_>) -> Self {
        Self {
            first_ordinal: window.ordinal,
            last_ordinal: window.ordinal,
            window_count: 1,
            byte_count: window.len_bytes(),
        }
    }

    pub(crate) const fn merge(self, window: &ScrubWindow<'_>) -> Self {
        Self {
            first_ordinal: self.first_ordinal,
            last_ordinal: window.ordinal,
            window_count: self.window_count + 1,
            byte_count: self.byte_count + window.len_bytes(),
        }
    }

    pub const fn first_ordinal(self) -> ScrubWindowOrdinal {
        self.first_ordinal
    }

    pub const fn last_ordinal(self) -> ScrubWindowOrdinal {
        self.last_ordinal
    }

    pub const fn window_count(self) -> u64 {
        self.window_count
    }

    pub const fn byte_count(self) -> u64 {
        self.byte_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubWindow<'lease> {
    ordinal: ScrubWindowOrdinal,
    source: ScrubWindowSource,
    bytes: &'lease [u8],
}

impl<'lease> ScrubWindow<'lease> {
    pub fn online_from_protected_view(
        ordinal: ScrubWindowOrdinal,
        view: ProtectedPhysicalByteView<'lease>,
    ) -> Self {
        Self {
            ordinal,
            source: ScrubWindowSource::OnlineProtectedRead,
            bytes: view.as_bytes(),
        }
    }

    pub(crate) const fn offline_declared(ordinal: ScrubWindowOrdinal, bytes: &'lease [u8]) -> Self {
        Self {
            ordinal,
            source: ScrubWindowSource::OfflineDeclaredVerifierInput,
            bytes,
        }
    }

    pub const fn ordinal(self) -> ScrubWindowOrdinal {
        self.ordinal
    }

    pub const fn source(self) -> ScrubWindowSource {
        self.source
    }

    pub const fn as_bytes(self) -> &'lease [u8] {
        self.bytes
    }

    pub const fn len_bytes(self) -> u64 {
        self.bytes.len() as u64
    }

    pub const fn is_empty(self) -> bool {
        self.bytes.is_empty()
    }

    pub fn checksum(self) -> u64 {
        self.bytes
            .iter()
            .fold(0u64, |acc, byte| acc.wrapping_add(*byte as u64))
    }
}
