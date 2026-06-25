use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

use crate::derived_topology::invalidation_plan::catalog::{
    catalog_digest, DerivedInvalidationFamilyCatalogCloseout, DerivedTopologyProductFamilyIdentity,
};
use crate::derived_topology::invalidation_plan::execution::{
    DerivedInvalidationExecutionReceipt, PlannedDerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::inventory::{
    current_derived_invalidation_authority_inventory, DerivedInvalidationAuthorityInventoryCloseout,
};
use crate::derived_topology::invalidation_plan::migrated_products::{
    close_covered_derived_product_migration_sweep, status_rows_from_migrated_family_closeouts,
    CoveredDerivedProductMigrationError, CoveredDerivedProductMigrationSweepCloseout,
    MigratedDerivedProductFamilyCloseout,
};
use crate::derived_topology::invalidation_plan::operator_cutover::{
    close_derived_invalidation_operator_cutover, DerivedInvalidationOperatorCutoverCloseout,
    DerivedInvalidationOperatorCutoverReceipt,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, loop_cycles_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};
use crate::topology_operators::application::TopologyMutationApplicationEvidence;
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(super) fn selected_plan_for_closeout(
    catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
) -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        catalog_closeout,
        &loop_cycles_touched_closure("phase-ten.closeout"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("milestone ten selected plan")
}

pub(super) fn execution_receipt(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> DerivedInvalidationExecutionReceipt {
    DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &PlannedDerivedInvalidationProductExecutor,
    )
    .expect("milestone ten execution receipt")
}

pub(super) fn full_migration_sweep(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> CoveredDerivedProductMigrationSweepCloseout {
    let receipt = execution_receipt(selected_plan);
    let family_closeouts = DerivedTopologyProductFamilyIdentity::REQUIRED
        .iter()
        .copied()
        .map(|family| family_specific_closeout_for_receipt_family(family, &receipt))
        .collect::<Vec<_>>();
    let family_refs = family_closeouts.iter().collect::<Vec<_>>();
    close_covered_derived_product_migration_sweep(
        selected_plan,
        status_rows_from_migrated_family_closeouts(
            &family_refs,
            "all-required-families-are-migrated",
        ),
    )
    .expect("full product migration sweep")
}

pub(super) fn partial_migration_sweep(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> Result<CoveredDerivedProductMigrationSweepCloseout, CoveredDerivedProductMigrationError> {
    close_covered_derived_product_migration_sweep(
        selected_plan,
        status_rows_from_migrated_family_closeouts(&[], "milestone-ten-partial"),
    )
}

pub(super) fn inventory_closeout() -> DerivedInvalidationAuthorityInventoryCloseout {
    DerivedInvalidationAuthorityInventoryCloseout::close(
        current_derived_invalidation_authority_inventory(),
    )
    .expect("authority inventory closeout")
}

pub(super) fn operator_closeout(
    migration_sweep: &CoveredDerivedProductMigrationSweepCloseout,
    selected_plan: &DerivedInvalidationSelectedPlan,
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> DerivedInvalidationOperatorCutoverCloseout {
    let operator_cutover = DerivedInvalidationOperatorCutoverReceipt::bind_operator_cutover(
        migration_sweep,
        selected_plan,
        execution_receipt,
        &matching_operator_touch_proof(),
        &admitted_operator_evidence(),
    )
    .expect("operator cutover proof");
    close_derived_invalidation_operator_cutover(operator_cutover).expect("operator closeout")
}

fn family_specific_closeout_for_receipt_family(
    family: DerivedTopologyProductFamilyIdentity,
    receipt: &DerivedInvalidationExecutionReceipt,
) -> MigratedDerivedProductFamilyCloseout {
    if let Some(row) = receipt
        .executed_rows()
        .iter()
        .find(|row| row.family_identity() == family)
    {
        let product_output_digest = row
            .product_output_digest()
            .unwrap_or_else(|| row.execution_report_digest());
        return MigratedDerivedProductFamilyCloseout::new(
            family,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            product_output_digest,
            &family_specific_fixture_digest("old-authority-residue", family, row.row_digest()),
            &family_specific_fixture_digest("counters", family, row.row_digest()),
        );
    }

    if let Some(row) = receipt
        .unaffected_rows()
        .iter()
        .find(|row| row.family_identity() == family)
    {
        return MigratedDerivedProductFamilyCloseout::new(
            family,
            receipt.selected_plan_digest(),
            receipt.execution_receipt_digest(),
            row.row_digest(),
            &family_specific_fixture_digest("product-output", family, row.row_digest()),
            &family_specific_fixture_digest("old-authority-residue", family, row.row_digest()),
            &family_specific_fixture_digest("counters", family, row.row_digest()),
        );
    }

    panic!("execution receipt should expose every required family for closeout fixture")
}

fn family_specific_fixture_digest(
    label: &str,
    family: DerivedTopologyProductFamilyIdentity,
    row_digest: &str,
) -> String {
    catalog_digest([
        "worth-topo:milestone-ten-family-specific-closeout-fixture:v1".to_string(),
        format!("label:{label}"),
        format!("family:{}", family.as_str()),
        format!("row:{row_digest}"),
    ])
}

fn matching_operator_touch_proof() -> TopologyDeclaredTouchedGraphBasisProof {
    let basis = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(1)),
            TopologyTouchedEntity::new(entity_id(2)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(3))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
        vec![TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    TopologyDeclaredTouchedGraphBasisProof::from_basis_for_tests("phase-ten.closeout", basis)
        .expect("matching operator touched basis")
}

fn admitted_operator_evidence() -> TopologyMutationApplicationEvidence {
    TopologyMutationApplicationEvidence::from_cutover_test_parts(
        Some("graph-obligation-envelope.phase-ten".to_string()),
        Some("graph-obligation-dispatch.phase-ten".to_string()),
        1,
    )
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
