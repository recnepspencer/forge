use worth_store_authority::StoreCurrentAuthorityWitness;
use worth_store_physical_isolation::S5CertifiedStoreExecutionCloseout;

fn main() {
    let store_authority: StoreCurrentAuthorityWitness = unimplemented!();
    let _ = S5CertifiedStoreExecutionCloseout::from_executed_store_handoff(
        store_authority,
        1,
        1,
        1,
        unimplemented!(),
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
        1,
    );
}
