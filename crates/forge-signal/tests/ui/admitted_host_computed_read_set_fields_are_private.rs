use forge_signal::facade::AdmittedHostComputedReadSet;
use forge_signal::facade::NodeId;

fn main() {
    let _ = AdmittedHostComputedReadSet {
        node: NodeId::new(1, 0),
        dependencies: loop {},
    };
}
