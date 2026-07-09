use worth_query::facade::runtime::WorthQueryGraphReadOperationCapabilityRequirement;

fn main() {
    let _ = WorthQueryGraphReadOperationCapabilityRequirement::registration_required(
        "domain.operation",
        "domain",
        "domain.operation.access",
    );
}

