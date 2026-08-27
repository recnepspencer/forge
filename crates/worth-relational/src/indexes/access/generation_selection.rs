use crate::indexes::data::{
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexPublicationStatus,
};
use crate::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;

pub(super) fn exact_published_generation<'a>(
    runtime: &'a RelationalRuntime,
    snapshot: &SnapshotHandle,
    definition: &DerivedIndexDefinition,
) -> Option<&'a DerivedIndexGeneration> {
    let branch_id = snapshot.branch_id();
    let schema_version = runtime
        .read_truth()
        .query_plan_context(snapshot)?
        .schema_version;
    runtime
        .indexes
        .generations
        .get(&definition.index_id)?
        .iter()
        .rev()
        .find(|generation| {
            generation.status == DerivedIndexPublicationStatus::Published
                && generation.applicability.version_id == snapshot.version_id
                && generation.applicability.schema_version == schema_version
                && (!definition.branch_scoped || generation.applicability.branch_id == *branch_id)
        })
}
