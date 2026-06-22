use forge_query::facade::ForgeQueryGraphReadDomainOperationDeclaration;

fn main() {
    let _ = ForgeQueryGraphReadDomainOperationDeclaration {
        key: forge_query::facade::ForgeQueryGraphReadOperationKey::new(
            "domain.operation",
            1,
            "domain",
        )
        .unwrap(),
        admitted_references: Vec::new(),
        support_families: Vec::new(),
    };
}
