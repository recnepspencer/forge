use worth_query::facade::read::WorthQueryReadRequest;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn misuse(request: WorthQueryReadRequest, workspace: &mut WorthQueryWorkspace) {
    request.open(workspace);
}

fn main() {}
