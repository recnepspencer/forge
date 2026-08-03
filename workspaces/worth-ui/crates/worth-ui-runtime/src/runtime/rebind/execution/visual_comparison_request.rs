use worth_ui_inspection::UiVisualArtifactPolicy;

use super::UiRebindReceipt;
use crate::inspection::visual_snapshot::{
    new_unbudgeted_comparison_request, UiUnbudgetedVisualSnapshotComparisonRequest,
    UiVisualRebindComparisonEvidence, UiVisualSnapshotReceipt,
};

impl<'receipt, Predecessor, Successor>
    UiUnbudgetedVisualSnapshotComparisonRequest<'receipt, Predecessor, Successor>
where
    Predecessor: UiVisualArtifactPolicy,
    Successor: UiVisualArtifactPolicy,
{
    pub fn between(
        predecessor: &'receipt UiVisualSnapshotReceipt<Predecessor>,
        successor: &'receipt UiVisualSnapshotReceipt<Successor>,
        rebind: &'receipt UiRebindReceipt,
    ) -> Self {
        let publication = rebind.mounted_publication();
        let changed_identity = rebind.plan().identity_decisions().iter().any(|entry| {
            matches!(
                entry.decision(),
                crate::runtime::rebind::UiIdentityLifecycleDecision::Create
                    | crate::runtime::rebind::UiIdentityLifecycleDecision::Retire
                    | crate::runtime::rebind::UiIdentityLifecycleDecision::Rebind
                    | crate::runtime::rebind::UiIdentityLifecycleDecision::Remount
            )
        });
        let continuity = if changed_identity {
            worth_ui_inspection::UiVisualIdentityContinuity::Rebound
        } else {
            worth_ui_inspection::UiVisualIdentityContinuity::Preserved
        };
        let evidence = UiVisualRebindComparisonEvidence::new(
            rebind.session_identity(),
            publication.and_then(|receipt| receipt.predecessor()),
            publication.map(|receipt| receipt.frame()),
            continuity,
        );
        new_unbudgeted_comparison_request(predecessor, successor, evidence)
    }
}
