use crate::runtime::admission::WorthUiCandidateAdmissionDenial;
use crate::runtime::query_live_rebind::decision::decide_entry;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiNodeReplacementPlan, WorthUiQueryBindingComparison,
    WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial, WorthUiRuntimeImpactNarrowing,
};

pub(crate) struct WorthUiQueryLiveRebindPlanner;

impl WorthUiQueryLiveRebindPlanner {
    pub(crate) fn plan(
        comparison: &WorthUiQueryBindingComparison,
        node_plan: &WorthUiNodeReplacementPlan,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiQueryLiveRebindPlan, WorthUiQueryLiveRebindPlanDenial> {
        reject_ambiguous_node_plan(node_plan)?;
        reject_comparison_plan_mismatch(comparison, node_plan)?;
        reject_narrowing_mismatch(comparison, narrowing)?;
        reject_admitted_mismatch(comparison, admitted)?;
        reject_changed_query_support_receipt(admitted)?;

        let entries = comparison.entries().iter().map(decide_entry).collect();
        Ok(WorthUiQueryLiveRebindPlan::new(
            comparison.active_artifact_digest(),
            comparison.candidate_artifact_digest(),
            entries,
        ))
    }
}

fn reject_changed_query_support_receipt(
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiQueryLiveRebindPlanDenial> {
    match admitted.verify_receipts_unchanged() {
        Ok(()) => Ok(()),
        Err(WorthUiCandidateAdmissionDenial::QuerySupportReceiptChanged {
            admitted_receipt_digest,
            current_receipt_digest,
        }) => Err(
            WorthUiQueryLiveRebindPlanDenial::AdmittedQuerySupportReceiptChanged {
                admitted_receipt_digest,
                current_receipt_digest,
            },
        ),
        Err(_) => {
            unreachable!("admitted replacement receipt verification only checks receipt drift")
        }
    }
}

fn reject_ambiguous_node_plan(
    node_plan: &WorthUiNodeReplacementPlan,
) -> Result<(), WorthUiQueryLiveRebindPlanDenial> {
    if node_plan.is_unambiguous() {
        Ok(())
    } else {
        Err(WorthUiQueryLiveRebindPlanDenial::AmbiguousNodeReplacementPlan)
    }
}

fn reject_comparison_plan_mismatch(
    comparison: &WorthUiQueryBindingComparison,
    node_plan: &WorthUiNodeReplacementPlan,
) -> Result<(), WorthUiQueryLiveRebindPlanDenial> {
    if comparison.active_artifact_digest() == node_plan.active_artifact_digest()
        && comparison.candidate_artifact_digest() == node_plan.candidate_artifact_digest()
    {
        Ok(())
    } else {
        Err(WorthUiQueryLiveRebindPlanDenial::ComparisonDigestMismatch {
            comparison_active_artifact_digest: comparison.active_artifact_digest(),
            plan_active_artifact_digest: node_plan.active_artifact_digest(),
            comparison_candidate_artifact_digest: comparison.candidate_artifact_digest(),
            plan_candidate_artifact_digest: node_plan.candidate_artifact_digest(),
        })
    }
}

fn reject_narrowing_mismatch(
    comparison: &WorthUiQueryBindingComparison,
    narrowing: &WorthUiRuntimeImpactNarrowing,
) -> Result<(), WorthUiQueryLiveRebindPlanDenial> {
    if comparison.active_artifact_digest() == narrowing.active_artifact_digest()
        && comparison.candidate_artifact_digest() == narrowing.candidate_artifact_digest()
    {
        Ok(())
    } else {
        Err(WorthUiQueryLiveRebindPlanDenial::NarrowingDigestMismatch {
            comparison_active_artifact_digest: comparison.active_artifact_digest(),
            narrowing_active_artifact_digest: narrowing.active_artifact_digest(),
            comparison_candidate_artifact_digest: comparison.candidate_artifact_digest(),
            narrowing_candidate_artifact_digest: narrowing.candidate_artifact_digest(),
        })
    }
}

fn reject_admitted_mismatch(
    comparison: &WorthUiQueryBindingComparison,
    admitted: &WorthUiAdmittedReplacementCandidate,
) -> Result<(), WorthUiQueryLiveRebindPlanDenial> {
    let admitted_candidate_artifact_digest = admitted.artifact_bundle().artifact_digest().raw();
    if comparison.candidate_artifact_digest() == admitted_candidate_artifact_digest {
        Ok(())
    } else {
        Err(
            WorthUiQueryLiveRebindPlanDenial::AdmittedCandidateDigestMismatch {
                comparison_candidate_artifact_digest: comparison.candidate_artifact_digest(),
                admitted_candidate_artifact_digest,
            },
        )
    }
}
