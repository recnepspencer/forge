use crate::application::{ForgeQueryDeclarationFamilyMarker, ForgeQueryDomainEntryMarker};

use super::async_resource::ForgeQueryAsyncDeclarationClause;
use super::temporal::ForgeQueryTemporalDeclarationClause;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationCanonicalEntryKind {
    Header,
    Shape,
    Value,
    Field,
    Identity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationCanonicalValue {
    Null,
    Bool(bool),
    SignedInteger(i128),
    UnsignedInteger(u128),
    ExactText(String),
    DecimalText(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationCanonicalEntry {
    locus: String,
    kind: ForgeQueryDeclarationCanonicalEntryKind,
    value: ForgeQueryDeclarationCanonicalValue,
}

impl ForgeQueryDeclarationCanonicalEntry {
    pub fn new(
        locus: impl Into<String>,
        kind: ForgeQueryDeclarationCanonicalEntryKind,
        value: ForgeQueryDeclarationCanonicalValue,
    ) -> Self {
        Self {
            locus: locus.into(),
            kind,
            value,
        }
    }

    pub fn text(locus: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(
            locus,
            ForgeQueryDeclarationCanonicalEntryKind::Field,
            ForgeQueryDeclarationCanonicalValue::ExactText(value.into()),
        )
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn kind(&self) -> ForgeQueryDeclarationCanonicalEntryKind {
        self.kind
    }

    pub fn value(&self) -> &ForgeQueryDeclarationCanonicalValue {
        &self.value
    }
}

pub trait ForgeQueryDeclarationInput<D: ForgeQueryDomainEntryMarker> {
    type Family: ForgeQueryDeclarationFamilyMarker<D>;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry>;

    fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
        Vec::new()
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        Vec::new()
    }
}
