use crate::derived_topology::invalidation_plan::catalog::{
    current_derived_invalidation_family_catalog, DerivedInvalidationFamilyCatalogCloseout,
};
use crate::derived_topology::invalidation_plan::inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
#[cfg(test)]
use crate::derived_topology::invalidation_plan::catalog::{
    DerivedInvalidationFamilyCatalog, DerivedTopologyConsumedGraphFacts,
    DerivedTopologyDiagnosticPosture, DerivedTopologyInvalidationPredicate,
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyProductFamilyRecord, DerivedTopologyProductFamilyRecordInput,
    DerivedTopologyQueryReceiptPosture, DerivedTopologySpatialEvidencePosture,
    DerivedTopologySupportPosture, DerivedTopologyUpdatePosture,
};

pub(crate) fn catalog_closeout() -> DerivedInvalidationFamilyCatalogCloseout {
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout =
        DerivedInvalidationAuthorityInventoryCloseout::close(inventory).unwrap();
    let catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .unwrap();
    DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap()
}

#[cfg(test)]
pub(crate) fn catalog_closeout_with_loop_cycles_postures(
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
    legality_receipt_posture: DerivedTopologyLegalityReceiptPosture,
) -> DerivedInvalidationFamilyCatalogCloseout {
    catalog_closeout_with_loop_cycles_contract(
        query_receipt_posture,
        legality_receipt_posture,
        DerivedTopologyUpdatePosture::IncrementalEligible,
    )
}

#[cfg(test)]
pub(crate) fn catalog_closeout_with_loop_cycles_contract(
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
    legality_receipt_posture: DerivedTopologyLegalityReceiptPosture,
    update_posture: DerivedTopologyUpdatePosture,
) -> DerivedInvalidationFamilyCatalogCloseout {
    let inventory = current_derived_invalidation_authority_inventory();
    let inventory_closeout =
        DerivedInvalidationAuthorityInventoryCloseout::close(inventory).unwrap();
    let current_catalog =
        current_derived_invalidation_family_catalog(inventory_closeout.phase_two_seed().clone())
            .unwrap();
    let families = current_catalog
        .families()
        .iter()
        .cloned()
        .map(|family| {
            if family.identity() == DerivedTopologyProductFamilyIdentity::LoopCycles {
                product_family_record(
                    family.identity(),
                    family.consumed_graph_facts().clone(),
                    family.invalidation_predicate(),
                    query_receipt_posture,
                    legality_receipt_posture,
                    update_posture,
                    family.spatial_evidence_posture(),
                )
            } else {
                family
            }
        })
        .collect::<Vec<_>>();
    let catalog = DerivedInvalidationFamilyCatalog::new(
        inventory_closeout.phase_two_seed().clone(),
        families,
    );
    DerivedInvalidationFamilyCatalogCloseout::close(catalog).unwrap()
}

#[cfg(test)]
fn product_family_record(
    identity: DerivedTopologyProductFamilyIdentity,
    consumed_graph_facts: DerivedTopologyConsumedGraphFacts,
    invalidation_predicate: DerivedTopologyInvalidationPredicate,
    query_receipt_posture: DerivedTopologyQueryReceiptPosture,
    legality_receipt_posture: DerivedTopologyLegalityReceiptPosture,
    update_posture: DerivedTopologyUpdatePosture,
    spatial_evidence_posture: DerivedTopologySpatialEvidencePosture,
) -> DerivedTopologyProductFamilyRecord {
    DerivedTopologyProductFamilyRecord::from_input(DerivedTopologyProductFamilyRecordInput {
        identity,
        consumed_graph_facts: Some(consumed_graph_facts),
        invalidation_predicate: Some(invalidation_predicate),
        update_posture: Some(update_posture),
        spatial_evidence_posture: Some(spatial_evidence_posture),
        query_receipt_posture: Some(query_receipt_posture),
        legality_receipt_posture: Some(legality_receipt_posture),
        diagnostic_posture: Some(DerivedTopologyDiagnosticPosture::ProductFamilyWitnessRequired),
        support_posture: Some(DerivedTopologySupportPosture::QuerySupportRequired),
    })
    .unwrap()
}
