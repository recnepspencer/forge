mod denial;
mod reference;

pub(crate) use denial::{mismatch_for_page, mismatch_for_reference, mismatch_for_segment};
pub use denial::{
    GenerationCountedReferenceDenial, PhysicalReferenceGenerationMismatch,
    PhysicalReferenceGenerationMismatchKind,
};
pub use reference::{CurrentGenerationPhysicalReference, GenerationCountedPhysicalReference};
