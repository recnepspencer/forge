#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactDisposition {
    Produced,
    Transferred,
    Borrowed,
    Leased,
    Replaced,
    Cancelled,
    Released,
    Disposed,
}

impl WorthQueryArtifactDisposition {
    pub(crate) const fn canonical_name(self) -> &'static str {
        match self {
            Self::Produced => "produced",
            Self::Transferred => "transferred",
            Self::Borrowed => "borrowed",
            Self::Leased => "leased",
            Self::Replaced => "replaced",
            Self::Cancelled => "cancelled",
            Self::Released => "released",
            Self::Disposed => "disposed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDisposedArtifact {
    owner_identity: String,
    occurrence_identity: String,
    provider_disposed: bool,
}

impl WorthQueryDisposedArtifact {
    pub(super) fn new(
        owner_identity: String,
        occurrence_identity: String,
        provider_disposed: bool,
    ) -> Self {
        Self {
            owner_identity,
            occurrence_identity,
            provider_disposed,
        }
    }

    pub fn owner_identity(&self) -> &str {
        &self.owner_identity
    }

    pub fn occurrence_identity(&self) -> &str {
        &self.occurrence_identity
    }

    pub const fn provider_disposed(&self) -> bool {
        self.provider_disposed
    }
}
