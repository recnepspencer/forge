//! Certification-only construction for real Query projection worlds.
//!
//! The fixture installs the production WORTH UI domain package and operation
//! executors. It does not construct binding, fact, patch, or authority outcomes.

pub fn scalar_projection_workspace(
    supports_async_lifecycle: bool,
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    crate::scalar_text_projection_fixture::projection_workspace(supports_async_lifecycle)
}

pub fn remasked_scalar_projection_workspace() -> worth_query::facade::runtime::WorthQueryWorkspace {
    crate::scalar_text_projection_fixture::remasked_projection_workspace()
}

pub fn collection_projection_workspace() -> worth_query::facade::runtime::WorthQueryWorkspace {
    crate::scalar_text_projection_fixture::collection_projection_workspace()
}

pub fn collection_projection_workspace_without_entity_lookup(
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    crate::scalar_text_projection_fixture::collection_projection_workspace_without_entity_lookup()
}

pub fn collection_projection_workspace_without_dependency_impact(
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    crate::scalar_text_projection_fixture::collection_projection_workspace_without_dependency_impact(
    )
}

pub fn partial_collection_projection_workspace() -> worth_query::facade::runtime::WorthQueryWorkspace
{
    crate::scalar_text_projection_fixture::partial_collection_projection_workspace()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCollectionProjectionSeedPosture {
    Complete,
    Partial,
    ResetOnly,
}

pub fn seeded_collection_projection_workspace(
    rows: Vec<(String, String)>,
    posture: WorthUiCollectionProjectionSeedPosture,
) -> (
    worth_query::facade::runtime::WorthQueryWorkspace,
    Vec<worth_query::facade::foundation::WorthQueryEntityIdentity>,
) {
    if rows.is_empty() {
        let workspace = match posture {
            WorthUiCollectionProjectionSeedPosture::Complete => collection_projection_workspace(),
            WorthUiCollectionProjectionSeedPosture::Partial => {
                partial_collection_projection_workspace()
            }
            WorthUiCollectionProjectionSeedPosture::ResetOnly => {
                collection_projection_workspace_without_entity_lookup()
            }
        };
        return (workspace, Vec::new());
    }
    let (workspace, seed) =
        crate::scalar_text_projection_fixture::seeded_collection_projection_workspace(
            rows.clone(),
            posture == WorthUiCollectionProjectionSeedPosture::Partial,
            posture != WorthUiCollectionProjectionSeedPosture::ResetOnly,
        );
    let entities = rows
        .iter()
        .map(|(identity, _)| {
            seed.entity(identity)
                .expect("every authored projection seed has a Query entity")
                .clone()
        })
        .collect();
    (workspace, entities)
}

pub fn insert_projection_status(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    identity: &str,
    status: &str,
) -> worth_query::facade::foundation::WorthQueryEntityIdentity {
    crate::scalar_text_projection_fixture::insert_collection_status(workspace, identity, status)
}

pub fn remove_projection_entity(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    entity: worth_query::facade::foundation::WorthQueryEntityIdentity,
) {
    workspace
        .delete(entity)
        .expect("certification projection entity deletion");
}

pub fn update_projection_status(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    entity: worth_query::facade::foundation::WorthQueryEntityIdentity,
    status: &str,
) {
    crate::scalar_text_projection_fixture::update_status(workspace, entity, status);
}

pub fn update_projection_status_batch(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    updates: Vec<(
        worth_query::facade::foundation::WorthQueryEntityIdentity,
        String,
    )>,
) {
    let commands = updates
        .into_iter()
        .fold(
            worth_query::facade::runtime::WorthQueryMutationBatchBuilder::new(),
            |batch, (entity, status)| {
                batch.update(entity, |row| {
                    row.set_aspect(
                        worth_query::facade::runtime::WorthQueryAspectTouch::from_authoring_ingress_text(
                            "query_text.status",
                        )
                        .expect("projection status touch"),
                        worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string(status),
                    )
                })
            },
        )
        .build()
        .expect("QP04 update batch declaration");
    workspace
        .write_batch_intent(commands)
        .execute()
        .expect("QP04 atomic Query update batch");
}

pub fn update_projection_identity(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    entity: worth_query::facade::foundation::WorthQueryEntityIdentity,
    identity: &str,
) {
    crate::scalar_text_projection_fixture::update_identity(workspace, entity, identity);
}
