mod declaration_batches;
mod local_file_state;
mod maintenance_world;
mod resource_budget;
mod sqlite_state;

use crate::{
    backend::records::StoreState, ConservativeRetentionPolicy, DerivedFamilyRetentionPolicy,
    MaintenanceBatchClass, MaintenanceDeclaration, MaintenanceExecutionStatus,
    MaintenanceLocalityScope, PinnedSnapshotPolicy, RetentionPolicyClass, SnapshotCaptureRequest,
    WORTHStore, WORTHStoreBuilder,
};
use worth_relational::facade::runtime::RelationalRuntime;

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};

use self::declaration_batches::{
    derived_family_rebuild_batch, duplicate_compaction_batch, maintenance_audit_batch,
    replication_preparation_batch, same_lane_distinct_compaction_batch, snapshot_refresh_batch,
    tier_move_batch, tier_placement_batch,
};
use self::local_file_state::{
    force_local_file_cancelled, force_local_file_deferred, force_local_file_escalated,
    force_local_file_global_scope_escalated, force_local_file_high_demand,
    force_local_file_high_latency_guard, force_local_file_recovered, force_local_file_reserved,
    force_local_file_started, force_local_file_supersession_epoch,
};
use self::maintenance_world::{
    build_maintenance_ready_store, build_maintenance_ready_store_with_builder, layout_request,
    stable_basis_request_for_store, stable_digest, update_entity_on_branch_with_commit,
};
use self::sqlite_state::{
    force_sqlite_cancelled, force_sqlite_deferred, force_sqlite_escalated,
    force_sqlite_global_scope_escalated, force_sqlite_high_demand, force_sqlite_recovered,
    force_sqlite_reserved, force_sqlite_started,
};

#[path = "milestone_11_maintenance/admission.rs"]
mod admission;
#[path = "milestone_11_maintenance/foreground.rs"]
mod foreground;
#[path = "milestone_11_maintenance/plan_transitions.rs"]
mod plan_transitions;
#[path = "milestone_11_maintenance/rebuild.rs"]
mod rebuild;
#[path = "milestone_11_maintenance/restart_status.rs"]
mod restart_status;
#[path = "milestone_11_maintenance/resume.rs"]
mod resume;
