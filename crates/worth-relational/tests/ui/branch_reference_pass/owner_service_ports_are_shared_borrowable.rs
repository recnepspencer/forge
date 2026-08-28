use worth_relational::facade::branch::RelationalForkPort;
use worth_relational::facade::mvcc::RelationalPreparationPort;
use worth_relational::facade::runtime::RelationalRuntime;

fn assert_clone_send_sync<T: Clone + Send + Sync>() {}

fn obtain_independent_ports(
    runtime: &RelationalRuntime,
) -> (RelationalPreparationPort, RelationalForkPort) {
    (runtime.preparation_port(), runtime.fork_port())
}

fn main() {
    assert_clone_send_sync::<RelationalPreparationPort>();
    assert_clone_send_sync::<RelationalForkPort>();
    let _ = obtain_independent_ports;
}
