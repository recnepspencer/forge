use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::{
    open_planar_posture::{
        OpenPlanarPostureCase, OpenPlanarPostureError, OpenPlanarPostureReceipt,
    },
    user_response::{
        source::WorthUserResponseSourceKind, WorthPolicyDecision, WorthUserOutcomeCauseKind,
        WorthUserResponseSource,
    },
};

impl WorthUserResponseSource {
    pub fn from_open_planar_posture_case(
        posture_case: OpenPlanarPostureCase,
        posture_identity: impl Into<String>,
    ) -> Self {
        let posture_identity = posture_identity.into();
        let message = message_for_case(posture_case);
        open_planar_posture_source(
            posture_case,
            message,
            posture_identity.clone(),
            posture_identity,
        )
    }

    pub fn from_open_planar_posture(receipt: &OpenPlanarPostureReceipt) -> Self {
        let message = message_for_case(receipt.posture_case());
        let evidence_digest = receipt.posture_digest().to_string();
        let source_identity = receipt.workload_identity().to_string();
        open_planar_posture_source(
            receipt.posture_case(),
            message,
            evidence_digest,
            source_identity,
        )
    }

    pub fn from_open_planar_posture_error(error: OpenPlanarPostureError) -> Self {
        let cause_kind = match error {
            OpenPlanarPostureError::BoundedSurrogateAttempted
            | OpenPlanarPostureError::CleanFailDidNotConsumeOpenTopology
            | OpenPlanarPostureError::UnsupportedSurfaceDidNotConsumeOpenTopology
            | OpenPlanarPostureError::MismatchedOutcomeCase { .. } => {
                WorthUserOutcomeCauseKind::IntegrityMismatch
            }
            OpenPlanarPostureError::MissingTransformPosture => {
                WorthUserOutcomeCauseKind::DeniedMovementOrRotation
            }
            OpenPlanarPostureError::TopologyWasNotOpen
            | OpenPlanarPostureError::SurfaceSupportWasAdmitted
            | OpenPlanarPostureError::CleanFailAttemptedBoundedConversion => {
                WorthUserOutcomeCauseKind::UnsupportedInput
            }
            OpenPlanarPostureError::MismatchedDiagnosticSubject => {
                WorthUserOutcomeCauseKind::MissingEvidence
            }
            _ => WorthUserOutcomeCauseKind::MissingEvidence,
        };
        let human_reason = error.human_reason();
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "open-planar-posture-error".to_string(),
                format!("{error:?}"),
                human_reason.clone(),
            ],
        );
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: human_reason,
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }
}

fn open_planar_posture_source(
    posture_case: OpenPlanarPostureCase,
    message: String,
    evidence_digest: String,
    source_identity: String,
) -> WorthUserResponseSource {
    match posture_case {
        OpenPlanarPostureCase::PolicyRequiredHalfSpace => WorthUserResponseSource {
            kind: WorthUserResponseSourceKind::PolicyRequired {
                message,
                evidence_digest,
                source_identity,
                choices: vec![WorthPolicyDecision::pause_for_manual_inspection()],
            },
        },
        OpenPlanarPostureCase::UnsupportedOpenSheet
        | OpenPlanarPostureCase::UnsupportedOpenWire
        | OpenPlanarPostureCase::BoundedOperatorIncompatibility => WorthUserResponseSource {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: WorthUserOutcomeCauseKind::UnsupportedInput,
                message,
                evidence_digest,
                source_identity,
            },
        },
        OpenPlanarPostureCase::PredicateUncertain => WorthUserResponseSource {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: WorthUserOutcomeCauseKind::PredicateUncertain,
                message,
                evidence_digest,
                source_identity,
            },
        },
        OpenPlanarPostureCase::IntegrityMismatch => WorthUserResponseSource {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: WorthUserOutcomeCauseKind::IntegrityMismatch,
                message,
                evidence_digest,
                source_identity,
            },
        },
        OpenPlanarPostureCase::TransformDivergence => WorthUserResponseSource {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
                message,
                evidence_digest,
                source_identity,
            },
        },
    }
}

fn message_for_case(posture_case: OpenPlanarPostureCase) -> String {
    match posture_case {
        OpenPlanarPostureCase::UnsupportedOpenSheet => {
            "Open sheet topology is real, but bounded boolean overlap is not admitted for open sheets in M6.5.".to_string()
        }
        OpenPlanarPostureCase::UnsupportedOpenWire => {
            "Open wire topology is real, but bounded boolean overlap is not admitted for open wires in M6.5.".to_string()
        }
        OpenPlanarPostureCase::PolicyRequiredHalfSpace => {
            "Half-space input needs an explicit interpretation policy before bounded overlap can continue.".to_string()
        }
        OpenPlanarPostureCase::PredicateUncertain => {
            "Predicate evidence is uncertain for this open planar posture, so no automatic option is available.".to_string()
        }
        OpenPlanarPostureCase::BoundedOperatorIncompatibility => {
            "Bounded planar operators cannot consume this open or unbounded posture in M6.5.".to_string()
        }
        OpenPlanarPostureCase::IntegrityMismatch => {
            "A finite bounded surrogate does not match the open planar truth and cannot be used.".to_string()
        }
        OpenPlanarPostureCase::TransformDivergence => {
            "Movement or rotation changes the open planar posture, so the transformed input is not equivalent.".to_string()
        }
    }
}
