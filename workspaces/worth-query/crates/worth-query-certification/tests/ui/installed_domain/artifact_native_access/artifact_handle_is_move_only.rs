use worth_query::facade::domain::WorthQueryMoveOnlyArtifactHandle;

fn duplicate(handle: WorthQueryMoveOnlyArtifactHandle) {
    let _copy = handle.clone();
}

fn main() {}
