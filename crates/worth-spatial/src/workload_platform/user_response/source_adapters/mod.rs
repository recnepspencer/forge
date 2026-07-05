mod boolean_readiness_workload;
mod dirty_planar_clean_fail;
mod grazing_basket_stack;
mod mixed_surface_kill_box;
mod nmt_radial_fan;
mod open_class_triad_parity;
mod open_planar_posture;
mod planar_boolean_outcome;
mod projection_fact_parity;
mod retained_cancellation;

pub use planar_boolean_outcome::{PlanarBooleanUserResponseClass, PlanarBooleanUserResponseSource};

use worth_math::sign::TriSign;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    choices::overlap_policy_choices, source::WorthUserResponseSourceKind, WorthPolicyDecision,
    WorthUserOutcomeCauseKind, WorthUserResponseSource,
};
use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityFactError;
use crate::planar_contracts::clean_fail_boundary::{
    PlanarCleanFailBoundaryReceipt, PlanarCleanFailClass,
};
use crate::planar_contracts::coplanar_overlap_contract::{
    CoplanarOverlapContractReceipt, CoplanarOverlapDenial, CoplanarOverlapDenialKind,
};
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::workload_platform::coplanar_overlap_storm::CoplanarOverlapStormWorkloadError;
use crate::workload_platform::high_valence_singularity::{
    HighValenceSingularityReceipt, HighValenceSingularityWorkloadError,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::CoplanarOverlapOperatorReceipt;
use crate::workload_platform::retained_replay_workload::{
    UnsupportedReplayReasonCode, UnsupportedReplayWorkload,
};
use crate::workload_platform::surface_support::UnsupportedSurfaceSupport;
use crate::workload_platform::thin_feature_scale_separation::{
    ThinFeatureScaleSeparationReceipt, ThinFeatureScaleSeparationWorkloadError,
};

impl WorthUserResponseSource {
    pub fn from_overlap_receipt(receipt: &CoplanarOverlapContractReceipt) -> Self {
        if let Some(exit) = receipt.policy_required_exits().first() {
            return Self {
                kind: WorthUserResponseSourceKind::PolicyRequired {
                    message: exit.reason().to_string(),
                    evidence_digest: exit.consumed_fact_digest().to_string(),
                    source_identity: receipt.fact_digest().to_string(),
                    choices: overlap_policy_choices(),
                },
            };
        }
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: "Overlap contracts were certified for this coplanar face pair."
                    .to_string(),
                evidence_digest: receipt.fact_digest().to_string(),
                source_identity: receipt.fact_digest().to_string(),
            },
        }
    }

    pub fn from_coplanar_overlap_operator(receipt: &CoplanarOverlapOperatorReceipt) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: "Coplanar overlap workload operator consumed platform evidence and certified the storm.".to_string(),
                evidence_digest: receipt.operator_digest().to_string(),
                source_identity: receipt.operator_digest().to_string(),
            },
        }
    }

    pub fn from_coplanar_overlap_storm_error(error: CoplanarOverlapStormWorkloadError) -> Self {
        let cause_kind = match error {
            CoplanarOverlapStormWorkloadError::MismatchedOperatorStageLink(_) => {
                WorthUserOutcomeCauseKind::IntegrityMismatch
            }
            _ => WorthUserOutcomeCauseKind::MissingEvidence,
        };
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "coplanar-overlap-storm-error".to_string(),
                format!("{error:?}"),
                error.human_reason(),
            ],
        );
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: error.human_reason(),
                source_identity: evidence_digest.clone(),
                evidence_digest,
            },
        }
    }

    pub fn from_high_valence_singularity(receipt: &HighValenceSingularityReceipt) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "High-valence singularity neighborhood was certified at valence {}.",
                    receipt.counters().neighborhood_valence()
                ),
                evidence_digest: receipt.singularity_digest().to_string(),
                source_identity: receipt.workload_identity().to_string(),
            },
        }
    }

    pub fn from_high_valence_singularity_error(error: HighValenceSingularityWorkloadError) -> Self {
        let cause_kind = match &error {
            HighValenceSingularityWorkloadError::UnsupportedValence { .. } => {
                WorthUserOutcomeCauseKind::UnsupportedInput
            }
            HighValenceSingularityWorkloadError::RebuildMotionIncompatible { .. } => {
                WorthUserOutcomeCauseKind::DeniedMovementOrRotation
            }
            HighValenceSingularityWorkloadError::PredicateUncertain => {
                WorthUserOutcomeCauseKind::PredicateUncertain
            }
            HighValenceSingularityWorkloadError::PolicyRequired => {
                WorthUserOutcomeCauseKind::PolicyRequired
            }
            HighValenceSingularityWorkloadError::IntegrityMismatch { .. } => {
                WorthUserOutcomeCauseKind::IntegrityMismatch
            }
            _ => WorthUserOutcomeCauseKind::MissingEvidence,
        };
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "high-valence-singularity-error".to_string(),
                format!("{error:?}"),
                error.human_reason(),
            ],
        );
        if cause_kind == WorthUserOutcomeCauseKind::PolicyRequired {
            return Self {
                kind: WorthUserResponseSourceKind::PolicyRequired {
                    message: error.human_reason(),
                    evidence_digest: evidence_digest.clone(),
                    source_identity: evidence_digest,
                    choices: vec![WorthPolicyDecision::PauseForManualInspection],
                },
            };
        }
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: error.human_reason(),
                source_identity: evidence_digest.clone(),
                evidence_digest,
            },
        }
    }

    pub fn from_thin_feature_scale_separation(receipt: &ThinFeatureScaleSeparationReceipt) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "Thin-feature scale separation was certified for {} topology-bound features across {} local scales.",
                    receipt.counters().thin_feature_count(),
                    receipt.counters().local_scale_order_count()
                ),
                evidence_digest: receipt.thin_feature_digest().to_string(),
                source_identity: receipt.workload_identity().to_string(),
            },
        }
    }

    pub fn from_thin_feature_scale_separation_error(
        error: ThinFeatureScaleSeparationWorkloadError,
    ) -> Self {
        let cause_kind = match &error {
            ThinFeatureScaleSeparationWorkloadError::UnsupportedTinyRotationPosture => {
                WorthUserOutcomeCauseKind::UnsupportedInput
            }
            ThinFeatureScaleSeparationWorkloadError::PredicateUncertain => {
                WorthUserOutcomeCauseKind::PredicateUncertain
            }
            ThinFeatureScaleSeparationWorkloadError::PolicyRequired => {
                WorthUserOutcomeCauseKind::PolicyRequired
            }
            ThinFeatureScaleSeparationWorkloadError::IntegrityMismatch { .. } => {
                WorthUserOutcomeCauseKind::IntegrityMismatch
            }
            _ => WorthUserOutcomeCauseKind::MissingEvidence,
        };
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "thin-feature-scale-separation-error".to_string(),
                format!("{error:?}"),
                error.human_reason(),
            ],
        );
        if cause_kind == WorthUserOutcomeCauseKind::PolicyRequired {
            return Self {
                kind: WorthUserResponseSourceKind::PolicyRequired {
                    message: error.human_reason(),
                    evidence_digest: evidence_digest.clone(),
                    source_identity: evidence_digest,
                    choices: vec![WorthPolicyDecision::PauseForManualInspection],
                },
            };
        }
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: error.human_reason(),
                source_identity: evidence_digest.clone(),
                evidence_digest,
            },
        }
    }

    pub fn from_clean_fail_boundary(receipt: &PlanarCleanFailBoundaryReceipt) -> Self {
        let (cause_kind, message) = clean_fail_response_cause_and_message(receipt);
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message,
                evidence_digest: receipt.clean_fail_boundary_digest().to_string(),
                source_identity: receipt.clean_fail_boundary_digest().to_string(),
            },
        }
    }

    pub fn from_overlap_denial(
        denial: &CoplanarOverlapDenial,
        diagnostic: &PlanarDiagnosticBundleReceipt,
    ) -> Self {
        let cause_kind = match denial.kind() {
            CoplanarOverlapDenialKind::MismatchedMovementRotationPosture => {
                WorthUserOutcomeCauseKind::DeniedMovementOrRotation
            }
            _ => WorthUserOutcomeCauseKind::OverlapDenied,
        };
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: denial.reason().to_string(),
                evidence_digest: diagnostic.diagnostic_bundle_digest().to_string(),
                source_identity: diagnostic.diagnostic_bundle_digest().to_string(),
            },
        }
    }

    pub fn from_predicate_authority_error(error: &PlanarPredicateAuthorityFactError) -> Self {
        match error {
            PlanarPredicateAuthorityFactError::PredicateUncertain {
                certified_sign,
                counters,
                ..
            } => Self {
                kind: WorthUserResponseSourceKind::NoOptions {
                    cause_kind: WorthUserOutcomeCauseKind::PredicateUncertain,
                    message: predicate_uncertainty_message(certified_sign.sign()).to_string(),
                    evidence_digest: truth_digest_parts(
                        TruthDigestScope::ArtifactIdentity,
                        &[
                            "predicate-uncertain-before-overlap".to_string(),
                            format!("sign:{:?}", certified_sign.sign()),
                            format!("input_points:{}", counters.input_point_count()),
                            format!("basis_parts:{}", counters.canonical_basis_part_count()),
                        ],
                    ),
                    source_identity: "predicate-authority-error".to_string(),
                },
            },
            PlanarPredicateAuthorityFactError::PredicateEvaluation { reason, .. } => {
                Self::from_predicate_evaluation_failure(reason)
            }
            PlanarPredicateAuthorityFactError::OutcomeNotBound { reason, .. } => Self {
                kind: WorthUserResponseSourceKind::NoOptions {
                    cause_kind: WorthUserOutcomeCauseKind::PredicateAuthorityNotBound,
                    message: reason.clone(),
                    evidence_digest: truth_digest_parts(
                        TruthDigestScope::ArtifactIdentity,
                        &[
                            "predicate-not-bound-before-overlap".to_string(),
                            reason.clone(),
                        ],
                    ),
                    source_identity: "predicate-authority-error".to_string(),
                },
            },
        }
    }

    pub fn from_unsupported_surface_support(unsupported: &UnsupportedSurfaceSupport) -> Self {
        let evidence_digest = unsupported
            .receipt()
            .map(|receipt| receipt.stage_identity().receipt_identity())
            .unwrap_or_else(|| unsupported_surface_support_posture_digest(unsupported));
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: WorthUserOutcomeCauseKind::UnsupportedInput,
                message: unsupported.human_reason().to_string(),
                source_identity: evidence_digest.clone(),
                evidence_digest,
            },
        }
    }

    pub fn from_unsupported_replay(unsupported: &UnsupportedReplayWorkload) -> Self {
        let cause_kind = match unsupported.reason_code() {
            UnsupportedReplayReasonCode::RetainedProjectionDrift => {
                WorthUserOutcomeCauseKind::IntegrityMismatch
            }
            _ => WorthUserOutcomeCauseKind::MissingEvidence,
        };
        let evidence_digest = unsupported_replay_posture_digest(unsupported);
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind,
                message: unsupported.human_reason().to_string(),
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }

    fn from_predicate_evaluation_failure(reason: &str) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: WorthUserOutcomeCauseKind::PredicateEvaluationFailed,
                message: reason.to_string(),
                evidence_digest: truth_digest_parts(
                    TruthDigestScope::ArtifactIdentity,
                    &[
                        "predicate-evaluation-before-overlap".to_string(),
                        reason.to_string(),
                    ],
                ),
                source_identity: "predicate-authority-error".to_string(),
            },
        }
    }
}

fn clean_fail_response_cause_and_message(
    receipt: &PlanarCleanFailBoundaryReceipt,
) -> (WorthUserOutcomeCauseKind, String) {
    match receipt.class() {
        PlanarCleanFailClass::DirtyInput => (
            WorthUserOutcomeCauseKind::DirtyInput,
            format!(
                "Input is {}; inspect topology before boolean certification.",
                receipt.basis().input().source_detail().replace('-', " ")
            ),
        ),
        PlanarCleanFailClass::UnboundedOrOpen => (
            WorthUserOutcomeCauseKind::UnsupportedInput,
            format!(
                "{} is unsupported for bounded boolean overlap.",
                capitalize(receipt.basis().input().source_detail())
            ),
        ),
    }
}

fn predicate_uncertainty_message(sign: TriSign) -> &'static str {
    match sign {
        TriSign::Zero => {
            "Exact predicate found collinear points; repair or choose policy before boolean certification."
        }
        TriSign::Pos | TriSign::Neg => {
            "Predicate authority could not bind certification; inspect predicate evidence before boolean certification."
        }
    }
}

fn unsupported_surface_support_posture_digest(unsupported: &UnsupportedSurfaceSupport) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "unsupported-surface-support-posture".to_string(),
            format!("reason:{:?}", unsupported.reason_code()),
            format!("family:{:?}", unsupported.family()),
            unsupported.posture().reason().to_string(),
        ],
    )
}

fn unsupported_replay_posture_digest(unsupported: &UnsupportedReplayWorkload) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "unsupported-retained-replay-posture".to_string(),
            format!("reason:{:?}", unsupported.reason_code()),
            unsupported.posture().reason().to_string(),
        ],
    )
}

fn capitalize(value: &str) -> String {
    let value = value.replace('-', " ");
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => value,
    }
}
