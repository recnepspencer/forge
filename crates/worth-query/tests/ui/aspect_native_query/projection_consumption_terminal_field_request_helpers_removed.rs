use worth_query::facade::foundation::ProjectMaterializedFacts;

fn main() {
    let _ = ProjectMaterializedFacts::declare().terminal_display_field("profile.display_name");
    let _ =
        ProjectMaterializedFacts::declare().terminal_derived_field_field("profile.display_name");
}
