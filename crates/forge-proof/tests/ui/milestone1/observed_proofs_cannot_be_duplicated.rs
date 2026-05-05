use forge_proof::{CanonicalOrder, Proof};

fn takes_owned(_: Proof<CanonicalOrder>) {}
fn requires_clone<T: Clone>(_: &T) {}

fn duplicate(proof: &Proof<CanonicalOrder>) {
    requires_clone(proof);
    let _copied = *proof;
    takes_owned(*proof);
}

fn main() {}
