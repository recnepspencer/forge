use worth_query::facade::domain::WorthQueryInstalledDeclarationContributionTarget;

fn unavailable<T>() -> T {
    panic!("compile-fail fixture must never execute")
}

fn main() {
    let _ = WorthQueryInstalledDeclarationContributionTarget::bind(
        unavailable(),
        unavailable(),
    );
}
