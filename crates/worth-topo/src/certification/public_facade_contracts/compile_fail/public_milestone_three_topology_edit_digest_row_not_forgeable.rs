use topology::facade::{
    MilestoneThreeHostileScenario, MilestoneThreeTopologyEditDigestRow,
};

fn main() {
    let _ = MilestoneThreeTopologyEditDigestRow {
        scenario: MilestoneThreeHostileScenario::CancellationChainParity,
        topology_edit_digest: arbitrary_private_digest(),
        row_digest: String::new(),
    };
}

fn arbitrary_private_digest<T>() -> T {
    panic!("compile-fail fixture should not execute")
}
