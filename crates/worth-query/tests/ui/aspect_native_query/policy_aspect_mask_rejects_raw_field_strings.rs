use worth_query::facade::foundation::PolicyAspectMask;

fn main() {
    let _ = PolicyAspectMask::allow_all().with_masked("secret", "salary");
    let _ =
        PolicyAspectMask::allow_all().with_non_disclosing_use_only("secret", "salary");
}
