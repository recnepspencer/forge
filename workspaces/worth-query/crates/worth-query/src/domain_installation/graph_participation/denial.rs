#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphParticipationInstallationDenialKind {
    InvalidDefinition,
    ConflictingDefinition,
    MissingProvider,
    ExtraProvider,
    DuplicateProvider,
    CommitAuthorityRequired,
    UnexpectedCommitAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphParticipationInstallationDenial {
    kind: WorthQueryGraphParticipationInstallationDenialKind,
    detail: String,
}

impl WorthQueryGraphParticipationInstallationDenial {
    pub(crate) fn new(
        kind: WorthQueryGraphParticipationInstallationDenialKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryGraphParticipationInstallationDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGraphParticipationLookupCounters {
    pub indexed_lookups: usize,
    pub provider_contacts: usize,
    pub unrelated_graph_scans: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphParticipationLookupDenialKind {
    NotInstalled,
    ForeignRuntime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphParticipationLookupDenial {
    kind: WorthQueryGraphParticipationLookupDenialKind,
    counters: WorthQueryGraphParticipationLookupCounters,
}

impl WorthQueryGraphParticipationLookupDenial {
    pub(crate) fn new(kind: WorthQueryGraphParticipationLookupDenialKind) -> Self {
        Self {
            kind,
            counters: WorthQueryGraphParticipationLookupCounters {
                indexed_lookups: 1,
                provider_contacts: 0,
                unrelated_graph_scans: 0,
            },
        }
    }

    pub fn kind(&self) -> WorthQueryGraphParticipationLookupDenialKind {
        self.kind
    }

    pub fn counters(&self) -> WorthQueryGraphParticipationLookupCounters {
        self.counters
    }
}
