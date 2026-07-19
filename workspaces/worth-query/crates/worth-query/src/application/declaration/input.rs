use crate::application::{WorthQueryDeclarationFamilyMarker, WorthQueryDomainEntryMarker};

use super::async_resource::WorthQueryAsyncDeclarationClause;
use super::temporal::WorthQueryTemporalDeclarationClause;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationCanonicalEntryKind {
    Header,
    Shape,
    Value,
    Field,
    Identity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationCanonicalValue {
    Null,
    Bool(bool),
    SignedInteger(i128),
    UnsignedInteger(u128),
    ExactText(String),
    DecimalText(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationCanonicalEntry {
    locus: String,
    kind: WorthQueryDeclarationCanonicalEntryKind,
    value: WorthQueryDeclarationCanonicalValue,
}

impl WorthQueryDeclarationCanonicalEntry {
    pub fn new(
        locus: impl Into<String>,
        kind: WorthQueryDeclarationCanonicalEntryKind,
        value: WorthQueryDeclarationCanonicalValue,
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
            WorthQueryDeclarationCanonicalEntryKind::Field,
            WorthQueryDeclarationCanonicalValue::ExactText(value.into()),
        )
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn kind(&self) -> WorthQueryDeclarationCanonicalEntryKind {
        self.kind
    }

    pub fn value(&self) -> &WorthQueryDeclarationCanonicalValue {
        &self.value
    }
}

pub trait WorthQueryDeclarationInput<D: WorthQueryDomainEntryMarker> {
    type Family: WorthQueryDeclarationFamilyMarker<D>;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry>;

    fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
        Vec::new()
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        Vec::new()
    }
}
