use worth_query::facade::runtime::{
    WorthQueryGraphReadResolvedOperation, WorthQueryGraphReadResolvedOperationFamily,
    WorthQueryGraphReadResolvedOperationKind,
};

fn main() {
    let _ = WorthQueryGraphReadResolvedOperation {
        family: WorthQueryGraphReadResolvedOperationFamily::DeclaredTraversal,
        kind: WorthQueryGraphReadResolvedOperationKind::DeclarationTraversal,
    };
}
