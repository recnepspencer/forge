use worth_query::facade::runtime::WorthQueryGraphReadFamilyIndexContract;

fn main() {
    fn attach_raw_index_name(contract: &WorthQueryGraphReadFamilyIndexContract) {
        let _ = contract.with_index_name("caller-provided-index-name");
    }
}
