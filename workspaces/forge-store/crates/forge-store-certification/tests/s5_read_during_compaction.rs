#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "s5_copy_on_write_publication/support.rs"]
#[allow(dead_code)]
mod publication_support;
#[path = "s4_recovery_source_precedence/source_precedence_fixture.rs"]
#[allow(dead_code)]
mod source_precedence_fixture;
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
#[allow(dead_code)]
mod support;

#[path = "s5_read_during_compaction/compaction_interleavings.rs"]
mod compaction_interleavings;
#[path = "s5_read_during_compaction/denial_counter_assertions.rs"]
mod denial_counter_assertions;
#[path = "s5_read_during_compaction/foreground_read_scenarios.rs"]
mod foreground_read_scenarios;
#[path = "s5_read_during_compaction/shared_production_setup.rs"]
mod shared_production_setup;
