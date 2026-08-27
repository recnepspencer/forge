use worth_signal::facade::{branch::admit_signal_branch_observation, PartitionToken};

fn main() {
    let token = PartitionToken::new("host-partition");
    let _admitted = admit_signal_branch_observation(token, todo!());
}
