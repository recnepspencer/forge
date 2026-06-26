use forge_query::facade::PolicyAwareLiveRelevanceContract;

fn main() {
    let relevance = relevance_fixture();
    let _ = relevance.authorized_fields();
}

fn relevance_fixture() -> PolicyAwareLiveRelevanceContract {
    panic!("fixture only")
}
