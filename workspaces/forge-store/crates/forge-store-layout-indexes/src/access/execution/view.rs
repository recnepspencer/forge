#[derive(Debug, PartialEq, Eq)]
pub enum ExecutedLayoutOperation {
    BTreeLookup(crate::BaselineBTreeLookupExecution),
    BTreeReplay(crate::BaselineBTreeReplayRecoveryExecution),
    LsmLookup(crate::BaselineLsmLookupExecution),
    LsmRunPublication(crate::BaselineLsmManifestPublicationExecution),
    LsmReplay(crate::BaselineLsmReplayExecution),
    LsmCompaction(crate::BaselineLsmCompactionPublicationReceipt),
    DegradedScan(super::DegradedScanExecution),
}

macro_rules! observe_owner_execution {
    ($owner:ty, $case:ident) => {
        impl From<$owner> for ExecutedLayoutOperation {
            fn from(executed: $owner) -> Self {
                Self::$case(executed)
            }
        }
    };
}

observe_owner_execution!(crate::BaselineBTreeLookupExecution, BTreeLookup);
observe_owner_execution!(crate::BaselineBTreeReplayRecoveryExecution, BTreeReplay);
observe_owner_execution!(crate::BaselineLsmLookupExecution, LsmLookup);
observe_owner_execution!(
    crate::BaselineLsmManifestPublicationExecution,
    LsmRunPublication
);
observe_owner_execution!(crate::BaselineLsmReplayExecution, LsmReplay);
observe_owner_execution!(
    crate::BaselineLsmCompactionPublicationReceipt,
    LsmCompaction
);
observe_owner_execution!(super::DegradedScanExecution, DegradedScan);
