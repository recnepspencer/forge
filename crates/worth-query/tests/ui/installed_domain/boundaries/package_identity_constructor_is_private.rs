use worth_query::facade::domain::WorthQueryDomainPackageIdentity;

fn unavailable<T>() -> T {
    panic!("compile-fail fixture must never execute")
}

fn main() {
    let _ = WorthQueryDomainPackageIdentity::new(unavailable());
}
