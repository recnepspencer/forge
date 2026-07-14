mod context;
mod cost;
mod declaration;
mod execution;
mod outcome;

pub use context::{inspection_basis, WorthQueryInspectionContext};
pub use cost::WorthQueryInspectionCost;
pub use declaration::{declare, WorthQueryInspectionDeclaration};
pub use outcome::{
    WorthQueryInspectionCompletion, WorthQueryInspectionCounters,
    WorthQueryInspectionMaterialization, WorthQueryInspectionMaterializationKind,
    WorthQueryInspectionNextAction, WorthQueryInspectionOutcome, WorthQueryInspectionReceipt,
    WorthQueryInspectionStop, WorthQueryInspectionStopSource, WorthQueryInspectionUnavailable,
    WorthQueryInspectionUnavailableSource,
};

pub struct WorthQueryInspectionRequest {
    declaration: WorthQueryInspectionDeclaration,
    context: WorthQueryInspectionContext,
}

#[cfg(test)]
mod tests;
