use worth_query::facade::domain::{
    WorthQueryArtifactProjectedChunkCursor, WorthQueryArtifactProjectedChunkView,
};

fn escape<'a>(
    cursor: &mut WorthQueryArtifactProjectedChunkCursor<'a>,
) -> WorthQueryArtifactProjectedChunkView<'a> {
    cursor.next(|view| view).unwrap().unwrap()
}

fn main() {}
