use crate::facade::history::BranchId;
use crate::facade::lineage::HistoricalResolutionRequest;
use crate::tests::support::*;

#[test]
fn topology_identity_survival_smoke_is_lineage_shape_preserving() {
    let mut runtime = cad_runtime();
    let created = create_entity_outcome(&mut runtime, "topology-source");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime
        .lineage_access()
        .for_record(entity)
        .unwrap()
        .lineage_id;

    let _replacement = update_entity(&mut runtime, entity, "topology-source-updated");
    let resolution = runtime
        .lineage_access()
        .resolve_historical_lineage(HistoricalResolutionRequest {
            branch_id: BranchId("main".to_string()),
            lineage_id: start_lineage,
        });

    assert!(!resolution.resolved.is_empty());
}
