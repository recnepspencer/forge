use worth_query::facade::domain::WorthQueryConsumerInvalidationDelta;

fn require_clone<T: Clone>() {}

fn main() {
    require_clone::<WorthQueryConsumerInvalidationDelta>();
}
