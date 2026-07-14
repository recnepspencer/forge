#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainIdentityComponentError {
    Empty,
    InvalidCharacter,
    InvalidBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainPackageValidationDenialKind {
    MarkerIdentityMismatch,
    MissingMarkerCapability,
    DuplicateInvariant,
    ConflictingInvariant,
    DuplicateGraphObligation,
    ConflictingGraphObligation,
    DuplicateGraphReadOperation,
    ConflictingGraphReadOperation,
    DuplicateDeclarationFamily,
    ConflictingDeclarationFamily,
    DuplicateContributionCategory,
    EmptyGraphReadRelationSet,
    InvalidInvariantPredicate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainInstallationDenialKind {
    DuplicateMarkerType,
    DuplicatePackageIdentity,
    ConflictingDomainOwner,
    ConflictingInvariant,
    ConflictingGraphObligation,
    ConflictingGraphReadOperation,
    AmbiguousGraphReadRelationScope,
    ConflictingDeclarationFamily,
    InvariantLoweringFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainInstallationDenial {
    kind: WorthQueryDomainInstallationDenialKind,
    subject: String,
}

impl WorthQueryDomainInstallationDenial {
    pub(crate) fn new(
        kind: WorthQueryDomainInstallationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryDomainInstallationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryDomainInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "domain installation denied during {}: {}",
            installation_denial_stage(self.kind),
            self.subject
        )
    }
}

impl std::error::Error for WorthQueryDomainInstallationDenial {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainPackageInstallationError {
    Validation(WorthQueryDomainPackageValidationDenial),
    Admission(WorthQueryDomainPackageAdmissionDenial),
    Installation(WorthQueryDomainInstallationDenial),
}

impl WorthQueryDomainPackageInstallationError {
    pub fn validation_denial(&self) -> Option<&WorthQueryDomainPackageValidationDenial> {
        match self {
            Self::Validation(denial) => Some(denial),
            Self::Admission(_) | Self::Installation(_) => None,
        }
    }

    pub fn admission_denial(&self) -> Option<&WorthQueryDomainPackageAdmissionDenial> {
        match self {
            Self::Admission(denial) => Some(denial),
            Self::Validation(_) | Self::Installation(_) => None,
        }
    }

    pub fn installation_denial(&self) -> Option<&WorthQueryDomainInstallationDenial> {
        match self {
            Self::Installation(denial) => Some(denial),
            Self::Validation(_) | Self::Admission(_) => None,
        }
    }
}

impl std::fmt::Display for WorthQueryDomainPackageInstallationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation(denial) => {
                write!(formatter, "domain package validation denied: {denial}")
            }
            Self::Admission(denial) => {
                write!(formatter, "domain package admission denied: {denial}")
            }
            Self::Installation(denial) => denial.fmt(formatter),
        }
    }
}

impl std::error::Error for WorthQueryDomainPackageInstallationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Validation(denial) => Some(denial),
            Self::Admission(denial) => Some(denial),
            Self::Installation(denial) => Some(denial),
        }
    }
}

fn installation_denial_stage(kind: WorthQueryDomainInstallationDenialKind) -> &'static str {
    match kind {
        WorthQueryDomainInstallationDenialKind::DuplicateMarkerType => "marker admission",
        WorthQueryDomainInstallationDenialKind::DuplicatePackageIdentity => "package admission",
        WorthQueryDomainInstallationDenialKind::ConflictingDomainOwner => "owner admission",
        WorthQueryDomainInstallationDenialKind::ConflictingInvariant => "invariant lowering",
        WorthQueryDomainInstallationDenialKind::ConflictingGraphObligation => {
            "graph-obligation lowering"
        }
        WorthQueryDomainInstallationDenialKind::ConflictingGraphReadOperation => {
            "graph-read operation lowering"
        }
        WorthQueryDomainInstallationDenialKind::AmbiguousGraphReadRelationScope => {
            "graph-read relation admission"
        }
        WorthQueryDomainInstallationDenialKind::ConflictingDeclarationFamily => {
            "declaration-family lowering"
        }
        WorthQueryDomainInstallationDenialKind::InvariantLoweringFailed => "invariant compilation",
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainPackageValidationDenial {
    kind: WorthQueryDomainPackageValidationDenialKind,
    detail: String,
}

impl WorthQueryDomainPackageValidationDenial {
    pub(crate) fn new(
        kind: WorthQueryDomainPackageValidationDenialKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryDomainPackageValidationDenialKind {
        self.kind
    }
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for WorthQueryDomainPackageValidationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.detail)
    }
}

impl std::error::Error for WorthQueryDomainPackageValidationDenial {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainPackageAdmissionDenialKind {
    UnsupportedCapability,
    DeferredCapability,
    DisabledConfiguration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainPackageAdmissionDenial {
    kind: WorthQueryDomainPackageAdmissionDenialKind,
    subject: String,
}

impl WorthQueryDomainPackageAdmissionDenial {
    pub(crate) fn new(
        kind: WorthQueryDomainPackageAdmissionDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryDomainPackageAdmissionDenialKind {
        self.kind
    }
    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryDomainPackageAdmissionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}: {}", self.kind, self.subject)
    }
}

impl std::error::Error for WorthQueryDomainPackageAdmissionDenial {}
