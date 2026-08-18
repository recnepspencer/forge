use crate::facade;
use std::sync::OnceLock;
use worth_foundational::facade::{AspectKey, AspectValue};

fn public_api_projection_aspects() -> Vec<AspectKey> {
    static ASPECTS: OnceLock<Vec<AspectKey>> = OnceLock::new();
    ASPECTS
        .get_or_init(|| vec![AspectKey::new("name").unwrap()])
        .clone()
}

struct PublicApiProjection;

impl facade::runtime::EntityRecordProjection for PublicApiProjection {
    const KIND: facade::identity::KindId = facade::identity::KindId(1);

    fn projection_scope() -> facade::runtime::ProjectionAspectScope {
        facade::runtime::ProjectionAspectScope::whole_aspects(public_api_projection_aspects())
    }

    fn from_record(record: facade::runtime::EntityProjectionRecord<'_>) -> Option<Self> {
        let AspectValue::String(_) = record.aspect_value(&AspectKey::new("name").unwrap())? else {
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
    let _txn_options = crate::tests::support::test_owner_transaction_options_for_main(&_runtime);
    let _durability_mode = facade::durability::DurabilityMode::InMemoryCanonical;
    let _diagnostics_scope = facade::diagnostics::DiagnosticsScope::Transaction;
    let _patch_mode = facade::publication::PatchPublicationMode::CommitNative;
    let _snapshot_policy = facade::snapshots::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation;
    let _projection_kind = <PublicApiProjection as facade::runtime::EntityRecordProjection>::KIND;
    let _projection_scope =
        <PublicApiProjection as facade::runtime::EntityRecordProjection>::projection_scope();
}

#[test]
fn relational_owner_mints_move_only_execution_basis_lease() {
    let mut runtime = public_api_runtime();
    let committed =
        crate::tests::support::create_entity_outcome(&mut runtime, "execution-basis-world");
    let version_id = committed.snapshot.version_id;
    let branch_id = committed.snapshot.branch_id.clone();
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));

    let lease = runtime
        .snapshots()
        .admit_execution_basis(&branch_id, version_id)
        .expect("owned reconstructible version should admit an execution basis");
    let handle = lease.snapshot_handle().clone();

    assert_eq!(handle.version_id, version_id);
    assert_eq!(lease.counters().version_availability_check_count(), 1);
    assert_eq!(lease.counters().branch_affinity_check_count(), 1);
    assert_eq!(lease.counters().snapshot_identity_allocation_count(), 1);
    assert_eq!(lease.counters().lease_registry_insert_count(), 1);
    assert!(runtime.read_truth().read_snapshot(&handle).is_some());

    let receipt = lease.release();
    assert!(receipt.released());
    assert!(runtime.read_truth().read_snapshot(&handle).is_none());
}

#[test]
fn execution_basis_lease_drop_closes_snapshot_authority() {
    let mut runtime = public_api_runtime();
    let committed =
        crate::tests::support::create_entity_outcome(&mut runtime, "drop-execution-basis");
    let version_id = committed.snapshot.version_id;
    let branch_id = committed.snapshot.branch_id.clone();
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
    let lease = runtime
        .snapshots()
        .admit_execution_basis(&branch_id, version_id)
        .expect("reconstructible version should admit");
    let identity = lease.identity().clone();
    assert!(runtime.snapshots().execution_basis_is_live(&identity));

    drop(lease);

    assert!(!runtime.snapshots().execution_basis_is_live(&identity));
}

#[test]
fn foreign_runtime_cannot_observe_an_execution_basis_as_live() {
    let mut runtime = public_api_runtime();
    let mut foreign = public_api_runtime();
    let committed =
        crate::tests::support::create_entity_outcome(&mut runtime, "foreign-execution-basis");
    let version_id = committed.snapshot.version_id;
    let branch_id = committed.snapshot.branch_id.clone();
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
    let lease = runtime
        .snapshots()
        .admit_execution_basis(&branch_id, version_id)
        .expect("owned reconstructible version should admit");
    let identity = lease.identity().clone();

    assert!(runtime.snapshots().execution_basis_is_live(&identity));
    assert!(!foreign.snapshots().execution_basis_is_live(&identity));
}

#[test]
fn foreign_runtime_branch_identity_cannot_admit_execution_basis() {
    let mut runtime = public_api_runtime();
    let foreign = public_api_runtime();
    let committed =
        crate::tests::support::create_entity_outcome(&mut runtime, "foreign-identity-basis");
    let version_id = committed.snapshot.version_id;
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
    let foreign_identity = foreign.main_branch_identity();

    let denial = match runtime
        .snapshots()
        .admit_execution_basis_for_identity(&foreign_identity, version_id)
    {
        Ok(_) => panic!("a foreign owner identity must not cross the runtime boundary"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        facade::runtime::RelationalExecutionBasisDenialKind::BranchMismatch
    );
    assert_eq!(denial.counters().version_availability_check_count(), 0);
    assert_eq!(denial.counters().branch_affinity_check_count(), 0);
    assert_eq!(denial.counters().snapshot_identity_allocation_count(), 0);
    assert_eq!(denial.counters().lease_registry_insert_count(), 0);
}

#[test]
fn independently_admitted_execution_bases_release_independently() {
    let mut runtime = public_api_runtime();
    let committed =
        crate::tests::support::create_entity_outcome(&mut runtime, "shared-execution-version");
    let version_id = committed.snapshot.version_id;
    let branch_id = committed.snapshot.branch_id.clone();
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));
    let first = runtime
        .snapshots()
        .admit_execution_basis(&branch_id, version_id)
        .expect("first basis should admit");
    let second = runtime
        .snapshots()
        .admit_execution_basis(&branch_id, version_id)
        .expect("second basis should admit");
    let first_handle = first.snapshot_handle().clone();
    let second_handle = second.snapshot_handle().clone();

    drop(first);
    assert!(runtime.read_truth().read_snapshot(&first_handle).is_none());
    assert!(runtime.read_truth().read_snapshot(&second_handle).is_some());
    drop(second);
    assert!(runtime.read_truth().read_snapshot(&second_handle).is_none());
}

#[test]
fn unavailable_version_cannot_mint_execution_basis() {
    let mut runtime = public_api_runtime();
    let unavailable = facade::identity::VersionId(u64::MAX);
    let branch_id = runtime.config().history.main_branch.clone();
    let denial = match runtime
        .snapshots()
        .admit_execution_basis(&branch_id, unavailable)
    {
        Ok(_) => panic!("unavailable version admitted an execution basis"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        facade::runtime::RelationalExecutionBasisDenialKind::VersionUnavailable
    );
    assert_eq!(denial.counters().version_availability_check_count(), 1);
    assert_eq!(denial.counters().snapshot_identity_allocation_count(), 0);
    assert_eq!(denial.counters().lease_registry_insert_count(), 0);
}

#[test]
fn execution_basis_rejects_a_typed_branch_that_does_not_own_the_version() {
    let mut runtime = public_api_runtime();
    let committed =
        crate::tests::support::create_entity_outcome(&mut runtime, "branch-bound-basis");
    let version_id = committed.snapshot.version_id;
    assert!(runtime.snapshots().release_snapshot(&committed.snapshot));

    let denial = match runtime
        .snapshots()
        .admit_execution_basis(&facade::history::BranchId("foreign".to_owned()), version_id)
    {
        Ok(_) => panic!("another branch admitted execution authority for the version"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        facade::runtime::RelationalExecutionBasisDenialKind::BranchMismatch
    );
    assert_eq!(denial.counters().version_availability_check_count(), 1);
    assert_eq!(denial.counters().branch_affinity_check_count(), 1);
    assert_eq!(denial.counters().snapshot_identity_allocation_count(), 0);
    assert_eq!(denial.counters().lease_registry_insert_count(), 0);
}

fn public_api_runtime() -> facade::runtime::RelationalRuntime {
    crate::tests::support::runtime_with_test_schema()
}
