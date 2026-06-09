use topology::runtime_support::{
    TopologyRuntimePostureCapability, TopologyRuntimePostureRow,
    TopologyRuntimePostureStatus,
};

fn main() {
    let _ = TopologyRuntimePostureRow {
        capability: TopologyRuntimePostureCapability::AuthoritativeWrites,
        status: TopologyRuntimePostureStatus::Admitted,
        reason: String::new(),
        row_digest: String::new(),
    };
}
