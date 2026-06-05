use hadwiger_research::facade::HadwigerResearchInvariantCatalog;

fn mutate(catalog: &mut HadwigerResearchInvariantCatalog) {
    let _ = catalog.rules_mut();
}

fn main() {}
