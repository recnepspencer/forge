use worth_query::facade::live::WorthQueryManagedLiveHandle;

fn duplicate(handle: WorthQueryManagedLiveHandle) {
    let _copy = handle.clone();
}

fn main() {}
