use crate::basis_lifecycle::BasisFamily;

use super::inventory::{EffectLoweredArtifactKind, EffectReceiptArtifactKind};
use super::planning::EffectAuthorityOwner;
use super::support_matrix::{EffectSupportCause, EffectSupportPosture};
use super::taxonomy::EffectFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct SupportRowDescriptor {
    pub(super) basis_family: BasisFamily,
    pub(super) effect_family: EffectFamily,
    pub(super) authority_owner: EffectAuthorityOwner,
    pub(super) lowered_artifact_kind: EffectLoweredArtifactKind,
    pub(super) receipt_artifact_kind: EffectReceiptArtifactKind,
    pub(super) posture: EffectSupportPosture,
    pub(super) cause: EffectSupportCause,
}

pub(super) fn support_rows() -> &'static [SupportRowDescriptor] {
    use BasisFamily::*;
    use EffectAuthorityOwner::*;
    use EffectFamily::*;
    use EffectLoweredArtifactKind::*;
    use EffectReceiptArtifactKind::*;
    use EffectSupportCause::*;
    use EffectSupportPosture::*;

    &[
        SupportRowDescriptor {
            basis_family: CurrentHead,
            effect_family: Mutation,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMutationIntentDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: CurrentHead,
            effect_family: Merge,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMergeWorkflowDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: CurrentHead,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: BranchHead,
            effect_family: Mutation,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMutationIntentDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: BranchHead,
            effect_family: Merge,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMergeWorkflowDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: BranchHead,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: TenantScoped,
            effect_family: Mutation,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMutationIntentDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: TenantScoped,
            effect_family: Merge,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMergeWorkflowDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Denied,
            cause: BranchAuthorityRequired,
        },
        SupportRowDescriptor {
            basis_family: TenantScoped,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: PolicyScoped,
            effect_family: Mutation,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMutationIntentDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: PolicyScoped,
            effect_family: Merge,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMergeWorkflowDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Denied,
            cause: BranchAuthorityRequired,
        },
        SupportRowDescriptor {
            basis_family: PolicyScoped,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: Admitted,
            cause: Supported,
        },
        SupportRowDescriptor {
            basis_family: Preview,
            effect_family: Mutation,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMutationIntentDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: RebindRequired,
            cause: PreviewRebindRequired,
        },
        SupportRowDescriptor {
            basis_family: Preview,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: RebindRequired,
            cause: PreviewRebindRequired,
        },
        SupportRowDescriptor {
            basis_family: PreviewDerived,
            effect_family: Mutation,
            authority_owner: WorthRelational,
            lowered_artifact_kind: LoweredMutationIntentDeclaration,
            receipt_artifact_kind: WorthQueryIntentExecution,
            posture: Advisory,
            cause: AdvisoryOnlyExecution,
        },
        SupportRowDescriptor {
            basis_family: PreviewDerived,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: RebindRequired,
            cause: PreviewRebindRequired,
        },
        SupportRowDescriptor {
            basis_family: StoreBacked,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: Deferred,
            cause: StoreBackedExecutionDeferred,
        },
        SupportRowDescriptor {
            basis_family: DurableReload,
            effect_family: Writeback,
            authority_owner: WorthRuntimeBridge,
            lowered_artifact_kind: QueryWritebackDeclaration,
            receipt_artifact_kind: WorthQueryWriteReceipt,
            posture: Deferred,
            cause: DurableReplayDeferred,
        },
    ]
}
