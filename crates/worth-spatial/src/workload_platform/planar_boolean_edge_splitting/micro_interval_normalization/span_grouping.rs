use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::PlanarBooleanRetainedIntervalSplitEntry;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::action::{PlanarBooleanMicroIntervalAction, PlanarBooleanMicroIntervalPolicy};
use super::consistency::reject_contradictory_interval_subdivision_basis;
use super::denial::{
    PlanarBooleanIntervalSubdivisionNormalizationDenial,
    PlanarBooleanIntervalSubdivisionNormalizationDenialKind,
};
use super::identity::interval_subdivision_identity;
use super::subdivision_row::PlanarBooleanNormalizedIntervalSubdivisionRow;

const MICRO_INTERVAL_TOLERANCE: f64 = 1.0e-12;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct IntervalSubdivisionGroupKey {
    source_edge_identity: String,
    carrier_identity: String,
    range_bits: [u64; 2],
    source_range_bits: [u64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
    normalized_interval_identity: String,
    normalized_range_bits: [u64; 2],
    interval_event_kind: PlanarBooleanIntervalEventKind,
    local_frame_identity: String,
    precision_basis_identity: String,
}

impl IntervalSubdivisionGroupKey {
    fn from_entry(
        entry: &PlanarBooleanRetainedIntervalSplitEntry,
    ) -> Result<Self, PlanarBooleanIntervalSubdivisionNormalizationDenial> {
        let range = entry.admitted_parameter_range();
        if !range[0].is_finite() || !range[1].is_finite() {
            return Err(PlanarBooleanIntervalSubdivisionNormalizationDenial::new(
                PlanarBooleanIntervalSubdivisionNormalizationDenialKind::NonFiniteIntervalBoundary,
                entry.entry_identity(),
                "retained interval subdivision contains a non-finite boundary",
            ));
        }
        if canonical_parameter_bits(range[0]) == canonical_parameter_bits(range[1]) {
            return Err(PlanarBooleanIntervalSubdivisionNormalizationDenial::new(
                PlanarBooleanIntervalSubdivisionNormalizationDenialKind::CollapsedIntervalSubdivision,
                entry.entry_identity(),
                "retained interval subdivision collapsed to a zero-length range",
            ));
        }
        Ok(Self {
            source_edge_identity: entry.source_edge_identity().to_string(),
            carrier_identity: entry.carrier_identity().to_string(),
            range_bits: [
                canonical_parameter_bits(range[0]),
                canonical_parameter_bits(range[1]),
            ],
            source_range_bits: [
                canonical_parameter_bits(entry.source_parameter_range()[0]),
                canonical_parameter_bits(entry.source_parameter_range()[1]),
            ],
            source_sense: entry.source_sense(),
            normalized_interval_identity: entry.normalized_interval_identity().to_string(),
            normalized_range_bits: [
                canonical_parameter_bits(entry.normalized_parameter_range()[0]),
                canonical_parameter_bits(entry.normalized_parameter_range()[1]),
            ],
            interval_event_kind: entry.interval_event_kind(),
            local_frame_identity: entry.local_frame_identity().to_string(),
            precision_basis_identity: entry.precision_basis_identity().to_string(),
        })
    }

    pub(super) fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub(super) fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub(super) fn range(&self) -> [f64; 2] {
        [
            f64::from_bits(self.range_bits[0]),
            f64::from_bits(self.range_bits[1]),
        ]
    }
    pub(super) fn source_range(&self) -> [f64; 2] {
        [
            f64::from_bits(self.source_range_bits[0]),
            f64::from_bits(self.source_range_bits[1]),
        ]
    }
    pub(super) fn source_sense(&self) -> PlanarBooleanSourceIntervalSense {
        self.source_sense
    }
    pub(super) fn normalized_interval_identity(&self) -> &str {
        &self.normalized_interval_identity
    }
    pub(super) fn normalized_range(&self) -> [f64; 2] {
        [
            f64::from_bits(self.normalized_range_bits[0]),
            f64::from_bits(self.normalized_range_bits[1]),
        ]
    }
    pub(super) fn interval_event_kind(&self) -> PlanarBooleanIntervalEventKind {
        self.interval_event_kind
    }
    pub(super) fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }
    pub(super) fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }
}

pub(super) struct IntervalSubdivisionGrouping {
    rows: Vec<PlanarBooleanNormalizedIntervalSubdivisionRow>,
    inspected_rows: usize,
    redundant_rows_collapsed: usize,
    micro_intervals_admitted: usize,
    micro_intervals_policy_required: usize,
    opposite_sense_rows_preserved: usize,
}

impl IntervalSubdivisionGrouping {
    pub(super) fn from_retained_entries(
        endpoint_schedule_identity: &str,
        entries: &[PlanarBooleanRetainedIntervalSplitEntry],
        policy: PlanarBooleanMicroIntervalPolicy,
    ) -> Result<Self, PlanarBooleanIntervalSubdivisionNormalizationDenial> {
        reject_contradictory_interval_subdivision_basis(entries)?;
        let mut groups = BTreeMap::<
            IntervalSubdivisionGroupKey,
            Vec<&PlanarBooleanRetainedIntervalSplitEntry>,
        >::new();
        for entry in entries {
            groups
                .entry(IntervalSubdivisionGroupKey::from_entry(entry)?)
                .or_default()
                .push(entry);
        }
        let inspected_rows = entries.len();
        let redundant_rows_collapsed = groups
            .values()
            .map(|group| group.len().saturating_sub(1))
            .sum::<usize>();
        let mut rows = Vec::with_capacity(groups.len());
        let mut micro_intervals_admitted = 0;
        let mut micro_intervals_policy_required = 0;
        let mut opposite_sense_rows_preserved = 0;
        for (key, group) in groups {
            let action = action_for_group(&key, policy, group[0].entry_identity())?;
            if action == PlanarBooleanMicroIntervalAction::AdmittedCollapse {
                micro_intervals_admitted += 1;
            }
            if action == PlanarBooleanMicroIntervalAction::PolicyRequired {
                micro_intervals_policy_required += 1;
            }
            if key.source_sense() == PlanarBooleanSourceIntervalSense::Reversed {
                opposite_sense_rows_preserved += group.len();
            }
            rows.push(row_from_group(
                endpoint_schedule_identity,
                key,
                group,
                action,
            ));
        }
        Ok(Self {
            rows,
            inspected_rows,
            redundant_rows_collapsed,
            micro_intervals_admitted,
            micro_intervals_policy_required,
            opposite_sense_rows_preserved,
        })
    }

    pub(super) fn rows(&self) -> &[PlanarBooleanNormalizedIntervalSubdivisionRow] {
        &self.rows
    }
    pub(super) fn into_rows(self) -> Vec<PlanarBooleanNormalizedIntervalSubdivisionRow> {
        self.rows
    }
    pub(super) fn inspected_rows(&self) -> usize {
        self.inspected_rows
    }
    pub(super) fn redundant_rows_collapsed(&self) -> usize {
        self.redundant_rows_collapsed
    }
    pub(super) fn micro_intervals_admitted(&self) -> usize {
        self.micro_intervals_admitted
    }
    pub(super) fn micro_intervals_policy_required(&self) -> usize {
        self.micro_intervals_policy_required
    }
    pub(super) fn opposite_sense_rows_preserved(&self) -> usize {
        self.opposite_sense_rows_preserved
    }
}

fn action_for_group(
    key: &IntervalSubdivisionGroupKey,
    policy: PlanarBooleanMicroIntervalPolicy,
    evidence_identity: &str,
) -> Result<PlanarBooleanMicroIntervalAction, PlanarBooleanIntervalSubdivisionNormalizationDenial> {
    let range = key.range();
    if (range[1] - range[0]).abs() >= MICRO_INTERVAL_TOLERANCE {
        return Ok(PlanarBooleanMicroIntervalAction::Retain);
    }
    match policy {
        PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance => {
            Err(PlanarBooleanIntervalSubdivisionNormalizationDenial::new(
                PlanarBooleanIntervalSubdivisionNormalizationDenialKind::MicroIntervalBelowAdmittedPolicy,
                evidence_identity,
                "micro-interval subdivision is below admitted policy tolerance",
            ))
        }
        PlanarBooleanMicroIntervalPolicy::AdmitExplicitCollapse => {
            Ok(PlanarBooleanMicroIntervalAction::AdmittedCollapse)
        }
        PlanarBooleanMicroIntervalPolicy::RequireExplicitDecision => {
            Ok(PlanarBooleanMicroIntervalAction::PolicyRequired)
        }
    }
}

fn row_from_group(
    endpoint_schedule_identity: &str,
    key: IntervalSubdivisionGroupKey,
    group: Vec<&PlanarBooleanRetainedIntervalSplitEntry>,
    action: PlanarBooleanMicroIntervalAction,
) -> PlanarBooleanNormalizedIntervalSubdivisionRow {
    let mut provenance_entry_identities = group
        .iter()
        .map(|entry| entry.entry_identity().to_string())
        .collect::<Vec<_>>();
    provenance_entry_identities.sort();
    let mut candidate_identities = group
        .iter()
        .map(|entry| entry.candidate_identity().to_string())
        .collect::<Vec<_>>();
    candidate_identities.sort();
    candidate_identities.dedup();
    let mut event_group_identities = group
        .iter()
        .flat_map(|entry| entry.event_group_identities().iter().cloned())
        .collect::<Vec<_>>();
    event_group_identities.sort();
    event_group_identities.dedup();
    let interval_event_identity = canonical_group_identity(&group, |entry| entry.event_identity());
    let source_interval_identity =
        canonical_group_identity(&group, |entry| entry.source_interval_identity());
    let subdivision_identity = interval_subdivision_identity(
        endpoint_schedule_identity,
        &key,
        &provenance_entry_identities,
        &event_group_identities,
        action,
    );
    PlanarBooleanNormalizedIntervalSubdivisionRow::new(
        subdivision_identity,
        interval_event_identity,
        candidate_identities,
        key.source_edge_identity().to_string(),
        key.carrier_identity().to_string(),
        key.range(),
        source_interval_identity,
        key.source_range(),
        key.source_sense(),
        key.normalized_interval_identity().to_string(),
        key.normalized_range(),
        key.interval_event_kind(),
        key.local_frame_identity().to_string(),
        key.precision_basis_identity().to_string(),
        action,
        provenance_entry_identities,
        event_group_identities,
    )
}

fn canonical_group_identity(
    group: &[&PlanarBooleanRetainedIntervalSplitEntry],
    identity: impl Fn(&PlanarBooleanRetainedIntervalSplitEntry) -> &str,
) -> String {
    group
        .iter()
        .map(|entry| identity(entry))
        .min()
        .expect("interval subdivision groups are never empty")
        .to_string()
}
