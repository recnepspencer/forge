use crate::undo_family_catalog::TopologyUndoFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyUndoSemanticGraphAdmissionError {
    MissingUndoFamilyDeclaration {
        family_identity: TopologyUndoFamilyIdentity,
    },
    InvalidationReceiptTouchedClosureMismatch {
        touched_closure_digest: String,
        receipt_touched_closure_digest: String,
    },
}
