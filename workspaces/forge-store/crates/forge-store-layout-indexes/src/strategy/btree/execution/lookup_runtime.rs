use super::{
    BaselineBTreeExecutionDenial, BaselineBTreeLookupAdmission, BaselineBTreeReadShape,
    BaselineBTreeReadSource, StableBTreeLookupExecution,
};
use crate::access::execution::BTreeLookupReady;
use forge_store_physical_format::PhysicalRecordSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BTreeLookupRuntime;

pub const fn btree_lookup_runtime() -> BTreeLookupRuntime {
    BTreeLookupRuntime
}

impl BTreeLookupRuntime {
    pub fn execute(
        self,
        ready: BTreeLookupReady,
        source: BaselineBTreeReadSource,
        probe_slot: PhysicalRecordSlot,
    ) -> Result<StableBTreeLookupExecution, BaselineBTreeExecutionDenial> {
        let shape = match ready.selected().operation() {
            crate::BTreeLookupOperation::Point => BaselineBTreeReadShape::PointLookup,
            crate::BTreeLookupOperation::Range => BaselineBTreeReadShape::RangeLookup,
            crate::BTreeLookupOperation::Prefix => BaselineBTreeReadShape::PrefixLookup,
        };
        let admission = BaselineBTreeLookupAdmission::admit(ready);
        source.execute(&admission, probe_slot, shape)
    }
}
