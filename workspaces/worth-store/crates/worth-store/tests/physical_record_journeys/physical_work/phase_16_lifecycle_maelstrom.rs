mod append_preparation;
mod evidence;
mod fixture;
pub(super) mod fresh_process;
mod joined_execution;
mod mutant_report;
mod shutdown_trace;
mod terminal_labels;
mod terminal_projection;
mod workflows;
mod world;

const MODEL: LifecycleMaelstromModel = LifecycleMaelstromModel {
    disjoint_read_effects: 2,
    retry_write_attempts: 2,
    retry_write_completions: 1,
    exact_writeback_effects: 1,
    append_generations: [2, 3],
    append_records: [b"phase-16-left-append", b"phase-16-right-append"],
};

struct LifecycleMaelstromModel {
    disjoint_read_effects: u64,
    retry_write_attempts: u64,
    retry_write_completions: u64,
    exact_writeback_effects: u64,
    append_generations: [u64; 2],
    append_records: [&'static [u8]; 2],
}

#[test]
fn lifecycle_maelstrom_joins_real_authority_effects_and_shutdown() {
    let world = world::open();
    let trace = joined_execution::execute(&world, &MODEL);
    shutdown_trace::close_and_finish(world, trace, &MODEL);
}
