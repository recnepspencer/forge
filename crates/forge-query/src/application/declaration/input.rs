use crate::application::ForgeQueryDomainEntryMarker;

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
    fn declaration_family(&self) -> &'static str;

    fn canonical_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry>;
}
