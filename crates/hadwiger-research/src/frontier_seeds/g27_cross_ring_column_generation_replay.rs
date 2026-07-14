use crate::candidate_screening::ScreeningRational;
use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_cross_ring_fusion_preflight::{
    preflight_g27_cross_ring_fusion_column_generation_checked, G27CrossRingFusionPreflightPosture,
};
use super::g27_geometric_fractional::G27GeometricFractionalError;

const G27_LOWER_BOUND_NUMERATOR: i128 = 4;
const G27_LOWER_BOUND_DENOMINATOR: i128 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27CrossRingColumnGenerationReplayPosture {
    RetiredAsymptoticDischargingCore,
    RetiredSelectedCoreTooWeak,
    BlockedByPreflightSuppression,
}

impl G27CrossRingColumnGenerationReplayPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RetiredAsymptoticDischargingCore => "retired_asymptotic_discharging_core",
            Self::RetiredSelectedCoreTooWeak => "retired_selected_core_too_weak",
            Self::BlockedByPreflightSuppression => "blocked_by_preflight_suppression",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27CrossRingPricingObligation {
    obligation_kind: String,
    detail: String,
}

impl G27CrossRingPricingObligation {
    pub fn obligation_kind(&self) -> &str {
        &self.obligation_kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    fn stable_token(&self) -> String {
        format!("{}:{}", self.obligation_kind, self.detail)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27CrossRingColumnGenerationReplayReport {
    core: HadwigerArtifactCore,
    selected_core_label: String,
    foreign_radicand: u32,
    shared_vertex: String,
    retained_g27_lower_bound: ScreeningRational,
    foreign_core_lower_bound: ScreeningRational,
    lift_needed_to_beat_g27: ScreeningRational,
    pricing_obligations: Vec<G27CrossRingPricingObligation>,
    posture: G27CrossRingColumnGenerationReplayPosture,
    conclusion: String,
}

impl G27CrossRingColumnGenerationReplayReport {
    pub fn selected_core_label(&self) -> &str {
        &self.selected_core_label
    }

    pub fn foreign_radicand(&self) -> u32 {
        self.foreign_radicand
    }

    pub fn shared_vertex(&self) -> &str {
        &self.shared_vertex
    }

    pub fn retained_g27_lower_bound(&self) -> &ScreeningRational {
        &self.retained_g27_lower_bound
    }

    pub fn foreign_core_lower_bound(&self) -> &ScreeningRational {
        &self.foreign_core_lower_bound
    }

    pub fn lift_needed_to_beat_g27(&self) -> &ScreeningRational {
        &self.lift_needed_to_beat_g27
    }

    pub fn pricing_obligations(&self) -> &[G27CrossRingPricingObligation] {
        &self.pricing_obligations
    }

    pub fn posture(&self) -> G27CrossRingColumnGenerationReplayPosture {
        self.posture
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27CrossRingColumnGenerationReplayReport, core);

pub fn replay_g27_cross_ring_column_generation_state_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27CrossRingColumnGenerationReplayReport, G27GeometricFractionalError> {
    let preflight = preflight_g27_cross_ring_fusion_column_generation_checked(handle)?;
    let selected = preflight.selected_candidate();
    let retained_g27_lower_bound =
        ScreeningRational::fraction(G27_LOWER_BOUND_NUMERATOR, G27_LOWER_BOUND_DENOMINATOR)?;
    let foreign_core_lower_bound = parse_core_lower_bound(selected.core_label())?;
    let lift_needed_to_beat_g27 = retained_g27_lower_bound.sub(&foreign_core_lower_bound);
    let pricing_obligations =
        pricing_obligations(selected.core_label(), selected.foreign_radicand());
    let posture = if preflight.posture() != G27CrossRingFusionPreflightPosture::FundColumnGeneration
    {
        G27CrossRingColumnGenerationReplayPosture::BlockedByPreflightSuppression
    } else if is_asymptotic_discharging_core(selected.core_label()) {
        G27CrossRingColumnGenerationReplayPosture::RetiredAsymptoticDischargingCore
    } else if !lift_needed_to_beat_g27.is_positive() {
        G27CrossRingColumnGenerationReplayPosture::RetiredSelectedCoreTooWeak
    } else {
        G27CrossRingColumnGenerationReplayPosture::RetiredSelectedCoreTooWeak
    };
    let conclusion = conclusion(posture, selected.core_label(), &lift_needed_to_beat_g27);
    let core = artifact_core(
        HadwigerArtifactKind::G27CrossRingColumnGenerationReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_cross_ring_column_generation_state_replay".to_string(),
        },
        vec![preflight.reference()],
        payload(
            selected.core_label(),
            selected.foreign_radicand(),
            selected.shared_vertex(),
            &retained_g27_lower_bound,
            &foreign_core_lower_bound,
            &lift_needed_to_beat_g27,
            &pricing_obligations,
            posture,
            &conclusion,
        ),
    )?;
    Ok(G27CrossRingColumnGenerationReplayReport {
        core,
        selected_core_label: selected.core_label().to_string(),
        foreign_radicand: selected.foreign_radicand(),
        shared_vertex: selected.shared_vertex().to_string(),
        retained_g27_lower_bound,
        foreign_core_lower_bound,
        lift_needed_to_beat_g27,
        pricing_obligations,
        posture,
        conclusion,
    })
}

fn parse_core_lower_bound(label: &str) -> Result<ScreeningRational, G27GeometricFractionalError> {
    let parts = label.split('_').collect::<Vec<_>>();
    if parts.len() < 2 {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "cross_ring_core_label",
        });
    }
    let numerator =
        parts[0]
            .parse::<i128>()
            .map_err(|_| G27GeometricFractionalError::MalformedData {
                source: "cross_ring_core_numerator",
            })?;
    let denominator =
        parts[1]
            .parse::<i128>()
            .map_err(|_| G27GeometricFractionalError::MalformedData {
                source: "cross_ring_core_denominator",
            })?;
    ScreeningRational::fraction(numerator, denominator).map_err(Into::into)
}

fn pricing_obligations(core_label: &str, radicand: u32) -> Vec<G27CrossRingPricingObligation> {
    vec![
        G27CrossRingPricingObligation {
            obligation_kind: "foreign_core_columns".to_string(),
            detail: format!("retain independent-set dictionary for {core_label}"),
        },
        G27CrossRingPricingObligation {
            obligation_kind: "cross_ring_constraints".to_string(),
            detail: format!("replay exact sqrt{radicand} cross-unit contacts at shared vertex 8"),
        },
        G27CrossRingPricingObligation {
            obligation_kind: "master_dual_certificate".to_string(),
            detail: "produce rational dual certificate with lift strictly above retained G27"
                .to_string(),
        },
        G27CrossRingPricingObligation {
            obligation_kind: "pricing_oracle".to_string(),
            detail: "independently replay that no omitted fused independent set violates the dual"
                .to_string(),
        },
    ]
}

fn is_asymptotic_discharging_core(core_label: &str) -> bool {
    core_label == "76_21_fractional_core"
}

fn conclusion(
    posture: G27CrossRingColumnGenerationReplayPosture,
    core_label: &str,
    lift: &ScreeningRational,
) -> String {
    match posture {
        G27CrossRingColumnGenerationReplayPosture::RetiredAsymptoticDischargingCore => format!(
            "{core_label} is an asymptotic Cranston-Rabern discharging construction, not a retained finite column core; retire this fusion target after exact lift-gap audit {}",
            lift.stable_token()
        ),
        G27CrossRingColumnGenerationReplayPosture::RetiredSelectedCoreTooWeak => {
            "selected core does not require positive lift; retire malformed comparison".to_string()
        }
        G27CrossRingColumnGenerationReplayPosture::BlockedByPreflightSuppression => {
            "preflight did not fund column generation".to_string()
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn payload(
    core_label: &str,
    foreign_radicand: u32,
    shared_vertex: &str,
    g27_bound: &ScreeningRational,
    core_bound: &ScreeningRational,
    lift: &ScreeningRational,
    obligations: &[G27CrossRingPricingObligation],
    posture: G27CrossRingColumnGenerationReplayPosture,
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.g27_cross_ring_column_generation.v1",
        ),
        HadwigerArtifactPayloadEntry::text("selected_core", core_label),
        HadwigerArtifactPayloadEntry::unsigned("foreign_radicand", foreign_radicand as u128),
        HadwigerArtifactPayloadEntry::text("shared_vertex", shared_vertex),
        HadwigerArtifactPayloadEntry::text("retained_g27_lower_bound", g27_bound.stable_token()),
        HadwigerArtifactPayloadEntry::text("foreign_core_lower_bound", core_bound.stable_token()),
        HadwigerArtifactPayloadEntry::text("lift_needed_to_beat_g27", lift.stable_token()),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for obligation in obligations {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "pricing_obligation",
            obligation.stable_token(),
        ));
    }
    payload
}
