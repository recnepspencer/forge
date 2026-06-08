mod artifact;
mod async_resource;
mod comparison;
mod future_projection;
mod input;
mod raw_input;
mod temporal;
mod version;

pub use artifact::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationCanonicalizationError,
};
pub use async_resource::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
};
pub use comparison::ForgeQueryCanonicalDeclarationComparison;
#[allow(unused_imports)]
pub use future_projection::{
    ForgeQueryDeclarationFutureProjection, ForgeQueryDeclarationFutureProjectionClass,
};
pub use input::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput,
};
pub use temporal::{
    ForgeQueryTemporalDeclarationClause, ForgeQueryTemporalDeclarationSupport,
    ForgeQueryTemporalDuration, ForgeQueryTemporalWindowKind,
};
pub use version::ForgeQueryDeclarationCanonicalizationVersion;

pub(crate) use artifact::forge_query_canonical_declaration;

#[cfg(test)]
mod capability_tests;

#[cfg(test)]
mod tests;
