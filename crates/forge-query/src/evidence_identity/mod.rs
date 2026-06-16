mod artifact;
mod encoder;
mod foundational;
mod scheme;
mod scope;
mod sealed;
mod tag;
mod terminal_reporting;

#[cfg(test)]
mod tests;

pub use artifact::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceIdentityComparisonError};
pub(crate) use encoder::ForgeQueryEvidenceIdentityEncoder;
pub use scheme::ForgeQueryEvidenceIdentityScheme;
pub use scope::ForgeQueryEvidenceScope;
pub use tag::ForgeQueryEvidenceTag;

pub(crate) use encoder::forge_query_evidence_identity;
pub(crate) use terminal_reporting::evidence_identity_reporting_label;
