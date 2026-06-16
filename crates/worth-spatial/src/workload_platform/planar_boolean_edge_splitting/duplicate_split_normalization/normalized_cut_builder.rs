use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntry;

use super::denial::{
    PlanarBooleanDuplicateSplitNormalizationDenial,
    PlanarBooleanDuplicateSplitNormalizationDenialKind,
};
use super::duplicate_key::duplicate_cut_kind_rank;
use super::identity::{duplicate_report_identity, normalized_cut_identity};
use super::normalized_cut::{
    PlanarBooleanNormalizedEndpointAuthority, PlanarBooleanNormalizedSplitCut,
};

pub(super) fn normalized_cut_from_duplicate_point_entries(
    entries: &[&PlanarBooleanRawEdgeSplitScheduleEntry],
) -> Result<PlanarBooleanNormalizedSplitCut, PlanarBooleanDuplicateSplitNormalizationDenial> {
    reject_contradictory_duplicate_group(entries)?;
    let first = entries[0];
    let provenance_entry_identities = canonical_multiset_values(
        entries
            .iter()
            .map(|entry| entry.entry_identity().to_string())
            .collect(),
    );
    let event_identities = canonical_values(
        entries
            .iter()
            .map(|entry| entry.event_identity().to_string())
            .collect(),
    );
    let parameter_fact_identities = canonical_values(
        entries
            .iter()
            .filter_map(|entry| entry.parameter_fact_identity().map(str::to_string))
            .collect(),
    );
    let event_group_identities = canonical_values(
        entries
            .iter()
            .flat_map(|entry| entry.event_group_identities().iter().cloned())
            .collect(),
    );
    let segment_pair_identities = canonical_values(
        entries
            .iter()
            .flat_map(|entry| entry.segment_pair_identities().iter().cloned())
            .collect(),
    );
    let predicate_receipt_identities = canonical_values(
        entries
            .iter()
            .flat_map(|entry| entry.predicate_receipt_identities().iter().cloned())
            .collect(),
    );
    let endpoint_authority = normalized_endpoint_authority(entries);
    let parameter_bits = canonical_parameter_bits(first.parameter());
    let cut_identity = normalized_cut_identity(
        first.source_edge_identity(),
        first.carrier_identity(),
        parameter_bits,
        duplicate_cut_kind_rank(first.kind()),
        first.local_frame_identity(),
        first.precision_basis_identity(),
        &provenance_entry_identities,
        &parameter_fact_identities,
        &event_group_identities,
        endpoint_authority.exact_endpoint_source_identity.as_deref(),
        endpoint_authority
            .exact_projected_endpoint_fact_identity
            .as_deref(),
        &endpoint_authority.shared_endpoint_source_identities,
        &endpoint_authority.shared_endpoint_projection_fact_digests,
    );
    let duplicate_report_identity =
        duplicate_report_identity(&cut_identity, &provenance_entry_identities);
    Ok(PlanarBooleanNormalizedSplitCut::new(
        cut_identity,
        duplicate_report_identity,
        first.source_edge_identity().to_string(),
        first.carrier_identity().to_string(),
        first.parameter(),
        parameter_bits,
        first.kind(),
        first.local_frame_identity().to_string(),
        first.precision_basis_identity().to_string(),
        provenance_entry_identities,
        event_identities,
        parameter_fact_identities,
        event_group_identities,
        segment_pair_identities,
        predicate_receipt_identities,
        endpoint_authority,
    ))
}

fn reject_contradictory_duplicate_group(
    entries: &[&PlanarBooleanRawEdgeSplitScheduleEntry],
) -> Result<(), PlanarBooleanDuplicateSplitNormalizationDenial> {
    let Some(first) = entries.first() else {
        return Err(PlanarBooleanDuplicateSplitNormalizationDenial::new(
            PlanarBooleanDuplicateSplitNormalizationDenialKind::ContradictoryDuplicateSplitPoint,
            "empty-duplicate-split-group",
            "duplicate split normalization requires non-empty duplicate groups",
        ));
    };
    if entries.iter().all(|entry| {
        entry.kind() == first.kind()
            && entry.carrier_identity() == first.carrier_identity()
            && entry.local_frame_identity() == first.local_frame_identity()
            && entry.precision_basis_identity() == first.precision_basis_identity()
    }) {
        return Ok(());
    }
    Err(PlanarBooleanDuplicateSplitNormalizationDenial::new(
        PlanarBooleanDuplicateSplitNormalizationDenialKind::ContradictoryDuplicateSplitPoint,
        first.entry_identity(),
        "duplicate split points with contradictory posture, carrier, frame, or precision basis must deny",
    ))
}

fn canonical_values(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn canonical_multiset_values(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}

fn normalized_endpoint_authority(
    entries: &[&PlanarBooleanRawEdgeSplitScheduleEntry],
) -> PlanarBooleanNormalizedEndpointAuthority {
    let exact_endpoint_source_identities = canonical_values(
        entries
            .iter()
            .filter_map(|entry| entry.exact_endpoint_source_identity().map(str::to_string))
            .collect(),
    );
    let exact_projected_endpoint_fact_identities = canonical_values(
        entries
            .iter()
            .filter_map(|entry| {
                entry
                    .exact_projected_endpoint_fact_identity()
                    .map(str::to_string)
            })
            .collect(),
    );
    PlanarBooleanNormalizedEndpointAuthority {
        exact_endpoint_source_identity: single_authority_value(exact_endpoint_source_identities),
        exact_projected_endpoint_fact_identity: single_authority_value(
            exact_projected_endpoint_fact_identities,
        ),
        shared_endpoint_source_identities: canonical_values(
            entries
                .iter()
                .flat_map(|entry| entry.shared_endpoint_source_identities().iter().cloned())
                .collect(),
        ),
        shared_endpoint_projection_fact_digests: canonical_values(
            entries
                .iter()
                .flat_map(|entry| {
                    entry
                        .shared_endpoint_projection_fact_digests()
                        .iter()
                        .cloned()
                })
                .collect(),
        ),
    }
}

fn single_authority_value(values: Vec<String>) -> Option<String> {
    if values.len() == 1 {
        values.into_iter().next()
    } else {
        None
    }
}
