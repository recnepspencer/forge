#[path = "compile_fail/support.rs"]
mod compile_fail_support;

#[path = "compile_fail/layout/bootstrap_runner.rs"]
mod bootstrap;
#[path = "compile_fail/layout/btree_runner.rs"]
mod btree;
#[path = "compile_fail/layout/closeout_runner.rs"]
mod closeout;
#[path = "compile_fail/layout/counter_evidence_runner.rs"]
mod counter_evidence;
#[path = "compile_fail/layout/foundations_runner.rs"]
mod foundations;
#[path = "compile_fail/layout/migration_runner.rs"]
mod migration;
#[path = "compile_fail/layout/public_facade_runner.rs"]
mod public_facade;
#[path = "compile_fail/layout/recovery_readmission_runner.rs"]
mod recovery_readmission;
#[path = "compile_fail/layout/strategy_admission_runner.rs"]
mod strategy_admission;
