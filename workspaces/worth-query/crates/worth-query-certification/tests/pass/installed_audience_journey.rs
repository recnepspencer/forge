use worth_query_host::facade::{
    installed::{
        self,
        collection::{WorthQueryCollectionCursor, WorthQueryCollectionPatch},
    },
    runtime::WorthQueryWorkspace,
};
use worth_query_replay::facade::WorthQueryCertificationReplayCounters;

struct ExampleFamily;

fn ordinary_entry(workspace: &mut WorthQueryWorkspace) {
    let root = workspace.observe_operating_world().unwrap();
    let _family = root.family(ExampleFamily);
    let _ = installed::operation::project_facts().entity_identities();
}

fn inspect_opaque_collection_artifacts(
    cursor: &WorthQueryCollectionCursor,
    patch: &WorthQueryCollectionPatch,
) {
    let _ = cursor.is_beginning();
    let _ = patch.maintenance_ordinal();
    let _ = patch.authority();
}

fn certification_entry(counters: WorthQueryCertificationReplayCounters) {
    let _ = counters;
}

fn main() {}
