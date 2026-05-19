use crate::identity::hash_parts;

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
    width: usize,
    evidence_width: usize,
    deferred_width: usize,
    slope_digest: String,
}

impl ForgeQueryLowerRuntimePerformanceSlopeRow {
    fn new(
        family: ForgeQueryLowerRuntimePerformanceFamily,
        width: usize,
        evidence_width: usize,
        deferred_width: usize,
    ) -> Self {
        let slope_digest = hash_parts(&[
            "lower_runtime_routing_slope_digest_v1".to_string(),
            format!("family:{}", family.as_str()),
            format!("width:{width}"),
            format!("evidence_width:{evidence_width}"),
            format!("deferred_width:{deferred_width}"),
        ]);
        Self {
            family,
            width,
            evidence_width,
            deferred_width,
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
    rows: Vec<ForgeQueryLowerRuntimePerformanceSlopeRow>,
}

impl ForgeQueryLowerRuntimePerformanceSlopeReport {
    fn new(rows: Vec<ForgeQueryLowerRuntimePerformanceSlopeRow>) -> Self {
        Self { rows }
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
    crossing_inventory_width: usize,
    route_plan_width: usize,
    boundary_evidence_width: usize,
    support_width: usize,
    deferred_width: usize,
) -> ForgeQueryLowerRuntimePerformanceSlopeReport {
    ForgeQueryLowerRuntimePerformanceSlopeReport::new(vec![
        ForgeQueryLowerRuntimePerformanceSlopeRow::new(
            ForgeQueryLowerRuntimePerformanceFamily::CapabilityEligibility,
            crossing_inventory_width,
            boundary_evidence_width,
            deferred_width,
        ),
        ForgeQueryLowerRuntimePerformanceSlopeRow::new(
            ForgeQueryLowerRuntimePerformanceFamily::RoutePlanAssembly,
            route_plan_width,
            boundary_evidence_width,
            deferred_width,
        ),
        ForgeQueryLowerRuntimePerformanceSlopeRow::new(
            ForgeQueryLowerRuntimePerformanceFamily::BoundaryReceiptAssembly,
            boundary_evidence_width,
            boundary_evidence_width,
            deferred_width,
        ),
        ForgeQueryLowerRuntimePerformanceSlopeRow::new(
            ForgeQueryLowerRuntimePerformanceFamily::BoundaryEnvelopeAssembly,
            boundary_evidence_width,
            boundary_evidence_width,
            deferred_width,
        ),
        ForgeQueryLowerRuntimePerformanceSlopeRow::new(
            ForgeQueryLowerRuntimePerformanceFamily::SupportLookup,
            support_width,
            boundary_evidence_width,
            deferred_width,
        ),
        ForgeQueryLowerRuntimePerformanceSlopeRow::new(
            ForgeQueryLowerRuntimePerformanceFamily::DebtRegistryLookup,
            deferred_width,
            boundary_evidence_width,
            deferred_width,
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slope_report_emits_all_phase_seven_outputs() {
        let report = certify_lower_runtime_performance_slopes(28, 18, 28, 37, 8);

        assert_eq!(report.rows().len(), 6);
        for row in report.rows() {
            assert_eq!(
                report.digest_for_output(row.family().output_name()),
                Some(row.slope_digest())
            );
        }
    }
}
