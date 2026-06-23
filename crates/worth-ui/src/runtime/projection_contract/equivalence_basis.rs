use super::{WorthUiProjectionFamily, WorthUiProjectionIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiProjectionEquivalenceBasisKind {
    ProjectionDigest,
    ThemeDigest,
    FrameDigest,
}

impl WorthUiProjectionEquivalenceBasisKind {
    pub(crate) fn token(self) -> &'static str {
        match self {
            Self::ProjectionDigest => "projection_digest",
            Self::ThemeDigest => "theme_digest",
            Self::FrameDigest => "frame_digest",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionEquivalenceBasis {
    identity: WorthUiProjectionIdentity,
    family: WorthUiProjectionFamily,
    kind: WorthUiProjectionEquivalenceBasisKind,
    value: u64,
}

impl WorthUiProjectionEquivalenceBasis {
    pub(crate) fn new(
        identity: WorthUiProjectionIdentity,
        family: WorthUiProjectionFamily,
        kind: WorthUiProjectionEquivalenceBasisKind,
        value: u64,
    ) -> Self {
        Self {
            identity,
            family,
            kind,
            value,
        }
    }

    pub fn identity(&self) -> &WorthUiProjectionIdentity {
        &self.identity
    }

    pub fn family(&self) -> WorthUiProjectionFamily {
        self.family
    }

    pub fn kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        self.kind
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn digest(&self) -> u64 {
        fold_bytes(
            fold_bytes(
                fold_bytes(0xcbf2_9ce4_8422_2325, self.identity.as_str().as_bytes()),
                self.family.token().as_bytes(),
            ),
            self.kind.token().as_bytes(),
        )
        .wrapping_add(self.value)
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
