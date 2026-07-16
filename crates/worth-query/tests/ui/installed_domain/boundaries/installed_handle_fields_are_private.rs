use worth_query::facade::domain::WorthQueryInstalledDomainHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConsumerDomain;

fn unavailable<T>() -> T {
    panic!("compile-fail fixture must never execute")
}

fn main() {
    let _ = WorthQueryInstalledDomainHandle::<ConsumerDomain> {
        authority: unavailable(),
        marker: unavailable(),
    };
}
