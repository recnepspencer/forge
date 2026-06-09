mod motif_artifacts;
mod motif_builder;
mod motif_digest_basis;
mod motif_errors;
mod motif_identity;
mod query_lowering;
mod terminal_relation_certificates;
mod terminal_relations;

pub use motif_artifacts::{
    MotifArtifact, MotifGeometryTemplateReference, MotifProofSupportPosture,
};
pub use motif_builder::MotifArtifactBuilder;
pub use motif_errors::MotifLanguageError;
pub use motif_identity::{
    MotifForbiddenSameColorPair, MotifParameterBinding, MotifTerminal, MotifUnitEdge, MotifVertex,
};
pub use query_lowering::{
    build_motif_from_seed_declaration_checked, certify_terminal_forcing_relation_checked,
};
pub use terminal_relation_certificates::{
    TerminalForcingRelationCertificate, TerminalForcingRelationEvidence,
};
pub use terminal_relations::{
    TerminalForcingRelation, TerminalForcingRelationKind, TerminalForcingRelationPosture,
};
