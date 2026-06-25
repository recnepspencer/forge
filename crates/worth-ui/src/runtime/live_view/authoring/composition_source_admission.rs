use crate::runtime::{
    WorthUiCompositionGraphAdmissionDenial, WorthUiCompositionGraphDenialCode,
    WorthUiPrimitiveSourceSpan,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionSourceAdmissionReport {
    denials: Vec<WorthUiCompositionSourceAdmissionDenial>,
    counters: WorthUiCompositionSourceAdmissionCounters,
    denial_set_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionSourceAdmissionDenial {
    code: WorthUiCompositionSourceDenialCode,
    subject: String,
    message: &'static str,
    expected_syntax: &'static str,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
    denial_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionSourceAdmissionCounters {
    node_count: usize,
    edge_count: usize,
    policy_count: usize,
    denial_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionSourceDenialCode {
    StaleControlReference,
    StaleInteractionReference,
    GraphAdmissionDenied(WorthUiCompositionGraphDenialCode),
}

impl WorthUiCompositionSourceAdmissionReport {
    pub(super) fn denied(
        denials: Vec<WorthUiCompositionSourceAdmissionDenial>,
        mut counters: WorthUiCompositionSourceAdmissionCounters,
    ) -> Self {
        counters.denial_count = denials.len();
        let denial_set_digest = digest_parts(denials.iter().map(|denial| {
            format!(
                "{}:{}:{}",
                denial.code.token(),
                denial.subject,
                denial.denial_digest
            )
        }));
        Self {
            denials,
            counters,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiCompositionSourceAdmissionDenial] {
        &self.denials
    }

    pub fn counters(&self) -> WorthUiCompositionSourceAdmissionCounters {
        self.counters
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

impl WorthUiCompositionSourceAdmissionDenial {
    pub(super) fn new(
        code: WorthUiCompositionSourceDenialCode,
        subject: impl Into<String>,
        message: &'static str,
        expected_syntax: &'static str,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        let subject = subject.into();
        let denial_digest = digest_parts([
            code.token().to_owned(),
            subject.clone(),
            message.to_owned(),
            expected_syntax.to_owned(),
        ]);
        Self {
            code,
            subject,
            message,
            expected_syntax,
            source_span,
            denial_digest,
        }
    }

    pub fn code(&self) -> WorthUiCompositionSourceDenialCode {
        self.code
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn expected_syntax(&self) -> &'static str {
        self.expected_syntax
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }

    pub(super) fn graph(
        denial: WorthUiCompositionGraphAdmissionDenial,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self::new(
            WorthUiCompositionSourceDenialCode::GraphAdmissionDenied(denial.code()),
            denial.subject(),
            "composition graph source must lower to an admissible graph",
            "valid composition graph topology",
            source_span,
        )
    }

    pub(super) fn source_span_key(&self) -> (usize, usize) {
        self.source_span
            .map(|span| (span.start_byte(), span.end_byte()))
            .unwrap_or((usize::MAX, usize::MAX))
    }
}

impl WorthUiCompositionSourceAdmissionCounters {
    pub(super) fn new(node_count: usize, edge_count: usize, policy_count: usize) -> Self {
        Self {
            node_count,
            edge_count,
            policy_count,
            denial_count: 0,
        }
    }

    pub fn node_count(self) -> usize {
        self.node_count
    }

    pub fn edge_count(self) -> usize {
        self.edge_count
    }

    pub fn policy_count(self) -> usize {
        self.policy_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }
}

impl WorthUiCompositionSourceDenialCode {
    pub fn token(self) -> &'static str {
        match self {
            Self::StaleControlReference => "composition.source.stale_control_reference",
            Self::StaleInteractionReference => "composition.source.stale_interaction_reference",
            Self::GraphAdmissionDenied(_) => "composition.source.graph_admission_denied",
        }
    }
}

fn digest_parts(parts: impl IntoIterator<Item = impl AsRef<str>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.as_ref().hash(&mut hasher);
    }
    hasher.finish()
}
