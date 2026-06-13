use topology::facade::TopologySeedReceipt;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::planar_contracts::clean_fail_boundary::PlanarCleanFailBoundaryReceipt;
use crate::workload_platform::surface_support::UnsupportedSurfaceSupport;
use crate::workload_platform::user_response::WorthUserResponseReceipt;

use super::{
    case::OpenPlanarPostureCase,
    counters::{OpenPlanarPostureCounterInput, OpenPlanarPostureCounters},
    evidence_guard,
    failure_policy::OpenPlanarPostureError,
    receipt::{OpenPlanarPostureReceipt, OpenPlanarPostureReceiptInput},
};

pub struct OpenPlanarPostureWorkload {
    declaration: String,
    topology: Option<TopologySeedReceipt>,
    unsupported_surface: Option<UnsupportedSurfaceSupport>,
    clean_fail_boundary: Option<PlanarCleanFailBoundaryReceipt>,
    posture_case: Option<OpenPlanarPostureCase>,
    attempted_bounded_surrogate: Option<TopologySeedReceipt>,
    user_response: Option<WorthUserResponseReceipt>,
}

impl OpenPlanarPostureWorkload {
    pub fn from_open_topology(receipt: TopologySeedReceipt) -> Self {
        Self {
            declaration: "open planar posture workload".to_string(),
            topology: Some(receipt),
            unsupported_surface: None,
            clean_fail_boundary: None,
            posture_case: None,
            attempted_bounded_surrogate: None,
            user_response: None,
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_unsupported_surface_support(mut self, support: UnsupportedSurfaceSupport) -> Self {
        self.unsupported_surface = Some(support);
        self
    }

    pub fn with_clean_fail_boundary(mut self, receipt: PlanarCleanFailBoundaryReceipt) -> Self {
        self.clean_fail_boundary = Some(receipt);
        self
    }

    pub fn classify_as(mut self, posture_case: OpenPlanarPostureCase) -> Self {
        self.posture_case = Some(posture_case);
        self
    }

    pub fn with_attempted_bounded_surrogate(mut self, receipt: TopologySeedReceipt) -> Self {
        self.attempted_bounded_surrogate = Some(receipt);
        self
    }

    pub fn with_user_response(mut self, receipt: WorthUserResponseReceipt) -> Self {
        self.user_response = Some(receipt);
        self
    }

    pub fn posture_identity_preview(&self) -> Result<String, OpenPlanarPostureError> {
        let topology = self
            .topology
            .as_ref()
            .ok_or(OpenPlanarPostureError::MissingOpenTopology)?;
        let topology_case = evidence_guard::require_open_topology(topology)?;
        let posture_case = self.posture_case.unwrap_or(topology_case);
        let unsupported_surface = self
            .unsupported_surface
            .as_ref()
            .ok_or(OpenPlanarPostureError::MissingUnsupportedSurfaceSupport)?;
        evidence_guard::require_unsupported_surface(unsupported_surface, topology)?;
        let unsupported_surface_identity = unsupported_surface_identity(unsupported_surface)?;
        let clean_fail_boundary = self
            .clean_fail_boundary
            .as_ref()
            .ok_or(OpenPlanarPostureError::MissingCleanFailBoundary)?;
        evidence_guard::require_clean_fail_boundary(clean_fail_boundary, topology, posture_case)?;
        let clean_fail_boundary_identity =
            clean_fail_boundary.clean_fail_boundary_digest().to_string();
        Ok(workload_identity(
            &self.declaration,
            &evidence_guard::topology_identity(topology),
            &unsupported_surface_identity,
            &clean_fail_boundary_identity,
            posture_case,
        ))
    }

    pub fn certify(self) -> Result<OpenPlanarPostureReceipt, OpenPlanarPostureError> {
        let topology = self
            .topology
            .as_ref()
            .ok_or(OpenPlanarPostureError::MissingOpenTopology)?;
        let topology_case = evidence_guard::require_open_topology(topology)?;
        let posture_case = self.posture_case.unwrap_or(topology_case);
        if self.posture_case.is_some()
            && topology_case == OpenPlanarPostureCase::UnsupportedOpenWire
            && posture_case == OpenPlanarPostureCase::UnsupportedOpenSheet
        {
            return Err(OpenPlanarPostureError::MismatchedOutcomeCase {
                expected: topology_case,
                actual: posture_case,
            });
        }
        let unsupported_surface = self
            .unsupported_surface
            .as_ref()
            .ok_or(OpenPlanarPostureError::MissingUnsupportedSurfaceSupport)?;
        evidence_guard::require_unsupported_surface(unsupported_surface, topology)?;
        let clean_fail_boundary = self
            .clean_fail_boundary
            .as_ref()
            .ok_or(OpenPlanarPostureError::MissingCleanFailBoundary)?;
        evidence_guard::require_clean_fail_boundary(clean_fail_boundary, topology, posture_case)?;
        let bounded_surrogate_rejections = evidence_guard::require_no_bounded_surrogate(
            self.attempted_bounded_surrogate.as_ref(),
        )?;
        let topology_identity = evidence_guard::topology_identity(topology);
        let unsupported_surface_identity = unsupported_surface_identity(unsupported_surface)?;
        let clean_fail_boundary_identity =
            clean_fail_boundary.clean_fail_boundary_digest().to_string();
        let diagnostic_receipt_identity = clean_fail_boundary
            .basis()
            .diagnostics()
            .diagnostic_bundle_digest()
            .to_string();
        let open_input_kind = clean_fail_boundary.basis().input().open_input_kind();
        let diagnostic_subject_kind = clean_fail_boundary
            .basis()
            .diagnostics()
            .basis()
            .subject()
            .kind();
        let workload_identity = workload_identity(
            &self.declaration,
            &topology_identity,
            &unsupported_surface_identity,
            &clean_fail_boundary_identity,
            posture_case,
        );
        let user_response = self
            .user_response
            .as_ref()
            .ok_or(OpenPlanarPostureError::MissingUserResponse)?;
        evidence_guard::require_user_response(user_response, posture_case, &workload_identity)?;
        let counters = OpenPlanarPostureCounters::from_input(OpenPlanarPostureCounterInput {
            topology_receipts: 1,
            unsupported_surface_receipts: 1,
            clean_fail_boundary_receipts: 1,
            transform_posture_receipts: 1,
            diagnostic_receipts: clean_fail_boundary
                .counters()
                .diagnostic_receipts_consumed(),
            user_outcome_receipts: 1,
            bounded_surrogate_rejections,
        });
        let posture_digest = posture_digest(
            &workload_identity,
            &topology_identity,
            &unsupported_surface_identity,
            &clean_fail_boundary_identity,
            &diagnostic_receipt_identity,
            posture_case,
            counters,
        );
        Ok(OpenPlanarPostureReceipt::new(
            OpenPlanarPostureReceiptInput {
                posture_digest,
                workload_identity,
                topology_receipt_identity: topology_identity,
                unsupported_surface_identity,
                clean_fail_boundary_identity,
                diagnostic_receipt_identity,
                open_input_kind,
                diagnostic_subject_kind,
                posture_case,
                counters,
                bounded_surrogate_was_not_used: true,
            },
        ))
    }
}

fn workload_identity(
    declaration: &str,
    topology_identity: &str,
    unsupported_surface_identity: &str,
    clean_fail_boundary_identity: &str,
    posture_case: OpenPlanarPostureCase,
) -> String {
    format!(
        "open-planar-posture:{}:{}:{}:{}:{}",
        declaration,
        topology_identity,
        unsupported_surface_identity,
        clean_fail_boundary_identity,
        posture_case.human_name()
    )
}

fn posture_digest(
    workload_identity: &str,
    topology_identity: &str,
    unsupported_surface_identity: &str,
    clean_fail_boundary_identity: &str,
    diagnostic_receipt_identity: &str,
    posture_case: OpenPlanarPostureCase,
    counters: OpenPlanarPostureCounters,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "open-planar-posture".to_string(),
            format!("workload:{workload_identity}"),
            format!("topology:{topology_identity}"),
            format!("unsupported_surface:{unsupported_surface_identity}"),
            format!("clean_fail:{clean_fail_boundary_identity}"),
            format!("diagnostic:{diagnostic_receipt_identity}"),
            format!("posture_case:{posture_case:?}"),
            format!("topology_receipts:{}", counters.topology_receipts()),
            format!(
                "unsupported_surface_receipts:{}",
                counters.unsupported_surface_receipts()
            ),
            format!(
                "clean_fail_receipts:{}",
                counters.clean_fail_boundary_receipts()
            ),
            format!(
                "transform_receipts:{}",
                counters.transform_posture_receipts()
            ),
            format!("diagnostic_receipts:{}", counters.diagnostic_receipts()),
            format!("response_receipts:{}", counters.user_outcome_receipts()),
            format!(
                "surrogate_rejections:{}",
                counters.bounded_surrogate_rejections()
            ),
        ],
    )
}

fn unsupported_surface_identity(
    unsupported_surface: &UnsupportedSurfaceSupport,
) -> Result<String, OpenPlanarPostureError> {
    unsupported_surface
        .receipt()
        .map(|receipt| receipt.stage_identity().receipt_identity().to_string())
        .ok_or(OpenPlanarPostureError::MissingUnsupportedSurfaceSupport)
}
