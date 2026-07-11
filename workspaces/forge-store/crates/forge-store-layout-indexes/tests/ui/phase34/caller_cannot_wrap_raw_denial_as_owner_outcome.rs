use forge_store_layout_indexes::layout_rebuild::{
    S8DerivedIndexRebuildDenied, S8DerivedIndexRebuildOutcome,
};

fn main() {
    let forged = S8DerivedIndexRebuildOutcome::Denied(
        S8DerivedIndexRebuildDenied::ParityRowsMustBeCanonical,
    );
    let _ = forged.production_transition();
}
