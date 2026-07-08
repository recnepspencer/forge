mod ui_declaration_artifact;
mod ui_declaration_digest;
mod ui_declaration_digest_projection;
mod ui_declaration_identity;
pub(crate) mod ui_declaration_lowering;
mod ui_declaration_provenance;

pub use ui_declaration_artifact::UiDeclarationArtifact;
pub use ui_declaration_digest::{
    UiDeclarationArtifactDigest, UiDeclarationAspectDigest, UiDeclarationFamilyDigest,
    UiDeclarationIdentityDigest, UiDeclarationPostureDigest, UiDeclarationStructuralDigest,
    UiDeclarationSupportDigest,
};
pub use ui_declaration_digest_projection::UiDeclarationDigestProjection;
pub(crate) use ui_declaration_identity::{
    authored_source_provenance_digest, stable_text_digest,
};
pub use ui_declaration_identity::{UiDeclarationEquivalenceContract, UiDeclarationIdentity};
pub use ui_declaration_provenance::UiDeclarationProvenance;
