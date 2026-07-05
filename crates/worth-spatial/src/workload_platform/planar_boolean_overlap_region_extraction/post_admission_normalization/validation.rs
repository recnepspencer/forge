use super::counters::PlanarBooleanPostAdmissionNormalizationCounters;
use super::denial::{
    PlanarBooleanPostAdmissionNormalizationDenial, PlanarBooleanPostAdmissionNormalizationDenialKind,
};
use super::rows::PlanarBooleanOverlapRegionCanonicalWindingSourceKind;
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanAdmittedOverlapRegionRow, PlanarBooleanBoundaryOnlyOverlapOutcomeRow,
    PlanarBooleanPostAdmissionNormalizationInput,
};

pub(super) fn validate_input_identities(
    input: PlanarBooleanPostAdmissionNormalizationInput<'_>,
    counters: &PlanarBooleanPostAdmissionNormalizationCounters,
) -> Result<(), PlanarBooleanPostAdmissionNormalizationDenial> {
    let admitted = input.region_candidate_boundary().admitted_overlap_regions();
    let boundary_only = input.region_candidate_boundary().boundary_only_overlap_outcomes();
    if admitted.request_identity() != boundary_only.request_identity()
        || admitted.arrangement_graph_identity() != boundary_only.arrangement_graph_identity()
        || admitted.cell_set_identity() != boundary_only.cell_set_identity()
        || admitted.ordering_basis_identity() != boundary_only.ordering_basis_identity()
    {
        return Err(PlanarBooleanPostAdmissionNormalizationDenial::new(
            PlanarBooleanPostAdmissionNormalizationDenialKind::InputIdentityMismatchDenied,
            admitted.request_identity(),
            *counters,
            "post-admission normalization requires admitted-region and boundary-only products from the same phase-eleven proof chain",
        ));
    }
    Ok(())
}

pub(super) fn canonicalized_strings(values: &[String]) -> Vec<String> {
    let mut values = values.to_vec();
    values.sort();
    values.dedup();
    values
}

pub(super) fn ordered_witness_strings(values: &[String]) -> Option<Vec<String>> {
    if values.is_empty() {
        return None;
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut ordered = Vec::with_capacity(values.len());
    for value in values {
        if !seen.insert(value.as_str()) {
            return None;
        }
        ordered.push(value.clone());
    }
    Some(ordered)
}

pub(super) fn admitted_witness_key(row: &PlanarBooleanAdmittedOverlapRegionRow) -> String {
    format!(
        "{}:{}:{}:{}:{}",
        row.island_identity(),
        row.neighborhood_identity(),
        row.area_overlap_component_identity(),
        row.canonical_boundary_segment_witness().join("|"),
        row.canonical_source_loop_witness().join("|"),
    )
}

pub(super) fn boundary_only_witness_key(row: &PlanarBooleanBoundaryOnlyOverlapOutcomeRow) -> String {
    format!(
        "{}:{}:{}:{}",
        row.island_identity(),
        row.neighborhood_identity(),
        row.canonical_boundary_segment_witness().join("|"),
        row.canonical_source_loop_witness().join("|"),
    )
}

pub(super) fn boundary_witness_mismatch_denial(
    source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
    rejected_identity: &str,
    counters: &PlanarBooleanPostAdmissionNormalizationCounters,
) -> PlanarBooleanPostAdmissionNormalizationDenial {
    PlanarBooleanPostAdmissionNormalizationDenial::new(
        PlanarBooleanPostAdmissionNormalizationDenialKind::BoundaryWitnessMismatchDenied,
        rejected_identity,
        *counters,
        match source_kind {
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion => {
                "post-admission canonical winding requires each admitted region to carry one coherent ordered boundary-segment witness, ordered source-loop witness, and non-empty carried source-edge proof"
            }
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome => {
                "post-admission boundary normalization requires each boundary-only outcome to carry one coherent ordered boundary-segment witness and ordered source-loop witness"
            }
        },
    )
}
