use worth_signal::facade::{branch::admit_signal_branch_observation, OutputIdentity};

fn main() {
    let token = OutputIdentity::new("host-output");
    let _admitted = admit_signal_branch_observation(token, todo!());
}
