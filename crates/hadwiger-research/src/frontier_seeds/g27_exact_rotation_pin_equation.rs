use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_rotation_pin_closure_search::search_g27_rotation_pin_closures_checked;

const HINGE_VERTEX: &str = "8";
const WITNESS_VERTEX: &str = "10";
const PIN_VERTEX: &str = "27";
const CLOSURE_PAIR_COUNT: usize = 9;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27ExactRotationPinEquationPosture {
    ManufacturedFieldExtensionRequired,
}

impl G27ExactRotationPinEquationPosture {
    pub fn as_str(self) -> &'static str {
        "manufactured_field_extension_required"
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27ExactRotationPinEquationReport {
    core: HadwigerArtifactCore,
    hinge_vertex: String,
    witness_vertex: String,
    pin_vertex: String,
    closure_pair_count: usize,
    moving_radius_squared: String,
    pin_distance_squared: String,
    witness_pin_dot: String,
    rotated_pin_dot: String,
    height_numerator: String,
    required_extension: String,
    closure_replay_obligation: String,
    posture: G27ExactRotationPinEquationPosture,
}

impl G27ExactRotationPinEquationReport {
    pub fn hinge_vertex(&self) -> &str {
        &self.hinge_vertex
    }

    pub fn witness_vertex(&self) -> &str {
        &self.witness_vertex
    }

    pub fn pin_vertex(&self) -> &str {
        &self.pin_vertex
    }

    pub fn closure_pair_count(&self) -> usize {
        self.closure_pair_count
    }

    pub fn moving_radius_squared(&self) -> &str {
        &self.moving_radius_squared
    }

    pub fn pin_distance_squared(&self) -> &str {
        &self.pin_distance_squared
    }

    pub fn witness_pin_dot(&self) -> &str {
        &self.witness_pin_dot
    }

    pub fn rotated_pin_dot(&self) -> &str {
        &self.rotated_pin_dot
    }

    pub fn height_numerator(&self) -> &str {
        &self.height_numerator
    }

    pub fn required_extension(&self) -> &str {
        &self.required_extension
    }

    pub fn closure_replay_obligation(&self) -> &str {
        &self.closure_replay_obligation
    }

    pub fn posture(&self) -> G27ExactRotationPinEquationPosture {
        self.posture
    }

    pub fn requires_new_field_extension(&self) -> bool {
        self.posture == G27ExactRotationPinEquationPosture::ManufacturedFieldExtensionRequired
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }

    pub fn registers_query_invariant_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(G27ExactRotationPinEquationReport, core);

pub fn derive_g27_exact_rotation_pin_equation_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27ExactRotationPinEquationReport, G27GeometricFractionalError> {
    let search = search_g27_rotation_pin_closures_checked(handle)?;
    let best =
        search
            .best_unsuppressed_candidate()
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "g27_rotation_pin_closure_search",
            })?;
    if best.witness_vertex() != WITNESS_VERTEX
        || best.pin_vertex() != PIN_VERTEX
        || best.closure_pairs().len() != CLOSURE_PAIR_COUNT
    {
        return Err(G27GeometricFractionalError::MalformedData {
            source: "g27_rotation_pin_closure_best_candidate",
        });
    }

    let posture = G27ExactRotationPinEquationPosture::ManufacturedFieldExtensionRequired;
    let report = ExactEquationTokens::for_best_candidate();
    let core = artifact_core(
        HadwigerArtifactKind::G27ExactRotationPinEquationReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_exact_rotation_pin_equation".to_string(),
        },
        vec![search.reference()],
        payload(&report, posture),
    )?;
    Ok(G27ExactRotationPinEquationReport {
        core,
        hinge_vertex: HINGE_VERTEX.to_string(),
        witness_vertex: WITNESS_VERTEX.to_string(),
        pin_vertex: PIN_VERTEX.to_string(),
        closure_pair_count: CLOSURE_PAIR_COUNT,
        moving_radius_squared: report.moving_radius_squared,
        pin_distance_squared: report.pin_distance_squared,
        witness_pin_dot: report.witness_pin_dot,
        rotated_pin_dot: report.rotated_pin_dot,
        height_numerator: report.height_numerator,
        required_extension: report.required_extension,
        closure_replay_obligation: report.closure_replay_obligation,
        posture,
    })
}

struct ExactEquationTokens {
    moving_radius_squared: String,
    pin_distance_squared: String,
    witness_pin_dot: String,
    rotated_pin_dot: String,
    height_numerator: String,
    required_extension: String,
    closure_replay_obligation: String,
}

impl ExactEquationTokens {
    fn for_best_candidate() -> Self {
        Self {
            moving_radius_squared: "3".to_string(),
            pin_distance_squared: "(9-sqrt33)/2".to_string(),
            witness_pin_dot: "(9-sqrt33)/4".to_string(),
            rotated_pin_dot: "(13-sqrt33)/4".to_string(),
            height_numerator: "(7+sqrt33)/8".to_string(),
            required_extension: "sqrt((7+sqrt33)/8)".to_string(),
            closure_replay_obligation:
                "extend exact replay from Q(sqrt3,sqrt11,sqrt33) to the manufactured radical"
                    .to_string(),
        }
    }
}

fn payload(
    report: &ExactEquationTokens,
    posture: G27ExactRotationPinEquationPosture,
) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text("schema", "forge.hadwiger.g27_rotation_equation.v1"),
        HadwigerArtifactPayloadEntry::text("hinge_vertex", HINGE_VERTEX),
        HadwigerArtifactPayloadEntry::text("witness_vertex", WITNESS_VERTEX),
        HadwigerArtifactPayloadEntry::text("pin_vertex", PIN_VERTEX),
        HadwigerArtifactPayloadEntry::unsigned("closure_pair_count", CLOSURE_PAIR_COUNT as u128),
        HadwigerArtifactPayloadEntry::text("moving_radius_squared", &report.moving_radius_squared),
        HadwigerArtifactPayloadEntry::text("pin_distance_squared", &report.pin_distance_squared),
        HadwigerArtifactPayloadEntry::text("witness_pin_dot", &report.witness_pin_dot),
        HadwigerArtifactPayloadEntry::text("rotated_pin_dot", &report.rotated_pin_dot),
        HadwigerArtifactPayloadEntry::text("height_numerator", &report.height_numerator),
        HadwigerArtifactPayloadEntry::text("required_extension", &report.required_extension),
        HadwigerArtifactPayloadEntry::text(
            "closure_replay_obligation",
            &report.closure_replay_obligation,
        ),
        HadwigerArtifactPayloadEntry::text("posture", posture.as_str()),
    ]
}
