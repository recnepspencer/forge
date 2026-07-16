use worth_query::facade::foundation::WorthQueryGraphReadDomainOperationDeclaration;

fn restamp(declaration: WorthQueryGraphReadDomainOperationDeclaration) {
    let _ = declaration.requires_support_family("WORTH.consumer.forged-support");
}

fn main() {}
