use worth_relational::facade::branch::{
    AdmittedRelationalForkSourceBasis, RelationalForkSourceDescriptor,
};

fn consume_fork_authority(_: AdmittedRelationalForkSourceBasis) {}

fn main() {
    let descriptor: RelationalForkSourceDescriptor = todo!();
    consume_fork_authority(descriptor);
}
