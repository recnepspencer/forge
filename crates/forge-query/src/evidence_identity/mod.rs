mod artifact;
mod consumer_scope_strings;
mod encoder;
mod foundational;
mod scheme;
mod scope;
mod scope_strings;
mod sealed;
mod tag;

#[cfg(test)]
mod tests;

pub use artifact::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceIdentityComparisonError};
pub(crate) use encoder::ForgeQueryEvidenceIdentityEncoder;
pub use scheme::ForgeQueryEvidenceIdentityScheme;
pub use scope::ForgeQueryEvidenceScope;
pub use tag::ForgeQueryEvidenceTag;

pub(crate) use encoder::forge_query_evidence_identity;
