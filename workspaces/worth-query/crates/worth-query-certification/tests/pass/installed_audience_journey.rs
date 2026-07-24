use worth_query_host::facade::{
    domain::{
        WorthQueryArtifactChunkRequest, WorthQueryArtifactNativeAccessCounters,
        WorthQueryArtifactNativeAccessDenial, WorthQueryTransferredArtifactHandle,
        WorthQueryWorkflowStageWorkspace,
    },
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

fn consume_native_artifact<'a>(
    workspace: &'a WorthQueryWorkflowStageWorkspace<'a>,
    artifact: &'a WorthQueryTransferredArtifactHandle,
    request: WorthQueryArtifactChunkRequest,
) -> Result<(usize, WorthQueryArtifactNativeAccessCounters), WorthQueryArtifactNativeAccessDenial> {
    let mut cursor = workspace.artifact_reader(artifact)?.chunks(request)?;
    let mut rows = 0;
    while cursor
        .next(|batch| rows += batch.row_count())?
        .is_some()
    {}
    Ok((rows, cursor.evidence().counters()))
}

fn main() {}
