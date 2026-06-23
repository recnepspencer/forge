use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::overlap_edge_chains::{
    PlanarBooleanOverlapEdgeChainMember, PlanarBooleanOverlapEdgeChainSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::PlanarBooleanSplitEdgeFragment;

use super::construction::CounterBuild;
use super::coverage_row::PlanarBooleanOverlapChainCoverageRow;
use super::denial::{
    PlanarBooleanSplitChainValidationDenial, PlanarBooleanSplitChainValidationDenialKind as Kind,
};
use super::identity::overlap_coverage_row_identity;
use super::indexed_inputs::SplitChainValidationIndexedInputs;

pub(super) fn validate_overlap_references(
    chains: &PlanarBooleanOverlapEdgeChainSet,
    indexed: &SplitChainValidationIndexedInputs<'_>,
    counters: &mut CounterBuild,
) -> Result<Vec<PlanarBooleanOverlapChainCoverageRow>, PlanarBooleanSplitChainValidationDenial> {
    let mut rows = Vec::new();
    for chain in chains.chains() {
        counters.overlap_chains_checked += 1;
        for member in chain.members() {
            counters.overlap_members_checked += 1;
            let fragment = indexed
                .fragment(member.fragment_identity())
                .ok_or_else(|| {
                    counters.dangling_references_rejected += 1;
                    counters.deny(
                        Kind::DanglingOverlapFragmentReference,
                        member.fragment_identity(),
                        "overlap chains may only reference minted split fragments",
                    )
                })?;
            reject_member_fragment_mismatch(member, fragment, counters)?;
            reject_member_interval_basis_malformed(member, counters)?;
            reject_member_outside_interval(member, counters)?;
        }
    }
    for (key, members) in indexed.overlap_groups() {
        let chain = indexed
            .chain(key.chain_identity)
            .expect("indexed overlap group must retain its chain metadata");
        reject_group_gaps_or_overlaps(
            source_span(members[0].source_parameter_range()),
            members,
            counters,
        )?;
        rows.push(PlanarBooleanOverlapChainCoverageRow::new(
            overlap_coverage_row_identity(
                chains.chain_set_identity(),
                chain.chain_identity(),
                key.source_interval_identity,
                key.source_edge_identity,
                key.carrier_identity,
            ),
            chain.chain_identity().to_string(),
            chain.interval_event_identity().to_string(),
            key.source_interval_identity.to_string(),
            key.source_edge_identity.to_string(),
            key.carrier_identity.to_string(),
            members.len(),
        ));
    }
    rows.sort_by(|a, b| a.row_identity().cmp(b.row_identity()));
    Ok(rows)
}

pub(super) fn reject_overlap_group_interval_basis_mismatch(
    members: &[&PlanarBooleanOverlapEdgeChainMember],
    counters: &mut CounterBuild,
) -> Result<(), PlanarBooleanSplitChainValidationDenial> {
    let Some(first) = members.first() else {
        return Ok(());
    };
    let expected_source_span_bits = range_bits(source_span(first.source_parameter_range()));
    let expected_normalized_span_bits = range_bits(source_span(first.normalized_parameter_range()));
    let expected_normalized_identity = first.normalized_interval_identity();
    for member in members.iter().skip(1) {
        if range_bits(source_span(member.source_parameter_range())) != expected_source_span_bits
            || range_bits(source_span(member.normalized_parameter_range()))
                != expected_normalized_span_bits
            || member.normalized_interval_identity() != expected_normalized_identity
        {
            counters.mismatched_interval_basis_rejected += 1;
            return Err(counters.deny(
                Kind::MismatchedOverlapIntervalBasis,
                member.member_identity(),
                "overlap chain members with the same source interval identity must share interval authority basis",
            ));
        }
    }
    Ok(())
}

fn reject_member_interval_basis_malformed(
    member: &PlanarBooleanOverlapEdgeChainMember,
    counters: &mut CounterBuild,
) -> Result<(), PlanarBooleanSplitChainValidationDenial> {
    let source = source_span(member.source_parameter_range());
    let normalized = source_span(member.normalized_parameter_range());
    if source[0].is_finite()
        && source[1].is_finite()
        && normalized[0].is_finite()
        && normalized[1].is_finite()
        && canonical_parameter_bits(source[0]) < canonical_parameter_bits(source[1])
        && canonical_parameter_bits(normalized[0]) < canonical_parameter_bits(normalized[1])
    {
        return Ok(());
    }
    counters.mismatched_interval_basis_rejected += 1;
    Err(counters.deny(
        Kind::MalformedOverlapIntervalBasis,
        member.member_identity(),
        "overlap chain source and normalized interval basis must be finite and non-collapsed",
    ))
}

fn reject_member_fragment_mismatch(
    member: &PlanarBooleanOverlapEdgeChainMember,
    fragment: &PlanarBooleanSplitEdgeFragment,
    counters: &mut CounterBuild,
) -> Result<(), PlanarBooleanSplitChainValidationDenial> {
    if member.source_edge_identity() == fragment.source_edge_identity()
        && member.carrier_identity() == fragment.carrier_identity()
        && member.local_frame_identity() == fragment.local_frame_identity()
        && member.precision_basis_identity() == fragment.precision_basis_identity()
        && range_bits(member.fragment_parameter_range()) == fragment.parameter_range_bits()
    {
        return Ok(());
    }
    counters.dangling_references_rejected += 1;
    Err(counters.deny(
        Kind::MismatchedOverlapFragmentAuthority,
        member.member_identity(),
        "overlap chain member authority must match the referenced split fragment",
    ))
}

fn reject_member_outside_interval(
    member: &PlanarBooleanOverlapEdgeChainMember,
    counters: &mut CounterBuild,
) -> Result<(), PlanarBooleanSplitChainValidationDenial> {
    let fragment = member.fragment_parameter_range();
    let source = source_span(member.source_parameter_range());
    if canonical_parameter_bits(fragment[0]) < canonical_parameter_bits(source[1])
        && canonical_parameter_bits(source[0]) < canonical_parameter_bits(fragment[1])
    {
        return Ok(());
    }
    counters.out_of_interval_references_rejected += 1;
    Err(counters.deny(
        Kind::OverlapFragmentOutsideSourceInterval,
        member.member_identity(),
        "overlap chain member fragment range must intersect its source interval for clipped coverage",
    ))
}

fn reject_group_gaps_or_overlaps(
    expected: [f64; 2],
    members: &[&PlanarBooleanOverlapEdgeChainMember],
    counters: &mut CounterBuild,
) -> Result<(), PlanarBooleanSplitChainValidationDenial> {
    reject_overlap_group_interval_basis_mismatch(members, counters)?;
    let mut prior_end = canonical_parameter_bits(expected[0]);
    for member in members {
        let range = clipped_member_range(member.fragment_parameter_range(), expected);
        let start = canonical_parameter_bits(range[0]);
        if start > prior_end {
            counters.gaps_rejected += 1;
            return Err(counters.deny(
                Kind::FragmentGap,
                member.member_identity(),
                "overlap chain members must cover their source interval without gaps",
            ));
        }
        if start < prior_end {
            counters.overlaps_rejected += 1;
            return Err(counters.deny(
                Kind::FragmentOverlap,
                member.member_identity(),
                "overlap chain members must cover their source interval without overlaps",
            ));
        }
        prior_end = canonical_parameter_bits(range[1]);
    }
    if prior_end != canonical_parameter_bits(expected[1]) {
        counters.gaps_rejected += 1;
        return Err(counters.deny(
            Kind::FragmentGap,
            members[0].member_identity(),
            "overlap chain coverage must reach the source interval end",
        ));
    }
    Ok(())
}

fn clipped_member_range(fragment: [f64; 2], expected: [f64; 2]) -> [f64; 2] {
    [fragment[0].max(expected[0]), fragment[1].min(expected[1])]
}

fn source_span(range: [f64; 2]) -> [f64; 2] {
    if canonical_parameter_bits(range[0]) <= canonical_parameter_bits(range[1]) {
        range
    } else {
        [range[1], range[0]]
    }
}

fn range_bits(range: [f64; 2]) -> [u64; 2] {
    [
        canonical_parameter_bits(range[0]),
        canonical_parameter_bits(range[1]),
    ]
}
