use worth_query::facade::live::WorthQueryManagedLiveHandle;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn close_twice(handle: WorthQueryManagedLiveHandle, workspace: &mut WorthQueryWorkspace) {
    let _first = handle.close(workspace);
    let _second = handle.close(workspace);
}

fn main() {}
