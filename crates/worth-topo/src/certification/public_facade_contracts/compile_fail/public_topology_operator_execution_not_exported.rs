use topology::facade::{
    TopologyOperatorExecution, TopologyOperatorExecutionError, TopologyOperatorExecutionPath,
};

fn main() {
    let _ = (
        std::any::type_name::<TopologyOperatorExecution>(),
        std::any::type_name::<TopologyOperatorExecutionError>(),
        std::any::type_name::<TopologyOperatorExecutionPath>(),
    );
}
