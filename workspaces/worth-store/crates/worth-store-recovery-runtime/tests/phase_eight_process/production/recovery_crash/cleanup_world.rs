use super::super::harness::ProcessWorld;

pub(super) fn require_raw_candidate(world: &ProcessWorld) {
    world.require_cleanup_candidate().unwrap_or_else(|error| {
        panic!("MUTANT_PREDICATE:c8-cleanup-world-lacks-covered-wal-artifact\n{error}")
    });
}
