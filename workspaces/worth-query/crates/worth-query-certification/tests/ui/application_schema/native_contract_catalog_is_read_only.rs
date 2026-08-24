use worth_query_host::facade::domain::WorthQueryInstalledApplicationSchemaContractCatalog;

fn inspect(catalog: &WorthQueryInstalledApplicationSchemaContractCatalog) {
    for contract in catalog.contracts() {
        let _ = contract.locus().schema();
        let _ = contract.locus().entity();
        let _ = contract.locus().aspect();
        let _ = contract.contract();
        let _ = contract.binding();
        let _ = contract.fields();
        let _ = contract.canonical_contract_basis();
        let _ = contract.canonical_contract_material();
    }
}

fn main() {}
