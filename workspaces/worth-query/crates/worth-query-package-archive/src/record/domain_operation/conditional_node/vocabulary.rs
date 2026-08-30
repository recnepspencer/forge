use worth_query_installation::facade::*;

use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

pub(super) fn role_tag(value: WorthQueryConditionalNodeRole) -> u16 {
    match value {
        WorthQueryConditionalNodeRole::Computed => 1,
        WorthQueryConditionalNodeRole::WorkflowStage => 2,
        WorthQueryConditionalNodeRole::OperationGate => 3,
    }
}
pub(super) fn role(tag: u16) -> Result<WorthQueryConditionalNodeRole, Denial> {
    match tag {
        1 => Ok(WorthQueryConditionalNodeRole::Computed),
        2 => Ok(WorthQueryConditionalNodeRole::WorkflowStage),
        3 => Ok(WorthQueryConditionalNodeRole::OperationGate),
        _ => unsupported(),
    }
}
pub(super) fn context_tag(value: WorthQueryConditionalNodeContext) -> u16 {
    match value {
        WorthQueryConditionalNodeContext::Basis => 1,
        WorthQueryConditionalNodeContext::Snapshot => 2,
        WorthQueryConditionalNodeContext::QueryContext => 3,
        WorthQueryConditionalNodeContext::OperationInput => 4,
        WorthQueryConditionalNodeContext::WorkflowRun => 5,
    }
}
pub(super) fn context(tag: u16) -> Result<WorthQueryConditionalNodeContext, Denial> {
    match tag {
        1 => Ok(WorthQueryConditionalNodeContext::Basis),
        2 => Ok(WorthQueryConditionalNodeContext::Snapshot),
        3 => Ok(WorthQueryConditionalNodeContext::QueryContext),
        4 => Ok(WorthQueryConditionalNodeContext::OperationInput),
        5 => Ok(WorthQueryConditionalNodeContext::WorkflowRun),
        _ => unsupported(),
    }
}
pub(super) fn maintenance_tag(value: WorthQueryMaintenancePosture) -> u16 {
    match value {
        WorthQueryMaintenancePosture::EagerOnEligibleInvalidation => 1,
        WorthQueryMaintenancePosture::LazyUntilObserved => 2,
        WorthQueryMaintenancePosture::OnDemandOnly => 3,
        WorthQueryMaintenancePosture::Temporal => 4,
    }
}
pub(super) fn maintenance(tag: u16) -> Result<WorthQueryMaintenancePosture, Denial> {
    match tag {
        1 => Ok(WorthQueryMaintenancePosture::EagerOnEligibleInvalidation),
        2 => Ok(WorthQueryMaintenancePosture::LazyUntilObserved),
        3 => Ok(WorthQueryMaintenancePosture::OnDemandOnly),
        4 => Ok(WorthQueryMaintenancePosture::Temporal),
        _ => unsupported(),
    }
}
pub(super) fn artifact_tag(value: WorthQueryArtifactPosture) -> u16 {
    match value {
        WorthQueryArtifactPosture::Ephemeral => 1,
        WorthQueryArtifactPosture::ReusableWhenEquivalent => 2,
        WorthQueryArtifactPosture::Durable => 3,
    }
}
pub(super) fn artifact(tag: u16) -> Result<WorthQueryArtifactPosture, Denial> {
    match tag {
        1 => Ok(WorthQueryArtifactPosture::Ephemeral),
        2 => Ok(WorthQueryArtifactPosture::ReusableWhenEquivalent),
        3 => Ok(WorthQueryArtifactPosture::Durable),
        _ => unsupported(),
    }
}
pub(super) fn relationship_tag(value: WorthQueryOutputRelationship) -> u16 {
    match value {
        WorthQueryOutputRelationship::IntermediateOnly => 1,
        WorthQueryOutputRelationship::ContributesToOperationOutput => 2,
        WorthQueryOutputRelationship::IsOperationOutput => 3,
        WorthQueryOutputRelationship::IsWorkflowStageOutput => 4,
    }
}
pub(super) fn relationship(tag: u16) -> Result<WorthQueryOutputRelationship, Denial> {
    match tag {
        1 => Ok(WorthQueryOutputRelationship::IntermediateOnly),
        2 => Ok(WorthQueryOutputRelationship::ContributesToOperationOutput),
        3 => Ok(WorthQueryOutputRelationship::IsOperationOutput),
        4 => Ok(WorthQueryOutputRelationship::IsWorkflowStageOutput),
        _ => unsupported(),
    }
}
pub(super) fn effect_tag(value: WorthQueryOperationEffectFamily) -> u16 {
    match value {
        WorthQueryOperationEffectFamily::Mutation => 1,
        WorthQueryOperationEffectFamily::Merge => 2,
        WorthQueryOperationEffectFamily::Writeback => 3,
    }
}
pub(super) fn effect(tag: u16) -> Result<WorthQueryOperationEffectFamily, Denial> {
    match tag {
        1 => Ok(WorthQueryOperationEffectFamily::Mutation),
        2 => Ok(WorthQueryOperationEffectFamily::Merge),
        3 => Ok(WorthQueryOperationEffectFamily::Writeback),
        _ => unsupported(),
    }
}
pub(super) fn temporal_wake_tag(value: WorthQueryTemporalWake) -> u16 {
    match value {
        WorthQueryTemporalWake::MonotonicClock => 1,
        WorthQueryTemporalWake::WallClock => 2,
        WorthQueryTemporalWake::OnSnapshotAdvance => 3,
    }
}
pub(super) fn temporal_wake(tag: u16) -> Result<WorthQueryTemporalWake, Denial> {
    match tag {
        1 => Ok(WorthQueryTemporalWake::MonotonicClock),
        2 => Ok(WorthQueryTemporalWake::WallClock),
        3 => Ok(WorthQueryTemporalWake::OnSnapshotAdvance),
        _ => unsupported(),
    }
}
fn unsupported<T>() -> Result<T, Denial> {
    Err(Denial::new(Kind::UnsupportedRecordVariant))
}
