use worth_query::facade::foundation::{AspectFieldKey, PolicyAspectMask};

fn main() {
    let mut mask =
        PolicyAspectMask::allow_all().with_masked(AspectFieldKey::from_authoring_parts("secret", "salary").unwrap());
    mask.entries.clear();
}
