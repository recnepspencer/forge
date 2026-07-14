use worth_store_layout_indexes::{
    BaselineBTreeLookupExecution, BaselineBTreeReplayRecoveryExecution,
    BaselineLsmCompactionPublicationReceipt, BaselineLsmLookupExecution,
    BaselineLsmManifestPublicationExecution, BaselineLsmReplayExecution, DegradedScanExecution,
    ExecutedLayoutOperation,
};

fn btree_lookup(value: BaselineBTreeReplayRecoveryExecution) -> ExecutedLayoutOperation {
    ExecutedLayoutOperation::BTreeLookup(value)
}

fn btree_replay(value: BaselineLsmLookupExecution) -> ExecutedLayoutOperation {
    ExecutedLayoutOperation::BTreeReplay(value)
}

fn lsm_lookup(value: BaselineLsmManifestPublicationExecution) -> ExecutedLayoutOperation {
    ExecutedLayoutOperation::LsmLookup(value)
}

fn lsm_publication(value: BaselineLsmReplayExecution) -> ExecutedLayoutOperation {
    ExecutedLayoutOperation::LsmRunPublication(value)
}

fn lsm_replay(value: BaselineLsmCompactionPublicationReceipt) -> ExecutedLayoutOperation {
    ExecutedLayoutOperation::LsmReplay(value)
}

fn lsm_compaction(value: DegradedScanExecution) -> ExecutedLayoutOperation {
    ExecutedLayoutOperation::LsmCompaction(value)
}

fn degraded(value: BaselineBTreeLookupExecution) -> ExecutedLayoutOperation {
    ExecutedLayoutOperation::DegradedScan(value)
}

fn main() {}
