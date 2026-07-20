mod context;
mod declaration;
mod execution;
mod outcome;

pub use context::{writeback, WorthQueryWritebackContext, WorthQueryWritebackContextStop};
pub use declaration::{
    declare_writeback, projected_state_diff, WorthQueryWritebackDeclaration,
    WorthQueryWritebackDeclarationIdentity, WorthQueryWritebackTrigger,
};
pub use outcome::{
    WorthQueryWritebackAftermath, WorthQueryWritebackCompletion, WorthQueryWritebackNextAction,
    WorthQueryWritebackOutcome, WorthQueryWritebackStop, WorthQueryWritebackStopSource,
};

use crate::ordinary::workflow::{
    WorthQueryAdmittedWorkflowEffect, WorthQueryLoweredWorkflowPlan, WorthQueryWorkflowCounters,
};

pub struct WorthQueryWritebackRequest {
    pub(crate) declaration: WorthQueryWritebackDeclaration,
    pub(crate) context: WorthQueryWritebackContext,
}
