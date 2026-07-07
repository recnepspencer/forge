use super::counters::PlanarBooleanOverlapRegionLedgerAssemblyCounters;
use super::denial::PlanarBooleanOverlapRegionLedgerAssemblyDenial;
use super::identity::{
    bundle_identity, decision_log_identity, decision_row_identity, ledger_identity,
    ledger_row_identity, receipt_identity,
};
use super::input::PlanarBooleanOverlapRegionLedgerAssemblyInput;
use super::product::{
    PlanarBooleanOverlapRegionDecisionLog, PlanarBooleanOverlapRegionLedger,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapRegionLedgerReceipt,
};
use super::rows::{
    PlanarBooleanOverlapRegionDecisionKind, PlanarBooleanOverlapRegionDecisionLogRow,
    PlanarBooleanOverlapRegionLedgerRow,
};
use super::validation::{
    canonical_rows_by_identity, persistent_names_by_region, require_canonical_row,
    require_signature_row, signature_rows_by_region, validate_identity_matches_canonical,
    validate_input_identities, validate_ledger_rows, validate_source_truth,
    validate_supporting_rows,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCanonicalWindingRow, PlanarBooleanOverlapRegionIdentityRow,
    PlanarBooleanOverlapRegionPersistentNamePropagationRow,
    PlanarBooleanOverlapRegionSubshapeSignatureRow,
};

pub(super) fn assemble_ledger_bundle(
    input: PlanarBooleanOverlapRegionLedgerAssemblyInput<'_>,
) -> Result<
    PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    PlanarBooleanOverlapRegionLedgerAssemblyDenial,
> {
    let mut counters = PlanarBooleanOverlapRegionLedgerAssemblyCounters::default();
    validate_input_identities(input, &counters)?;

    let bundle = input.identity_lineage();
    let canonical_bundle = bundle.source_post_admission_normalization();
    let canonical = canonical_bundle.overlap_region_canonical_winding();
    let boundary = canonical_bundle.source_region_candidate_boundary();
    let admitted = boundary.admitted_overlap_regions();
    let boundary_only = boundary.boundary_only_overlap_outcomes();
    let request_identity = canonical.request_identity().to_string();

    let identity_rows = sorted_identity_rows(bundle.overlap_region_identity_map().rows());
    let persistent_name_rows =
        sorted_persistent_name_rows(bundle.persistent_name_propagation_map().rows());
    let signature_rows = sorted_signature_rows(bundle.subshape_signature_map().rows());

    let persistent_name_rows_owned = persistent_name_rows
        .iter()
        .map(|row| (*row).clone())
        .collect::<Vec<_>>();
    let signature_rows_owned = signature_rows
        .iter()
        .map(|row| (*row).clone())
        .collect::<Vec<_>>();
    let identity_rows_owned = identity_rows
        .iter()
        .map(|row| (*row).clone())
        .collect::<Vec<_>>();

    validate_supporting_rows(
        &persistent_name_rows_owned,
        &signature_rows_owned,
        &identity_rows_owned,
        &mut counters,
    )?;

    let canonical_rows = canonical_rows_by_identity(canonical.rows());
    let names_by_region = persistent_names_by_region(&persistent_name_rows_owned);
    let signatures_by_region = signature_rows_by_region(&signature_rows_owned);

    let mut decision_rows = Vec::new();
    decision_rows.push(build_request_decision_row(&request_identity, canonical));
    counters.admitted_decision_rows(1);

    let mut ledger_rows = Vec::new();
    for identity_row in identity_rows {
        counters.examined_identity_row();
        let canonical_row = require_canonical_row(identity_row, &canonical_rows, &mut counters)?;
        validate_identity_matches_canonical(identity_row, canonical_row, &mut counters)?;
        validate_source_truth(identity_row, admitted, boundary_only, &mut counters)?;
        let signature_row = require_signature_row(
            identity_row.region_identity(),
            &signatures_by_region,
            &mut counters,
        )?;
        let propagated_names = names_by_region
            .get(identity_row.region_identity())
            .cloned()
            .unwrap_or_default();

        let row_decisions = build_decision_rows(
            &request_identity,
            identity_row,
            canonical_row,
            &propagated_names,
            signature_row,
        );
        counters.admitted_decision_rows(row_decisions.len());
        decision_rows.extend(row_decisions);

        ledger_rows.push(build_ledger_row(
            identity_row,
            signature_row,
            &propagated_names,
        ));
        counters.admitted_ledger_row();
    }

    decision_rows.sort_by(|left, right| left.decision_identity().cmp(right.decision_identity()));
    ledger_rows.sort_by(|left, right| left.ledger_row_identity().cmp(right.ledger_row_identity()));
    validate_ledger_rows(&ledger_rows, &mut counters)?;

    let decision_row_identities = decision_rows
        .iter()
        .map(|row| row.decision_identity().to_string())
        .collect::<Vec<_>>();
    let ledger_row_identities = ledger_rows
        .iter()
        .map(|row| row.ledger_row_identity().to_string())
        .collect::<Vec<_>>();
    let decision_identity = decision_log_identity(&request_identity, &decision_row_identities);
    let ledger_id = ledger_identity(&request_identity, &ledger_row_identities);
    let receipt_id = receipt_identity(&request_identity, &decision_identity, &ledger_id);

    Ok(PlanarBooleanOverlapRegionLedgerAssemblyBundle::new(
        bundle_identity(
            &request_identity,
            &decision_identity,
            &ledger_id,
            &receipt_id,
        ),
        PlanarBooleanOverlapRegionDecisionLog::new(
            decision_identity.clone(),
            request_identity.clone(),
            decision_rows,
        ),
        PlanarBooleanOverlapRegionLedger::new(
            ledger_id.clone(),
            request_identity.clone(),
            canonical.arrangement_graph_identity().to_string(),
            canonical.cell_set_identity().to_string(),
            canonical.ordering_basis_identity().to_string(),
            ledger_rows,
        ),
        PlanarBooleanOverlapRegionLedgerReceipt::new(
            receipt_id,
            request_identity,
            decision_identity,
            ledger_id,
            bundle
                .overlap_region_identity_map()
                .map_identity()
                .to_string(),
            bundle
                .persistent_name_propagation_map()
                .map_identity()
                .to_string(),
            bundle.subshape_signature_map().map_identity().to_string(),
        ),
        counters,
    ))
}

fn sorted_identity_rows(
    rows: &[PlanarBooleanOverlapRegionIdentityRow],
) -> Vec<&PlanarBooleanOverlapRegionIdentityRow> {
    let mut sorted = rows.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| left.region_identity().cmp(right.region_identity()));
    sorted
}

fn sorted_persistent_name_rows(
    rows: &[PlanarBooleanOverlapRegionPersistentNamePropagationRow],
) -> Vec<&PlanarBooleanOverlapRegionPersistentNamePropagationRow> {
    let mut sorted = rows.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| {
        left.propagation_identity()
            .cmp(right.propagation_identity())
    });
    sorted
}

fn sorted_signature_rows(
    rows: &[PlanarBooleanOverlapRegionSubshapeSignatureRow],
) -> Vec<&PlanarBooleanOverlapRegionSubshapeSignatureRow> {
    let mut sorted = rows.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| left.signature_identity().cmp(right.signature_identity()));
    sorted
}

fn build_request_decision_row(
    request_identity: &str,
    canonical: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingSet,
) -> PlanarBooleanOverlapRegionDecisionLogRow {
    let related_identities = vec![
        canonical.arrangement_graph_identity().to_string(),
        canonical.cell_set_identity().to_string(),
        canonical.ordering_basis_identity().to_string(),
    ];
    PlanarBooleanOverlapRegionDecisionLogRow::new(
        decision_row_identity(
            request_identity,
            PlanarBooleanOverlapRegionDecisionKind::Request,
            request_identity,
            &related_identities,
        ),
        PlanarBooleanOverlapRegionDecisionKind::Request,
        request_identity.to_string(),
        related_identities,
    )
}

fn build_ledger_row(
    identity_row: &PlanarBooleanOverlapRegionIdentityRow,
    signature_row: &PlanarBooleanOverlapRegionSubshapeSignatureRow,
    propagated_names: &[&PlanarBooleanOverlapRegionPersistentNamePropagationRow],
) -> PlanarBooleanOverlapRegionLedgerRow {
    let persistent_name_identities = propagated_names
        .iter()
        .map(|row| row.persistent_name_identity().to_string())
        .collect::<Vec<_>>();

    PlanarBooleanOverlapRegionLedgerRow::new(
        ledger_row_identity(
            identity_row.region_identity(),
            identity_row.canonical_winding_identity(),
            signature_row.signature_identity(),
        ),
        identity_row.region_identity().to_string(),
        identity_row.canonical_winding_identity().to_string(),
        identity_row.source_kind(),
        identity_row.source_identity().to_string(),
        identity_row
            .area_overlap_component_identity()
            .map(str::to_string),
        signature_row.correspondence_only(),
        persistent_name_identities,
        signature_row.signature_identity().to_string(),
        identity_row.lineage_identities().to_vec(),
        identity_row
            .canonical_boundary_segment_identities()
            .to_vec(),
        identity_row.canonical_source_loop_identities().to_vec(),
    )
}

fn build_decision_rows(
    request_identity: &str,
    identity_row: &PlanarBooleanOverlapRegionIdentityRow,
    canonical_row: &PlanarBooleanOverlapRegionCanonicalWindingRow,
    propagated_names: &[&PlanarBooleanOverlapRegionPersistentNamePropagationRow],
    signature_row: &PlanarBooleanOverlapRegionSubshapeSignatureRow,
) -> Vec<PlanarBooleanOverlapRegionDecisionLogRow> {
    let mut rows = Vec::new();
    rows.push(decision_row(
        request_identity,
        PlanarBooleanOverlapRegionDecisionKind::Participation,
        identity_row.region_identity(),
        identity_row.lineage_identities().to_vec(),
    ));
    rows.push(decision_row(
        request_identity,
        PlanarBooleanOverlapRegionDecisionKind::Adjacency,
        identity_row.neighborhood_identity(),
        vec![identity_row.island_identity().to_string()],
    ));
    rows.push(decision_row(
        request_identity,
        PlanarBooleanOverlapRegionDecisionKind::Arrangement,
        identity_row.canonical_winding_identity(),
        vec![
            identity_row.island_identity().to_string(),
            identity_row.neighborhood_identity().to_string(),
        ],
    ));
    rows.push(decision_row(
        request_identity,
        PlanarBooleanOverlapRegionDecisionKind::Contact,
        identity_row.canonical_winding_identity(),
        identity_row.source_edge_identities().to_vec(),
    ));
    rows.push(decision_row(
        request_identity,
        PlanarBooleanOverlapRegionDecisionKind::Winding,
        identity_row.canonical_winding_identity(),
        vec![
            format!("{:?}", identity_row.source_kind()),
            identity_row.source_identity().to_string(),
            format!("{:?}", identity_row.canonical_operand_side()),
            format!("{:?}", identity_row.canonical_winding_sign()),
        ],
    ));
    rows.push(decision_row(
        request_identity,
        PlanarBooleanOverlapRegionDecisionKind::Identity,
        identity_row.region_identity(),
        vec![identity_row.canonical_winding_identity().to_string()],
    ));

    if let Some(area_overlap_component_identity) = identity_row.area_overlap_component_identity() {
        rows.push(decision_row(
            request_identity,
            PlanarBooleanOverlapRegionDecisionKind::Area,
            area_overlap_component_identity,
            vec![identity_row.region_identity().to_string()],
        ));
    } else {
        rows.push(decision_row(
            request_identity,
            PlanarBooleanOverlapRegionDecisionKind::BoundaryOnly,
            identity_row.source_identity(),
            canonical_row.canonical_source_loop_identities().to_vec(),
        ));
    }

    for propagated_name in propagated_names {
        rows.push(decision_row(
            request_identity,
            PlanarBooleanOverlapRegionDecisionKind::PersistentNamePropagation,
            propagated_name.persistent_name_identity(),
            vec![propagated_name.region_identity().to_string()],
        ));
    }

    rows.push(decision_row(
        request_identity,
        PlanarBooleanOverlapRegionDecisionKind::SubshapeSignature,
        signature_row.signature_identity(),
        vec![
            signature_row.region_identity().to_string(),
            signature_row.signature_basis_identity().to_string(),
        ],
    ));
    rows
}

fn decision_row(
    request_identity: &str,
    kind: PlanarBooleanOverlapRegionDecisionKind,
    focal_identity: &str,
    related_identities: Vec<String>,
) -> PlanarBooleanOverlapRegionDecisionLogRow {
    PlanarBooleanOverlapRegionDecisionLogRow::new(
        decision_row_identity(request_identity, kind, focal_identity, &related_identities),
        kind,
        focal_identity.to_string(),
        related_identities,
    )
}
