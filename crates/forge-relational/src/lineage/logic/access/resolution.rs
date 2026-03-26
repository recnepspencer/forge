use std::collections::BTreeSet;

use crate::history::data::BranchId;
use crate::identity::data::LineageId;
use crate::lineage::data::{
    HistoricalLineageResolution, HistoricalLineageResolutionDigestBasis,
    HistoricalLineageResolutionMetrics, HistoricalResolutionBoundednessBasis,
    HistoricalResolutionDigestMode, HistoricalResolutionRequest, HistoricalResolutionTrace,
    LineageEventKind, RecordHistoryRequest,
};
use crate::lineage::logic::access::LineageAccess;

#[derive(Debug, Clone)]
struct BranchScopedHistoricalResolutionRequest {
    branch_id: BranchId,
    lineage_id: LineageId,
    boundedness_basis: HistoricalResolutionBoundednessBasis,
}

impl BranchScopedHistoricalResolutionRequest {
    fn from_request(request: HistoricalResolutionRequest) -> Self {
        Self {
            branch_id: request.branch_id,
            lineage_id: request.lineage_id,
            boundedness_basis: request.boundedness_basis,
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
        let mut scheduled_event_positions = self
            .runtime
            .lineage
            .branch_event_positions_for_sources(&request.branch_id, &current);
        let mut visited_event_positions = BTreeSet::new();
        let mut traversed_event_ids = Vec::new();
        let mut branch_event_scan_count = 0;

        while let Some(position) = scheduled_event_positions.first().copied() {
            scheduled_event_positions.remove(&position);
            if !visited_event_positions.insert(position) {
                continue;
            }
            branch_event_scan_count += 1;
            let event = &self.runtime.lineage.events[position];
            if !event
                .sources()
                .iter()
                .any(|source| current.contains(source))
            {
                continue;
            }
            match event.kind() {
                LineageEventKind::Replace
                | LineageEventKind::Split
                | LineageEventKind::Merge
                | LineageEventKind::Correspond => {
                    traversed_event_ids.push(event.event_id());
                    for source in event.sources() {
                        current.remove(source);
                    }
                    let new_targets = event.targets().iter().copied().collect::<BTreeSet<_>>();
                    current.extend(new_targets.iter().copied());
                    scheduled_event_positions.extend(
                        self.runtime
                            .lineage
                            .branch_event_positions_for_sources(&request.branch_id, &new_targets),
                    );
                }
                LineageEventKind::Create | LineageEventKind::Retire => {}
            }
        }
        let traversed_event_count = traversed_event_ids.len();
        self.runtime
            .performance_access()
            .count_lineage_historical_resolution(branch_event_scan_count, traversed_event_count);

        let metrics = HistoricalLineageResolutionMetrics {
            traversed_event_count,
            branch_event_scan_count,
            resolved_lineage_count: current.len(),
        };
        let digest_basis = HistoricalLineageResolutionDigestBasis::new(
            request.branch_id.clone(),
            request.lineage_id,
            current.iter().copied().collect(),
            traversed_event_ids.clone(),
            request.boundedness_basis,
            HistoricalResolutionDigestMode::ExactDigestCanonicalOrder,
        );

        HistoricalLineageResolution::new(
            request.branch_id,
            request.lineage_id,
            current.iter().copied().collect(),
            request.boundedness_basis,
            traversed_event_ids.clone(),
            digest_basis.clone(),
            HistoricalResolutionTrace::new(
                traversed_event_ids,
                request.boundedness_basis,
                digest_basis,
                metrics,
            ),
            metrics,
        )
    }

    pub fn resolve_record_history(
        &self,
        request: RecordHistoryRequest,
    ) -> Option<HistoricalLineageResolution> {
        let lineage = self.for_record(request.entity_id)?;
        Some(
            self.resolve_historical_lineage(HistoricalResolutionRequest {
                branch_id: request.branch_id,
                lineage_id: lineage.lineage_id,
                boundedness_basis: request.boundedness_basis,
            }),
        )
    }
}
