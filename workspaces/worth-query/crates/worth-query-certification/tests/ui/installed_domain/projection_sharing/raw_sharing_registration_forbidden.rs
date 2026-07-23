use worth_query::facade::live::WorthQueryManagedLiveHandle;
use worth_query::facade::runtime::WorthQueryWorkspace;

fn register_raw_handle(
    workspace: &mut WorthQueryWorkspace,
    handle: WorthQueryManagedLiveHandle,
) {
    workspace.register_shared_projection_owner(handle);
}

fn main() {}
