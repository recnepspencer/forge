use super::super::{BaselineBTreeLookupAdmission, BaselineBTreeReadSource};
use super::{BTreeLookupExecutionOutcome, BaselineBTreeExecutionDenial, BaselineBTreeReadShape};
use crate::access::execution::BTreeLookupReady;
use worth_store_physical_format::PhysicalRecordSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BTreeLookupRuntime;

pub(crate) const fn btree_lookup_runtime() -> BTreeLookupRuntime {
    BTreeLookupRuntime
}

impl BTreeLookupRuntime {
    pub(crate) fn execute(
        self,
        ready: BTreeLookupReady,
        source: BaselineBTreeReadSource,
        probe_slot: PhysicalRecordSlot,
    ) -> BTreeLookupExecutionOutcome {
        let shape = match ready.selected().operation() {
            crate::BTreeLookupOperation::Point => BaselineBTreeReadShape::PointLookup,
            crate::BTreeLookupOperation::Range => BaselineBTreeReadShape::RangeLookup,
            crate::BTreeLookupOperation::Prefix => BaselineBTreeReadShape::PrefixLookup,
        };
        let admission = BaselineBTreeLookupAdmission::admit(ready);
        BTreeLookupExecutionOutcome::issue(execute(source, &admission, probe_slot, shape))
    }
}

fn execute(
    source: BaselineBTreeReadSource,
    admission: &BaselineBTreeLookupAdmission,
    probe_slot: PhysicalRecordSlot,
    shape: BaselineBTreeReadShape,
) -> Result<super::StableBTreeLookupExecution, BaselineBTreeExecutionDenial> {
    source.execute(admission, probe_slot, shape)
}
