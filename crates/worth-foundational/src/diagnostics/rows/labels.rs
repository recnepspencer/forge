use crate::diagnostics::outcomes::FoundationalDiagnosticAbsenceCause;
use crate::diagnostics::primitives::{
    FoundationalDiagnosticBreachClass, FoundationalDiagnosticCodeId,
    FoundationalDiagnosticEvidencePosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticRowFamily {
    Decision,
    Failure,
    Comparison,
    Support,
    ProvenanceReady,
}

impl FoundationalDiagnosticRowFamily {
    pub const fn canonical_name(&self) -> &'static str {
        match self {
            Self::Decision => "decision",
            Self::Failure => "failure",
            Self::Comparison => "comparison",
            Self::Support => "support",
            Self::ProvenanceReady => "provenance_ready",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticLocalityClaim {
    ExactSubject,
    SubjectNeighborhood,
    WidenedScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalDiagnosticWidenedFalloutPosture {
    NotWidened,
    WidenedExpected,
    WidenedUnexpected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalDiagnosticSemanticLabelSet(Vec<FoundationalDiagnosticCodeId>);

impl FoundationalDiagnosticSemanticLabelSet {
    pub fn new(labels: impl IntoIterator<Item = FoundationalDiagnosticCodeId>) -> Self {
        let mut labels: Vec<_> = labels.into_iter().collect();
        labels.sort_by(|left, right| left.as_str().cmp(right.as_str()));
        labels.dedup_by(|left, right| left.as_str() == right.as_str());
        Self(labels)
    }

    pub fn labels(&self) -> &[FoundationalDiagnosticCodeId] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticSupportEvidencePosture {
    Present(FoundationalDiagnosticEvidencePosture),
    Absent(FoundationalDiagnosticAbsenceCause),
    OmittedConstructionBug(FoundationalDiagnosticBreachClass),
}
