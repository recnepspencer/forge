use worth_query::facade::foundation::{ProjectMaterializedFacts, ProjectionFactFieldPath};

fn main() {
    let _ = ProjectionFactFieldPath::from_authoring_path("profile.display_name").unwrap();

    let _ = ProjectMaterializedFacts::declare().display_field("profile.display_name");
    let _ =
        ProjectMaterializedFacts::declare().derived_scalar_field("profile.display_name");
}
