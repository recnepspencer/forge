use worth_query::facade::domain::WorthQueryExecutionSharingWitness;

fn require_clone<T: Clone>() {}

fn main() {
    require_clone::<WorthQueryExecutionSharingWitness>();
}
