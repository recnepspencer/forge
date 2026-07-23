use worth_query::facade::domain::WorthQueryAdmittedConsumerInvalidation;

fn require_clone<T: Clone>() {}

fn main() {
    require_clone::<WorthQueryAdmittedConsumerInvalidation<'static>>();
}
