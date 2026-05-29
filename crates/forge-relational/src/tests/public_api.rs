use crate::facade;
use forge_foundational::facade::AspectValue;
use std::sync::OnceLock;

fn public_api_projection_aspects() -> &'static [facade::publication::AspectKey] {
    static ASPECTS: OnceLock<Vec<facade::publication::AspectKey>> = OnceLock::new();
    ASPECTS
        .get_or_init(|| vec![facade::publication::AspectKey::new("name").unwrap()])
        .as_slice()
}

struct PublicApiProjection;

impl facade::runtime::EntityRecordProjection for PublicApiProjection {
    const KIND: facade::identity::KindId = facade::identity::KindId(1);

    fn required_aspects() -> &'static [facade::publication::AspectKey] {
        public_api_projection_aspects()
    }

    fn from_record(record: facade::runtime::EntityProjectionRecord<'_>) -> Option<Self> {
        let AspectValue::String(_) =
            record.aspect_value(&facade::publication::AspectKey::new("name").unwrap())?
        else {
            return None;
        };
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
    let _projection_aspects =
        <PublicApiProjection as facade::runtime::EntityRecordProjection>::required_aspects();
}
