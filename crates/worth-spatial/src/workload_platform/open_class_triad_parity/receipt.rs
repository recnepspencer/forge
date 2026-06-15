use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::lane_set::OpenClassParityLaneSet;
use super::open_class::OpenTopologyClass;
use crate::workload_platform::projection_fact_parity::ProjectionFactParityLane;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenClassTriadParityCounters {
    open_classes_compared: usize,
    lanes_per_class: usize,
    receipt_backed_lanes: usize,
    bounded_conversion_guards: usize,
}

impl OpenClassTriadParityCounters {
    pub(crate) fn new(
        open_classes_compared: usize,
        lanes_per_class: usize,
        receipt_backed_lanes: usize,
        bounded_conversion_guards: usize,
    ) -> Self {
        Self {
            open_classes_compared,
            lanes_per_class,
            receipt_backed_lanes,
            bounded_conversion_guards,
        }
    }

    pub fn open_classes_compared(self) -> usize {
        self.open_classes_compared
    }

    pub fn lanes_per_class(self) -> usize {
        self.lanes_per_class
    }

    pub fn receipt_backed_lanes(self) -> usize {
        self.receipt_backed_lanes
    }

    pub fn bounded_conversion_guards(self) -> usize {
        self.bounded_conversion_guards
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenClassTriadParityReceipt {
    triad_digest: String,
    declaration: String,
    lane_sets: Vec<OpenClassParityLaneSet>,
    counters: OpenClassTriadParityCounters,
}

impl OpenClassTriadParityReceipt {
    pub(crate) fn new(declaration: String, lane_sets: Vec<OpenClassParityLaneSet>) -> Self {
        let counters = OpenClassTriadParityCounters::new(
            lane_sets.len(),
            ProjectionFactParityLane::REQUIRED.len(),
            lane_sets
                .iter()
                .map(OpenClassParityLaneSet::receipt_backed_lane_count)
                .sum(),
            lane_sets.len(),
        );
        let triad_digest = triad_digest(&declaration, &lane_sets, counters);
        Self {
            triad_digest,
            declaration,
            lane_sets,
            counters,
        }
    }

    pub fn triad_digest(&self) -> &str {
        &self.triad_digest
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn counters(&self) -> OpenClassTriadParityCounters {
        self.counters
    }

    pub fn lane_sets(&self) -> &[OpenClassParityLaneSet] {
        &self.lane_sets
    }

    pub fn lane_set_for(
        &self,
        topology_class: OpenTopologyClass,
    ) -> Option<&OpenClassParityLaneSet> {
        self.lane_sets
            .iter()
            .find(|lane_set| lane_set.topology_class() == topology_class)
    }

    pub(super) fn require_class_for_attempt(
        &self,
        topology_class: OpenTopologyClass,
    ) -> Option<&OpenClassParityLaneSet> {
        self.lane_set_for(topology_class)
    }
}

fn triad_digest(
    declaration: &str,
    lane_sets: &[OpenClassParityLaneSet],
    counters: OpenClassTriadParityCounters,
) -> String {
    let mut parts = vec![
        "open-class-triad-parity".to_string(),
        declaration.to_string(),
        format!("classes:{}", counters.open_classes_compared()),
        format!("lanes_per_class:{}", counters.lanes_per_class()),
        format!("receipt_backed_lanes:{}", counters.receipt_backed_lanes()),
    ];
    for lane_set in lane_sets {
        parts.push(format!(
            "{:?}:{}:{}:{}",
            lane_set.topology_class(),
            lane_set.topology_identity(),
            lane_set.parity().parity_digest(),
            lane_set.open_boundary_digest()
        ));
        parts.push(format!("radial:{}", lane_set.radial_digest()));
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
