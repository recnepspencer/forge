use worth_query::facade::live::WorthQueryLiveRequest;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn misuse(request: WorthQueryLiveRequest, workspace: &mut WorthQueryWorkspace) {
    request.run(workspace);
}

fn main() {}
