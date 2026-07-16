use worth_query::facade::foundation::WorthQueryGraphReadDomainOperationDeclaration;

fn main() {
    let _ = WorthQueryGraphReadDomainOperationDeclaration::new(
        "neighbors",
        1,
        "WORTH.consumer.forged-owner",
    );
}
