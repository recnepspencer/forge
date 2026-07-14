use worth_query::facade::foundation::{ProjectMaterializedFacts, ProjectionConsumptionBindingContext, ProjectionConsumptionDeclaration, ProjectionConsumptionSource};

fn main() {
    let _ = ProjectionConsumptionDeclaration {
        source: unsafe { std::mem::zeroed::<ProjectionConsumptionSource>() },
        binding: unsafe { std::mem::zeroed::<ProjectionConsumptionBindingContext>() },
        requested: ProjectMaterializedFacts::declare().entity_identities(),
        declaration_digest: String::new(),
    };
}
