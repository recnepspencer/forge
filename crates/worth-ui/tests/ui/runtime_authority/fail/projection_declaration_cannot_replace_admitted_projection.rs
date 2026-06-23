use worth_ui::facade::{
    WorthUiAdmittedProjectionPlan, WorthUiHeaderMenuPlan, WorthUiProjectionDependencyDeclaration,
};

fn requires_admitted_projection(_projection: WorthUiAdmittedProjectionPlan<WorthUiHeaderMenuPlan>) {}

fn projection_dependency_declaration() -> WorthUiProjectionDependencyDeclaration {
    panic!("fixture should not run")
}

fn main() {
    let declaration = projection_dependency_declaration();

    requires_admitted_projection(declaration);
}
