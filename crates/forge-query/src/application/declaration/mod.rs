mod artifact;
mod comparison;
mod input;
mod raw_input;
mod version;

pub use artifact::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationCanonicalizationError,
};
pub use comparison::ForgeQueryCanonicalDeclarationComparison;
pub use input::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput,
};
pub use version::ForgeQueryDeclarationCanonicalizationVersion;

pub(crate) use artifact::forge_query_canonical_declaration;

#[cfg(test)]
mod capability_tests;

#[cfg(test)]
mod tests;
