use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_crossing_inventory,
    forge_query_lower_runtime_gap_registry, forge_query_lower_runtime_support_matrix,
    ForgeQueryLowerRuntimeCloseoutPosture, ForgeQueryLowerRuntimeSupportRow,
};

use super::super::surface::ForgeQueryLowerRuntimeRepresentativeSurface;
use super::counters::ForgeQueryLowerRuntimePerformanceCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimePerformanceProfileLabel {
    Small,
    Medium,
    Full,
}

impl ForgeQueryLowerRuntimePerformanceProfileLabel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Full => "full",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimePerformanceProfile {
    label: ForgeQueryLowerRuntimePerformanceProfileLabel,
    counters: ForgeQueryLowerRuntimePerformanceCounters,
    profile_digest: String,
}

impl ForgeQueryLowerRuntimePerformanceProfile {
    fn new(
        label: ForgeQueryLowerRuntimePerformanceProfileLabel,
        counters: ForgeQueryLowerRuntimePerformanceCounters,
    ) -> Self {
        let profile_digest = hash_parts(&[
            format!("label:{}", label.as_str()),
            counters.counter_snapshot_digest().to_string(),
        ]);
        Self {
            label,
            counters,
            profile_digest,
        }
    }

    pub fn label(&self) -> ForgeQueryLowerRuntimePerformanceProfileLabel {
        self.label
    }

    pub fn counters(&self) -> &ForgeQueryLowerRuntimePerformanceCounters {
        &self.counters
    }

    pub fn profile_digest(&self) -> &str {
        &self.profile_digest
    }

    #[cfg(test)]
    pub(crate) fn new_for_tests(
        label: ForgeQueryLowerRuntimePerformanceProfileLabel,
        counters: ForgeQueryLowerRuntimePerformanceCounters,
    ) -> Self {
        Self::new(label, counters)
    }
}

pub(crate) fn forge_query_lower_runtime_performance_profiles(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> Vec<ForgeQueryLowerRuntimePerformanceProfile> {
    let crossings = forge_query_lower_runtime_crossing_inventory();
    let support = forge_query_lower_runtime_support_matrix();
    let closeout = forge_query_lower_runtime_closeout_registry();
    let compatibility_debt_width = forge_query_lower_runtime_gap_registry().rows().len();
    let deferred = closeout
        .rows()
        .iter()
        .filter(|row| row.posture() == ForgeQueryLowerRuntimeCloseoutPosture::DeferredNeighbor)
        .collect::<Vec<_>>();
    let scenario_specs = [
        (
            ForgeQueryLowerRuntimePerformanceProfileLabel::Small,
            scaled_width(crossings.rows().len(), 4),
            scaled_width(surface.route_plans().len(), 4),
            scaled_width(surface.envelopes().len(), 4),
            scaled_width(support.rows().len(), 4),
            scaled_width(deferred.len(), 4),
        ),
        (
            ForgeQueryLowerRuntimePerformanceProfileLabel::Medium,
            scaled_width(crossings.rows().len(), 2),
            scaled_width(surface.route_plans().len(), 2),
            scaled_width(surface.envelopes().len(), 2),
            scaled_width(support.rows().len(), 2),
            scaled_width(deferred.len(), 2),
        ),
        (
            ForgeQueryLowerRuntimePerformanceProfileLabel::Full,
            crossings.rows().len(),
            surface.route_plans().len(),
            surface.envelopes().len(),
            support.rows().len(),
            deferred.len(),
        ),
    ];

    scenario_specs
        .into_iter()
        .map(
            |(
                label,
                crossing_width,
                route_plan_width,
                evidence_width,
                support_width,
                deferred_width,
            )| {
                let requests = &surface.requests()[..crossing_width];
                let eligibilities = &surface.eligibilities()[..crossing_width];
                let route_plans = &surface.route_plans()[..route_plan_width];
                let receipts = &surface.boundary_receipts()[..evidence_width];
                let envelopes = &surface.envelopes()[..evidence_width];
                let support_rows = &support.rows()[..support_width];
                let deferred_rows = &deferred[..deferred_width];
                let counters = ForgeQueryLowerRuntimePerformanceCounters::new(
                    crossing_width,
                    compatibility_debt_width,
                    route_plan_width,
                    evidence_width,
                    support_width,
                    deferred_width,
                    observe_capability_eligibility(requests, eligibilities),
                    observe_route_plan_assembly(route_plans),
                    observe_boundary_receipt_assembly(eligibilities, receipts),
                    observe_boundary_envelope_assembly(receipts, envelopes),
                    observe_support_lookup(envelopes, support_rows),
                    observe_debt_registry_lookup(deferred_rows),
                );
                ForgeQueryLowerRuntimePerformanceProfile::new(label, counters)
            },
        )
        .collect()
}

fn scaled_width(full_width: usize, divisor: usize) -> usize {
    if full_width == 0 {
        0
    } else {
        usize::max(1, full_width / divisor)
    }
}

fn observe_capability_eligibility(
    requests: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityRequest],
    eligibilities: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityEligibility],
) -> usize {
    assert_eq!(
        requests.len(),
        eligibilities.len(),
        "capability eligibility profiles must keep request and eligibility widths aligned"
    );
    requests
        .iter()
        .zip(eligibilities.iter())
        .for_each(|(request, eligibility)| {
            assert_eq!(
                eligibility.request().request_identity(),
                request.request_identity(),
                "eligibility profile must reuse the emitted request identity exactly"
            );
        });
    eligibilities.len()
}

fn observe_route_plan_assembly(
    route_plans: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeRoutePlan],
) -> usize {
    route_plans.len()
}

fn observe_boundary_receipt_assembly(
    eligibilities: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityEligibility],
    receipts: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt],
) -> usize {
    receipts.iter().for_each(|receipt| {
        assert!(
            eligibilities.iter().any(|eligibility| {
                receipt.request_identity() == eligibility.request().request_identity()
                    && receipt.eligibility_identity() == eligibility.eligibility_identity()
            }),
            "boundary receipt profile must be backed by an emitted eligibility/request pair"
        );
    });
    receipts.len()
}

fn observe_boundary_envelope_assembly(
    receipts: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt],
    envelopes: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope],
) -> usize {
    envelopes.iter().for_each(|envelope| {
        assert!(
            receipts.iter().any(|receipt| {
                receipt.boundary_execution_identity() == envelope.boundary_execution_identity()
            }),
            "boundary envelope profile must be backed by an emitted execution receipt"
        );
    });
    envelopes.len()
}

fn observe_support_lookup(
    envelopes: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope],
    support_rows: &[ForgeQueryLowerRuntimeSupportRow],
) -> usize {
    envelopes
        .iter()
        .map(|envelope| {
            let mut comparisons = 0usize;
            for row in support_rows {
                comparisons += 1;
                if row.seam_key() == envelope.seam_key() {
                    let _ = row.capability_label();
                    break;
                }
            }
            comparisons
        })
        .sum()
}

fn observe_debt_registry_lookup(
    deferred_rows: &[&crate::lower_runtime_routing::ForgeQueryLowerRuntimeCloseoutRow],
) -> usize {
    deferred_rows.len()
}
