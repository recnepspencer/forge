use super::WorthQueryNativeAccessKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeSelection {
    request_identity: u64,
    declaration_slot: usize,
}

impl WorthQueryNativeSelection {
    pub(super) fn mint(request_identity: u64, declaration_slot: usize) -> Self {
        Self {
            request_identity,
            declaration_slot,
        }
    }

    pub(super) fn request_identity(&self) -> u64 {
        self.request_identity
    }

    pub(super) fn declaration_slot(&self) -> usize {
        self.declaration_slot
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryNativeKeyResolutionCounters {
    pub declaration_checks: usize,
    pub indexed_slot_lookups: usize,
    pub path_matches: usize,
    pub key_scans: usize,
    pub path_parses: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeKeyResolution {
    key: WorthQueryNativeAccessKey,
    counters: WorthQueryNativeKeyResolutionCounters,
}

impl WorthQueryNativeKeyResolution {
    pub(super) fn new(
        key: WorthQueryNativeAccessKey,
        counters: WorthQueryNativeKeyResolutionCounters,
    ) -> Self {
        Self { key, counters }
    }

    pub fn key(&self) -> &WorthQueryNativeAccessKey {
        &self.key
    }

    pub fn into_key(self) -> WorthQueryNativeAccessKey {
        self.key
    }

    pub fn counters(&self) -> WorthQueryNativeKeyResolutionCounters {
        self.counters
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryNativeSelectionDenialKind {
    DeclarationMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeSelectionDenial {
    kind: WorthQueryNativeSelectionDenialKind,
    counters: WorthQueryNativeKeyResolutionCounters,
}

impl WorthQueryNativeSelectionDenial {
    pub(super) fn new(
        kind: WorthQueryNativeSelectionDenialKind,
        counters: WorthQueryNativeKeyResolutionCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub fn kind(&self) -> WorthQueryNativeSelectionDenialKind {
        self.kind
    }

    pub fn counters(&self) -> WorthQueryNativeKeyResolutionCounters {
        self.counters
    }
}
