use forge_query::facade::ForgeQueryDeclarationEnvelope;
use worth_geom::facade::{
    PrimitiveConditioningWitness, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveSupportNormalClass,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native_tolerance_precision::ToleranceAndPrecisionCertificationQueryDomain;
use crate::bindings::query_native_tolerance_precision_authoring::{
    CertifiedToleranceBound, PrimitiveConstructionToleranceAndPrecisionCertificationEntry,
    ToleranceAndPrecisionCertificateKind, ToleranceAndPrecisionCertificationCase,
    ToleranceAndPrecisionCertificationFactReceipt, ToleranceAndPrecisionCertificationPosture,
    ToleranceAndPrecisionToleranceBasis, ToleranceAndPrecisionUnsupportedReason,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ToleranceAndPrecisionCertificationPayload {
    family_key: &'static str,
    certificate_kind: ToleranceAndPrecisionCertificateKind,
    precision_policy_identity: String,
    tolerance_basis: ToleranceAndPrecisionToleranceBasis,
    certified_bound: CertifiedToleranceBound,
    certification_posture: ToleranceAndPrecisionCertificationPosture,
    escalation_trace: Vec<String>,
    unsupported_reason: Option<ToleranceAndPrecisionUnsupportedReason>,
}

impl ToleranceAndPrecisionCertificationPayload {
    pub fn from_case(case: &ToleranceAndPrecisionCertificationCase) -> Self {
        let witness = case.realization_posture().conditioning_witness();
        let certified_bound = CertifiedToleranceBound::new(
            witness.precision_headroom_ratio(),
            witness.support_normal_headroom_ratio(),
            witness.normalization_scale_applied(),
        );
        let (certification_posture, unsupported_reason) = certification_posture(witness);
        let escalation_trace = escalation_trace(case, certification_posture);
        Self {
            family_key: case.family().as_str(),
            certificate_kind: case.certificate_kind(),
            precision_policy_identity: case.precision_policy_identity().to_string(),
            tolerance_basis: case.tolerance_basis(),
            certified_bound,
            certification_posture,
            escalation_trace,
            unsupported_reason,
        }
    }

    pub fn from_bound_envelope(
        &self,
        envelope: &ForgeQueryDeclarationEnvelope<
            ToleranceAndPrecisionCertificationQueryDomain,
            PrimitiveConstructionToleranceAndPrecisionCertificationEntry,
        >,
    ) -> ToleranceAndPrecisionCertificationFactReceipt {
        let fact_digest = digest_parts(&[
            self.family_key.to_string(),
            self.precision_policy_identity.clone(),
            format!("{:?}", self.tolerance_basis.origin()),
            format!("{:?}", self.tolerance_basis.facing_vector()),
            self.certified_bound
                .precision_headroom_ratio()
                .to_bits()
                .to_string(),
            self.certified_bound
                .support_normal_headroom_ratio()
                .to_bits()
                .to_string(),
            self.certified_bound
                .normalization_scale_applied()
                .to_bits()
                .to_string(),
            format!("{:?}", self.certification_posture),
            format!("{:?}", self.unsupported_reason),
            envelope.declaration_digest().to_string(),
            format!("{:?}", envelope.envelope_digest()),
        ]);
        ToleranceAndPrecisionCertificationFactReceipt::new(
            self.certificate_kind,
            self.precision_policy_identity.clone(),
            self.tolerance_basis,
            self.certified_bound,
            self.certification_posture,
            fact_digest,
            self.escalation_trace.clone(),
            self.unsupported_reason,
        )
    }
}

fn certification_posture(
    witness: &PrimitiveConditioningWitness,
) -> (
    ToleranceAndPrecisionCertificationPosture,
    Option<ToleranceAndPrecisionUnsupportedReason>,
) {
    if witness.feature_conditioning_class() == PrimitiveFeatureConditioningClass::Collapsed {
        return (
            ToleranceAndPrecisionCertificationPosture::Unsupported,
            Some(ToleranceAndPrecisionUnsupportedReason::CollapsedFeatureScale),
        );
    }
    if witness.support_normal_class() == PrimitiveSupportNormalClass::Degenerate {
        return (
            ToleranceAndPrecisionCertificationPosture::Unsupported,
            Some(ToleranceAndPrecisionUnsupportedReason::DegenerateSupportNormals),
        );
    }
    if witness.feature_conditioning_class() == PrimitiveFeatureConditioningClass::NearThreshold
        || witness.support_normal_class() == PrimitiveSupportNormalClass::NearDegenerate
        || witness.normalization_disposition()
            != PrimitiveNormalizationDisposition::WorldSpaceSufficient
    {
        return (
            ToleranceAndPrecisionCertificationPosture::CertifiedAfterEscalation,
            None,
        );
    }
    (
        ToleranceAndPrecisionCertificationPosture::CertifiedStable,
        None,
    )
}

fn escalation_trace(
    case: &ToleranceAndPrecisionCertificationCase,
    posture: ToleranceAndPrecisionCertificationPosture,
) -> Vec<String> {
    if posture != ToleranceAndPrecisionCertificationPosture::CertifiedAfterEscalation {
        return Vec::new();
    }
    let witness = case.realization_posture().conditioning_witness();
    let mut trace = vec![format!(
        "selected_strategy:{}",
        case.realization_posture().selected_strategy().as_str()
    )];
    trace.push(format!(
        "attempted:{}",
        case.realization_posture()
            .attempted_strategies()
            .iter()
            .map(|strategy| strategy.as_str())
            .collect::<Vec<_>>()
            .join("->")
    ));
    trace.push(format!(
        "normalization:{}",
        witness.normalization_disposition().as_str()
    ));
    trace.push(format!(
        "feature_conditioning:{}",
        witness.feature_conditioning_class().as_str()
    ));
    trace.push(format!(
        "support_normals:{}",
        witness.support_normal_class().as_str()
    ));
    trace
}

fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
