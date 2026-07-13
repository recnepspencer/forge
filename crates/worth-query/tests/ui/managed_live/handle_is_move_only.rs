use worth_query::facade::read::WorthQueryManagedLiveHandle;

fn duplicate(handle: WorthQueryManagedLiveHandle) {
    let _copy = handle.clone();
}

fn main() {}
