mod artifact;
mod async_resource;
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
pub use comparison::WorthQueryCanonicalDeclarationComparison;
#[allow(unused_imports)]
pub use future_projection::{
    WorthQueryDeclarationFutureProjection, WorthQueryDeclarationFutureProjectionClass,
};
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
