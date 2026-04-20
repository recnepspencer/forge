use crate::layout::DedupAdmittedBlockReuse;

use super::{
    PersistedAdmittedAspectLayoutReadPlan, PersistedChunkModelFrozenPhysicalLayout,
    PersistedDedupAdmittedBlockReuse, PersistedMilestone6LayoutMaterialization,
    PersistedMilestone6LayoutMaterializationRecord, PersistedMilestone7IndependentLayoutReference,
    PersistedMilestone9PhysicalChunkReference,
};

pub(super) fn validate_persisted_milestone_6_layout_materialization_record(
    record: &PersistedMilestone6LayoutMaterializationRecord,
) -> Result<(), String> {
    validate_persisted_milestone_6_layout_materialization(&record.materialization)?;
    if record.artifact_id != record.materialization.artifact_id {
        return Err(format!(
            "persisted milestone 6 materialization record key `{}` drifted from payload artifact id `{}`",
            record.artifact_id, record.materialization.artifact_id
        ));
    }
    Ok(())
}

pub(super) fn validate_persisted_milestone_6_layout_materialization(
    materialization: &PersistedMilestone6LayoutMaterialization,
) -> Result<(), String> {
    let expected_plan = match crate::layout::classify_layout_request(
        materialization.admitted_plan.request.clone(),
    )
    .map_err(|error| error.to_string())?
    {
        crate::AspectLayoutReadPlanDecision::Admitted(plan) => plan,
        crate::AspectLayoutReadPlanDecision::Fallback(plan) => {
            return Err(format!(
                "persisted milestone 6 materialization `{}` referenced a request that now classifies as fallback: {}",
                materialization.artifact_id,
                plan.reason()
            ));
        }
        crate::AspectLayoutReadPlanDecision::Rejected(plan) => {
            return Err(format!(
                "persisted milestone 6 materialization `{}` referenced a request that now classifies as rejected: {}",
                materialization.artifact_id,
                plan.reason()
            ));
        }
    };
    let expected_block_reuse = DedupAdmittedBlockReuse::new(
        &expected_plan,
        materialization.block_reuse.equivalence_contract_version,
    );
    let expected_frozen_layout = crate::layout::freeze_chunk_model_from_plan(&expected_plan)
        .map_err(|error| error.to_string())?;
    let expected_milestone_7_reference =
        crate::layout::admit_milestone_7_reference_from_plan(&expected_plan)
            .map_err(|error| error.to_string())?;
    let expected_milestone_9_reference =
        crate::layout::admit_milestone_9_reference_from_frozen(&expected_frozen_layout);
    let expected_artifact_id = crate::layout::layout_materialization_artifact_id(&expected_plan);

    if materialization.artifact_id != expected_artifact_id {
        return Err(format!(
            "persisted milestone 6 materialization artifact id `{}` did not match expected `{expected_artifact_id}`",
            materialization.artifact_id
        ));
    }
    if materialization.admitted_plan != PersistedAdmittedAspectLayoutReadPlan::from(&expected_plan) {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical admitted layout plan for its request",
            materialization.artifact_id
        ));
    }
    if materialization.block_reuse != PersistedDedupAdmittedBlockReuse::from(&expected_block_reuse) {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical structural block reuse witness for its admitted plan",
            materialization.artifact_id
        ));
    }
    if materialization.frozen_layout != PersistedChunkModelFrozenPhysicalLayout::from(&expected_frozen_layout) {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical frozen chunk layout for its admitted plan",
            materialization.artifact_id
        ));
    }
    if materialization.milestone_7_reference != PersistedMilestone7IndependentLayoutReference::from(&expected_milestone_7_reference) {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical Milestone 7 reference for its admitted plan",
            materialization.artifact_id
        ));
    }
    if materialization.milestone_9_reference != PersistedMilestone9PhysicalChunkReference::from(&expected_milestone_9_reference) {
        return Err(format!(
            "persisted milestone 6 materialization `{}` drifted from the canonical Milestone 9 physical chunk reference for its frozen layout",
            materialization.artifact_id
        ));
    }
    if materialization.semantic_truth_digest.is_empty() {
        return Err(format!(
            "persisted milestone 6 materialization `{}` was missing semantic truth digest",
            materialization.artifact_id
        ));
    }
    if materialization.authoritative_commit_count == 0 {
        return Err(format!(
            "persisted milestone 6 materialization `{}` was missing authoritative commit count",
            materialization.artifact_id
        ));
    }
    Ok(())
}
