use crate::capabilities::AspectPlanSource;
use crate::merge::data::{VisibleMergeRecord, VisibleMergeRecordKind};
use crate::schema::data::LoweredAspectPlan;

pub(crate) fn lowered_plan_for_record<'a>(
    runtime: &'a crate::logic::runtime::RelationalRuntime,
    record: &VisibleMergeRecord,
) -> Option<&'a LoweredAspectPlan> {
    let kind_id = record.source_kind_id.or(record.kind_id)?;
    match record.record_kind {
        VisibleMergeRecordKind::Entity => runtime.entity_aspect_plan(kind_id),
        VisibleMergeRecordKind::Relation => runtime.relation_aspect_plan(kind_id),
    }
}
