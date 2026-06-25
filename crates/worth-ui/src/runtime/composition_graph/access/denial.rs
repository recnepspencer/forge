use super::super::digest::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAccessDenial {
    code: WorthUiCompositionGraphAccessDenialCode,
    subject: String,
    expected: &'static str,
    denial_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionGraphAccessDenialCode {
    MissingParent,
    MissingNode,
    MissingEdge,
    MissingPolicy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionGraphAccessReport {
    denials: Vec<WorthUiCompositionGraphAccessDenial>,
    denial_set_digest: u64,
}

impl WorthUiCompositionGraphAccessDenial {
    pub(super) fn new(
        code: WorthUiCompositionGraphAccessDenialCode,
        subject: impl Into<String>,
        expected: &'static str,
    ) -> Self {
        let subject = subject.into();
        let denial_digest =
            digest_parts(["composition_graph_access_denial", code.token(), &subject]);
        Self {
            code,
            subject,
            expected,
            denial_digest,
        }
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }

    pub fn code(&self) -> WorthUiCompositionGraphAccessDenialCode {
        self.code
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn expected(&self) -> &'static str {
        self.expected
    }
}

impl WorthUiCompositionGraphAccessReport {
    pub(super) fn from_denials(denials: Vec<WorthUiCompositionGraphAccessDenial>) -> Self {
        let denial_set_digest = digest_parts(
            denials
                .iter()
                .map(|denial| denial.denial_digest().to_string()),
        );
        Self {
            denials,
            denial_set_digest,
        }
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }

    pub fn denials(&self) -> &[WorthUiCompositionGraphAccessDenial] {
        &self.denials
    }
}

impl WorthUiCompositionGraphAccessDenialCode {
    fn token(self) -> &'static str {
        match self {
            Self::MissingParent => "missing_parent",
            Self::MissingNode => "missing_node",
            Self::MissingEdge => "missing_edge",
            Self::MissingPolicy => "missing_policy",
        }
    }
}
