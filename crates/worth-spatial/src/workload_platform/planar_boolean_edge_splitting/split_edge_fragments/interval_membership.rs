use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::PlanarBooleanNormalizedIntervalSubdivisionRow;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct FragmentIntervalMembership {
    pub(super) interval_subdivision_identities: Vec<String>,
    pub(super) normalized_interval_identities: Vec<String>,
    pub(super) event_group_identities: Vec<String>,
    pub(super) provenance_identities: Vec<String>,
    pub(super) source_senses: Vec<PlanarBooleanSourceIntervalSense>,
}

impl FragmentIntervalMembership {
    pub(super) fn for_range(
        fragment_range: [f64; 2],
        subdivisions: &[PlanarBooleanNormalizedIntervalSubdivisionRow],
    ) -> Self {
        let mut membership = Self::default();
        for subdivision in subdivisions {
            if !ranges_overlap(fragment_range, subdivision.admitted_parameter_range()) {
                continue;
            }
            membership
                .interval_subdivision_identities
                .push(subdivision.subdivision_identity().to_string());
            membership
                .normalized_interval_identities
                .push(subdivision.normalized_interval_identity().to_string());
            membership
                .event_group_identities
                .extend(subdivision.event_group_identities().iter().cloned());
            membership
                .provenance_identities
                .extend(subdivision.provenance_entry_identities().iter().cloned());
            membership.source_senses.push(subdivision.source_sense());
        }
        canonicalize_membership(&mut membership);
        membership
    }

    pub(super) fn is_interval_attributed(&self) -> bool {
        !self.interval_subdivision_identities.is_empty()
    }
}

fn ranges_overlap(a: [f64; 2], b: [f64; 2]) -> bool {
    let [a0, a1] = ordered_range(a);
    let [b0, b1] = ordered_range(b);
    a0 < b1 && b0 < a1
}

fn ordered_range(range: [f64; 2]) -> [f64; 2] {
    if range[0] <= range[1] {
        range
    } else {
        [range[1], range[0]]
    }
}

fn canonicalize_membership(membership: &mut FragmentIntervalMembership) {
    membership.interval_subdivision_identities.sort();
    membership.interval_subdivision_identities.dedup();
    membership.normalized_interval_identities.sort();
    membership.normalized_interval_identities.dedup();
    membership.event_group_identities.sort();
    membership.event_group_identities.dedup();
    membership.provenance_identities.sort();
    membership.provenance_identities.dedup();
    membership.source_senses.sort();
    membership.source_senses.dedup();
}
