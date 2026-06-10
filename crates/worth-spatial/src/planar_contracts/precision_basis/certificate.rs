use worth_math::arithmetic::precision::PrecisionEscalation;

use super::{planar_precision_digest, PlanarPrecisionBasis, PlanarPrecisionPerformanceCounters};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPrecisionCertificateReceipt {
    basis: PlanarPrecisionBasis,
    precision_escalation: PrecisionEscalation,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: PlanarPrecisionPerformanceCounters,
}

impl PlanarPrecisionCertificateReceipt {
    pub(crate) fn new(
        basis: PlanarPrecisionBasis,
        precision_escalation: PrecisionEscalation,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: PlanarPrecisionPerformanceCounters,
    ) -> Self {
        Self {
            basis,
            precision_escalation,
            declaration_digest,
            envelope_digest,
            fact_digest,
            counters,
        }
    }

    pub(crate) fn fact_digest_for(
        basis: &PlanarPrecisionBasis,
        precision_escalation: &PrecisionEscalation,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> String {
        planar_precision_digest(&Self::digest_parts(
            basis,
            precision_escalation,
            declaration_digest,
            envelope_digest,
        ))
    }

    pub(crate) fn digest_parts(
        basis: &PlanarPrecisionBasis,
        precision_escalation: &PrecisionEscalation,
        declaration_digest: &str,
        envelope_digest: &str,
    ) -> Vec<String> {
        vec![
            format!("local_frame:{}", basis.local_frame_identity()),
            format!("topology_basis:{}", basis.topology_basis_identity()),
            format!(
                "movement_rotation:{}",
                basis.movement_rotation_posture_identity()
            ),
            format!("tolerance:{}", basis.tolerance_policy_identity()),
            format!("predicate_fact:{}", basis.predicate_fact_digest()),
            format!(
                "predicate_declaration:{}",
                basis.predicate_declaration_digest()
            ),
            format!("predicate_envelope:{}", basis.predicate_envelope_digest()),
            format!("local_order:{}", basis.local_feature_scale_order()),
            format!("world_order:{}", basis.world_magnitude_order()),
            format!("scale_separation:{}", basis.scale_separation_orders()),
            format!("normalization:{}", basis.normalization_scale().to_bits()),
            format!("resolved_at:{:?}", precision_escalation.get_resolved_at()),
            format!("float_agreed:{}", precision_escalation.get_float_agreed()),
            format!(
                "expansion_length:{:?}",
                precision_escalation.get_expansion_length()
            ),
            format!("target:{}", precision_escalation.get_target_triple()),
            format!("declaration:{declaration_digest}"),
            format!("envelope:{envelope_digest}"),
        ]
    }

    pub fn basis(&self) -> &PlanarPrecisionBasis {
        &self.basis
    }

    pub fn precision_escalation(&self) -> &PrecisionEscalation {
        &self.precision_escalation
    }

    pub fn predicate_fact_digest(&self) -> &str {
        self.basis.predicate_fact_digest()
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn scale_separation_orders(&self) -> i32 {
        self.basis.scale_separation_orders()
    }

    pub fn counters(&self) -> PlanarPrecisionPerformanceCounters {
        self.counters
    }
}
