use forge_query::facade::PolicyAspectMask;

fn main() {
    let mut mask = PolicyAspectMask::allow_all().with_masked("secret", "salary");
    mask.entries.clear();
}
