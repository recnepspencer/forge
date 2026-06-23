use forge_query::facade::runtime::{
    ForgeQueryGraphReadResolvedOperation, ForgeQueryGraphReadResolvedOperationFamily,
    ForgeQueryGraphReadResolvedOperationKind,
};

fn main() {
    let _ = ForgeQueryGraphReadResolvedOperation {
        family: ForgeQueryGraphReadResolvedOperationFamily::DeclaredTraversal,
        kind: ForgeQueryGraphReadResolvedOperationKind::DeclarationTraversal,
    };
}
