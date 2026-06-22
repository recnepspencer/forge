use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitScopeAdmission;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

pub struct PlanarBooleanSplitSourceEdgeCarrierRecoveryInput<'a> {
    scope_admission: &'a PlanarBooleanEdgeSplitScopeAdmission,
    event_ledger: &'a PlanarBooleanEventLedgerReceipt,
}

impl<'a> PlanarBooleanSplitSourceEdgeCarrierRecoveryInput<'a> {
    pub fn from_scope_and_event_ledger(
        scope_admission: &'a PlanarBooleanEdgeSplitScopeAdmission,
        event_ledger: &'a PlanarBooleanEventLedgerReceipt,
    ) -> Self {
        Self {
            scope_admission,
            event_ledger,
        }
    }

    pub(crate) fn scope_admission(&self) -> &'a PlanarBooleanEdgeSplitScopeAdmission {
        self.scope_admission
    }

    pub(crate) fn event_ledger(&self) -> &'a PlanarBooleanEventLedgerReceipt {
        self.event_ledger
    }
}
