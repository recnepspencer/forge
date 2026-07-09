use crate::identity::hash_parts;

use super::super::surface::WorthQueryLowerRuntimeRepresentativeSurface;
use super::profiles::{
    worth_query_lower_runtime_performance_profiles, WorthQueryLowerRuntimePerformanceProfile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryLowerRuntimePerformanceFamily {
    CapabilityEligibility,
    RoutePlanAssembly,
    BoundaryReceiptAssembly,
    BoundaryEnvelopeAssembly,
    SupportLookup,
    DebtRegistryLookup,
}

impl WorthQueryLowerRuntimePerformanceFamily {
    pub fn output_name(self) -> &'static str {
        match self {
            Self::CapabilityEligibility => "capability_eligibility_slope_digest",
            Self::RoutePlanAssembly => "route_plan_assembly_slope_digest",
            Self::BoundaryReceiptAssembly => "boundary_receipt_assembly_slope_digest",
            Self::BoundaryEnvelopeAssembly => "boundary_envelope_assembly_slope_digest",
            Self::SupportLookup => "support_lookup_slope_digest",
            Self::DebtRegistryLookup => "debt_registry_lookup_slope_digest",
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::CapabilityEligibility => "capability-eligibility",
            Self::RoutePlanAssembly => "route-plan-assembly",
            Self::BoundaryReceiptAssembly => "boundary-receipt-assembly",
            Self::BoundaryEnvelopeAssembly => "boundary-envelope-assembly",
            Self::SupportLookup => "support-lookup",
            Self::DebtRegistryLookup => "debt-registry-lookup",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimePerformanceSlopeRow {
    family: WorthQueryLowerRuntimePerformanceFamily,
    slope_digest: String,
}

impl WorthQueryLowerRuntimePerformanceSlopeRow {
    fn new(
        family: WorthQueryLowerRuntimePerformanceFamily,
        profiles: &[WorthQueryLowerRuntimePerformanceProfile],
    ) -> Self {
        let slope_digest = slope_digest_for_profiles(family, profiles);
        Self {
            family,
            slope_digest,
        }
    }

    pub fn family(&self) -> WorthQueryLowerRuntimePerformanceFamily {
        self.family
    }

    pub fn slope_digest(&self) -> &str {
        &self.slope_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLowerRuntimePerformanceSlopeReport {
    profiles: Vec<WorthQueryLowerRuntimePerformanceProfile>,
    rows: Vec<WorthQueryLowerRuntimePerformanceSlopeRow>,
}

impl WorthQueryLowerRuntimePerformanceSlopeReport {
    fn new(profiles: Vec<WorthQueryLowerRuntimePerformanceProfile>) -> Self {
        let rows = [
            WorthQueryLowerRuntimePerformanceFamily::CapabilityEligibility,
            WorthQueryLowerRuntimePerformanceFamily::RoutePlanAssembly,
            WorthQueryLowerRuntimePerformanceFamily::BoundaryReceiptAssembly,
            WorthQueryLowerRuntimePerformanceFamily::BoundaryEnvelopeAssembly,
            WorthQueryLowerRuntimePerformanceFamily::SupportLookup,
            WorthQueryLowerRuntimePerformanceFamily::DebtRegistryLookup,
        ]
        .into_iter()
        .map(|family| WorthQueryLowerRuntimePerformanceSlopeRow::new(family, &profiles))
        .collect();
        Self { profiles, rows }
    }

    pub fn profiles(&self) -> &[WorthQueryLowerRuntimePerformanceProfile] {
        &self.profiles
    }

    pub fn full_profile(&self) -> &WorthQueryLowerRuntimePerformanceProfile {
        self.profiles
            .last()
            .expect("performance report must contain a full profile")
    }

    pub fn rows(&self) -> &[WorthQueryLowerRuntimePerformanceSlopeRow] {
        &self.rows
    }

    pub fn digest_for_output(&self, output_name: &str) -> Option<&str> {
        self.rows
            .iter()
            .find(|row| row.family().output_name() == output_name)
            .map(WorthQueryLowerRuntimePerformanceSlopeRow::slope_digest)
    }
}

pub fn certify_lower_runtime_performance_slopes(
    surface: &WorthQueryLowerRuntimeRepresentativeSurface,
) -> WorthQueryLowerRuntimePerformanceSlopeReport {
    WorthQueryLowerRuntimePerformanceSlopeReport::new(
        worth_query_lower_runtime_performance_profiles(surface),
    )
}

#[cfg(test)]
pub(super) fn test_slope_digest_for_profiles(
    family: WorthQueryLowerRuntimePerformanceFamily,
    profiles: &[WorthQueryLowerRuntimePerformanceProfile],
) -> String {
    slope_digest_for_profiles(family, profiles)
}

fn family_operations(
    family: WorthQueryLowerRuntimePerformanceFamily,
    profile: &WorthQueryLowerRuntimePerformanceProfile,
) -> usize {
    let counters = profile.counters();
    match family {
        WorthQueryLowerRuntimePerformanceFamily::CapabilityEligibility => {
            counters.capability_eligibility_operations()
        }
        WorthQueryLowerRuntimePerformanceFamily::RoutePlanAssembly => {
            counters.route_plan_assembly_operations()
        }
        WorthQueryLowerRuntimePerformanceFamily::BoundaryReceiptAssembly => {
            counters.boundary_receipt_assembly_operations()
        }
        WorthQueryLowerRuntimePerformanceFamily::BoundaryEnvelopeAssembly => {
            counters.boundary_envelope_assembly_operations()
        }
        WorthQueryLowerRuntimePerformanceFamily::SupportLookup => {
            counters.support_lookup_operations()
        }
        WorthQueryLowerRuntimePerformanceFamily::DebtRegistryLookup => {
            counters.debt_registry_lookup_operations()
        }
    }
}

fn slope_digest_for_profiles(
    family: WorthQueryLowerRuntimePerformanceFamily,
    profiles: &[WorthQueryLowerRuntimePerformanceProfile],
) -> String {
    hash_parts(
        &profiles
            .iter()
            .map(|profile| {
                let counters = profile.counters();
                let mut parts = vec![
                    format!("family:{}", family.as_str()),
                    format!("profile:{}", profile.label().as_str()),
                    format!("ops:{}", family_operations(family, profile)),
                ];
                match family {
                    WorthQueryLowerRuntimePerformanceFamily::CapabilityEligibility => {
                        parts.push(format!("crossings:{}", counters.crossing_inventory_width()));
                    }
                    WorthQueryLowerRuntimePerformanceFamily::RoutePlanAssembly => {
                        parts.push(format!("routes:{}", counters.route_plan_width()));
                    }
                    WorthQueryLowerRuntimePerformanceFamily::BoundaryReceiptAssembly
                    | WorthQueryLowerRuntimePerformanceFamily::BoundaryEnvelopeAssembly => {
                        parts.push(format!("evidence:{}", counters.boundary_evidence_width()));
                    }
                    WorthQueryLowerRuntimePerformanceFamily::SupportLookup => {
                        parts.push(format!("support:{}", counters.support_width()));
                        parts.push(format!("evidence:{}", counters.boundary_evidence_width()));
                    }
                    WorthQueryLowerRuntimePerformanceFamily::DebtRegistryLookup => {
                        parts.push(format!("deferred:{}", counters.deferred_width()));
                    }
                }
                parts.join("|")
            })
            .collect::<Vec<_>>(),
    )
}
