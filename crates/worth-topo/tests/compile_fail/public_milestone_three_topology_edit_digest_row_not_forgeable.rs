use topology::facade::{
    DeterministicDigest, MilestoneThreeHostileScenario, MilestoneThreeTopologyEditDigestRow,
    TopologyEditDigest,
};

fn main() {
    let _ = MilestoneThreeTopologyEditDigestRow {
        scenario: MilestoneThreeHostileScenario::CancellationChainParity,
        topology_edit_digest: TopologyEditDigest {
            digest: DeterministicDigest {
                algorithm: String::new(),
                digest_hex: String::new(),
                row_count: 0,
            },
            contract_count: 0,
            family_count: 0,
            changed_scope_count: 0,
            naming_scope_count: 0,
            derived_region_count: 0,
        },
        row_digest: String::new(),
    };
}
