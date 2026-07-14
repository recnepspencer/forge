use crate::StoreJsonResidueOccurrence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreJsonResidueDenial {
    SourceScanFailed(String),
    MissingClassification(StoreJsonResidueOccurrence),
    InvalidClassification(StoreJsonResidueOccurrence),
    ForbiddenDedicatedWorkspaceProduction(StoreJsonResidueOccurrence),
    OrdinaryPreludeJsonExport(StoreJsonResidueOccurrence),
}
