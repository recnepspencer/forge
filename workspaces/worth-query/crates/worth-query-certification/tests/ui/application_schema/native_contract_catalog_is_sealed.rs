use worth_query_host::facade::domain::{
    WorthQueryInstalledApplicationAspectContract,
    WorthQueryInstalledApplicationAspectLocus,
    WorthQueryInstalledApplicationSchemaContractCatalog,
};

fn cannot_mutate(catalog: &mut WorthQueryInstalledApplicationSchemaContractCatalog) {
    catalog.contracts.clear();
}

fn main() {
    let _ = WorthQueryInstalledApplicationSchemaContractCatalog::new;
    let _ = WorthQueryInstalledApplicationAspectContract::new;
    let _ = WorthQueryInstalledApplicationAspectLocus::new;
}
