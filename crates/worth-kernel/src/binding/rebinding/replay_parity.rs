use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::bindings::{BindingContinuityClass, RebindingOutcomeClass};

use crate::binding::rebinding::{
    ordinary_shape_from_rebinding_decision, PrimitiveRebindingBranchLocalInspection,
    PrimitiveRebindingDeclarationEntry, PrimitiveRebindingHistoricalInspection,
    PrimitiveRebindingQueryDomain,
};

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) enum PrimitiveRebindingReplaySource {
    Historical(PrimitiveRebindingHistoricalInspection),
    BranchLocal(PrimitiveRebindingBranchLocalInspection),
}

impl PrimitiveRebindingReplaySource {
    fn decision(&self) -> &worth_spatial::facade::bindings::AdmittedRebindingDecision {
        match self {
            Self::Historical(value) => value.decision(),
            Self::BranchLocal(value) => value.decision(),
        }
    }

    fn source_kind(&self) -> &'static str {
        match self {
            Self::Historical(_) => "historical",
            Self::BranchLocal(_) => "branch_local",
        }
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct PrimitiveRebindingReplayParity {
    replay_digest: String,
    binding_identity: String,
    anchor_identity: String,
    outcome_class: RebindingOutcomeClass,
    continuity_class: BindingContinuityClass,
    selected_candidate_identity: Option<String>,
    selected_candidate_label: Option<String>,
    unsupported_reason: String,
    next_step: Option<ForgeQueryOrdinaryNextStep>,
    ordinary_kind: &'static str,
    left_source_kind: &'static str,
    right_source_kind: &'static str,
}

#[cfg_attr(not(test), allow(dead_code))]
impl PrimitiveRebindingReplayParity {
    pub(crate) fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub(crate) fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub(crate) fn anchor_identity(&self) -> &str {
        &self.anchor_identity
    }

    pub(crate) fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class
    }

    pub(crate) fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub(crate) fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub(crate) fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }

    pub(crate) fn unsupported_reason(&self) -> &str {
        &self.unsupported_reason
    }

    pub(crate) fn next_step(&self) -> Option<ForgeQueryOrdinaryNextStep> {
        self.next_step
    }

    pub(crate) fn ordinary_kind(&self) -> &str {
        self.ordinary_kind
    }

    pub(crate) fn left_source_kind(&self) -> &str {
        self.left_source_kind
    }

    pub(crate) fn right_source_kind(&self) -> &str {
        self.right_source_kind
    }
}

#[allow(dead_code)]
pub(crate) enum PrimitiveRebindingReplayParityError {
    RetainedIdentityMismatch {
        reason: &'static str,
    },
    ExplanationBasisMismatch {
        reason: &'static str,
    },
    ReplayNextStepMismatch {
        reason: &'static str,
        left_kind: &'static str,
        right_kind: &'static str,
        left_next_step: Option<ForgeQueryOrdinaryNextStep>,
        right_next_step: Option<ForgeQueryOrdinaryNextStep>,
        left_posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
        right_posture_kind: Option<ForgeQueryOrdinaryPostureKind>,
    },
}

impl PrimitiveRebindingReplayParityError {
    pub(crate) fn reason(&self) -> &'static str {
        match self {
            Self::RetainedIdentityMismatch { reason }
            | Self::ExplanationBasisMismatch { reason }
            | Self::ReplayNextStepMismatch { reason, .. } => reason,
        }
    }
}

impl std::fmt::Debug for PrimitiveRebindingReplayParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveRebindingReplayParityError")
            .field("reason", &self.reason())
            .finish()
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn primitive_rebinding_replay_parity<C>(
    left_entry: &PrimitiveRebindingDeclarationEntry,
    left_source: PrimitiveRebindingReplaySource,
    right_entry: &PrimitiveRebindingDeclarationEntry,
    right_source: PrimitiveRebindingReplaySource,
    _handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveRebindingQueryDomain, C>,
) -> Result<PrimitiveRebindingReplayParity, PrimitiveRebindingReplayParityError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveRebindingQueryDomain>,
{
    let left_decision = left_source.decision();
    let right_decision = right_source.decision();
    let left_explanation = left_decision.explanation();
    let right_explanation = right_decision.explanation();

    ensure_equal(
        format!("{:?}", left_entry.binding_kind()),
        format!("{:?}", right_entry.binding_kind()),
        "replay parity requires retained histories to preserve the same binding family identity",
    )?;
    ensure_equal(
        left_explanation.prior_identity(),
        right_explanation.prior_identity(),
        "replay parity requires retained histories to preserve the same binding identity",
    )?;
    ensure_equal(
        left_explanation.prior_site_identity(),
        right_explanation.prior_site_identity(),
        "replay parity requires retained histories to preserve the same anchor identity",
    )?;
    ensure_equal(
        format!("{:?}", left_decision.outcome_class()),
        format!("{:?}", right_decision.outcome_class()),
        "replay parity requires equivalent retained histories to preserve rebinding outcome class",
    )?;
    ensure_equal(
        format!("{:?}", left_explanation.continuity_class()),
        format!("{:?}", right_explanation.continuity_class()),
        "replay parity requires equivalent retained histories to preserve continuity class",
    )?;
    ensure_equal(
        format!("{:?}", left_explanation.motion_posture()),
        format!("{:?}", right_explanation.motion_posture()),
        "replay parity requires equivalent retained histories to preserve motion posture",
    )?;
    ensure_equal(
        format!("{:?}", left_explanation.neighborhood_family()),
        format!("{:?}", right_explanation.neighborhood_family()),
        "replay parity requires equivalent retained histories to preserve binding family",
    )?;
    ensure_equal(
        left_explanation.selected_candidate_identity().unwrap_or("none"),
        right_explanation.selected_candidate_identity().unwrap_or("none"),
        "replay parity requires equivalent retained histories to preserve selected binding identity",
    )?;
    ensure_equal(
        left_explanation.selected_candidate_label().unwrap_or("none"),
        right_explanation.selected_candidate_label().unwrap_or("none"),
        "replay parity requires equivalent retained histories to preserve selected binding explanation",
    )?;
    ensure_equal(
        format!("{:?}", left_explanation.unsupported_reason()),
        format!("{:?}", right_explanation.unsupported_reason()),
        "replay parity requires equivalent retained histories to preserve diagnostics posture",
    )?;
    ensure_equal(
        sorted_join(left_explanation.candidate_identities()),
        sorted_join(right_explanation.candidate_identities()),
        "replay parity requires equivalent retained histories to preserve candidate identity explanation basis",
    )?;
    ensure_equal(
        sorted_join(left_explanation.candidate_labels()),
        sorted_join(right_explanation.candidate_labels()),
        "replay parity requires equivalent retained histories to preserve candidate label explanation basis",
    )?;
    ensure_equal(
        sorted_join(left_explanation.candidate_site_identities()),
        sorted_join(right_explanation.candidate_site_identities()),
        "replay parity requires equivalent retained histories to preserve candidate anchor explanation basis",
    )?;

    let left_ordinary = ordinary_shape_from_rebinding_decision(left_decision);
    let right_ordinary = ordinary_shape_from_rebinding_decision(right_decision);
    if left_ordinary != right_ordinary {
        return Err(PrimitiveRebindingReplayParityError::ReplayNextStepMismatch {
            reason:
                "replay parity requires retained histories to preserve the same ordinary next-step truth",
            left_kind: left_ordinary.kind(),
            right_kind: right_ordinary.kind(),
            left_next_step: left_ordinary.next_step(),
            right_next_step: right_ordinary.next_step(),
            left_posture_kind: left_ordinary.posture_kind(),
            right_posture_kind: right_ordinary.posture_kind(),
        });
    }

    Ok(PrimitiveRebindingReplayParity {
        replay_digest: truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("binding_kind:{:?}", left_entry.binding_kind()),
                format!("outcome:{:?}", left_decision.outcome_class()),
                format!("continuity:{:?}", left_explanation.continuity_class()),
                format!("motion:{:?}", left_explanation.motion_posture()),
                format!("family:{:?}", left_explanation.neighborhood_family()),
                format!("prior:{}", left_explanation.prior_identity()),
                format!("prior_site:{}", left_explanation.prior_site_identity()),
                format!(
                    "selected_identity:{}",
                    left_explanation
                        .selected_candidate_identity()
                        .unwrap_or("none")
                ),
                format!(
                    "selected_label:{}",
                    left_explanation
                        .selected_candidate_label()
                        .unwrap_or("none")
                ),
                format!("unsupported:{:?}", left_explanation.unsupported_reason()),
                format!(
                    "candidate_identities:{}",
                    sorted_join(left_explanation.candidate_identities())
                ),
                format!(
                    "candidate_labels:{}",
                    sorted_join(left_explanation.candidate_labels())
                ),
                format!(
                    "candidate_sites:{}",
                    sorted_join(left_explanation.candidate_site_identities())
                ),
                format!("ordinary_kind:{}", left_ordinary.kind()),
                format!("ordinary_posture:{:?}", left_ordinary.posture_kind()),
                format!("next_step:{:?}", left_ordinary.next_step()),
            ],
        ),
        binding_identity: left_explanation.prior_identity().to_string(),
        anchor_identity: left_explanation.prior_site_identity().to_string(),
        outcome_class: left_decision.outcome_class(),
        continuity_class: left_explanation.continuity_class().clone(),
        selected_candidate_identity: left_explanation
            .selected_candidate_identity()
            .map(ToOwned::to_owned),
        selected_candidate_label: left_explanation
            .selected_candidate_label()
            .map(ToOwned::to_owned),
        unsupported_reason: format!("{:?}", left_explanation.unsupported_reason()),
        next_step: left_ordinary.next_step(),
        ordinary_kind: left_ordinary.kind(),
        left_source_kind: left_source.source_kind(),
        right_source_kind: right_source.source_kind(),
    })
}

fn ensure_equal<T: Eq>(
    left: T,
    right: T,
    reason: &'static str,
) -> Result<(), PrimitiveRebindingReplayParityError> {
    if left == right {
        Ok(())
    } else if reason.contains("explanation")
        || reason.contains("diagnostics")
        || reason.contains("candidate")
    {
        Err(PrimitiveRebindingReplayParityError::ExplanationBasisMismatch { reason })
    } else {
        Err(PrimitiveRebindingReplayParityError::RetainedIdentityMismatch { reason })
    }
}

fn sorted_join(values: &[String]) -> String {
    let mut values = values.to_vec();
    values.sort();
    values.join("|")
}
