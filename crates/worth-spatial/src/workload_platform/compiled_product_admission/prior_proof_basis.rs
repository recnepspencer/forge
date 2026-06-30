use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::compiled_product_admission::support_posture::EvidenceLookupSupportPosture;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;
use crate::workload_platform::retained_cancellation_chain::RetainedCancellationChainReceipt;

pub(crate) fn evidence_lookup(
    selected_plan: &EvidenceLookupSelectedPlan,
    support: &EvidenceLookupSupportPosture,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-index-prior-proof:v2".to_string(),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("topology-support:{}", support.topology_support_digest()),
            format!("query-support:{}", support.query_support_digest()),
        ],
    )
}

pub(crate) fn retained_cancellation(receipt: &RetainedCancellationChainReceipt) -> String {
    let checkpoint_list = receipt
        .checkpoints()
        .iter()
        .map(|checkpoint| checkpoint.checkpoint_identity())
        .collect::<Vec<_>>()
        .join("|");
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:retained-cancellation-prior-proof:v1".to_string(),
            format!("checkpoint-history:{checkpoint_list}"),
        ],
    )
}
