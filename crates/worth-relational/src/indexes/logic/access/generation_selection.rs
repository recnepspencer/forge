use crate::indexes::data::{
    DerivedIndexDefinition, DerivedIndexGeneration, DerivedIndexPublicationStatus,
};
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;

pub(super) fn exact_published_generation<'a>(
    runtime: &'a RelationalRuntime,
    snapshot: &SnapshotHandle,
    definition: &DerivedIndexDefinition,
) -> Option<&'a DerivedIndexGeneration> {
    let branch_id = runtime
        .history
        .commit_graph
        .values()
        .find(|node| node.commit.version_id == snapshot.version_id)
        .map(|node| &node.commit.branch_id);
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
                && (!definition.branch_scoped
                    || branch_id
                        .is_some_and(|branch| generation.applicability.branch_id == *branch))
        })
}
