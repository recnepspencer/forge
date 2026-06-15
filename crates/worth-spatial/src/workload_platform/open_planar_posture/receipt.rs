use crate::planar_contracts::{
    clean_fail_boundary::PlanarOpenInputKind, planar_diagnostics::PlanarDiagnosticSubjectKind,
};

use super::{case::OpenPlanarPostureCase, counters::OpenPlanarPostureCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenPlanarPostureReceipt {
    posture_digest: String,
    workload_identity: String,
    topology_receipt_identity: String,
    unsupported_surface_identity: String,
    clean_fail_boundary_identity: String,
    diagnostic_receipt_identity: String,
    open_input_kind: Option<PlanarOpenInputKind>,
    diagnostic_subject_kind: PlanarDiagnosticSubjectKind,
    posture_case: OpenPlanarPostureCase,
    counters: OpenPlanarPostureCounters,
    bounded_surrogate_was_not_used: bool,
}

impl OpenPlanarPostureReceipt {
    pub(crate) fn new(input: OpenPlanarPostureReceiptInput) -> Self {
        Self {
            posture_digest: input.posture_digest,
            workload_identity: input.workload_identity,
            topology_receipt_identity: input.topology_receipt_identity,
            unsupported_surface_identity: input.unsupported_surface_identity,
            clean_fail_boundary_identity: input.clean_fail_boundary_identity,
            diagnostic_receipt_identity: input.diagnostic_receipt_identity,
            open_input_kind: input.open_input_kind,
            diagnostic_subject_kind: input.diagnostic_subject_kind,
            posture_case: input.posture_case,
            counters: input.counters,
            bounded_surrogate_was_not_used: input.bounded_surrogate_was_not_used,
        }
    }

    pub fn posture_digest(&self) -> &str {
        &self.posture_digest
    }

    pub fn workload_identity(&self) -> &str {
        &self.workload_identity
    }

    pub fn topology_receipt_identity(&self) -> &str {
        &self.topology_receipt_identity
    }

    pub fn unsupported_surface_identity(&self) -> &str {
        &self.unsupported_surface_identity
    }

    pub fn clean_fail_boundary_identity(&self) -> &str {
        &self.clean_fail_boundary_identity
    }

    pub fn diagnostic_receipt_identity(&self) -> &str {
        &self.diagnostic_receipt_identity
    }

    pub fn open_input_kind(&self) -> Option<PlanarOpenInputKind> {
        self.open_input_kind
    }

    pub fn diagnostic_subject_kind(&self) -> PlanarDiagnosticSubjectKind {
        self.diagnostic_subject_kind
    }

    pub fn posture_case(&self) -> OpenPlanarPostureCase {
        self.posture_case
    }

    pub fn counters(&self) -> OpenPlanarPostureCounters {
        self.counters
    }

    pub fn bounded_surrogate_was_not_used(&self) -> bool {
        self.bounded_surrogate_was_not_used
    }
}

pub(crate) struct OpenPlanarPostureReceiptInput {
    pub(crate) posture_digest: String,
    pub(crate) workload_identity: String,
    pub(crate) topology_receipt_identity: String,
    pub(crate) unsupported_surface_identity: String,
    pub(crate) clean_fail_boundary_identity: String,
    pub(crate) diagnostic_receipt_identity: String,
    pub(crate) open_input_kind: Option<PlanarOpenInputKind>,
    pub(crate) diagnostic_subject_kind: PlanarDiagnosticSubjectKind,
    pub(crate) posture_case: OpenPlanarPostureCase,
    pub(crate) counters: OpenPlanarPostureCounters,
    pub(crate) bounded_surrogate_was_not_used: bool,
}
