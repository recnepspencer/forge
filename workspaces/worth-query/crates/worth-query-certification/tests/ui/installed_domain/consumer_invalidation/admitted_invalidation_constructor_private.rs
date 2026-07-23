use worth_query::facade::domain::WorthQueryAdmittedConsumerInvalidation;

#[allow(unreachable_code)]
fn forge_admission() -> WorthQueryAdmittedConsumerInvalidation<'static> {
    WorthQueryAdmittedConsumerInvalidation { ..panic!() }
}

fn main() {}
