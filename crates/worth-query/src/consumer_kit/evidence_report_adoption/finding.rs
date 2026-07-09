use super::source_set::WorthQueryEvidenceReportAdoptionResidueClassification;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryEvidenceReportAdoptionSyntaxClass {
    FunctionCall,
    UseImport,
    PathReference,
}

impl WorthQueryEvidenceReportAdoptionSyntaxClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FunctionCall => "function-call",
            Self::UseImport => "use-import",
            Self::PathReference => "path-reference",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryEvidenceReportAdoptionFindingKind {
    CoveredSurfaceUsesWorthDigest,
    UnclassifiedWorthDigestResidue,
}

impl WorthQueryEvidenceReportAdoptionFindingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CoveredSurfaceUsesWorthDigest => "covered-surface-uses-worth-digest",
            Self::UnclassifiedWorthDigestResidue => "unclassified-worth-digest-residue",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEvidenceReportAdoptionFinding {
    kind: WorthQueryEvidenceReportAdoptionFindingKind,
    source_label: String,
    source_path: Option<String>,
    symbol: String,
    syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass,
    classification: WorthQueryEvidenceReportAdoptionResidueClassification,
    line: usize,
    column: usize,
}

impl WorthQueryEvidenceReportAdoptionFinding {
    pub(crate) fn new(
        kind: WorthQueryEvidenceReportAdoptionFindingKind,
        source_label: impl Into<String>,
        source_path: Option<&str>,
        symbol: impl Into<String>,
        syntax_class: WorthQueryEvidenceReportAdoptionSyntaxClass,
        classification: WorthQueryEvidenceReportAdoptionResidueClassification,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            kind,
            source_label: source_label.into(),
            source_path: source_path.map(str::to_string),
            symbol: symbol.into(),
            syntax_class,
            classification,
            line,
            column,
        }
    }

    pub fn kind(&self) -> WorthQueryEvidenceReportAdoptionFindingKind {
        self.kind
    }

    pub fn source_label(&self) -> &str {
        &self.source_label
    }

    pub fn source_path(&self) -> Option<&str> {
        self.source_path.as_deref()
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn syntax_class(&self) -> WorthQueryEvidenceReportAdoptionSyntaxClass {
        self.syntax_class
    }

    pub fn classification(&self) -> WorthQueryEvidenceReportAdoptionResidueClassification {
        self.classification
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
