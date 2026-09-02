use worth_proof::AuthorityWitness;
use worth_runtime_world::facade::RuntimeWorldOwnerRoot;

struct ForgedAuthority;

impl worth_proof::AuthorityMarker for ForgedAuthority {}

fn forged_witness() -> AuthorityWitness<ForgedAuthority> {
    loop {}
}

fn main() {
    let _ = RuntimeWorldOwnerRoot::<(), (), (), (), ()>::new(forged_witness());
}
