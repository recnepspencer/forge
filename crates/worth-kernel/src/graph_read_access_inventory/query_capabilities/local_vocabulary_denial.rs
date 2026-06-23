use super::capability_row::QueryGraphReadAccessCapabilityKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthLocalGraphReadAccessVocabularyDenialKind {
    UnknownQueryGraphReadAccessLabel,
    WrongCapabilityKind {
        expected: QueryGraphReadAccessCapabilityKind,
        actual: QueryGraphReadAccessCapabilityKind,
    },
    WrongAuthorityFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthLocalGraphReadAccessVocabularyDenial {
    rejected_label: String,
    kind: WorthLocalGraphReadAccessVocabularyDenialKind,
}

impl WorthLocalGraphReadAccessVocabularyDenial {
    pub(super) fn unknown(rejected_label: &str) -> Self {
        Self {
            rejected_label: rejected_label.to_string(),
            kind: WorthLocalGraphReadAccessVocabularyDenialKind::UnknownQueryGraphReadAccessLabel,
        }
    }

    pub(super) fn wrong_capability_kind(
        rejected_label: &str,
        expected: QueryGraphReadAccessCapabilityKind,
        actual: QueryGraphReadAccessCapabilityKind,
    ) -> Self {
        Self {
            rejected_label: rejected_label.to_string(),
            kind: WorthLocalGraphReadAccessVocabularyDenialKind::WrongCapabilityKind {
                expected,
                actual,
            },
        }
    }

    pub(super) fn wrong_authority_family(rejected_label: &str) -> Self {
        Self {
            rejected_label: rejected_label.to_string(),
            kind: WorthLocalGraphReadAccessVocabularyDenialKind::WrongAuthorityFamily,
        }
    }

    pub fn rejected_label(&self) -> &str {
        &self.rejected_label
    }

    pub fn kind(&self) -> WorthLocalGraphReadAccessVocabularyDenialKind {
        self.kind
    }

    pub fn requires_query_owned_vocabulary(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueryGraphReadAccessLabelAdmission {
    label: &'static str,
    kind: QueryGraphReadAccessCapabilityKind,
}

impl QueryGraphReadAccessLabelAdmission {
    pub(super) fn new(label: &'static str, kind: QueryGraphReadAccessCapabilityKind) -> Self {
        Self { label, kind }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn kind(&self) -> QueryGraphReadAccessCapabilityKind {
        self.kind
    }
}
