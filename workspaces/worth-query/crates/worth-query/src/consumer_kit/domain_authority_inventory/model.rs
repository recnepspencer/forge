#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDomainAuthorityClass {
    PackageInput,
    CanonicalInstallation,
    InstalledHandleCapability,
    DerivedIndex,
    DiagnosticProjection,
    PhysicalBoundaryAdapter,
    CompatibilityPath,
    ProhibitedCompetingAuthority,
}

impl WorthQueryDomainAuthorityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PackageInput => "package-input",
            Self::CanonicalInstallation => "canonical-installation",
            Self::InstalledHandleCapability => "installed-handle-capability",
            Self::DerivedIndex => "derived-index",
            Self::DiagnosticProjection => "diagnostic-projection",
            Self::PhysicalBoundaryAdapter => "physical-boundary-adapter",
            Self::CompatibilityPath => "compatibility-path",
            Self::ProhibitedCompetingAuthority => "prohibited-competing-authority",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainAuthorityInventoryRow {
    symbol: &'static str,
    defining_path: &'static str,
    exporting_path: Option<&'static str>,
    current_class: WorthQueryDomainAuthorityClass,
    target_class: WorthQueryDomainAuthorityClass,
    final_owner: &'static str,
}

impl WorthQueryDomainAuthorityInventoryRow {
    pub(crate) const fn new(
        symbol: &'static str,
        defining_path: &'static str,
        exporting_path: Option<&'static str>,
        current_class: WorthQueryDomainAuthorityClass,
        target_class: WorthQueryDomainAuthorityClass,
        final_owner: &'static str,
    ) -> Self {
        Self {
            symbol,
            defining_path,
            exporting_path,
            current_class,
            target_class,
            final_owner,
        }
    }

    pub fn symbol(&self) -> &'static str {
        self.symbol
    }
    pub fn defining_path(&self) -> &'static str {
        self.defining_path
    }
    pub fn exporting_path(&self) -> Option<&'static str> {
        self.exporting_path
    }
    pub fn current_class(&self) -> WorthQueryDomainAuthorityClass {
        self.current_class
    }
    pub fn target_class(&self) -> WorthQueryDomainAuthorityClass {
        self.target_class
    }
    pub fn final_owner(&self) -> &'static str {
        self.final_owner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainAuthoritySource {
    path: String,
    text: String,
}

impl WorthQueryDomainAuthoritySource {
    pub fn new(path: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            text: text.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryDomainAuthoritySourceSite {
    path: String,
    line: usize,
    symbol: String,
}

impl WorthQueryDomainAuthoritySourceSite {
    pub(crate) fn new(path: &str, line: usize, symbol: impl Into<String>) -> Self {
        Self {
            path: path.to_string(),
            line,
            symbol: symbol.into(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDomainAuthorityFindingKind {
    InvalidRustSource,
    UnclassifiedSemanticAuthority,
    MissingClassifiedAuthority,
    DuplicateClassifiedAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainAuthorityFinding {
    kind: WorthQueryDomainAuthorityFindingKind,
    site: WorthQueryDomainAuthoritySourceSite,
}

impl WorthQueryDomainAuthorityFinding {
    pub(crate) fn new(
        kind: WorthQueryDomainAuthorityFindingKind,
        site: WorthQueryDomainAuthoritySourceSite,
    ) -> Self {
        Self { kind, site }
    }

    pub fn kind(&self) -> WorthQueryDomainAuthorityFindingKind {
        self.kind
    }
    pub fn site(&self) -> &WorthQueryDomainAuthoritySourceSite {
        &self.site
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainAuthorityInventoryAudit {
    observed_authority_count: usize,
    findings: Vec<WorthQueryDomainAuthorityFinding>,
}

impl WorthQueryDomainAuthorityInventoryAudit {
    pub(crate) fn new(
        observed_authority_count: usize,
        findings: Vec<WorthQueryDomainAuthorityFinding>,
    ) -> Self {
        Self {
            observed_authority_count,
            findings,
        }
    }

    pub fn observed_authority_count(&self) -> usize {
        self.observed_authority_count
    }
    pub fn findings(&self) -> &[WorthQueryDomainAuthorityFinding] {
        &self.findings
    }
    pub fn is_complete(&self) -> bool {
        self.findings.is_empty()
    }
}
