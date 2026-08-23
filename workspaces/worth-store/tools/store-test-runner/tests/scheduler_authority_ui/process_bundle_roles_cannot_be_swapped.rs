use worth_store_process_bundle::{
    BoundArtifact, FreshRecoveryProcessBundle, ObserverProcessRole,
};

fn observer_only(_: &BoundArtifact<ObserverProcessRole>) {}

fn swap_roles(bundle: &FreshRecoveryProcessBundle) {
    observer_only(bundle.writer());
}

fn main() {
    let _ = swap_roles;
}
