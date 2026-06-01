use topology::facade::{
    MilestoneThreeHostileScenario, MilestoneThreeTopologyMutationDigestRow,
};

fn main() {
    let _ = MilestoneThreeTopologyMutationDigestRow {
        scenario: MilestoneThreeHostileScenario::CancellationChainParity,
        topology_mutation_digest: arbitrary_private_digest(),
        row_digest: String::new(),
    };
}

fn arbitrary_private_digest<T>() -> T {
    panic!("compile-fail fixture should not execute")
}
