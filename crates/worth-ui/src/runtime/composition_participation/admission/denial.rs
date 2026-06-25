use super::super::projection::digest_parts;
use super::super::projection::WorthUiAccessibilityAssociationKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionParticipationDenial {
    code: WorthUiCompositionParticipationDenialCode,
    association_kind: WorthUiAccessibilityAssociationKind,
    source_identity: String,
    target_identity: String,
    denial_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionParticipationDenialCode {
    MissingSourceNode,
    MissingTargetNode,
    InvalidSourceKind,
    InvalidTargetKind,
    SourceNotAccessible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionParticipationDenialReport {
    denials: Vec<WorthUiCompositionParticipationDenial>,
    counters: WorthUiCompositionParticipationDenialCounters,
    denial_set_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionParticipationDenialCounters {
    declaration_count: usize,
    denial_count: usize,
    source_span_ready_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiCompositionParticipationDenial {
    pub(crate) fn new(
        code: WorthUiCompositionParticipationDenialCode,
        association_kind: WorthUiAccessibilityAssociationKind,
        source_identity: impl Into<String>,
        target_identity: impl Into<String>,
    ) -> Self {
        let source_identity = source_identity.into();
        let target_identity = target_identity.into();
        let denial_digest = digest_parts([
            "composition_participation_denial",
            code.token(),
            association_kind.token(),
            source_identity.as_str(),
            target_identity.as_str(),
        ]);
        Self {
            code,
            association_kind,
            source_identity,
            target_identity,
            denial_digest,
        }
    }

    pub fn code(&self) -> WorthUiCompositionParticipationDenialCode {
        self.code
    }

    pub fn association_kind(&self) -> WorthUiAccessibilityAssociationKind {
        self.association_kind
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn target_identity(&self) -> &str {
        &self.target_identity
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }
}

impl WorthUiCompositionParticipationDenialReport {
    pub(crate) fn denied(
        denials: Vec<WorthUiCompositionParticipationDenial>,
        declaration_count: usize,
    ) -> Self {
        let counters =
            WorthUiCompositionParticipationDenialCounters::new(declaration_count, denials.len());
        let denial_set_digest = digest_parts(
            ["composition_participation_denial_set".to_owned()]
                .into_iter()
                .chain(
                    denials
                        .iter()
                        .map(|denial| denial.denial_digest().to_string()),
                ),
        );
        Self {
            denials,
            counters,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiCompositionParticipationDenial] {
        &self.denials
    }

    pub fn counters(&self) -> WorthUiCompositionParticipationDenialCounters {
        self.counters
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

impl WorthUiCompositionParticipationDenialCounters {
    fn new(declaration_count: usize, denial_count: usize) -> Self {
        Self {
            declaration_count,
            denial_count,
            source_span_ready_count: 0,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn declaration_count(self) -> usize {
        self.declaration_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }

    pub fn source_span_ready_count(self) -> usize {
        self.source_span_ready_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

impl WorthUiCompositionParticipationDenialCode {
    pub const fn token(self) -> &'static str {
        match self {
            Self::MissingSourceNode => "missing_source_node",
            Self::MissingTargetNode => "missing_target_node",
            Self::InvalidSourceKind => "invalid_source_kind",
            Self::InvalidTargetKind => "invalid_target_kind",
            Self::SourceNotAccessible => "source_not_accessible",
        }
    }
}
