use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadStreamingPageBudget {
    digest: String,
    max_page_width: usize,
    max_resident_frontier: usize,
    max_resident_visited: usize,
    max_page_result_bytes: u64,
}

impl WorthQueryGraphReadStreamingPageBudget {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn max_page_width(&self) -> usize {
        self.max_page_width
    }

    pub fn max_resident_frontier(&self) -> usize {
        self.max_resident_frontier
    }

    pub fn max_resident_visited(&self) -> usize {
        self.max_resident_visited
    }

    pub fn max_page_result_bytes(&self) -> u64 {
        self.max_page_result_bytes
    }

    pub(crate) fn frontier_default() -> Self {
        Self::new(1, 1, 1, 2048)
    }

    fn new(
        max_page_width: usize,
        max_resident_frontier: usize,
        max_resident_visited: usize,
        max_page_result_bytes: u64,
    ) -> Self {
        let digest = hash_parts(&[
            "worth_query_graph_read_streaming_page_budget_v1".to_string(),
            format!("max_page_width:{max_page_width}"),
            format!("max_resident_frontier:{max_resident_frontier}"),
            format!("max_resident_visited:{max_resident_visited}"),
            format!("max_page_result_bytes:{max_page_result_bytes}"),
        ]);
        Self {
            digest,
            max_page_width,
            max_resident_frontier,
            max_resident_visited,
            max_page_result_bytes,
        }
    }
}
