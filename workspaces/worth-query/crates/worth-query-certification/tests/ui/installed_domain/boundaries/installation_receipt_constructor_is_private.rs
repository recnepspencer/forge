use worth_query::facade::domain::WorthQueryDomainInstallationReceipt;

fn unavailable<T>() -> T {
    panic!("compile-fail fixture must never execute")
}

fn main() {
    let _ = WorthQueryDomainInstallationReceipt::new(
        unavailable(),
        unavailable(),
        unavailable(),
        unavailable(),
        unavailable(),
        unavailable(),
        unavailable(),
    );
}
