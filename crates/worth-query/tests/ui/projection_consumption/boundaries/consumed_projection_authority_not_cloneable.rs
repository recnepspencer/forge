use worth_query::facade::WorthQueryConsumedProjectionAuthority;

fn duplicate(authority: &WorthQueryConsumedProjectionAuthority) {
    let _: WorthQueryConsumedProjectionAuthority = authority.clone();
}

fn main() {}
