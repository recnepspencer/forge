use std::sync::Arc;

use super::artifact::ForgeQueryEvidenceIdentity;
use super::foundational::{derive_evidence_identity, text_entry};
use super::scheme::ForgeQueryEvidenceIdentityScheme;
use super::scope::ForgeQueryEvidenceScope;
use super::tag::ForgeQueryEvidenceTag;
use forge_foundational::facade::{
    CanonicalBasisEntry, CanonicalBasisEntryKind, FoundationalIdentityKind,
};
use forge_runtime_bridge::facade::{
    BridgeIdentityEvidence, BridgeTruthBoundaryBridgedIdentity,
};

pub(crate) fn forge_query_evidence_identity(
    scope: ForgeQueryEvidenceScope,
) -> ForgeQueryEvidenceIdentityEncoder {
    ForgeQueryEvidenceIdentityEncoder::new(scope, ForgeQueryEvidenceIdentityScheme::V1)
}

#[cfg(test)]
pub(crate) fn forge_query_evidence_identity_with_scheme(
    scope: ForgeQueryEvidenceScope,
    scheme: ForgeQueryEvidenceIdentityScheme,
) -> ForgeQueryEvidenceIdentityEncoder {
    ForgeQueryEvidenceIdentityEncoder::new(scope, scheme)
}

pub(crate) struct ForgeQueryEvidenceIdentityEncoder {
    scope: ForgeQueryEvidenceScope,
    scheme: ForgeQueryEvidenceIdentityScheme,
    entries: Vec<CanonicalBasisEntry>,
}

impl ForgeQueryEvidenceIdentityEncoder {
    pub(crate) fn for_scope(scope: ForgeQueryEvidenceScope) -> Self {
        Self::new(scope, ForgeQueryEvidenceIdentityScheme::V1)
    }

    pub(crate) fn new(
        scope: ForgeQueryEvidenceScope,
        scheme: ForgeQueryEvidenceIdentityScheme,
    ) -> Self {
        let entries = vec![
            text_entry(
                "evidence.scheme",
                CanonicalBasisEntryKind::Header,
                scheme.as_str(),
            ),
            text_entry(
                "evidence.scope",
                CanonicalBasisEntryKind::Header,
                scope.as_str(),
            ),
        ];
        Self {
            scope,
            scheme,
            entries,
        }
    }

    pub(crate) fn field_shape(mut self, tag: ForgeQueryEvidenceTag, value: impl AsRef<str>) -> Self {
        self.push_text(CanonicalBasisEntryKind::Shape, tag, value);
        self
    }

    pub(crate) fn field_evidence_identity(
        mut self,
        tag: ForgeQueryEvidenceTag,
        value: &ForgeQueryEvidenceIdentity,
    ) -> Self {
        self.push_text(CanonicalBasisEntryKind::Identity, tag, value.as_str());
        self
    }

    pub(crate) fn field_bridge_authority_identity<Kind>(
        mut self,
        tag: ForgeQueryEvidenceTag,
        value: &BridgeTruthBoundaryBridgedIdentity<Arc<str>, Kind>,
    ) -> Self
    where
        Kind: FoundationalIdentityKind,
    {
        self.push_text(
            CanonicalBasisEntryKind::Identity,
            tag,
            value.value().as_ref(),
        );
        self
    }

    pub(crate) fn field_bridge_retained_evidence_identity(
        mut self,
        tag: ForgeQueryEvidenceTag,
        value: &BridgeIdentityEvidence,
    ) -> Self {
        self.push_text(
            CanonicalBasisEntryKind::Identity,
            tag,
            forge_runtime_bridge::facade::bridge_identity_reporting_label(value),
        );
        self
    }

    pub(crate) fn field_value(mut self, tag: ForgeQueryEvidenceTag, value: impl AsRef<str>) -> Self {
        self.push_text(CanonicalBasisEntryKind::Value, tag, value);
        self
    }

    pub(crate) fn field_bool(mut self, tag: ForgeQueryEvidenceTag, value: bool) -> Self {
        self.push_text(
            CanonicalBasisEntryKind::Value,
            tag,
            if value { "true" } else { "false" },
        );
        self
    }

    pub(crate) fn field_usize(mut self, tag: ForgeQueryEvidenceTag, value: usize) -> Self {
        self.push_text(CanonicalBasisEntryKind::Value, tag, value.to_string());
        self
    }

    pub(crate) fn field_evidence_identity_sequence<'a, I>(
        mut self,
        tag: ForgeQueryEvidenceTag,
        values: I,
    ) -> Self
    where
        I: IntoIterator<Item = &'a ForgeQueryEvidenceIdentity>,
    {
        let mut count = 0usize;
        for (index, value) in values.into_iter().enumerate() {
            self.entries.push(text_entry(
                sequence_item_locus(tag, index),
                CanonicalBasisEntryKind::Identity,
                value.as_str(),
            ));
            count = index + 1;
        }
        self.push_sequence_count(tag, count);
        self
    }

    pub(crate) fn field_value_sequence<I, S>(mut self, tag: ForgeQueryEvidenceTag, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut count = 0usize;
        for (index, value) in values.into_iter().enumerate() {
            self.entries.push(text_entry(
                sequence_item_locus(tag, index),
                CanonicalBasisEntryKind::Value,
                value.as_ref(),
            ));
            count = index + 1;
        }
        self.push_sequence_count(tag, count);
        self
    }

    pub(crate) fn optional_shape(self, tag: ForgeQueryEvidenceTag, value: Option<&str>) -> Self {
        match value {
            Some(value) => self.field_shape(tag, value),
            None => self,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn optional_evidence_identity(
        self,
        tag: ForgeQueryEvidenceTag,
        value: Option<&ForgeQueryEvidenceIdentity>,
    ) -> Self {
        match value {
            Some(value) => self.field_evidence_identity(tag, value),
            None => self,
        }
    }

    pub(crate) fn optional_identity(
        self,
        tag: ForgeQueryEvidenceTag,
        value: Option<impl AsRef<str>>,
    ) -> Self {
        match value.as_ref().map(|value| value.as_ref()) {
            Some(value) => self.field_value(tag, value),
            None => self,
        }
    }

    pub(crate) fn optional_value(self, tag: ForgeQueryEvidenceTag, value: Option<&str>) -> Self {
        match value {
            Some(value) => self.field_value(tag, value),
            None => self,
        }
    }

    pub(crate) fn seal(self) -> ForgeQueryEvidenceIdentity {
        ForgeQueryEvidenceIdentity::new(derive_evidence_identity(
            self.scope,
            self.scheme,
            self.entries,
        ))
    }

    fn push_text(
        &mut self,
        kind: CanonicalBasisEntryKind,
        tag: ForgeQueryEvidenceTag,
        value: impl AsRef<str>,
    ) {
        self.entries
            .push(text_entry(field_locus(tag), kind, value.as_ref()));
    }

    fn push_sequence_count(&mut self, tag: ForgeQueryEvidenceTag, count: usize) {
        self.entries.push(text_entry(
            sequence_count_locus(tag),
            CanonicalBasisEntryKind::Shape,
            count.to_string(),
        ));
    }
}

fn field_locus(tag: ForgeQueryEvidenceTag) -> String {
    format!("evidence.field.{}", tag.as_str())
}

fn sequence_count_locus(tag: ForgeQueryEvidenceTag) -> String {
    format!("evidence.sequence.{}.count", tag.as_str())
}

fn sequence_item_locus(tag: ForgeQueryEvidenceTag, index: usize) -> String {
    format!("evidence.sequence.{}.item.{index}", tag.as_str())
}
