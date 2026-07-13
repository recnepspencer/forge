use forge_store_physical_isolation::{
    CompactionCutoverState, CompactionOwnerCase, CompactionOwnerCaseId,
};

fn main() {
    let _forged = CompactionOwnerCase {
        id: CompactionOwnerCaseId("physical.compaction.forged"),
        from: CompactionCutoverState::PlanAdmitted,
        to: CompactionCutoverState::Denied,
    };
}
