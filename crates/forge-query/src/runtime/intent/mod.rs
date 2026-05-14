use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;
use serde_json::Value;

use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryMutationReceipt, ForgeQueryWorkspaceError};

use super::{
    ForgeQueryAuthorityLane, ForgeQueryBranchBasisAdmission, ForgeQueryEffectAction,
    ForgeQueryEffectAdmission, ForgeQueryEffectPolicy, ForgeQueryPreviewBasisAdmission,
    ForgeQueryWriteReceipt,
};

pub trait ForgeQueryIntentAuthorityAdapter {
    fn execute_intent(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError>;
}

mod admission;
mod branch;
mod declaration;
mod denial;
mod effect_triggered;
mod execution;
mod failure;
mod preview;
mod provenance;
mod receipt;

pub(crate) use admission::{
    admit_authoritative_intent_declaration, admit_authoritative_intent_execution,
    admit_effect_triggered_intent_declaration, ForgeQueryIntentAdmissionDenial,
};
pub(in crate::runtime) use admission::{
    admit_branch_intent_declaration, admit_preview_intent_declaration,
};
pub use branch::ForgeQueryBranchIntentReceipt;
pub use declaration::{ForgeQueryIntentDeclaration, ForgeQueryIntentSourceLane};
pub use denial::ForgeQueryIntentDenialEvidence;
pub use effect_triggered::ForgeQueryEffectIntentReceipt;
pub use execution::{ForgeQueryIntentExecution, ForgeQueryIntentExecutionKind};
pub use failure::ForgeQueryIntentExecutionFailureEvidence;
pub use preview::ForgeQueryPreviewIntentReceipt;
pub use provenance::ForgeQueryIntentExecutionProvenance;
pub use receipt::ForgeQueryIntentReceipt;
