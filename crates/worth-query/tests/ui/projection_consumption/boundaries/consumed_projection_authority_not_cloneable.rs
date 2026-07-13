use worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority;

fn duplicate(authority: &WorthQueryConsumedProjectionAuthority) {
    let _: WorthQueryConsumedProjectionAuthority = authority.clone();
}

fn main() {}
