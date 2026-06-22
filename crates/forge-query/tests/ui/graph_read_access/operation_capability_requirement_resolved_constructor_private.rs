use forge_query::facade::runtime::ForgeQueryGraphReadOperationCapabilityRequirement;

fn main() {
    let _ = ForgeQueryGraphReadOperationCapabilityRequirement::registration_required(
        "domain.operation",
        "domain",
        "domain.operation.access",
    );
}

