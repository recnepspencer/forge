use forge_foundational::facade::{CanonicalBasisEntry, CanonicalBasisEntryKind};

use super::artifact::ForgeQueryEvidenceIdentity;
use super::foundational::{derive_evidence_identity, text_entry};
use super::scheme::ForgeQueryEvidenceIdentityScheme;
use super::scope::ForgeQueryEvidenceScope;
use super::tag::ForgeQueryEvidenceTag;

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

pub struct ForgeQueryEvidenceIdentityEncoder {
    scope: ForgeQueryEvidenceScope,
    scheme: ForgeQueryEvidenceIdentityScheme,
    entries: Vec<CanonicalBasisEntry>,
}

impl ForgeQueryEvidenceIdentityEncoder {
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

    pub fn field_shape(mut self, tag: ForgeQueryEvidenceTag, value: impl AsRef<str>) -> Self {
        self.push_text(CanonicalBasisEntryKind::Shape, tag, value);
        self
    }

    pub fn field_identity(mut self, tag: ForgeQueryEvidenceTag, value: impl AsRef<str>) -> Self {
        self.push_text(CanonicalBasisEntryKind::Identity, tag, value);
        self
    }

    pub fn field_value(mut self, tag: ForgeQueryEvidenceTag, value: impl AsRef<str>) -> Self {
        self.push_text(CanonicalBasisEntryKind::Value, tag, value);
        self
    }

    pub fn field_bool(mut self, tag: ForgeQueryEvidenceTag, value: bool) -> Self {
        self.push_text(
            CanonicalBasisEntryKind::Value,
            tag,
            if value { "true" } else { "false" },
        );
        self
    }

    pub fn field_usize(mut self, tag: ForgeQueryEvidenceTag, value: usize) -> Self {
        self.push_text(CanonicalBasisEntryKind::Value, tag, value.to_string());
        self
    }

    pub fn field_identity_sequence<I, S>(mut self, tag: ForgeQueryEvidenceTag, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for (index, value) in values.into_iter().enumerate() {
            self.entries.push(text_entry(
                format!("evidence.{}.{}", tag.as_str(), index),
                CanonicalBasisEntryKind::Identity,
                value.as_ref(),
            ));
        }
        self
    }

    pub(crate) fn optional_shape(self, tag: ForgeQueryEvidenceTag, value: Option<&str>) -> Self {
        match value {
            Some(value) => self.field_shape(tag, value),
            None => self,
        }
    }

    pub(crate) fn optional_identity(self, tag: ForgeQueryEvidenceTag, value: Option<&str>) -> Self {
        match value {
            Some(value) => self.field_identity(tag, value),
            None => self,
        }
    }

    pub(crate) fn optional_value(self, tag: ForgeQueryEvidenceTag, value: Option<&str>) -> Self {
        match value {
            Some(value) => self.field_value(tag, value),
            None => self,
        }
    }

    pub fn seal(self) -> ForgeQueryEvidenceIdentity {
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
        self.entries.push(text_entry(
            format!("evidence.{}", tag.as_str()),
            kind,
            value.as_ref(),
        ));
    }
}
