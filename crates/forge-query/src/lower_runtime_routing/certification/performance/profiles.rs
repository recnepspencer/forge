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
                    observe_boundary_receipt_assembly(route_plans, receipts),
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
    requests
        .iter()
        .zip(eligibilities.iter())
        .map(|(request, eligibility)| {
            usize::from(eligibility.request().request_digest() == request.request_digest()) + 2
        })
        .sum()
}

fn observe_route_plan_assembly(
    route_plans: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeRoutePlan],
) -> usize {
    route_plans
        .iter()
        .map(|plan| {
            let _ = plan.eligibility().eligibility_digest();
            let _ = plan.route_subject();
            let _ = plan.route_digest();
            3
        })
        .sum()
}

fn observe_boundary_receipt_assembly(
    route_plans: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeRoutePlan],
    receipts: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt],
) -> usize {
    receipts
        .iter()
        .map(|receipt| {
            let mut comparisons = 0usize;
            for plan in route_plans {
                comparisons += 1;
                if receipt.request_digest() == plan.eligibility().request().request_digest()
                    && receipt.eligibility_digest() == plan.eligibility().eligibility_digest()
                {
                    let _ = receipt.boundary_execution_digest();
                    break;
                }
            }
            comparisons + 1
        })
        .sum()
}

fn observe_boundary_envelope_assembly(
    receipts: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt],
    envelopes: &[crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope],
) -> usize {
    envelopes
        .iter()
        .map(|envelope| {
            let mut comparisons = 0usize;
            for receipt in receipts {
                comparisons += 1;
                if receipt.boundary_execution_digest() == envelope.boundary_execution_digest() {
                    let _ = envelope.envelope_digest();
                    break;
                }
            }
            comparisons + 1
        })
        .sum()
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
    deferred_rows
        .iter()
        .map(|row| {
            let _ = row.seam_key();
            let _ = row.closeout_target();
            let _ = row.required_closeout();
            3
        })
        .sum()
}
