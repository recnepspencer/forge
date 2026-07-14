use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_exact_geometry_support::{parse_w_integer_weights, EXPECTED_VERTEX_COUNT};
use super::g27_w_circles_full_terminal_export_replay::replay_g27_w_circles_full_terminal_export_checked;
use super::g27_w_circles_full_terminal_export_support::{
    replay_terminal, FullTerminalArtifact, Rational,
};
use super::g27_w_circles_parent_lift_readiness_replay::replay_g27_w_circles_parent_lift_readiness_checked;
use super::g27_w_circles_row_family_semantics_replay::replay_g27_w_circles_row_family_semantics_checked;
use super::g27_w_circles_semantic_partition_replay::replay_g27_w_circles_semantic_partition_checked;

const TERMINALS: &str = include_str!("../../docs/w607-full-terminal-export-preflight.json");
const TARGET_WEIGHTED_ALPHA: i128 = 512_933;
const EXPECTED_TERMINALS: usize = 135;
const EXPECTED_ROWS: usize = 80_143;
const EXPECTED_PARENT_ROWS: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesCertificateAdmissionGapReplayStatus {
    CertificateAdmissionBlockedTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesCertificateAdmissionGapReplayReport {
    core: HadwigerArtifactCore,
    admitted_scope: String,
    admitted_bound_floor: i128,
    admitted_bound_ceil: i128,
    target_bound: i128,
    target_pass: bool,
    theorem_authority: bool,
    blockers: Vec<String>,
    status: G27WCirclesCertificateAdmissionGapReplayStatus,
    conclusion: String,
}

impl G27WCirclesCertificateAdmissionGapReplayReport {
    pub fn summary(&self) -> (i128, i128, i128, bool, bool) {
        (
            self.admitted_bound_floor,
            self.admitted_bound_ceil,
            self.target_bound,
            self.target_pass,
            self.theorem_authority,
        )
    }

    pub fn admitted_scope(&self) -> &str {
        &self.admitted_scope
    }

    pub fn blockers(&self) -> &[String] {
        &self.blockers
    }

    pub fn status(&self) -> G27WCirclesCertificateAdmissionGapReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesCertificateAdmissionGapReplayReport, core);

pub fn replay_g27_w_circles_certificate_admission_gap_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesCertificateAdmissionGapReplayReport, G27GeometricFractionalError> {
    let terminal = replay_g27_w_circles_full_terminal_export_checked(handle)?;
    let semantic = replay_g27_w_circles_semantic_partition_checked(handle)?;
    let row_family = replay_g27_w_circles_row_family_semantics_checked(handle)?;
    let readiness = replay_g27_w_circles_parent_lift_readiness_checked(handle)?;
    if terminal.summary() != (EXPECTED_TERMINALS, EXPECTED_ROWS, 586_224, 0) {
        return malformed("w607_gap_terminal_summary");
    }
    if semantic.summary() != (64, EXPECTED_TERMINALS, EXPECTED_ROWS) {
        return malformed("w607_gap_semantic_summary");
    }
    if row_family.summary() != (EXPECTED_ROWS, EXPECTED_PARENT_ROWS) {
        return malformed("w607_gap_row_family_summary");
    }
    if readiness.summary() != (EXPECTED_ROWS, EXPECTED_PARENT_ROWS, false)
        || readiness.parent_lift_ids() != ["parent_lift_1"]
    {
        return malformed("w607_gap_readiness_summary");
    }
    let (floor, ceil) = exact_worst_terminal_bounds()?;
    if floor != 586_224 || ceil != 586_225 || ceil <= TARGET_WEIGHTED_ALPHA {
        return malformed("w607_gap_bound_policy");
    }
    report(
        readiness.reference(),
        floor,
        ceil,
        vec![
            "blocked_target_bound: admitted scoped bound ceil 586225 exceeds target 512933"
                .to_string(),
            "blocked_hn_claim: weighted-alpha certificate is not a plane chromatic lower-bound claim"
                .to_string(),
            "blocked_theorem_authority: scope remains declared mixed terminal partition, not final W607 root theorem"
                .to_string(),
        ],
    )
}

fn exact_worst_terminal_bounds() -> Result<(i128, i128), G27GeometricFractionalError> {
    let weights = parse_w_integer_weights()?;
    let artifact: FullTerminalArtifact =
        serde_json::from_str(TERMINALS).map_err(|_| malformed_err("w607_gap_json"))?;
    let mut worst = Rational::zero();
    for terminal in &artifact.terminals {
        let replay = replay_terminal(terminal, &weights, EXPECTED_VERTEX_COUNT)?;
        if replay.objective > worst {
            worst = replay.objective;
        }
    }
    Ok((worst.floor_i128(), worst.ceil_i128()))
}

fn report(
    parent: crate::domain_artifacts::HadwigerArtifactReference,
    admitted_bound_floor: i128,
    admitted_bound_ceil: i128,
    blockers: Vec<String>,
) -> Result<G27WCirclesCertificateAdmissionGapReplayReport, G27GeometricFractionalError> {
    let admitted_scope = "DeclaredMixedTerminalPartition".to_string();
    let target_pass = admitted_bound_ceil <= TARGET_WEIGHTED_ALPHA;
    let theorem_authority = false;
    if target_pass || theorem_authority {
        return malformed("w607_gap_overclaim");
    }
    let conclusion = format!(
        "admitted scoped certificate bound ceil {admitted_bound_ceil} for {admitted_scope}; target {TARGET_WEIGHTED_ALPHA} blocked and theorem authority false"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesCertificateAdmissionGapReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_certificate_admission_gap_replay".to_string(),
        },
        vec![parent],
        payload(
            &admitted_scope,
            admitted_bound_floor,
            admitted_bound_ceil,
            target_pass,
            theorem_authority,
            &blockers,
            &conclusion,
        ),
    )?;
    Ok(G27WCirclesCertificateAdmissionGapReplayReport {
        core,
        admitted_scope,
        admitted_bound_floor,
        admitted_bound_ceil,
        target_bound: TARGET_WEIGHTED_ALPHA,
        target_pass,
        theorem_authority,
        blockers,
        status: G27WCirclesCertificateAdmissionGapReplayStatus::CertificateAdmissionBlockedTarget,
        conclusion,
    })
}

fn payload(
    admitted_scope: &str,
    admitted_bound_floor: i128,
    admitted_bound_ceil: i128,
    target_pass: bool,
    theorem_authority: bool,
    blockers: &[String],
    conclusion: &str,
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.w607_certificate_admission_gap_replay.v1",
        ),
        HadwigerArtifactPayloadEntry::text("admitted_scope", admitted_scope),
        HadwigerArtifactPayloadEntry::unsigned(
            "admitted_bound_floor",
            admitted_bound_floor as u128,
        ),
        HadwigerArtifactPayloadEntry::unsigned("admitted_bound_ceil", admitted_bound_ceil as u128),
        HadwigerArtifactPayloadEntry::unsigned("target_bound", TARGET_WEIGHTED_ALPHA as u128),
        HadwigerArtifactPayloadEntry::text("target_pass", target_pass.to_string()),
        HadwigerArtifactPayloadEntry::text("theorem_authority", theorem_authority.to_string()),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ];
    for blocker in blockers {
        payload.push(HadwigerArtifactPayloadEntry::text("blocker", blocker));
    }
    payload
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(malformed_err(source))
}

fn malformed_err(source: &'static str) -> G27GeometricFractionalError {
    G27GeometricFractionalError::MalformedData { source }
}
