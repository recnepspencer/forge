#[path = "../../../support/recovery/closeout/fixture.rs"]
mod closeout_fixture;
#[path = "../stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "../../../support/physical_isolation/copy_on_write_publication/support.rs"]
#[allow(dead_code)]
mod publication_support;
#[path = "../../../support/recovery/recovery_source_precedence/source_precedence_fixture.rs"]
#[allow(dead_code)]
mod source_precedence_fixture;
#[path = "../../../support/physical_isolation/epoch_scope_and_root_kind/support.rs"]
#[allow(dead_code)]
mod support;

#[path = "cases/compaction_interleavings.rs"]
mod compaction_interleavings;
#[path = "cases/denial_counter_assertions.rs"]
mod denial_counter_assertions;
#[path = "cases/foreground_read_scenarios.rs"]
mod foreground_read_scenarios;
#[path = "cases/shared_production_setup.rs"]
mod shared_production_setup;
