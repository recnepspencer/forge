use worth_relational::facade::merge::{
    DeletionMergeClass, LoweredMergeBlockedReason, MergeConflictClass,
    RelationalMergeInspectionAdmission, RelationalMergeInspectionRow,
};

use super::MergeClassAdmission;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg(test)]
pub(super) struct RelationalMergeClassShape {
    pub family: &'static str,
    pub class: &'static str,
}

#[cfg(test)]
pub(super) fn relational_merge_class_shape(
    row: &RelationalMergeInspectionRow,
) -> RelationalMergeClassShape {
    match row.blocked_reason() {
        Some(blocked_reason) => blocked_reason_shape(blocked_reason),
        None => merge_conflict_class_shape(row.classification()),
    }
}

#[cfg(test)]
pub(super) fn relational_merge_class_label(row: &RelationalMergeInspectionRow) -> String {
    merge_class_display_label(relational_merge_class_shape(row)).to_string()
}

#[cfg(test)]
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

#[cfg(test)]
fn merge_class_display_label(shape: RelationalMergeClassShape) -> &'static str {
    match shape.family {
        "deletion" => deletion_merge_display_label(shape.class),
        _ => shape.class,
    }
}

#[cfg(test)]
fn merge_conflict_class_shape(class: &MergeConflictClass) -> RelationalMergeClassShape {
    match class {
        MergeConflictClass::ExactSharedTruth => RelationalMergeClassShape {
            family: "conflict",
            class: "exact_shared_truth",
        },
        MergeConflictClass::SourceOnlyAddition => RelationalMergeClassShape {
            family: "conflict",
            class: "source_only_addition",
        },
        MergeConflictClass::SchemaDeclaredCorrespondence => RelationalMergeClassShape {
            family: "conflict",
            class: "schema_declared_correspondence",
        },
        MergeConflictClass::Deletion(class) => RelationalMergeClassShape {
            family: "deletion",
            class: deletion_merge_class_label(*class),
        },
        MergeConflictClass::DivergentVisibleState => RelationalMergeClassShape {
            family: "conflict",
            class: "divergent_visible_state",
        },
        MergeConflictClass::StrategyIntentConflict => RelationalMergeClassShape {
            family: "conflict",
            class: "strategy_intent_conflict",
        },
        MergeConflictClass::RelationEndpointDivergence => RelationalMergeClassShape {
            family: "conflict",
            class: "relation_endpoint_divergence",
        },
    }
}

#[cfg(test)]
fn blocked_reason_shape(reason: LoweredMergeBlockedReason) -> RelationalMergeClassShape {
    match reason {
        LoweredMergeBlockedReason::SourceDeletedTargetLive => RelationalMergeClassShape {
            family: "deletion",
            class: "source_deleted_target_live",
        },
        LoweredMergeBlockedReason::SourceLiveTargetDeleted => RelationalMergeClassShape {
            family: "deletion",
            class: "source_live_target_deleted",
        },
        LoweredMergeBlockedReason::DeletedOnBothSides => RelationalMergeClassShape {
            family: "deletion",
            class: "deleted_on_both_sides",
        },
        LoweredMergeBlockedReason::DeletedVsModified => RelationalMergeClassShape {
            family: "deletion",
            class: "deleted_vs_modified",
        },
        LoweredMergeBlockedReason::DeletedVsRewired => RelationalMergeClassShape {
            family: "deletion",
            class: "deleted_vs_rewired",
        },
        _ => RelationalMergeClassShape {
            family: "blocked",
            class: blocked_reason_label(reason),
        },
    }
}

#[cfg(test)]
fn deletion_merge_class_label(class: DeletionMergeClass) -> &'static str {
    match class {
        DeletionMergeClass::SourceDeletedTargetLive => "source_deleted_target_live",
        DeletionMergeClass::SourceLiveTargetDeleted => "source_live_target_deleted",
        DeletionMergeClass::DeletedOnBothSides => "deleted_on_both_sides",
        DeletionMergeClass::DeletedVsModified => "deleted_vs_modified",
        DeletionMergeClass::DeletedVsRewired => "deleted_vs_rewired",
    }
}

#[cfg(test)]
fn deletion_merge_display_label(class: &'static str) -> &'static str {
    match class {
        "source_deleted_target_live" => "deletion:source_deleted_target_live",
        "source_live_target_deleted" => "deletion:source_live_target_deleted",
        "deleted_on_both_sides" => "deletion:deleted_on_both_sides",
        "deleted_vs_modified" => "deletion:deleted_vs_modified",
        "deleted_vs_rewired" => "deletion:deleted_vs_rewired",
        _ => unreachable!("deletion merge class label must be typed"),
    }
}

#[cfg(test)]
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
        LoweredMergeBlockedReason::SourceDeletedTargetLive => "source_deleted_target_live",
        LoweredMergeBlockedReason::SourceLiveTargetDeleted => "source_live_target_deleted",
        LoweredMergeBlockedReason::DeletedOnBothSides => "deleted_on_both_sides",
        LoweredMergeBlockedReason::DeletedVsModified => "deleted_vs_modified",
        LoweredMergeBlockedReason::DeletedVsRewired => "deleted_vs_rewired",
    }
}
