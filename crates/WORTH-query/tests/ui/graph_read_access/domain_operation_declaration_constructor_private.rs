use worth_query::facade::WorthQueryGraphReadDomainOperationDeclaration;

fn main() {
    let _ = WorthQueryGraphReadDomainOperationDeclaration {
        key: worth_query::facade::WorthQueryGraphReadOperationKey::new(
            "domain.operation",
            1,
            "domain",
        )
        .unwrap(),
        admitted_references: Vec::new(),
        support_families: Vec::new(),
    };
}
