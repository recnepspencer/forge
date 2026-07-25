mod artifact;
mod async_resource;
mod async_resource_identity;
mod comparison;
mod future_projection;
mod input;
mod raw_input;
mod temporal;
mod version;

pub use artifact::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclarationCanonicalizationError,
};
pub use async_resource::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
};
pub use async_resource_identity::{
    WorthQueryAsyncResourceRequestIdentity, WorthQueryAsyncResourceRequestIdentityError,
};
pub use comparison::WorthQueryCanonicalDeclarationComparison;
pub use future_projection::WorthQueryDeclarationFutureProjection;
pub use input::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue, WorthQueryDeclarationInput,
};
pub use temporal::{
    WorthQueryTemporalDeclarationClause, WorthQueryTemporalDeclarationSupport,
    WorthQueryTemporalDuration, WorthQueryTemporalWindowKind,
};
pub use version::WorthQueryDeclarationCanonicalizationVersion;

pub(crate) use artifact::worth_query_canonical_declaration;

#[cfg(test)]
mod capability_tests;

#[cfg(test)]
mod tests;
