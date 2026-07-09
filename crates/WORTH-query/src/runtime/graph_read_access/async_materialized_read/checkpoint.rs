use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadCheckpointInterval {
    digest: String,
    frontier_pages: usize,
}

impl WorthQueryGraphReadCheckpointInterval {
    pub fn frontier_pages(frontier_pages: usize) -> Self {
        let normalized_frontier_pages = frontier_pages.max(1);
        let digest = hash_parts(&[
            "worth_query_graph_read_checkpoint_interval_v1".to_string(),
            format!("frontier_pages:{normalized_frontier_pages}"),
        ]);
        Self {
            digest,
            frontier_pages: normalized_frontier_pages,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn frontier_page_count(&self) -> usize {
        self.frontier_pages
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadMaterializationCheckpoint {
    digest: String,
    request_digest: String,
    sequence: usize,
    touched_edges: usize,
    emitted_rows: usize,
    resident_bytes: usize,
}

impl WorthQueryGraphReadMaterializationCheckpoint {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn sequence(&self) -> usize {
        self.sequence
    }

    pub fn touched_edges(&self) -> usize {
        self.touched_edges
    }

    pub fn emitted_rows(&self) -> usize {
        self.emitted_rows
    }

    pub fn resident_bytes(&self) -> usize {
        self.resident_bytes
    }

    pub(crate) fn from_progress(
        request_digest: impl Into<String>,
        sequence: usize,
        touched_edges: usize,
        emitted_rows: usize,
        resident_bytes: usize,
    ) -> Self {
        let request_digest = request_digest.into();
        let digest = hash_parts(&[
            "worth_query_graph_read_materialization_checkpoint_v1".to_string(),
            format!("request:{request_digest}"),
            format!("sequence:{sequence}"),
            format!("touched_edges:{touched_edges}"),
            format!("emitted_rows:{emitted_rows}"),
            format!("resident_bytes:{resident_bytes}"),
        ]);
        Self {
            digest,
            request_digest,
            sequence,
            touched_edges,
            emitted_rows,
            resident_bytes,
        }
    }
}
