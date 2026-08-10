mod admission;
mod application;
mod classification;
mod cleanup;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DependencyReconciliationReport {
    pub added: u32,
    pub removed: u32,
    pub unchanged: u32,
}

pub(super) use classification::SubscriberBatchOp;
