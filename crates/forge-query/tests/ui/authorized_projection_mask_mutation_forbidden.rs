use forge_query::facade::{AspectFieldKey, PolicyAspectMask};

fn main() {
    let mut mask =
        PolicyAspectMask::allow_all().with_masked(AspectFieldKey::new("secret", "salary").unwrap());
    mask.entries.clear();
}
