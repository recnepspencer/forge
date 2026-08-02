#[path = "../support/physical_isolation/checkpoint_read_fixture/checkpoint_read_fixture.rs"]
mod checkpoint_read_fixture;

#[path = "../scenarios/physical_isolation/compaction_range_interlock_bounds/compaction_range_interlock_bounds.rs"]
mod compaction_range_interlock_bounds;

#[path = "../scenarios/physical_isolation/copy_on_write_publication/copy_on_write_publication.rs"]
mod copy_on_write_publication;

#[path = "../scenarios/physical_isolation/epoch_scope_and_root_kind/epoch_scope_and_root_kind.rs"]
mod epoch_scope_and_root_kind;

#[path = "../scenarios/physical_isolation/epoch_scope_foundational_lowering/epoch_scope_foundational_lowering.rs"]
mod epoch_scope_foundational_lowering;

#[path = "../scenarios/physical_isolation/evidence_materialization/evidence_materialization.rs"]
mod evidence_materialization;

#[path = "../scenarios/physical_isolation/latch_acquisition_order/latch_acquisition_order.rs"]
mod latch_acquisition_order;

#[path = "../scenarios/physical_isolation/latch_algorithm_shape/latch_algorithm_shape.rs"]
mod latch_algorithm_shape;

#[path = "../scenarios/physical_isolation/physical_isolation_closeout/physical_isolation_closeout.rs"]
mod physical_isolation_closeout;

#[path = "../scenarios/physical_isolation/physical_isolation_entry/physical_isolation_entry.rs"]
mod physical_isolation_entry;

#[path = "../scenarios/physical_isolation/physical_semantic_isolation/physical_semantic_isolation.rs"]
mod physical_semantic_isolation;

#[path = "../scenarios/physical_isolation/read_during_checkpoint/read_during_checkpoint.rs"]
mod read_during_checkpoint;

#[path = "../scenarios/physical_isolation/read_during_compaction/read_during_compaction.rs"]
mod read_during_compaction;

#[path = "../scenarios/physical_isolation/reclaim_reachability_hazard_barriers/reclaim_reachability_hazard_barriers.rs"]
mod reclaim_reachability_hazard_barriers;

#[path = "../scenarios/physical_isolation/stable_read_execution/stable_read_execution.rs"]
mod stable_read_execution;

#[path = "../scenarios/physical_isolation/stable_read_plan_admission/stable_read_plan_admission.rs"]
mod stable_read_plan_admission;

#[path = "../scenarios/physical_isolation/stable_read_plan_native_footprint/stable_read_plan_native_footprint.rs"]
mod stable_read_plan_native_footprint;

#[path = "../support/recovery/independent_verifier_observation.rs"]
mod independent_verifier_observation;
#[path = "../support/physical_isolation/interleaving_harness_support/interleaving_harness_support.rs"]
mod physical_interleaving_support;
#[path = "../scenarios/physical_isolation/readiness/shortcut_report.rs"]
mod physical_isolation_shortcut_report;
#[path = "../support/recovery/recovery_offline_verifier/runtime_recovery_fixture.rs"]
mod runtime_recovery_fixture;
#[path = "../scenarios/physical_isolation/stable_read_plan_scenarios/stable_read_plan_scenarios.rs"]
mod stable_read_plan_scenarios;
