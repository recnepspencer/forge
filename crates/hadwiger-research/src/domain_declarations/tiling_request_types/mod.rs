use super::request_types::{require_non_empty, HadwigerResearchDeclarationShapeError};

mod contact_conflict_core_request_types;
mod iteration_packet_request_types;
mod motif_terminal_request_types;
mod periodic_closure_request_types;

pub use contact_conflict_core_request_types::{
    ConflictGraphExtractionDeclaration, CoreExtractionDeclaration, TileContactWitnessDeclaration,
    TilingEquivalenceClassificationDeclaration, TilingReactivationDeclaration,
    TilingSuppressionDeclaration,
};
pub use iteration_packet_request_types::{
    LowerBoundTilingIterationDeclaration, UpperBoundTilingIterationDeclaration,
};
pub use motif_terminal_request_types::{MotifSeedDeclaration, TerminalForcingStudyDeclaration};
pub use periodic_closure_request_types::{
    GeneratedPatternClosureDeclaration, PeriodicQuotientCellDeclaration,
};

fn reject_duplicate_identity(
    values: &[String],
    candidate: &str,
    field: &'static str,
) -> Result<(), HadwigerResearchDeclarationShapeError> {
    if values.iter().any(|existing| existing == candidate) {
        return Err(
            HadwigerResearchDeclarationShapeError::DuplicateIdentityField {
                field,
                value: candidate.to_string(),
            },
        );
    }
    Ok(())
}
