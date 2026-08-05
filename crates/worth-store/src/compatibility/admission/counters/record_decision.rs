use super::*;

impl CompatibilityAdmissionCounters {
    pub(crate) fn record_relation_recheck(&mut self) {
        self.relation_recheck_count += 1;
    }

    pub(crate) fn record_edge_missing_rejection(&mut self) {
        self.edge_missing_rejection_count += 1;
    }

    pub(in crate::compatibility::admission) fn record_admitted_relation(
        &mut self,
        relation: CompatibilityRelation,
    ) {
        match relation {
            CompatibilityRelation::Native => self.admitted_native_count += 1,
            CompatibilityRelation::BackwardRead | CompatibilityRelation::ForwardRead => {
                self.admitted_forward_backward_count += 1;
            }
            CompatibilityRelation::AdapterRequired => self.admitted_adapter_count += 1,
            CompatibilityRelation::DerivedRebuildRequired | CompatibilityRelation::Incompatible => {
            }
        }
    }
}
