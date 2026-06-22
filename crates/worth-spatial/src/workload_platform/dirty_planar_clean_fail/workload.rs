use topology::facade::TopologySeedCleanFailReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::planar_contracts::clean_fail_boundary::PlanarCleanFailBoundaryReceipt;
use crate::workload_platform::user_response::WorthUserResponseReceipt;

use super::{
    case::DirtyPlanarCleanFailCase,
    counters::{DirtyPlanarCleanFailCounterInput, DirtyPlanarCleanFailCounters},
    evidence_guard,
    failure_policy::{DirtyPlanarCleanFailError, DirtyPlanarCleanFailRecoveryPosture},
    receipt::DirtyPlanarCleanFailReceipt,
};

pub struct DirtyPlanarCleanFailWorkload {
    declaration: String,
    topology_clean_fail: Option<TopologySeedCleanFailReceipt>,
    clean_fail_boundary: Option<PlanarCleanFailBoundaryReceipt>,
    recovery_posture: DirtyPlanarCleanFailRecoveryPosture,
    user_response: Option<WorthUserResponseReceipt>,
}

impl DirtyPlanarCleanFailWorkload {
    pub fn from_topology_clean_fail(receipt: TopologySeedCleanFailReceipt) -> Self {
        Self {
            declaration: "dirty planar input clean-fail workload".to_string(),
            topology_clean_fail: Some(receipt),
            clean_fail_boundary: None,
            recovery_posture: DirtyPlanarCleanFailRecoveryPosture::ExplainsWithoutRepair,
            user_response: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_clean_fail_boundary(mut self, receipt: PlanarCleanFailBoundaryReceipt) -> Self {
        self.clean_fail_boundary = Some(receipt);
        self
    }

    pub fn with_recovery_posture(mut self, posture: DirtyPlanarCleanFailRecoveryPosture) -> Self {
        self.recovery_posture = posture;
        self
    }

    pub fn with_user_response(mut self, receipt: WorthUserResponseReceipt) -> Self {
        self.user_response = Some(receipt);
        self
    }

    pub fn certify(self) -> Result<DirtyPlanarCleanFailReceipt, DirtyPlanarCleanFailError> {
        let topology_clean_fail = self
            .topology_clean_fail
            .as_ref()
            .ok_or(DirtyPlanarCleanFailError::MissingTopologyCleanFail)?;
        let clean_fail_boundary = self
            .clean_fail_boundary
            .as_ref()
            .ok_or(DirtyPlanarCleanFailError::MissingCleanFailBoundary)?;
        if self.recovery_posture == DirtyPlanarCleanFailRecoveryPosture::AttemptsTruthUpgrade {
            return Err(DirtyPlanarCleanFailError::RecoveryAttemptedTruthUpgrade);
        }
        let topology_case = evidence_guard::require_dirty_topology_clean_fail(topology_clean_fail)?;
        let topology_identity = topology_clean_fail.clean_fail_identity();
        if clean_fail_boundary.basis().input().source_digest() != topology_identity {
            return Err(DirtyPlanarCleanFailError::CleanFailBoundaryDidNotConsumeTopologyReceipt);
        }
        let dirty_case =
            evidence_guard::require_clean_fail_boundary(clean_fail_boundary, topology_case)?;
        evidence_guard::require_recovery_and_diagnostics(clean_fail_boundary)?;
        evidence_guard::require_transform_posture(clean_fail_boundary)?;
        evidence_guard::require_stable_identity_does_not_hide_dirty_geometry(
            clean_fail_boundary,
            &topology_identity,
        )?;
        let user_response = self
            .user_response
            .as_ref()
            .ok_or(DirtyPlanarCleanFailError::MissingUserResponse)?;
        evidence_guard::require_dirty_user_response(user_response, clean_fail_boundary)?;
        let counters = dirty_counters(clean_fail_boundary, user_response);
        let clean_fail_boundary_identity = clean_fail_boundary.clean_fail_boundary_digest();
        let workload_identity = format!(
            "dirty-planar-clean-fail:{}:{}",
            self.declaration, topology_identity
        );
        let clean_fail_digest = dirty_clean_fail_digest(
            &workload_identity,
            &topology_identity,
            clean_fail_boundary_identity,
            dirty_case,
            counters,
        );
        Ok(DirtyPlanarCleanFailReceipt::new(
            clean_fail_digest,
            workload_identity,
            topology_identity,
            clean_fail_boundary_identity.to_string(),
            dirty_case,
            counters,
        ))
    }
}

fn dirty_counters(
    clean_fail_boundary: &PlanarCleanFailBoundaryReceipt,
    _user_response: &WorthUserResponseReceipt,
) -> DirtyPlanarCleanFailCounters {
    DirtyPlanarCleanFailCounters::from_input(DirtyPlanarCleanFailCounterInput {
        topology_clean_fail_receipts: 1,
        clean_fail_boundary_receipts: 1,
        recovery_receipts: clean_fail_boundary.counters().recovery_receipts_consumed(),
        transform_posture_receipts: 1,
        diagnostic_receipts: clean_fail_boundary
            .counters()
            .diagnostic_receipts_consumed(),
        user_outcome_receipts: 1,
    })
}

fn dirty_clean_fail_digest(
    workload_identity: &str,
    topology_identity: &str,
    clean_fail_boundary_identity: &str,
    dirty_case: DirtyPlanarCleanFailCase,
    counters: DirtyPlanarCleanFailCounters,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "dirty-planar-clean-fail".to_string(),
            format!("workload:{workload_identity}"),
            format!("topology:{topology_identity}"),
            format!("boundary:{clean_fail_boundary_identity}"),
            format!("dirty_case:{dirty_case:?}"),
            format!(
                "topology_receipts:{}",
                counters.topology_clean_fail_receipts()
            ),
            format!(
                "clean_fail_receipts:{}",
                counters.clean_fail_boundary_receipts()
            ),
            format!("recovery_receipts:{}", counters.recovery_receipts()),
            format!(
                "transform_receipts:{}",
                counters.transform_posture_receipts()
            ),
            format!("diagnostic_receipts:{}", counters.diagnostic_receipts()),
            format!("response_receipts:{}", counters.user_outcome_receipts()),
        ],
    )
}
