#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarativeSurfaceSource {
    path: String,
    text: String,
}

impl WorthQueryDeclarativeSurfaceSource {
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
pub struct WorthQueryDeclarativeSurfaceSourceSite {
    path: String,
    line: usize,
    owner: Option<String>,
    function_name: String,
}

impl WorthQueryDeclarativeSurfaceSourceSite {
    pub(crate) fn new(path: &str, line: usize, function_name: &str) -> Self {
        Self {
            path: path.to_string(),
            line,
            owner: None,
            function_name: function_name.to_string(),
        }
    }

    pub(crate) fn method(path: &str, line: usize, owner: &str, function_name: &str) -> Self {
        Self {
            path: path.to_string(),
            line,
            owner: Some(owner.to_string()),
            function_name: function_name.to_string(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    pub fn owner(&self) -> Option<&str> {
        self.owner.as_deref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryDeclarativeSurfaceFindingKind {
    InvalidRustSource,
    UnclassifiedPublicPhaseSurface,
    QuarantinedPhaseSurfaceStillPublic,
    MissingRegisteredSurface,
    DuplicatePublicPhaseSurface,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarativeSurfaceFinding {
    kind: WorthQueryDeclarativeSurfaceFindingKind,
    site: WorthQueryDeclarativeSurfaceSourceSite,
}

impl WorthQueryDeclarativeSurfaceFinding {
    pub(crate) fn new(
        kind: WorthQueryDeclarativeSurfaceFindingKind,
        site: WorthQueryDeclarativeSurfaceSourceSite,
    ) -> Self {
        Self { kind, site }
    }

    pub fn kind(&self) -> WorthQueryDeclarativeSurfaceFindingKind {
        self.kind
    }

    pub fn site(&self) -> &WorthQueryDeclarativeSurfaceSourceSite {
        &self.site
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarativeSurfaceAudit {
    observed_surface_count: usize,
    classified_surface_count: usize,
    findings: Vec<WorthQueryDeclarativeSurfaceFinding>,
}

impl WorthQueryDeclarativeSurfaceAudit {
    pub(crate) fn new(
        observed_surface_count: usize,
        classified_surface_count: usize,
        findings: Vec<WorthQueryDeclarativeSurfaceFinding>,
    ) -> Self {
        Self {
            observed_surface_count,
            classified_surface_count,
            findings,
        }
    }

    pub fn observed_surface_count(&self) -> usize {
        self.observed_surface_count
    }

    pub fn classified_surface_count(&self) -> usize {
        self.classified_surface_count
    }

    pub fn findings(&self) -> &[WorthQueryDeclarativeSurfaceFinding] {
        &self.findings
    }

    pub fn is_complete(&self) -> bool {
        self.findings.is_empty()
    }
}
