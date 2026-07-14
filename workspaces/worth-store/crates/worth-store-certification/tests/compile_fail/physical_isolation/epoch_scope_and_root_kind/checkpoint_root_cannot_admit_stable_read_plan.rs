use worth_store_physical_isolation::{
    physical_epoch_vector_for_current_root, CheckpointPublicationRoot,
};

fn main() {
    let checkpoint: CheckpointPublicationRoot = todo!();
    let _ = physical_epoch_vector_for_current_root(checkpoint);
}
