use hadwiger_research::facade::HadwigerResearchInvariantCatalog;

fn mutate(catalog: &mut HadwigerResearchInvariantCatalog) {
    let _ = catalog.obligations_mut();
}

fn main() {}
