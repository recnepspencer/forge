use std::collections::BTreeSet;

use crate::history::data::BranchId;
use crate::identity::data::LineageId;
use crate::lineage::data::{
    HistoricalLineageResolution, HistoricalLineageResolutionMetrics, HistoricalResolutionRequest,
    HistoricalResolutionTrace, LineageEventKind, RecordHistoryRequest,
};
use crate::lineage::logic::access::LineageAccess;

#[derive(Debug, Clone)]
struct BranchScopedHistoricalResolutionRequest {
    branch_id: BranchId,
    lineage_id: LineageId,
}

impl BranchScopedHistoricalResolutionRequest {
    fn from_request(request: HistoricalResolutionRequest) -> Self {
        Self {
            branch_id: request.branch_id,
            lineage_id: request.lineage_id,
        }
    }
}

impl<'runtime> LineageAccess<'runtime> {
    pub fn resolve_historical_lineage(
        &self,
        request: HistoricalResolutionRequest,
    ) -> HistoricalLineageResolution {
        self.resolve_branch_scoped_history(BranchScopedHistoricalResolutionRequest::from_request(
            request,
        ))
    }

    fn resolve_branch_scoped_history(
        &self,
        request: BranchScopedHistoricalResolutionRequest,
    ) -> HistoricalLineageResolution {
        let mut current = BTreeSet::from([request.lineage_id]);
        let mut traversed_event_ids = Vec::new();
        let mut branch_event_scan_count = 0;

        for event in self.runtime.lineage.branch_events(&request.branch_id) {
            branch_event_scan_count += 1;
            if !event.sources.iter().any(|source| current.contains(source)) {
                continue;
            }
            match event.kind {
                LineageEventKind::Replace
                | LineageEventKind::Split
                | LineageEventKind::Merge
                | LineageEventKind::Correspond => {
                    traversed_event_ids.push(event.event_id);
                    for source in &event.sources {
                        current.remove(source);
                    }
                    current.extend(event.targets.iter().copied());
                }
                LineageEventKind::Create | LineageEventKind::Retire => {}
            }
        }
        let traversed_event_count = traversed_event_ids.len();
        self.runtime.performance_access().count_lineage_historical_resolution(
            branch_event_scan_count,
            traversed_event_count,
        );

        let metrics = HistoricalLineageResolutionMetrics {
            traversed_event_count,
            branch_event_scan_count,
            resolved_lineage_count: current.len(),
        };

        HistoricalLineageResolution {
            branch_id: request.branch_id,
            start: request.lineage_id,
            resolved: current.iter().copied().collect(),
            traversed_event_ids: traversed_event_ids.clone(),
            trace: HistoricalResolutionTrace {
                traversed_event_ids,
                metrics,
            },
            metrics,
        }
    }

    pub fn resolve_record_history(
        &self,
        request: RecordHistoryRequest,
    ) -> Option<HistoricalLineageResolution> {
        let lineage = self.for_record(request.entity_id)?;
        Some(self.resolve_historical_lineage(HistoricalResolutionRequest {
            branch_id: request.branch_id,
            lineage_id: lineage.lineage_id,
        }))
    }
}
