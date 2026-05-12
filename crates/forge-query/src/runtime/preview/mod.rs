use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{
    admit_authority_requirements, validate_inputs, ForgeQueryAspectMutationBuilder,
    ForgeQueryAuthorityLane, ForgeQueryBatchWriteReceipt, ForgeQueryDeleteMutationBuilder,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewRuntime, ForgeQueryEffectAction,
    ForgeQueryEffectAdmission, ForgeQueryEffectHandle, ForgeQueryEffectInspectionEvidence,
    ForgeQueryEffectPolicy, ForgeQueryEffectTriggerSourceKind,
    ForgeQueryExistingTruthTargetBinding, ForgeQueryInstalledOperation,
    ForgeQueryIntentDeclaration, ForgeQueryIntentDenialEvidence, ForgeQueryIntentSourceLane,
    ForgeQueryLiveView, ForgeQueryMutationBatchBuilder, ForgeQueryMutationMetadata,
    ForgeQueryOperationInput, ForgeQueryOperationOutput, ForgeQueryPatchBatch,
    ForgeQueryPreviewBasisAdmission, ForgeQueryPreviewIntentReceipt, ForgeQueryProgramEffect,
    ForgeQueryProgramTrace, ForgeQueryRunReceipt, ForgeQueryRuntime, ForgeQueryRuntimeError,
    ForgeQueryRuntimeFacadeFamily, ForgeQuerySymbolicTargetReference,
    ForgeQuerySymbolicTargetReferenceDenial, ForgeQuerySymbolicTargetReferenceDenialKind,
    ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
};
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryMutationDelta, ForgeQueryMutationKind};
mod aspects;
mod basics;
mod binding;
mod evidence;
mod mutation_ops;
mod outcome;
mod session_closeout;
mod session_execution;
mod workflow_ops;

use aspects::{relevant_computed_aspects, relevant_effect_aspects, relevant_live_aspects};
pub use binding::{
    ForgeQueryPreviewEffectBindingDisposition, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewHandleBindingFamily,
};
pub use evidence::{
    ForgeQueryPreviewCloseoutEvidence, ForgeQueryPreviewCloseoutKind,
    ForgeQueryPreviewExecutionEvidence, ForgeQueryPreviewExecutionKind,
    ForgeQueryPreviewPromotionDenialEvidence, ForgeQueryPreviewPromotionDenialKind,
    ForgeQueryPreviewResidueClass,
};
pub use outcome::{ForgeQueryPreviewDiff, ForgeQueryPreviewOutcome};

pub struct ForgeQueryPreviewSession<'a> {
    label: String,
    runtime: &'a mut ForgeQueryRuntime,
    effect_policy: ForgeQueryEffectPolicy,
    basis_admission: ForgeQueryPreviewBasisAdmission,
    basis_snapshot_token: String,
    pending_commands: Vec<ForgeQueryWriteCommand>,
    writes: Vec<ForgeQueryWriteReceipt>,
    handle_bindings: Vec<ForgeQueryPreviewHandleBindingEvidence>,
    execution_evidence: Vec<ForgeQueryPreviewExecutionEvidence>,
    intent_receipts: Vec<ForgeQueryPreviewIntentReceipt>,
    promoted: bool,
    discarded: bool,
}

impl<'a> ForgeQueryPreviewSession<'a> {
    pub(super) fn new(
        label: impl Into<String>,
        runtime: &'a mut ForgeQueryRuntime,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: ForgeQueryPreviewBasisAdmission,
    ) -> Self {
        let basis_snapshot_token = runtime.snapshot_token();
        Self {
            label: label.into(),
            runtime,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            pending_commands: Vec::new(),
            writes: Vec::new(),
            handle_bindings: Vec::new(),
            execution_evidence: Vec::new(),
            intent_receipts: Vec::new(),
            promoted: false,
            discarded: false,
        }
    }
}
