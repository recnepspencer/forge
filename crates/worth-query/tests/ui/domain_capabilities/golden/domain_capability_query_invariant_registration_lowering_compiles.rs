use worth_query::facade::runtime::{WorthQueryRuntime, InvariantCatalog, InvariantRegistration, InvariantRule};

fn ordinary_query_invariant_registration_lane() {
    let _builder = WorthQueryRuntime::builder().invariant_catalog(InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(7),
        )],
    });
}

fn main() {}
