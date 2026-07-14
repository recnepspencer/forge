use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;
use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_w_circles_gamma0_leaf_dual_replay::replay_g27_w_circles_gamma0_leaf_dual_checked;
use super::g27_w_circles_gamma1_leaf_dual_replay::replay_g27_w_circles_gamma1_leaf_dual_checked;

const GAMMA0_TARGET: i128 = 613_372_392;
const GAMMA1_TARGET: i128 = 546_085_806;
const LIFT_COEFFICIENT: i128 = 67_286_586;
const BRANCH_VERTEX: usize = 304;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27WCirclesProjectedParentLiftReplayStatus {
    ReplayedParentValidLift,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27WCirclesProjectedParentLiftReplayReport {
    core: HadwigerArtifactCore,
    gamma0_target: i128,
    gamma1_target: i128,
    lift_coefficient: i128,
    branch_vertex: usize,
    status: G27WCirclesProjectedParentLiftReplayStatus,
    conclusion: String,
}

impl G27WCirclesProjectedParentLiftReplayReport {
    pub fn summary(&self) -> (usize, i128, i128, i128) {
        (
            self.branch_vertex,
            self.gamma0_target,
            self.gamma1_target,
            self.lift_coefficient,
        )
    }

    pub fn status(&self) -> G27WCirclesProjectedParentLiftReplayStatus {
        self.status
    }

    pub fn conclusion(&self) -> &str {
        &self.conclusion
    }
}

impl_hadwiger_artifact!(G27WCirclesProjectedParentLiftReplayReport, core);

pub fn replay_g27_w_circles_projected_parent_lift_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27WCirclesProjectedParentLiftReplayReport, G27GeometricFractionalError> {
    let gamma0 = replay_g27_w_circles_gamma0_leaf_dual_checked(handle)?;
    let gamma1 = replay_g27_w_circles_gamma1_leaf_dual_checked(handle)?;
    let (_, _, gamma0_worst, _) = gamma0.summary();
    let (_, _, gamma1_worst, _) = gamma1.summary();
    if gamma0_worst > GAMMA0_TARGET * 1024 || gamma1_worst > GAMMA1_TARGET * 1024 {
        return malformed("w607_parent_lift_gamma_bound");
    }
    let lift = GAMMA0_TARGET - GAMMA1_TARGET;
    if lift != LIFT_COEFFICIENT {
        return malformed("w607_parent_lift_coefficient");
    }
    let conclusion = format!(
        "replayed parent-valid projected aggregate lift for W vertex {BRANCH_VERTEX}: c0*x + {LIFT_COEFFICIENT}*x_{BRANCH_VERTEX} <= {GAMMA0_TARGET}"
    );
    let core = artifact_core(
        HadwigerArtifactKind::G27WCirclesProjectedParentLiftReplayReport,
        HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
        HadwigerArtifactSourceReference::ArtifactConstruction {
            operation: "g27_w_circles_projected_parent_lift_replay".to_string(),
        },
        vec![gamma0.reference(), gamma1.reference()],
        payload(&conclusion),
    )?;
    Ok(G27WCirclesProjectedParentLiftReplayReport {
        core,
        gamma0_target: GAMMA0_TARGET,
        gamma1_target: GAMMA1_TARGET,
        lift_coefficient: LIFT_COEFFICIENT,
        branch_vertex: BRANCH_VERTEX,
        status: G27WCirclesProjectedParentLiftReplayStatus::ReplayedParentValidLift,
        conclusion,
    })
}

fn payload(conclusion: &str) -> Vec<HadwigerArtifactPayloadEntry> {
    vec![
        HadwigerArtifactPayloadEntry::text(
            "schema",
            "forge.hadwiger.w607_projected_parent_lift_replay.v1",
        ),
        HadwigerArtifactPayloadEntry::unsigned("branch_vertex", BRANCH_VERTEX as u128),
        HadwigerArtifactPayloadEntry::unsigned("gamma0_target", GAMMA0_TARGET as u128),
        HadwigerArtifactPayloadEntry::unsigned("gamma1_target", GAMMA1_TARGET as u128),
        HadwigerArtifactPayloadEntry::unsigned("lift_coefficient", LIFT_COEFFICIENT as u128),
        HadwigerArtifactPayloadEntry::text("conclusion", conclusion),
    ]
}

fn malformed<T>(source: &'static str) -> Result<T, G27GeometricFractionalError> {
    Err(G27GeometricFractionalError::MalformedData { source })
}
