use forge_query::facade::QuerySubscriptionDeclarationArtifact;

fn declaration_projection_golden_path(declaration: &QuerySubscriptionDeclarationArtifact) {
    let _ = declaration.declaration_projection().label();
    let _ = declaration.equivalence_projection().label();
}

fn main() {}
