use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::nmt_radial_fan::{NmtRadialFanDenial, NmtRadialFanReceipt};
use crate::workload_platform::user_response::{
    source::WorthUserResponseSourceKind, WorthUserOutcomeCauseKind, WorthUserResponseSource,
};

impl WorthUserResponseSource {
    pub fn from_nmt_radial_fan(receipt: &NmtRadialFanReceipt) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "Open radial fan kept {} posture with {} incident faces and {} non-manifold edge.",
                    receipt.topology_posture_label(),
                    receipt.counters().incident_face_count(),
                    receipt.counters().non_manifold_edge_count()
                ),
                evidence_digest: receipt.fan_digest().to_string(),
                source_identity: receipt.workload_identity().to_string(),
            },
        }
    }

    pub fn from_nmt_radial_fan_denial(denial: &NmtRadialFanDenial) -> Self {
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "nmt-open-radial-fan-denial".to_string(),
                format!("{denial:?}"),
                denial.human_reason(),
            ],
        );
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: cause_kind(denial),
                message: denial.human_reason(),
                source_identity: evidence_digest.clone(),
                evidence_digest,
            },
        }
    }
}

fn cause_kind(denial: &NmtRadialFanDenial) -> WorthUserOutcomeCauseKind {
    match denial {
        NmtRadialFanDenial::ClosedManifoldLaunderingAttempt { .. } => {
            WorthUserOutcomeCauseKind::IntegrityMismatch
        }
        NmtRadialFanDenial::UnsupportedSurfaceFamily { .. } => {
            WorthUserOutcomeCauseKind::UnsupportedInput
        }
        NmtRadialFanDenial::DirtyInput { .. } => WorthUserOutcomeCauseKind::DirtyInput,
        NmtRadialFanDenial::PredicateUncertain { .. } => {
            WorthUserOutcomeCauseKind::PredicateUncertain
        }
        NmtRadialFanDenial::LabelOnlyMotion => WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
        _ => WorthUserOutcomeCauseKind::MissingEvidence,
    }
}
