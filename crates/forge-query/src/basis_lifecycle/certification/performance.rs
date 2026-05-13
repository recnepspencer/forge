use crate::identity::hash_parts;

use super::super::{
    admit_basis_capability, discover_basis_lifecycle_support, emit_observation_basis_receipt,
    envelope_basis_use, evaluate_basis_observation_eligibility, normalize_raw_basis_intent,
    readmit_lower_runtime_evidence, scope_basis_for_observation, BasisEligibilityCounters,
    BasisFamily, BasisOperationLane, LowerRuntimeBasisEvidence, ObservationLaneWitness,
    RawBasisIntent,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleSlopeFamily {
    Normalization,
    Eligibility,
    LowerRuntimeBinding,
    ScopedUse,
    ReceiptEmission,
    EnvelopeMaterialization,
    SupportLookup,
}

impl BasisLifecycleSlopeFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normalization => "normalization",
            Self::Eligibility => "eligibility",
            Self::LowerRuntimeBinding => "lower_runtime_binding",
            Self::ScopedUse => "scoped_use",
            Self::ReceiptEmission => "receipt_emission",
            Self::EnvelopeMaterialization => "envelope_materialization",
            Self::SupportLookup => "support_lookup",
        }
    }

    pub fn output_name(&self) -> &'static str {
        match self {
            Self::Normalization => "basis_normalization_slope_digest",
            Self::Eligibility => "basis_eligibility_slope_digest",
            Self::LowerRuntimeBinding => "basis_lower_runtime_binding_slope_digest",
            Self::ScopedUse => "basis_scoped_use_slope_digest",
            Self::ReceiptEmission => "basis_receipt_slope_digest",
            Self::EnvelopeMaterialization => "basis_envelope_materialization_slope_digest",
            Self::SupportLookup => "basis_support_lookup_slope_digest",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleSlopeDigest {
    family: BasisLifecycleSlopeFamily,
    operation_lane: &'static str,
    counters: BasisEligibilityCounters,
    counter_digest: String,
    bounded_by: &'static str,
    slope_digest: String,
}

impl BasisLifecycleSlopeDigest {
    fn new(
        family: BasisLifecycleSlopeFamily,
        operation_lane: &'static str,
        counters: BasisEligibilityCounters,
        bounded_by: &'static str,
    ) -> Self {
        let counter_digest = counters.digest();
        let slope_digest = hash_parts(&[
            "basis_lifecycle_slope_digest_v1".to_string(),
            format!("family:{}", family.as_str()),
            format!("operation_lane:{operation_lane}"),
            format!("counter:{counter_digest}"),
            format!("bounded_by:{bounded_by}"),
        ]);
        Self {
            family,
            operation_lane,
            counters,
            counter_digest,
            bounded_by,
            slope_digest,
        }
    }

    pub fn family(&self) -> BasisLifecycleSlopeFamily {
        self.family
    }

    pub fn operation_lane(&self) -> &'static str {
        self.operation_lane
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }

    pub fn bounded_by(&self) -> &'static str {
        self.bounded_by
    }

    pub fn slope_digest(&self) -> &str {
        &self.slope_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecyclePerformanceSlopeReport {
    rows: Vec<BasisLifecycleSlopeDigest>,
    report_digest: String,
}

impl BasisLifecyclePerformanceSlopeReport {
    fn new(rows: Vec<BasisLifecycleSlopeDigest>) -> Self {
        let report_digest = hash_parts(
            &rows
                .iter()
                .map(|row| row.slope_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[BasisLifecycleSlopeDigest] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn digest_for_output(&self, output_name: &str) -> Option<&str> {
        self.rows
            .iter()
            .find(|row| row.family().output_name() == output_name)
            .map(|row| row.slope_digest())
    }
}

pub fn certify_basis_lifecycle_performance_slopes() -> BasisLifecyclePerformanceSlopeReport {
    let lane = <ObservationLaneWitness as BasisOperationLane>::lane_name();
    let normalized = normalize_raw_basis_intent(RawBasisIntent::CurrentHead, lane)
        .expect("performance normalization must succeed");
    let normalization_counters = normalized.counters().clone();
    let eligibility =
        evaluate_basis_observation_eligibility(normalized).expect("performance lane must admit");
    let eligibility_counters = eligibility.counters().clone();
    let capability = admit_basis_capability(eligibility);
    let scoped = scope_basis_for_observation(capability);
    let scoped_counters = scoped.counters().clone();
    let bound = readmit_lower_runtime_evidence(
        scoped,
        LowerRuntimeBasisEvidence::from_runtime_basis("runtime-current-head", "runtime-perf", 1),
    )
    .expect("performance lower-runtime readmission must bind");
    let lower_runtime_counters = bound.counters().clone();
    let receipt = emit_observation_basis_receipt(bound);
    let receipt_counters = receipt.counters().clone();
    let envelope = envelope_basis_use(receipt);
    let envelope_counters = envelope.counters().clone();
    let support = discover_basis_lifecycle_support(BasisFamily::CurrentHead, lane);
    let support_counters = support.counters().clone();

    BasisLifecyclePerformanceSlopeReport::new(vec![
        row(
            BasisLifecycleSlopeFamily::Normalization,
            lane,
            normalization_counters,
            "raw_basis_intent_width",
        ),
        row(
            BasisLifecycleSlopeFamily::Eligibility,
            lane,
            eligibility_counters,
            "operation_lane_width",
        ),
        row(
            BasisLifecycleSlopeFamily::LowerRuntimeBinding,
            lane,
            lower_runtime_counters,
            "retained_evidence_lookup_width",
        ),
        row(
            BasisLifecycleSlopeFamily::ScopedUse,
            lane,
            scoped_counters,
            "single_admitted_capability",
        ),
        row(
            BasisLifecycleSlopeFamily::ReceiptEmission,
            lane,
            receipt_counters,
            "retained_evidence_lookup_width",
        ),
        row(
            BasisLifecycleSlopeFamily::EnvelopeMaterialization,
            lane,
            envelope_counters,
            "single_basis_receipt",
        ),
        row(
            BasisLifecycleSlopeFamily::SupportLookup,
            lane,
            support_counters,
            "support_matrix_row_width",
        ),
    ])
}

fn row(
    family: BasisLifecycleSlopeFamily,
    operation_lane: &'static str,
    counters: BasisEligibilityCounters,
    bounded_by: &'static str,
) -> BasisLifecycleSlopeDigest {
    BasisLifecycleSlopeDigest::new(family, operation_lane, counters, bounded_by)
}

#[cfg(test)]
mod tests;
