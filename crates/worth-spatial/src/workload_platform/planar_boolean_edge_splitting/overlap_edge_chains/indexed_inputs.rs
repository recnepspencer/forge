use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::{
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanNormalizedIntervalSubdivisionRow,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentSet,
};

use super::denial::{PlanarBooleanOverlapEdgeChainDenial, PlanarBooleanOverlapEdgeChainDenialKind};

pub(super) struct OverlapChainIndexedInputs<'a> {
    fragment_rows_inspected: usize,
    fragments_by_subdivision: BTreeMap<&'a str, Vec<&'a PlanarBooleanSplitEdgeFragment>>,
}

impl<'a> OverlapChainIndexedInputs<'a> {
    pub(super) fn new(
        schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    ) -> Result<Self, PlanarBooleanOverlapEdgeChainDenial> {
        let mut subdivisions_by_identity = BTreeMap::new();
        for schedule in schedules.schedules() {
            for subdivision in schedule.interval_subdivisions() {
                subdivisions_by_identity.insert(subdivision.subdivision_identity(), subdivision);
            }
        }
        let mut fragments_by_subdivision =
            BTreeMap::<&str, Vec<&PlanarBooleanSplitEdgeFragment>>::new();
        let mut fragment_rows_inspected = 0;
        for fragment in fragments.fragments() {
            fragment_rows_inspected += 1;
            for subdivision_identity in fragment.interval_subdivision_identities() {
                if !subdivisions_by_identity.contains_key(subdivision_identity.as_str()) {
                    return Err(PlanarBooleanOverlapEdgeChainDenial::new(
                        PlanarBooleanOverlapEdgeChainDenialKind::MissingSubdivisionReference,
                        fragment.fragment_identity(),
                        "overlap chain construction requires fragment interval references to belong to the normalized interval schedule",
                    ));
                }
                let subdivision = subdivisions_by_identity[subdivision_identity.as_str()];
                reject_mismatched_fragment_authority(fragment, subdivision)?;
                fragments_by_subdivision
                    .entry(subdivision_identity.as_str())
                    .or_default()
                    .push(fragment);
            }
        }
        canonicalize_fragment_members(&mut fragments_by_subdivision);
        Ok(Self {
            fragment_rows_inspected,
            fragments_by_subdivision,
        })
    }

    pub(super) fn fragment_rows_inspected(&self) -> usize {
        self.fragment_rows_inspected
    }

    pub(super) fn fragments_for_subdivision(
        &self,
        subdivision: &PlanarBooleanNormalizedIntervalSubdivisionRow,
    ) -> Result<&[&'a PlanarBooleanSplitEdgeFragment], PlanarBooleanOverlapEdgeChainDenial> {
        self.fragments_by_subdivision
            .get(subdivision.subdivision_identity())
            .map(Vec::as_slice)
            .filter(|fragments| !fragments.is_empty())
            .ok_or_else(|| {
                PlanarBooleanOverlapEdgeChainDenial::new(
                    PlanarBooleanOverlapEdgeChainDenialKind::MissingFragmentReference,
                    subdivision.subdivision_identity(),
                    "overlap chain construction requires every interval subdivision to be covered by split fragments",
                )
            })
    }
}

fn reject_mismatched_fragment_authority(
    fragment: &PlanarBooleanSplitEdgeFragment,
    subdivision: &PlanarBooleanNormalizedIntervalSubdivisionRow,
) -> Result<(), PlanarBooleanOverlapEdgeChainDenial> {
    if fragment.source_edge_identity() == subdivision.source_edge_identity()
        && fragment.carrier_identity() == subdivision.carrier_identity()
        && fragment.local_frame_identity() == subdivision.local_frame_identity()
        && fragment.precision_basis_identity() == subdivision.precision_basis_identity()
        && fragment
            .normalized_interval_identities()
            .iter()
            .any(|identity| identity == subdivision.normalized_interval_identity())
        && fragment_overlaps_subdivision(fragment, subdivision)
    {
        return Ok(());
    }
    Err(PlanarBooleanOverlapEdgeChainDenial::new(
        PlanarBooleanOverlapEdgeChainDenialKind::MismatchedFragmentAuthority,
        fragment.fragment_identity(),
        "overlap chain construction requires fragment authority to match the referenced interval subdivision",
    ))
}

fn fragment_overlaps_subdivision(
    fragment: &PlanarBooleanSplitEdgeFragment,
    subdivision: &PlanarBooleanNormalizedIntervalSubdivisionRow,
) -> bool {
    let fragment_range = fragment.parameter_range();
    let subdivision_range = subdivision.admitted_parameter_range();
    fragment_range[0] < subdivision_range[1] && subdivision_range[0] < fragment_range[1]
}

fn canonicalize_fragment_members(
    fragments_by_subdivision: &mut BTreeMap<&str, Vec<&PlanarBooleanSplitEdgeFragment>>,
) {
    for fragments in fragments_by_subdivision.values_mut() {
        fragments.sort_by(|a, b| {
            a.source_edge_identity()
                .cmp(b.source_edge_identity())
                .then_with(|| a.carrier_identity().cmp(b.carrier_identity()))
                .then_with(|| a.parameter_range_bits().cmp(&b.parameter_range_bits()))
                .then_with(|| a.fragment_identity().cmp(b.fragment_identity()))
        });
        fragments.dedup_by(|a, b| a.fragment_identity() == b.fragment_identity());
    }
}
