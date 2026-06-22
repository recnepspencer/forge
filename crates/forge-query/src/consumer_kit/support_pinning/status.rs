#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryPinnedSupportStatus {
    Supported,
    DeferredDebt,
    Unsupported,
}

impl ForgeQueryPinnedSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::DeferredDebt => "deferred-debt",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryPinnedTeachingPosture {
    OrdinaryRuntimeDx,
    VisibleButDeferred,
    VisibleVocabularyOnly,
    SupportGateOnly,
}

impl ForgeQueryPinnedTeachingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryRuntimeDx => "ordinary-runtime-dx",
            Self::VisibleButDeferred => "visible-but-deferred",
            Self::VisibleVocabularyOnly => "visible-vocabulary-only",
            Self::SupportGateOnly => "support-gate-only",
        }
    }
}
