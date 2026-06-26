use forge_query::facade::ProjectMaterializedFacts;

fn main() {
    let _ = ProjectMaterializedFacts::declare().terminal_display_field("profile.display_name");
    let _ =
        ProjectMaterializedFacts::declare().terminal_derived_scalar_field("profile.display_name");
}
