use hadwiger_research::facade::HadwigerResearchInvariantCatalog;

fn register(catalog: &HadwigerResearchInvariantCatalog) {
    let _ = catalog.register_with_query_runtime_unchecked();
}

fn main() {}
