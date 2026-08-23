mod action;
mod selection;

pub use action::{
    ModeledSourceCandidateRole, SourcePrecedenceAction, SourcePrecedenceActionKind,
    SourcePrecedenceDenial,
};
pub use selection::{require_selectable_source, SourceAuthorityPosture};
