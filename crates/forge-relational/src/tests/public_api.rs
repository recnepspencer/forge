use crate::facade;

struct PublicApiProjection;

impl facade::runtime::EntityRecordProjection for PublicApiProjection {
    const KIND: facade::identity::KindId = facade::identity::KindId(1);

    fn from_record(record: &facade::runtime::EntityReadRecord) -> Option<Self> {
        record.payload.as_json()?.get("name")?.as_str()?;
        Some(Self)
    }
}

#[test]
fn facade_namespaces_expose_domain_groupings() {
    let _branch: facade::history::BranchId = facade::history::BranchId("main".to_string());
    let _entity: facade::identity::EntityId =
        facade::identity::EntityId::new(facade::identity::PartitionId::main(), 1, 0);
    let _config = facade::config::RelationalRuntimeProfile::CertificationCore;
    let _runtime = facade::runtime::RelationalRuntimeApi::builder()
        .schema_registry(facade::schema::RelationalSchemaRegistry::new())
        .build();
    let _txn_options = facade::transactions::TransactionOptions::default();
    let _durability_mode = facade::durability::DurabilityMode::InMemoryCanonical;
    let _diagnostics_scope = facade::diagnostics::DiagnosticsScope::Transaction;
    let _patch_mode = facade::publication::PatchPublicationMode::CommitNative;
    let _snapshot_policy = facade::snapshots::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation;
    let _projection_kind = <PublicApiProjection as facade::runtime::EntityRecordProjection>::KIND;
    let _projection_aspect = facade::runtime::ProjectionAspect::new("name");
}
