use worth_store_physical_isolation::{
    CheckpointPublicationIdentity, CheckpointPublicationRoot, PhysicalOrderingContract, RootEpoch,
};

fn main() {
    let epoch: RootEpoch = todo!();
    let checkpoint_identity: CheckpointPublicationIdentity = todo!();
    let _ = CheckpointPublicationRoot::from_checkpoint_publication(
        epoch,
        PhysicalOrderingContract::root_swap_acquire_release(),
        checkpoint_identity,
    );
}
