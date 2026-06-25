use schema::facade::platform::relations::TopologyRelationKind;

use crate::derived_topology::invalidation_plan::catalog::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalog,
    DerivedInvalidationFamilyCatalogCloseout, DerivedTopologyConsumedGraphFacts,
    DerivedTopologyDiagnosticPosture, DerivedTopologyInvalidationPredicate,
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyProductFamilyRecord, DerivedTopologyProductFamilyRecordInput,
    DerivedTopologyQueryReceiptPosture, DerivedTopologySpatialEvidencePosture,
    DerivedTopologySupportPosture, DerivedTopologyUpdatePosture,
};
use crate::derived_topology::invalidation_plan::inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
use crate::topology_operators::TopologyTouchedAspect;

pub(crate) fn catalog_closeout() -> DerivedInvalidationFamilyCatalogCloseout {
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout =
        DerivedInvalidationAuthorityInventoryCloseout::close(inventory).unwrap();
    let catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .unwrap();
    DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap()
}

pub(crate) fn catalog_closeout_with_loop_cycles_postures(
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
    legality_receipt_posture: DerivedTopologyLegalityReceiptPosture,
) -> DerivedInvalidationFamilyCatalogCloseout {
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout =
        DerivedInvalidationAuthorityInventoryCloseout::close(inventory).unwrap();
    let families = DerivedTopologyProductFamilyIdentity::REQUIRED
        .iter()
        .copied()
        .map(|identity| {
            let is_loop_cycles = identity == DerivedTopologyProductFamilyIdentity::LoopCycles;
            product_family_for_posture_fixture(
                identity,
                if is_loop_cycles {
                    query_receipt_posture
                } else {
                    DerivedTopologyQueryReceiptPosture::NotRequiredForFamilyDeclaration
                },
                if is_loop_cycles {
                    legality_receipt_posture
                } else {
                    DerivedTopologyLegalityReceiptPosture::NotRequiredForFamilyDeclaration
                },
                is_loop_cycles,
            )
        })
        .collect::<Vec<_>>();
    let catalog = DerivedInvalidationFamilyCatalog::new(
        inventory_closeout.phase_two_seed().clone(),
        families,
    );
    DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap()
}

fn product_family_for_posture_fixture(
    identity: DerivedTopologyProductFamilyIdentity,
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
    legality_receipt_posture: DerivedTopologyLegalityReceiptPosture,
    should_match_loop_touch: bool,
) -> DerivedTopologyProductFamilyRecord {
    let (relation_kinds, aspects) = if should_match_loop_touch {
        (
            vec![TopologyRelationKind::HalfEdgeNext],
            vec![TopologyTouchedAspect::TopologyBoundary],
        )
    } else {
        (
            vec![TopologyRelationKind::ModelOwnsBody],
            vec![TopologyTouchedAspect::GeometryBinding],
        )
    };
    DerivedTopologyProductFamilyRecord::from_input(DerivedTopologyProductFamilyRecordInput {
        identity,
        consumed_graph_facts: Some(DerivedTopologyConsumedGraphFacts::new(
            relation_kinds,
            aspects,
        )),
        invalidation_predicate: Some(
            DerivedTopologyInvalidationPredicate::ConsumedGraphFactsIntersectTouchedClosure,
        ),
        update_posture: Some(DerivedTopologyUpdatePosture::IncrementalEligible),
        spatial_evidence_posture: Some(
            DerivedTopologySpatialEvidencePosture::NoSpatialEvidenceConsumed,
        ),
        query_receipt_posture: Some(query_receipt_posture),
        legality_receipt_posture: Some(legality_receipt_posture),
        diagnostic_posture: Some(DerivedTopologyDiagnosticPosture::ProductFamilyWitnessRequired),
        support_posture: Some(DerivedTopologySupportPosture::QuerySupportRequired),
    })
    .unwrap()
}
