use forge_store::{
    AdmittedAspectLayoutReadPlan, AspectLayoutPerformanceEnvelope, AspectLayoutReadRequest,
    AspectLayoutSliceId, AspectLayoutTarget, AspectProjectionSet, AspectReadRegime,
    AspectScopeClass, ComplexityStatus, SingleEntityAspectScope, StructuralBlockId,
};
use forge_relational::facade::history::{BranchId, CommitId};

fn main() {
    let request = AspectLayoutReadRequest::new(
        AspectLayoutTarget::new(BranchId("main".to_string()), CommitId(1)),
        AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-a")),
        AspectProjectionSet::new(vec!["profile".to_string()]),
    );
    let _ = AdmittedAspectLayoutReadPlan {
        request,
        slice_ids: vec![AspectLayoutSliceId::new("slice-a")],
        structural_block_id: StructuralBlockId::new("block-a"),
        performance: AspectLayoutPerformanceEnvelope {
            strategy: AspectReadRegime::DirectLayoutSlice,
            scope_class: "single_entity".to_string(),
            complexity_status: ComplexityStatus::Verified,
            fallback_class: forge_store::AspectLayoutFallbackClass::None,
            layout_slices_read: 1,
            blocks_decoded: 1,
            control_replay_breadth: 1,
            chunk_count: 0,
        },
    };
}
