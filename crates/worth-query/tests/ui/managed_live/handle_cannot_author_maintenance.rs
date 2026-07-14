use worth_query::facade::live::WorthQueryManagedLiveHandle;

fn author_maintenance(handle: &mut WorthQueryManagedLiveHandle) {
    handle.advance_maintenance();
}

fn main() {}
