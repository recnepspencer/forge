use forge_query::facade::runtime::ForgeQueryGraphReadFamilyIndexContract;

fn main() {
    fn attach_raw_index_name(contract: &ForgeQueryGraphReadFamilyIndexContract) {
        let _ = contract.with_index_name("caller-provided-index-name");
    }
}
