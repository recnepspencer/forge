use super::source_set::ForgeQueryEvidenceReportAdoptionResidueClassification;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEvidenceReportAdoptionSyntaxClass {
    FunctionCall,
    UseImport,
    PathReference,
}

impl ForgeQueryEvidenceReportAdoptionSyntaxClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FunctionCall => "function-call",
            Self::UseImport => "use-import",
            Self::PathReference => "path-reference",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEvidenceReportAdoptionFindingKind {
    CoveredSurfaceUsesWorthDigest,
    UnclassifiedWorthDigestResidue,
}

impl ForgeQueryEvidenceReportAdoptionFindingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CoveredSurfaceUsesWorthDigest => "covered-surface-uses-worth-digest",
            Self::UnclassifiedWorthDigestResidue => "unclassified-worth-digest-residue",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEvidenceReportAdoptionFinding {
    kind: ForgeQueryEvidenceReportAdoptionFindingKind,
    source_label: String,
    source_path: Option<String>,
    symbol: String,
    syntax_class: ForgeQueryEvidenceReportAdoptionSyntaxClass,
    classification: ForgeQueryEvidenceReportAdoptionResidueClassification,
    line: usize,
    column: usize,
}

impl ForgeQueryEvidenceReportAdoptionFinding {
    pub(crate) fn new(
        kind: ForgeQueryEvidenceReportAdoptionFindingKind,
        source_label: impl Into<String>,
        source_path: Option<&str>,
        symbol: impl Into<String>,
        syntax_class: ForgeQueryEvidenceReportAdoptionSyntaxClass,
        classification: ForgeQueryEvidenceReportAdoptionResidueClassification,
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

    pub fn kind(&self) -> ForgeQueryEvidenceReportAdoptionFindingKind {
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

    pub fn syntax_class(&self) -> ForgeQueryEvidenceReportAdoptionSyntaxClass {
        self.syntax_class
    }

    pub fn classification(&self) -> ForgeQueryEvidenceReportAdoptionResidueClassification {
        self.classification
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
