mod context;
mod declaration;
mod execution;
mod outcome;

pub use context::{inspection_basis, WorthQueryInspectionContext};
pub use declaration::{inspect, WorthQueryInspectionDeclaration};
pub use outcome::{
    WorthQueryInspectionCompletion, WorthQueryInspectionCost, WorthQueryInspectionCounters,
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
