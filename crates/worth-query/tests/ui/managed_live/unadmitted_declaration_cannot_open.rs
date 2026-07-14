use worth_query::facade::live::WorthQueryLiveDeclaration;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn activate_unadmitted(
    declaration: WorthQueryLiveDeclaration,
    workspace: &mut WorthQueryWorkspace,
) {
    declaration.open(workspace);
}

fn main() {}
