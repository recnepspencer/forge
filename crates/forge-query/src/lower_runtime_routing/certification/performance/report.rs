use crate::identity::hash_parts;

use super::super::surface::ForgeQueryLowerRuntimeRepresentativeSurface;
use super::profiles::{
    forge_query_lower_runtime_performance_profiles, ForgeQueryLowerRuntimePerformanceProfile,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimePerformanceFamily {
    CapabilityEligibility,
    RoutePlanAssembly,
    BoundaryReceiptAssembly,
    BoundaryEnvelopeAssembly,
    SupportLookup,
    DebtRegistryLookup,
}

impl ForgeQueryLowerRuntimePerformanceFamily {
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
pub struct ForgeQueryLowerRuntimePerformanceSlopeRow {
    family: ForgeQueryLowerRuntimePerformanceFamily,
    slope_digest: String,
}

impl ForgeQueryLowerRuntimePerformanceSlopeRow {
    fn new(
        family: ForgeQueryLowerRuntimePerformanceFamily,
        profiles: &[ForgeQueryLowerRuntimePerformanceProfile],
    ) -> Self {
        let slope_digest = hash_parts(
            &profiles
                .iter()
                .map(|profile| {
                    format!(
                        "{}|{}|{}|{}|{}",
                        family.as_str(),
                        profile.label().as_str(),
                        profile.counters().crossing_inventory_width(),
                        profile.counters().boundary_evidence_width(),
                        family_operations(family, profile)
                    )
                })
                .collect::<Vec<_>>(),
        );
        Self {
            family,
            slope_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryLowerRuntimePerformanceFamily {
        self.family
    }

    pub fn slope_digest(&self) -> &str {
        &self.slope_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimePerformanceSlopeReport {
    profiles: Vec<ForgeQueryLowerRuntimePerformanceProfile>,
    rows: Vec<ForgeQueryLowerRuntimePerformanceSlopeRow>,
}

impl ForgeQueryLowerRuntimePerformanceSlopeReport {
    fn new(profiles: Vec<ForgeQueryLowerRuntimePerformanceProfile>) -> Self {
        let rows = [
            ForgeQueryLowerRuntimePerformanceFamily::CapabilityEligibility,
            ForgeQueryLowerRuntimePerformanceFamily::RoutePlanAssembly,
            ForgeQueryLowerRuntimePerformanceFamily::BoundaryReceiptAssembly,
            ForgeQueryLowerRuntimePerformanceFamily::BoundaryEnvelopeAssembly,
            ForgeQueryLowerRuntimePerformanceFamily::SupportLookup,
            ForgeQueryLowerRuntimePerformanceFamily::DebtRegistryLookup,
        ]
        .into_iter()
        .map(|family| ForgeQueryLowerRuntimePerformanceSlopeRow::new(family, &profiles))
        .collect();
        Self { profiles, rows }
    }

    pub fn profiles(&self) -> &[ForgeQueryLowerRuntimePerformanceProfile] {
        &self.profiles
    }

    pub fn full_profile(&self) -> &ForgeQueryLowerRuntimePerformanceProfile {
        self.profiles
            .last()
            .expect("performance report must contain a full profile")
    }

    pub fn rows(&self) -> &[ForgeQueryLowerRuntimePerformanceSlopeRow] {
        &self.rows
    }

    pub fn digest_for_output(&self, output_name: &str) -> Option<&str> {
        self.rows
            .iter()
            .find(|row| row.family().output_name() == output_name)
            .map(ForgeQueryLowerRuntimePerformanceSlopeRow::slope_digest)
    }
}

pub fn certify_lower_runtime_performance_slopes(
    surface: &ForgeQueryLowerRuntimeRepresentativeSurface,
) -> ForgeQueryLowerRuntimePerformanceSlopeReport {
    ForgeQueryLowerRuntimePerformanceSlopeReport::new(
        forge_query_lower_runtime_performance_profiles(surface),
    )
}

fn family_operations(
    family: ForgeQueryLowerRuntimePerformanceFamily,
    profile: &ForgeQueryLowerRuntimePerformanceProfile,
) -> usize {
    let counters = profile.counters();
    match family {
        ForgeQueryLowerRuntimePerformanceFamily::CapabilityEligibility => {
            counters.capability_eligibility_operations()
        }
        ForgeQueryLowerRuntimePerformanceFamily::RoutePlanAssembly => {
            counters.route_plan_assembly_operations()
        }
        ForgeQueryLowerRuntimePerformanceFamily::BoundaryReceiptAssembly => {
            counters.boundary_receipt_assembly_operations()
        }
        ForgeQueryLowerRuntimePerformanceFamily::BoundaryEnvelopeAssembly => {
            counters.boundary_envelope_assembly_operations()
        }
        ForgeQueryLowerRuntimePerformanceFamily::SupportLookup => {
            counters.support_lookup_operations()
        }
        ForgeQueryLowerRuntimePerformanceFamily::DebtRegistryLookup => {
            counters.debt_registry_lookup_operations()
        }
    }
}
