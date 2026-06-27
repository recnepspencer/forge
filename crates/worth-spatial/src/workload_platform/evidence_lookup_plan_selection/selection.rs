use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyCatalogCloseout, EvidenceLookupFamilyDeclaration,
    EvidenceLookupFamilyIndexPostureKind,
};
use crate::workload_platform::evidence_lookup_input_admission::{
    EvidenceLookupAdmittedInput, EvidenceLookupQueryAdmissionSupport,
    EvidenceLookupTopologyAdmissionSupport,
};

use super::counters::EvidenceLookupPlanSelectionCounters;
use super::error::{EvidenceLookupPlanSelectionError, EvidenceLookupPlanSelectionErrorKind};
use super::plan_row::{
    EvidenceLookupPlanRowOutcome, EvidenceLookupSelectedPlanRow, EvidenceLookupSelectedPlanRowParts,
};
use super::query_posture::EvidenceLookupPlanQueryPosture;
use super::selected_plan::EvidenceLookupSelectedPlan;
use super::strategy::EvidenceLookupSelectedStrategy;
use super::topology_posture::EvidenceLookupPlanTopologyPosture;

pub fn select_evidence_lookup_plan(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    admitted: &EvidenceLookupAdmittedInput,
) -> Result<EvidenceLookupSelectedPlan, EvidenceLookupPlanSelectionError> {
    if catalog.catalog_digest() != admitted.catalog_digest() {
        return Err(EvidenceLookupPlanSelectionError::new(
            EvidenceLookupPlanSelectionErrorKind::CatalogAdmissionDigestMismatch,
            "selected lookup planning requires the same catalog that admitted the input",
        ));
    }

    let mut counters = EvidenceLookupPlanSelectionCounters::with_candidate_family_count(
        catalog.declarations().len(),
    );
    let mut topology_support = topology_support_by_family(admitted.topology_support(), counters)?;
    let mut query_support = query_support_by_family(admitted.query_support(), counters)?;
    let selected_family_identities =
        selected_family_identity_set(admitted.family_selection().family_identities());
    let mut rows = Vec::with_capacity(catalog.declarations().len());

    for family in catalog.declarations() {
        let row = select_family_row(
            family,
            admitted,
            &selected_family_identities,
            &mut topology_support,
            &mut query_support,
            &mut counters,
        )?;
        rows.push(row);
    }
    reject_unconsumed_admission_support(&topology_support, &query_support, counters)?;

    Ok(EvidenceLookupSelectedPlan::new(
        catalog.catalog_digest().to_string(),
        admitted.admission_digest().to_string(),
        admitted.spatial_touch_digest().to_string(),
        admitted.stage_receipt_digest().to_string(),
        admitted.stage(),
        rows,
        counters,
    ))
}

fn reject_unconsumed_admission_support(
    topology_support: &BTreeMap<String, EvidenceLookupTopologyAdmissionSupport>,
    query_support: &BTreeMap<String, EvidenceLookupQueryAdmissionSupport>,
    counters: EvidenceLookupPlanSelectionCounters,
) -> Result<(), EvidenceLookupPlanSelectionError> {
    if topology_support.is_empty() && query_support.is_empty() {
        return Ok(());
    }
    Err(EvidenceLookupPlanSelectionError::new(
        EvidenceLookupPlanSelectionErrorKind::AdmittedSupportCardinalityMismatch,
        format!(
            "selected lookup planning consumed catalog rows with {} topology support rows and {} query support rows left over",
            topology_support.len(),
            query_support.len()
        ),
    )
    .with_counters(counters))
}

fn select_family_row(
    family: &EvidenceLookupFamilyDeclaration,
    admitted: &EvidenceLookupAdmittedInput,
    selected_family_identities: &BTreeSet<String>,
    topology_support: &mut BTreeMap<String, EvidenceLookupTopologyAdmissionSupport>,
    query_support: &mut BTreeMap<String, EvidenceLookupQueryAdmissionSupport>,
    counters: &mut EvidenceLookupPlanSelectionCounters,
) -> Result<EvidenceLookupSelectedPlanRow, EvidenceLookupPlanSelectionError> {
    counters.count_selected_family_membership_probe();
    if !selected_family_identities.contains(family.identity().as_str()) {
        counters.count_unaffected_family();
        return Ok(unaffected_row(family, admitted));
    }

    counters.count_selected_spatial_region();
    counters.count_selected_stage_receipt();
    let topology = selected_topology_posture(family, topology_support, counters)?;
    let query = selected_query_posture(family, query_support, counters)?;
    if family.query_posture().requires_query_evidence() {
        counters.count_required_query_posture_row();
    }
    if query.is_missing_required_query_posture() {
        return Ok(selected_row(
            family,
            admitted,
            topology,
            query,
            None,
            EvidenceLookupPlanRowOutcome::RequiredQueryPosture,
        ));
    }
    if topology.is_missing_required_topology_posture() {
        counters.count_denied_family();
        return Ok(selected_row(
            family,
            admitted,
            topology,
            query,
            None,
            EvidenceLookupPlanRowOutcome::Denied,
        ));
    }

    let strategy = EvidenceLookupSelectedStrategy::from_index_posture(family.index_posture())
        .map_err(|error| error.with_counters(*counters))?;
    match family.index_posture().kind() {
        EvidenceLookupFamilyIndexPostureKind::SparseLookupPlanRequired => {
            counters.count_sparse_lookup_plan();
        }
        EvidenceLookupFamilyIndexPostureKind::BoundedDenseLookupPlanRequired => {
            counters.count_bounded_dense_lookup_plan();
        }
        EvidenceLookupFamilyIndexPostureKind::IndexNotRequiredForDeclarationOnly => {}
    }
    counters.count_selected_family();
    Ok(selected_row(
        family,
        admitted,
        topology,
        query,
        Some(strategy),
        EvidenceLookupPlanRowOutcome::Selected,
    ))
}

fn selected_topology_posture(
    family: &EvidenceLookupFamilyDeclaration,
    topology_support: &mut BTreeMap<String, EvidenceLookupTopologyAdmissionSupport>,
    counters: &mut EvidenceLookupPlanSelectionCounters,
) -> Result<EvidenceLookupPlanTopologyPosture, EvidenceLookupPlanSelectionError> {
    let Some(support) = topology_support.remove(family.identity().as_str()) else {
        return Err(EvidenceLookupPlanSelectionError::new(
            EvidenceLookupPlanSelectionErrorKind::MissingTopologyPlanPosture,
            family.identity().as_str(),
        )
        .with_counters(*counters));
    };
    counters.count_topology_support_row_consumed();
    let posture = EvidenceLookupPlanTopologyPosture::from_support(
        Some(&support),
        family.topology_input_posture().required_family_identity(),
    );
    if matches!(
        posture.state(),
        super::topology_posture::EvidenceLookupPlanTopologyPostureState::Satisfied { .. }
    ) {
        counters.count_topology_receipt_ref_consumed();
    }
    Ok(posture)
}

fn selected_query_posture(
    family: &EvidenceLookupFamilyDeclaration,
    query_support: &mut BTreeMap<String, EvidenceLookupQueryAdmissionSupport>,
    counters: &mut EvidenceLookupPlanSelectionCounters,
) -> Result<EvidenceLookupPlanQueryPosture, EvidenceLookupPlanSelectionError> {
    let Some(support) = query_support.remove(family.identity().as_str()) else {
        return Err(EvidenceLookupPlanSelectionError::new(
            EvidenceLookupPlanSelectionErrorKind::MissingAdmittedSupportFamily,
            family.identity().as_str(),
        )
        .with_counters(*counters));
    };
    counters.count_query_support_row_consumed();
    Ok(EvidenceLookupPlanQueryPosture::from_family_and_admission(
        family.query_posture(),
        Some(&support),
    ))
}

fn unaffected_row(
    family: &EvidenceLookupFamilyDeclaration,
    admitted: &EvidenceLookupAdmittedInput,
) -> EvidenceLookupSelectedPlanRow {
    selected_row(
        family,
        admitted,
        EvidenceLookupPlanTopologyPosture::not_evaluated_for_unaffected_family(),
        EvidenceLookupPlanQueryPosture::not_evaluated_for_unaffected_family(),
        None,
        EvidenceLookupPlanRowOutcome::Unaffected,
    )
}

fn selected_family_identity_set(family_identities: &[String]) -> BTreeSet<String> {
    family_identities.iter().cloned().collect()
}

fn topology_support_by_family(
    support_rows: &[EvidenceLookupTopologyAdmissionSupport],
    counters: EvidenceLookupPlanSelectionCounters,
) -> Result<
    BTreeMap<String, EvidenceLookupTopologyAdmissionSupport>,
    EvidenceLookupPlanSelectionError,
> {
    let mut support_by_family = BTreeMap::new();
    for support in support_rows {
        if support_by_family
            .insert(support.family_identity().to_string(), support.clone())
            .is_some()
        {
            return Err(EvidenceLookupPlanSelectionError::new(
                EvidenceLookupPlanSelectionErrorKind::DuplicateAdmittedSupportFamily,
                support.family_identity(),
            )
            .with_counters(counters));
        }
    }
    Ok(support_by_family)
}

fn query_support_by_family(
    support_rows: &[EvidenceLookupQueryAdmissionSupport],
    counters: EvidenceLookupPlanSelectionCounters,
) -> Result<BTreeMap<String, EvidenceLookupQueryAdmissionSupport>, EvidenceLookupPlanSelectionError>
{
    let mut support_by_family = BTreeMap::new();
    for support in support_rows {
        if support_by_family
            .insert(support.family_identity().to_string(), support.clone())
            .is_some()
        {
            return Err(EvidenceLookupPlanSelectionError::new(
                EvidenceLookupPlanSelectionErrorKind::DuplicateAdmittedSupportFamily,
                support.family_identity(),
            )
            .with_counters(counters));
        }
    }
    Ok(support_by_family)
}

fn selected_row(
    family: &EvidenceLookupFamilyDeclaration,
    admitted: &EvidenceLookupAdmittedInput,
    topology_posture: EvidenceLookupPlanTopologyPosture,
    query_posture: EvidenceLookupPlanQueryPosture,
    strategy: Option<EvidenceLookupSelectedStrategy>,
    outcome: EvidenceLookupPlanRowOutcome,
) -> EvidenceLookupSelectedPlanRow {
    EvidenceLookupSelectedPlanRow::from_parts(EvidenceLookupSelectedPlanRowParts {
        family: family.clone(),
        spatial_touch_digest: admitted.spatial_touch_digest().to_string(),
        stage_receipt_digest: admitted.stage_receipt_digest().to_string(),
        topology_posture,
        query_posture,
        strategy,
        outcome,
    })
}
