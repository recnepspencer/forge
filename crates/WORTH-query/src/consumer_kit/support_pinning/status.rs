#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryPinnedSupportStatus {
    Supported,
    DeferredDebt,
    Unsupported,
}

impl WorthQueryPinnedSupportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::DeferredDebt => "deferred-debt",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryPinnedTeachingPosture {
    OrdinaryRuntimeDx,
    VisibleButDeferred,
    VisibleVocabularyOnly,
    SupportGateOnly,
}

impl WorthQueryPinnedTeachingPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryRuntimeDx => "ordinary-runtime-dx",
            Self::VisibleButDeferred => "visible-but-deferred",
            Self::VisibleVocabularyOnly => "visible-vocabulary-only",
            Self::SupportGateOnly => "support-gate-only",
        }
    }
}
