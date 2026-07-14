use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::mathematical_verification::ExactRational;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_geometric_fractional_lead_report::{
    materialize_g27_pressure_escape_lead_checked, G27PressureEscapeLeadReport,
};
use super::g27_moser_anchor_scan::{
    scan_g27_row_685_moser_anchor_breakers_checked, G27MoserAnchorScanReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27OutsideMoserAnchorPosture {
    SuppressedInsideMoser,
    ShapeCheckedOutsideMoser,
}

impl G27OutsideMoserAnchorPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SuppressedInsideMoser => "suppressed_inside_moser",
            Self::ShapeCheckedOutsideMoser => "shape_checked_outside_moser",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27OutsideMoserAxis {
    X,
    Y,
}

impl G27OutsideMoserAxis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::X => "x",
            Self::Y => "y",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27QuadraticAnchorExtension {
    axis: G27OutsideMoserAxis,
    radicand: i128,
    coefficient: ExactRational,
}

impl G27QuadraticAnchorExtension {
    pub fn new(
        axis: G27OutsideMoserAxis,
        radicand: i128,
        coefficient: ExactRational,
    ) -> Result<Self, G27GeometricFractionalError> {
        if radicand <= 1 || !is_squarefree(radicand as u128) || coefficient.is_zero_public() {
            return Err(G27GeometricFractionalError::MalformedData {
                source: "quadratic_anchor_extension",
            });
        }
        Ok(Self {
            axis,
            radicand,
            coefficient,
        })
    }

    pub fn axis(&self) -> G27OutsideMoserAxis {
        self.axis
    }

    pub fn radicand(&self) -> i128 {
        self.radicand
    }

    pub fn coefficient(&self) -> &ExactRational {
        &self.coefficient
    }

    pub(crate) fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.axis.as_str(),
            self.radicand,
            self.coefficient.stable_token_public()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27OutsideMoserAnchorCandidate {
    anchor_id: String,
    moser_coefficients: [i32; 4],
    extension: Option<G27QuadraticAnchorExtension>,
}

impl G27OutsideMoserAnchorCandidate {
    pub fn moser_basis(
        anchor_id: impl Into<String>,
        moser_coefficients: [i32; 4],
    ) -> Result<Self, G27GeometricFractionalError> {
        Ok(Self {
            anchor_id: non_empty(anchor_id, "anchor_id")?,
            moser_coefficients,
            extension: None,
        })
    }

    pub fn quadratic_extension(
        anchor_id: impl Into<String>,
        moser_coefficients: [i32; 4],
        extension: G27QuadraticAnchorExtension,
    ) -> Result<Self, G27GeometricFractionalError> {
        Ok(Self {
            anchor_id: non_empty(anchor_id, "anchor_id")?,
            moser_coefficients,
            extension: Some(extension),
        })
    }

    pub fn anchor_id(&self) -> &str {
        &self.anchor_id
    }

    pub fn moser_coefficients(&self) -> [i32; 4] {
        self.moser_coefficients
    }

    pub fn is_outside_moser(&self) -> bool {
        self.extension.is_some()
    }

    pub fn extension(&self) -> Option<&G27QuadraticAnchorExtension> {
        self.extension.as_ref()
    }

    pub(crate) fn stable_token(&self) -> String {
        let [a, b, c, d] = self.moser_coefficients;
        let extension = self
            .extension
            .as_ref()
            .map(G27QuadraticAnchorExtension::stable_token)
            .unwrap_or_else(|| "inside_moser".to_string());
        format!("{}:{a}:{b}:{c}:{d}:{extension}", self.anchor_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27OutsideMoserAnchorReplayReport {
    core: HadwigerArtifactCore,
    source_lead: G27PressureEscapeLeadReport,
    moser_scan: G27MoserAnchorScanReport,
    candidate: G27OutsideMoserAnchorCandidate,
    posture: G27OutsideMoserAnchorPosture,
    reason: String,
}

impl G27OutsideMoserAnchorReplayReport {
    pub fn source_lead(&self) -> &G27PressureEscapeLeadReport {
        &self.source_lead
    }

    pub fn moser_scan(&self) -> &G27MoserAnchorScanReport {
        &self.moser_scan
    }

    pub fn candidate(&self) -> &G27OutsideMoserAnchorCandidate {
        &self.candidate
    }

    pub fn posture(&self) -> G27OutsideMoserAnchorPosture {
        self.posture
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27OutsideMoserAnchorReplayReport, core);

pub fn replay_g27_outside_moser_anchor_checked(
    handle: &HadwigerResearchHandle,
    candidate: G27OutsideMoserAnchorCandidate,
) -> Result<G27OutsideMoserAnchorReplayReport, G27GeometricFractionalError> {
    let source_lead = materialize_g27_pressure_escape_lead_checked(handle)?;
    let moser_scan = scan_g27_row_685_moser_anchor_breakers_checked(handle)?;
    let (posture, reason) = if candidate.is_outside_moser() {
        (
            G27OutsideMoserAnchorPosture::ShapeCheckedOutsideMoser,
            "quadratic extension is exact and outside the retained Moser basis; unit attachments are not admitted by this shape replay"
                .to_string(),
        )
    } else {
        (
            G27OutsideMoserAnchorPosture::SuppressedInsideMoser,
            "candidate has only retained Moser-basis coefficients and is capped by the Moser suppression scan"
                .to_string(),
        )
    };
    let core = artifact_core(
        HadwigerArtifactKind::G27OutsideMoserAnchorReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_outside_moser_anchor_replay".to_string(),
        },
        vec![source_lead.reference(), moser_scan.reference()],
        replay_payload(&candidate, posture, &reason),
    )?;
    Ok(G27OutsideMoserAnchorReplayReport {
        core,
        source_lead,
        moser_scan,
        candidate,
        posture,
        reason,
    })
}

fn replay_payload(
    candidate: &G27OutsideMoserAnchorCandidate,
    posture: G27OutsideMoserAnchorPosture,
    reason: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_anchor_replay.v1"),
        HadwigerArtifactPayloadEntry::text("candidate", candidate.stable_token()),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
        HadwigerArtifactPayloadEntry::text("reason", reason),
    ]
}

fn non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, G27GeometricFractionalError> {
    let value = value.into();
    if value.trim().is_empty() {
        Err(G27GeometricFractionalError::Artifact(
            HadwigerArtifactShapeError::EmptyField { field },
        ))
    } else {
        Ok(value)
    }
}

fn is_squarefree(value: u128) -> bool {
    let mut divisor = 2;
    while divisor * divisor <= value {
        if value % (divisor * divisor) == 0 {
            return false;
        }
        divisor += 1;
    }
    true
}

trait PublicExactRationalAccess {
    fn is_zero_public(&self) -> bool;
    fn stable_token_public(&self) -> String;
}

impl PublicExactRationalAccess for ExactRational {
    fn is_zero_public(&self) -> bool {
        self.stable_token() == "0/1"
    }

    fn stable_token_public(&self) -> String {
        self.stable_token()
    }
}
