use forge_store::{
    AspectLayoutSliceId, DedupAdmittedBlockReuse, EquivalenceContractVersion, StructuralBlockId,
};
use forge_relational::facade::history::{BranchId, CommitId};

fn main() {
    let _ = DedupAdmittedBlockReuse {
        branch_id: BranchId("main".to_string()),
        frontier_commit_id: CommitId(1),
        scope_class: "single_entity".to_string(),
        structural_block_id: StructuralBlockId::new("block-a"),
        equivalence_contract_version: EquivalenceContractVersion::new(2),
        slice_ids: vec![AspectLayoutSliceId::new("slice-a")],
    };
}
