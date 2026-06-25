use super::row::{
    DerivedInvalidationDenialRow, DerivedInvalidationResidueRow, DerivedInvalidationSelectedRow,
    DerivedInvalidationUnaffectedRow,
};
use super::selected_plan::{DerivedInvalidationSelectedPlan, DerivedInvalidationSelectedPlanInput};
use super::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationLegalitySupportEvidence,
    DerivedInvalidationQuerySupportEvidence, DerivedInvalidationSelectionCounters,
    DerivedInvalidationSelectionError, DerivedInvalidationSelectionErrorKind,
    DerivedInvalidationTouchedClosure,
};
use crate::derived_topology::invalidation_plan::catalog::{
    DerivedInvalidationFamilyCatalogCloseout, DerivedTopologyProductFamilyRecord,
};

pub(super) fn lower_selected_invalidation_plan(
    catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
    touched_closure: &DerivedInvalidationTouchedClosure,
    query_support: &DerivedInvalidationQuerySupportEvidence,
    legality_support: &DerivedInvalidationLegalitySupportEvidence,
    density_policy: DerivedInvalidationDensityPolicy,
) -> Result<DerivedInvalidationSelectedPlan, DerivedInvalidationSelectionError> {
    reject_empty_touched_closure(touched_closure)?;
    let mut selected_rows = Vec::new();
    let mut unaffected_rows = Vec::new();
    let mut denied_rows = Vec::new();
    let residue_rows = residue_rows_from_catalog_closeout(catalog_closeout);
    let mut matched_product_count = 0;

    for family in catalog_closeout.catalog().families() {
        if !family.matches_touched_basis(touched_closure.basis()) {
            unaffected_rows.push(DerivedInvalidationUnaffectedRow::from_family(family));
            continue;
        }
        matched_product_count += 1;
        if push_denials(family, query_support, legality_support, &mut denied_rows) {
            continue;
        }
        selected_rows.push(DerivedInvalidationSelectedRow::from_family(
            family,
            query_support.required_receipt_digest(family.query_receipt_posture()),
            legality_support.required_receipt_digest(family.legality_receipt_posture()),
        ));
    }

    let counters = DerivedInvalidationSelectionCounters::from_rows(
        touched_closure.counters(),
        catalog_closeout.catalog().families().len(),
        matched_product_count,
        &selected_rows,
        unaffected_rows.len(),
        &denied_rows,
        &residue_rows,
    );
    ensure_selection_breadth_is_bounded(&counters, catalog_closeout.catalog().families().len())?;
    Ok(DerivedInvalidationSelectedPlan::from_parts(
        DerivedInvalidationSelectedPlanInput {
            phase_three_seed_digest: catalog_closeout
                .phase_three_seed()
                .seed_digest()
                .to_string(),
            catalog_digest: catalog_closeout.catalog().catalog_digest().to_string(),
            touched_closure_digest: touched_closure.closure_digest().to_string(),
            query_support_digest: query_support.support_digest().to_string(),
            legality_support_digest: legality_support.support_digest().to_string(),
            density_policy,
            selected_rows,
            unaffected_rows,
            denied_rows,
            residue_rows,
            counters,
        },
    ))
}

fn push_denials(
    family: &DerivedTopologyProductFamilyRecord,
    query_support: &DerivedInvalidationQuerySupportEvidence,
    legality_support: &DerivedInvalidationLegalitySupportEvidence,
    denied_rows: &mut Vec<DerivedInvalidationDenialRow>,
) -> bool {
    let initial_len = denied_rows.len();
    if !query_support.supports(family.query_receipt_posture()) {
        denied_rows.push(DerivedInvalidationDenialRow::missing_query_support(family));
    }
    if !legality_support.supports(family.legality_receipt_posture()) {
        denied_rows.push(DerivedInvalidationDenialRow::missing_legality_support(
            family,
        ));
    }
    denied_rows.len() != initial_len
}

fn residue_rows_from_catalog_closeout(
    catalog_closeout: &DerivedInvalidationFamilyCatalogCloseout,
) -> Vec<DerivedInvalidationResidueRow> {
    let phase_two_seed = catalog_closeout.catalog().phase_two_seed();
    if phase_two_seed.capped_residue_count() == 0 {
        Vec::new()
    } else {
        vec![DerivedInvalidationResidueRow::from_phase_two_seed(
            phase_two_seed,
        )]
    }
}

fn reject_empty_touched_closure(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> Result<(), DerivedInvalidationSelectionError> {
    let counters = touched_closure.counters();
    let touched_count = counters.entity_count()
        + counters.relation_count()
        + counters.relation_kind_count()
        + counters.touched_aspect_count()
        + counters.topology_scope_count();
    if touched_count == 0 {
        return Err(DerivedInvalidationSelectionError::new(
            DerivedInvalidationSelectionErrorKind::TouchedClosureEmpty,
            "derived invalidation selection requires a non-empty touched closure",
        ));
    }
    Ok(())
}

fn ensure_selection_breadth_is_bounded(
    counters: &DerivedInvalidationSelectionCounters,
    catalog_family_count: usize,
) -> Result<(), DerivedInvalidationSelectionError> {
    if counters.candidate_product_count() > catalog_family_count
        || counters.matched_product_count() > counters.candidate_product_count()
        || counters.invalidated_product_count() > counters.candidate_product_count()
    {
        return Err(DerivedInvalidationSelectionError::new(
            DerivedInvalidationSelectionErrorKind::CounterLeakage,
            "derived invalidation selection counters exceeded catalog and touched closure bounds",
        ));
    }
    Ok(())
}
