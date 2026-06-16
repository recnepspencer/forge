use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::{
    PlanarBooleanSplitEdgeFragmentSchedule, PlanarBooleanSplitEdgeFragmentSet,
};

use super::construction::CounterBuild;
use super::coverage_row::PlanarBooleanSplitFragmentCoverageRow;
use super::denial::{
    PlanarBooleanSplitChainValidationDenial, PlanarBooleanSplitChainValidationDenialKind as Kind,
};
use super::identity::fragment_coverage_row_identity;

pub(super) fn validate_fragment_domains(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    counters: &mut CounterBuild,
) -> Result<Vec<PlanarBooleanSplitFragmentCoverageRow>, PlanarBooleanSplitChainValidationDenial> {
    let mut rows = Vec::with_capacity(fragments.schedules().len());
    let mut globally_seen_fragments = BTreeSet::new();
    for schedule in fragments.schedules() {
        counters.fragment_schedules_checked += 1;
        counters.source_edges_checked += 1;
        let row = validate_schedule(
            fragments.fragment_set_identity(),
            schedule,
            counters,
            &mut globally_seen_fragments,
        )?;
        rows.push(row);
    }
    Ok(rows)
}

fn validate_schedule<'a>(
    fragment_set_identity: &str,
    schedule: &'a PlanarBooleanSplitEdgeFragmentSchedule,
    counters: &mut CounterBuild,
    globally_seen_fragments: &mut BTreeSet<&'a str>,
) -> Result<PlanarBooleanSplitFragmentCoverageRow, PlanarBooleanSplitChainValidationDenial> {
    let Some(first) = schedule.fragments().first() else {
        return Err(counters.deny(
            Kind::EmptyFragmentSchedule,
            schedule.schedule_identity(),
            "split chain validation requires at least one fragment per source edge",
        ));
    };
    let first_range = first.parameter_range();
    if !first_range[0].is_finite()
        || !first_range[1].is_finite()
        || canonical_parameter_bits(first_range[0]) >= canonical_parameter_bits(first_range[1])
    {
        return Err(counters.deny(
            Kind::MalformedFragmentRange,
            first.fragment_identity(),
            "split fragments must have finite increasing parameter ranges",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut prior_end_bits = canonical_parameter_bits(0.0);
    if canonical_parameter_bits(first.parameter_range()[0]) != prior_end_bits {
        counters.gaps_rejected += 1;
        return Err(counters.deny(
            Kind::FragmentGap,
            first.fragment_identity(),
            "first fragment must begin at source parameter zero",
        ));
    }
    for fragment in schedule.fragments() {
        counters.fragments_checked += 1;
        if !seen.insert(fragment.fragment_identity()) {
            counters.dangling_references_rejected += 1;
            return Err(counters.deny(
                Kind::DuplicateFragmentIdentity,
                fragment.fragment_identity(),
                "fragment identities must be unique inside a split fragment schedule",
            ));
        }
        if !globally_seen_fragments.insert(fragment.fragment_identity()) {
            counters.dangling_references_rejected += 1;
            return Err(counters.deny(
                Kind::DuplicateFragmentIdentity,
                fragment.fragment_identity(),
                "fragment identities must be unique across split-chain validation",
            ));
        }
        if fragment.source_edge_identity() != schedule.source_edge_identity()
            || fragment.carrier_identity() != schedule.carrier_identity()
        {
            counters.dangling_references_rejected += 1;
            return Err(counters.deny(
                Kind::DanglingOverlapFragmentReference,
                fragment.fragment_identity(),
                "fragment authority must match its source-edge schedule",
            ));
        }
        let range = fragment.parameter_range();
        if !range[0].is_finite()
            || !range[1].is_finite()
            || canonical_parameter_bits(range[0]) >= canonical_parameter_bits(range[1])
        {
            return Err(counters.deny(
                Kind::MalformedFragmentRange,
                fragment.fragment_identity(),
                "split fragments must have finite increasing parameter ranges",
            ));
        }
        let start_bits = canonical_parameter_bits(range[0]);
        if start_bits > prior_end_bits {
            counters.gaps_rejected += 1;
            return Err(counters.deny(
                Kind::FragmentGap,
                fragment.fragment_identity(),
                "adjacent split fragments must not leave source parameter gaps",
            ));
        }
        if start_bits < prior_end_bits {
            counters.overlaps_rejected += 1;
            return Err(counters.deny(
                Kind::FragmentOverlap,
                fragment.fragment_identity(),
                "adjacent split fragments must not overlap in source parameter space",
            ));
        }
        prior_end_bits = canonical_parameter_bits(range[1]);
    }
    if prior_end_bits != canonical_parameter_bits(1.0) {
        counters.gaps_rejected += 1;
        return Err(counters.deny(
            Kind::FragmentGap,
            schedule.schedule_identity(),
            "last fragment must end at source parameter one",
        ));
    }
    Ok(PlanarBooleanSplitFragmentCoverageRow::new(
        fragment_coverage_row_identity(
            fragment_set_identity,
            schedule.schedule_identity(),
            schedule.source_edge_identity(),
            schedule.carrier_identity(),
        ),
        schedule.schedule_identity().to_string(),
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        schedule.fragments().len(),
        canonical_parameter_bits(0.0),
        canonical_parameter_bits(1.0),
    ))
}
