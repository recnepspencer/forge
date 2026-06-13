use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::mixed_surface_kill_box::MixedSurfaceKillBoxDenial;
use crate::workload_platform::surface_support::SurfaceSupportReceiptSet;
use crate::workload_platform::user_response::{
    source::WorthUserResponseSourceKind, WorthUserOutcomeCauseKind, WorthUserResponseSource,
};

impl WorthUserResponseSource {
    pub fn from_mixed_surface_plane_support(receipt: &SurfaceSupportReceiptSet) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: "Plane surface support is certified and acceptable as pre-boolean input."
                    .to_string(),
                evidence_digest: receipt.stage_identity().receipt_identity(),
                source_identity: receipt.upstream_geometry_binding_identity().to_string(),
            },
        }
    }

    pub fn from_mixed_surface_kill_box_denial(denial: &MixedSurfaceKillBoxDenial) -> Self {
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "mixed-surface-kill-box-denial".to_string(),
                format!("{denial:?}"),
                denial.human_reason(),
            ],
        );
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: cause_kind(denial),
                message: denial.human_reason(),
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }
}

fn cause_kind(denial: &MixedSurfaceKillBoxDenial) -> WorthUserOutcomeCauseKind {
    match denial {
        MixedSurfaceKillBoxDenial::SurfaceFamilyReceiptMismatch { .. }
        | MixedSurfaceKillBoxDenial::KernelSummarySubstitution
        | MixedSurfaceKillBoxDenial::WrongFamilyUserResponse { .. } => {
            WorthUserOutcomeCauseKind::IntegrityMismatch
        }
        MixedSurfaceKillBoxDenial::UnsupportedFamilyReadinessAttempt { .. } => {
            WorthUserOutcomeCauseKind::UnsupportedInput
        }
        MixedSurfaceKillBoxDenial::GeneratedFeatureSmugglingAttempt => {
            WorthUserOutcomeCauseKind::OverlapDenied
        }
        _ => WorthUserOutcomeCauseKind::MissingEvidence,
    }
}
