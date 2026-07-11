#[path = "support/layout/btree/fixture.rs"]
mod support;

#[path = "scenarios/layout/strategy_admission/checkpoint_cutover_layout.rs"]
mod checkpoint_cutover_layout;
#[path = "scenarios/layout/btree/crash_boundary_layout.rs"]
mod crash_boundary_layout;
#[path = "scenarios/layout/btree/readmission_layout.rs"]
mod readmission_layout;
#[path = "scenarios/layout/btree/replay_layout.rs"]
mod replay_layout;
