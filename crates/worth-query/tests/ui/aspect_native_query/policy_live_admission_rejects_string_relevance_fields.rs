use worth_query::facade::policy::PolicyAwareLiveRelevanceContract;

fn main() {
    let relevance = relevance_fixture();
    let _: &[String] = relevance.authorized_field_paths();
}

fn relevance_fixture() -> PolicyAwareLiveRelevanceContract {
    panic!("fixture only")
}
