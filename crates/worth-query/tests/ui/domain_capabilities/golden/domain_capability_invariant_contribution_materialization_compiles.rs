#[path = "../support/installed_domain.rs"]
mod installed_domain;

use worth_query::facade::runtime::{WorthQueryIntentDeclaration, WorthQueryIntentInput, InvariantCatalog, InvariantRegistration, InvariantRule};

fn invariant_common_lane() {
    let declaration = WorthQueryIntentDeclaration::strategy_commit(
        "worth.spatial.commit",
        "spatial.commit",
        "1",
        "geometry.patch",
        WorthQueryIntentInput::object([("edge", WorthQueryIntentInput::string("e-1"))]),
    );
    let invariant_catalog = InvariantCatalog {
        registrations: vec![InvariantRegistration::commit_boundary_blocking(
            InvariantRule::MaxMergedIntents(7),
        )],
    };

    let installation = installed_domain::install("invariant-golden");
    let _artifact = installation
        .contributions()
        .for_intent(&declaration).expect("installed contribution authority must remain current")
        .register_invariant_catalog("spatial.non_manifold_edge_split", invariant_catalog)
        .because("geometry kernel must reject non-manifold edge splits")
        .materialize();
}

fn main() {}
