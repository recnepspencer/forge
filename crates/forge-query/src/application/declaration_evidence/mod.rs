mod artifact;
mod bundle;
mod class;
mod denial;
mod provenance;
mod receipt;
mod subject;
mod support;

pub use artifact::ForgeQueryDeclarationFoundationalEvidence;
pub use class::{
    ForgeQueryDeclarationFoundationalEvidenceChecked,
    ForgeQueryDeclarationFoundationalEvidenceClass,
};
pub use denial::ForgeQueryDeclarationFoundationalEvidenceDenial;
pub use subject::ForgeQueryDeclarationFoundationalEvidenceInput;

pub(crate) use artifact::forge_query_declaration_foundational_evidence;

#[cfg(test)]
mod tests;
