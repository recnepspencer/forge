use crate::workload_composition::CompletedBooleanSplitHandoff;
use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use worth_spatial::facade::workload_vocabulary::SpatialGeometryEvidenceTouchAuthority;

pub(crate) struct LookupConflictRouteInputs {
    authority: SpatialGeometryEvidenceTouchAuthority,
    execution_receipt: EvidenceLookupExecutionReceipt,
}

pub(crate) fn lookup_conflict_route_inputs(
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) -> LookupConflictRouteInputs {
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let execution_receipt = completed_split_handoff
        .test_event_ledger_lookup_packet()
        .expect("event-ledger lookup packet")
        .execution_receipt()
        .clone();

    LookupConflictRouteInputs {
        authority,
        execution_receipt,
    }
}

impl LookupConflictRouteInputs {
    pub fn authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.authority
    }

    pub fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }
}
