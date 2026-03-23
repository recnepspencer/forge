use crate::history::data::{
    AspectFilter, LineageAspectHistory, LineageAspectHistoryQueryResult,
};
use crate::lineage::data::HistoricalResolutionRequest;
use crate::lineage::logic::access::LineageAccess;

impl<'runtime> LineageAccess<'runtime> {
    pub fn entity_aspect_history(
        &self,
        request: HistoricalResolutionRequest,
        filter: Option<&AspectFilter>,
    ) -> Option<LineageAspectHistory> {
        self.entity_aspect_history_with_trace(request, filter)
            .history
    }

    pub fn entity_aspect_history_with_trace(
        &self,
        request: HistoricalResolutionRequest,
        filter: Option<&AspectFilter>,
    ) -> LineageAspectHistoryQueryResult {
        self.runtime
            .history_access()
            .lineage_entity_aspect_history_with_trace(
                &request.branch_id,
                request.lineage_id,
                filter,
            )
    }
}
