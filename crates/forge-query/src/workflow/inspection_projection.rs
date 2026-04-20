use forge_relational::facade::merge::{
    DeletionMergeClass, LoweredMergeBlockedReason, MergeConflictClass,
    RelationalMergeInspectionAdmission, RelationalMergeInspectionRow,
};

use super::MergeClassAdmission;

pub(super) fn relational_merge_class_label(row: &RelationalMergeInspectionRow) -> String {
    match row.blocked_reason() {
        Some(blocked_reason) => blocked_reason_label(blocked_reason).to_string(),
        None => merge_conflict_class_label(row.classification()),
    }
}

pub(super) fn relational_merge_class_admission(
    row: &RelationalMergeInspectionRow,
) -> MergeClassAdmission {
    match row.admission() {
        RelationalMergeInspectionAdmission::ExecutionAdmissible => {
            MergeClassAdmission::ExecutionAdmissible
        }
        RelationalMergeInspectionAdmission::ExecutionDenied => MergeClassAdmission::ExecutionDenied,
    }
}

fn merge_conflict_class_label(class: &MergeConflictClass) -> String {
    match class {
        MergeConflictClass::ExactSharedTruth => "exact_shared_truth".to_string(),
        MergeConflictClass::SourceOnlyAddition => "source_only_addition".to_string(),
        MergeConflictClass::SchemaDeclaredCorrespondence => {
            "schema_declared_correspondence".to_string()
        }
        MergeConflictClass::Deletion(class) => {
            format!("deletion:{}", deletion_merge_class_label(*class))
        }
        MergeConflictClass::DivergentVisibleState => "divergent_visible_state".to_string(),
        MergeConflictClass::StrategyIntentConflict => "strategy_intent_conflict".to_string(),
        MergeConflictClass::RelationEndpointDivergence => {
            "relation_endpoint_divergence".to_string()
        }
    }
}

fn deletion_merge_class_label(class: DeletionMergeClass) -> &'static str {
    match class {
        DeletionMergeClass::SourceDeletedTargetLive => "source_deleted_target_live",
        DeletionMergeClass::SourceLiveTargetDeleted => "source_live_target_deleted",
        DeletionMergeClass::DeletedOnBothSides => "deleted_on_both_sides",
        DeletionMergeClass::DeletedVsModified => "deleted_vs_modified",
        DeletionMergeClass::DeletedVsRewired => "deleted_vs_rewired",
    }
}

fn blocked_reason_label(reason: LoweredMergeBlockedReason) -> &'static str {
    match reason {
        LoweredMergeBlockedReason::ManualConflictResolutionRequired => {
            "manual_conflict_resolution_required"
        }
        LoweredMergeBlockedReason::StrategyIntentConflictRequiresManualResolution => {
            "strategy_intent_conflict_requires_manual_resolution"
        }
        LoweredMergeBlockedReason::MissingVisibleState => "missing_visible_state",
        LoweredMergeBlockedReason::MissingAncestorValueBasis => "missing_ancestor_value_basis",
        LoweredMergeBlockedReason::UnvalidatedSchemaCorrespondence => {
            "unvalidated_schema_correspondence"
        }
        LoweredMergeBlockedReason::RelationEndpointRewiredLocal => {
            "relation_endpoint_rewired_local"
        }
        LoweredMergeBlockedReason::RelationEndpointRewiredEscalated => {
            "relation_endpoint_rewired_escalated"
        }
        LoweredMergeBlockedReason::TopologyRegionConflict => "topology_region_conflict",
        LoweredMergeBlockedReason::SourceDeletedTargetLive => "deletion:source_deleted_target_live",
        LoweredMergeBlockedReason::SourceLiveTargetDeleted => "deletion:source_live_target_deleted",
        LoweredMergeBlockedReason::DeletedOnBothSides => "deletion:deleted_on_both_sides",
        LoweredMergeBlockedReason::DeletedVsModified => "deletion:deleted_vs_modified",
        LoweredMergeBlockedReason::DeletedVsRewired => "deletion:deleted_vs_rewired",
    }
}
