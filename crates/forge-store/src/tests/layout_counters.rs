use crate::{
    AspectLayoutReadPlanDecision, AspectLayoutReadRequest, AspectLayoutTarget, AspectProjectionSet,
    AspectScopeClass, EntitySetUniformAspectScope, ForgeStoreBuilder, SingleEntityAspectScope,
};

use super::harness::fixtures::runtime::{create_entity, latest_envelope, runtime_with_demo_schema};

#[test]
fn layout_counters_track_admitted_and_fallback_paths() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let root = latest_envelope(&runtime);
    let branch_id = root.branch_context.clone();
    let commit_id = root.commit.commit_id;

    let mut store = ForgeStoreBuilder::new().in_memory().build().unwrap();
    store.append_canonical_commit(root).unwrap();

    let admitted = store
        .plan_aspect_layout_read(AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id.clone(), commit_id),
            AspectScopeClass::SingleEntity(SingleEntityAspectScope::new("entity-a")),
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ))
        .unwrap();
    let admitted = match admitted {
        AspectLayoutReadPlanDecision::Admitted(plan) => plan,
        other => panic!("expected admitted plan, got {other:?}"),
    };

    let _reuse = store.admit_structural_block_reuse(admitted.clone()).unwrap();
    let frozen = store.freeze_chunk_model(admitted.clone()).unwrap();
    let _milestone_7 = store
        .admit_milestone_7_independent_layout_reference(admitted)
        .unwrap();
    let _milestone_9 = store
        .admit_milestone_9_physical_chunk_reference(frozen)
        .unwrap();

    let fallback = store
        .plan_aspect_layout_read(AspectLayoutReadRequest::new(
            AspectLayoutTarget::new(branch_id, commit_id),
            AspectScopeClass::EntitySetUniform(EntitySetUniformAspectScope::new(
                (0..40).map(|index| format!("entity-{index}")).collect(),
            )),
            AspectProjectionSet::new(vec!["profile".to_string()]),
        ))
        .unwrap();
    assert!(matches!(fallback, AspectLayoutReadPlanDecision::Fallback(_)));

    let counters = store.counters();
    assert_eq!(counters.aspect_layout_plan_count, 2);
    assert_eq!(counters.aspect_layout_admitted_count, 1);
    assert_eq!(counters.aspect_layout_fallback_count, 1);
    assert_eq!(counters.aspect_layout_rejected_count, 0);
    assert_eq!(counters.structural_block_reuse_admission_count, 1);
    assert_eq!(counters.chunk_model_freeze_count, 1);
    assert_eq!(counters.milestone_7_layout_reference_admission_count, 1);
    assert_eq!(
        counters.milestone_9_physical_chunk_reference_admission_count,
        1
    );
    assert!(counters.aspect_layout_slice_read_count >= 1);
}
