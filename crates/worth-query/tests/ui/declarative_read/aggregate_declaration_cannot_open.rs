use worth_query::facade::aggregate::WorthQueryCountRequest;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn misuse(request: WorthQueryCountRequest, workspace: &mut WorthQueryWorkspace) {
    request.open(workspace);
}

fn main() {}
