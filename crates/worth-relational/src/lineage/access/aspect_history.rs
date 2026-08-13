use crate::history::data::{LineageAspectHistory, LineageAspectHistoryQueryResult};
use crate::lineage::access::LineageAccess;
use crate::lineage::data::HistoricalResolutionRequest;
use crate::visibility::materialization::read_records::ProjectionAspectFilter;

impl<'runtime> LineageAccess<'runtime> {
    pub fn entity_aspect_history(
        &self,
        request: HistoricalResolutionRequest,
        filter: Option<&ProjectionAspectFilter>,
    ) -> Option<LineageAspectHistory> {
        self.entity_aspect_history_with_trace(request, filter)
            .history
    }

    pub fn entity_aspect_history_with_trace(
        &self,
        request: HistoricalResolutionRequest,
        filter: Option<&ProjectionAspectFilter>,
    ) -> LineageAspectHistoryQueryResult {
        self.runtime
            .history()
            .lineage_entity_aspect_history_with_trace(
                &request.branch_id,
                request.lineage_id,
                filter,
            )
    }
}
