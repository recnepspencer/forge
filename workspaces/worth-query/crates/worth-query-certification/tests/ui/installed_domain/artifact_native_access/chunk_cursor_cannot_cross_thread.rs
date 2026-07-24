use worth_query::facade::domain::WorthQueryArtifactChunkCursor;

fn cross_thread(cursor: WorthQueryArtifactChunkCursor<'static>) {
    std::thread::spawn(move || drop(cursor));
}

fn main() {}
