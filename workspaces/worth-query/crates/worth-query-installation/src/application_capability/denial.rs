#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationCapabilityInstallationDenialKind {
    CapabilityNotInstalled,
    CapabilityMeaningChanged,
    ForeignRuntime,
    StaleGeneration,
    PackageIdentityChanged,
    SchemaMeaningChanged,
    CanonicalEntryLimitExceeded,
    CanonicalByteLimitExceeded,
    CanonicalDigestSlotRejected,
    AuthorityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationCapabilityInstallationDenial {
    kind: WorthQueryApplicationCapabilityInstallationDenialKind,
    subject: String,
}

impl WorthQueryApplicationCapabilityInstallationDenial {
    pub(crate) fn new(
        kind: WorthQueryApplicationCapabilityInstallationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryApplicationCapabilityInstallationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryApplicationCapabilityInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application capability installation denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryApplicationCapabilityInstallationDenial {}
