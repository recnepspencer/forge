#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryConsumerOrchestrationPhase {
    Canonicalize,
    Bind,
    Validate,
    Admit,
    Plan,
    Lower,
    Execute,
    AssembleOutcome,
    Inspect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryConsumerOrchestrationErrorKind {
    InvalidRustSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerOrchestrationError {
    kind: WorthQueryConsumerOrchestrationErrorKind,
    source_path: String,
    line: usize,
    column: usize,
    message: String,
}

impl WorthQueryConsumerOrchestrationError {
    pub(crate) fn invalid_rust_source(source_path: &str, error: syn::Error) -> Self {
        let start = error.span().start();
        Self {
            kind: WorthQueryConsumerOrchestrationErrorKind::InvalidRustSource,
            source_path: source_path.to_string(),
            line: start.line,
            column: start.column + 1,
            message: error.to_string(),
        }
    }
    pub fn kind(&self) -> WorthQueryConsumerOrchestrationErrorKind {
        self.kind
    }
    pub fn source_path(&self) -> &str {
        &self.source_path
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl WorthQueryConsumerOrchestrationPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canonicalize => "canonicalize",
            Self::Bind => "bind",
            Self::Validate => "validate",
            Self::Admit => "admit",
            Self::Plan => "plan",
            Self::Lower => "lower",
            Self::Execute => "execute",
            Self::AssembleOutcome => "assemble-outcome",
            Self::Inspect => "inspect",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryConsumerOrchestrationSite {
    path: String,
    line: usize,
    column: usize,
    function_name: String,
}

impl WorthQueryConsumerOrchestrationSite {
    pub(crate) fn new(
        path: impl Into<String>,
        line: usize,
        column: usize,
        function_name: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            line,
            column,
            function_name: function_name.into(),
        }
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn column(&self) -> usize {
        self.column
    }
    pub fn function_name(&self) -> &str {
        &self.function_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerOrchestrationFinding {
    site: WorthQueryConsumerOrchestrationSite,
    phases: Vec<WorthQueryConsumerOrchestrationPhase>,
}

impl WorthQueryConsumerOrchestrationFinding {
    pub(crate) fn new(
        site: WorthQueryConsumerOrchestrationSite,
        phases: Vec<WorthQueryConsumerOrchestrationPhase>,
    ) -> Self {
        Self { site, phases }
    }
    pub fn site(&self) -> &WorthQueryConsumerOrchestrationSite {
        &self.site
    }
    pub fn phases(&self) -> &[WorthQueryConsumerOrchestrationPhase] {
        &self.phases
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerOrchestrationAudit {
    scanned_function_count: usize,
    findings: Vec<WorthQueryConsumerOrchestrationFinding>,
}

impl WorthQueryConsumerOrchestrationAudit {
    pub(crate) fn new(
        scanned_function_count: usize,
        findings: Vec<WorthQueryConsumerOrchestrationFinding>,
    ) -> Self {
        Self {
            scanned_function_count,
            findings,
        }
    }
    pub fn scanned_function_count(&self) -> usize {
        self.scanned_function_count
    }
    pub fn findings(&self) -> &[WorthQueryConsumerOrchestrationFinding] {
        &self.findings
    }
    pub fn has_local_orchestration(&self) -> bool {
        !self.findings.is_empty()
    }
}
