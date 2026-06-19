use crate::runtime::{
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationSelection,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphObligationSupportStatus,
};

use super::super::kit_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationInMemoryProof {
    selected_obligation_count: usize,
    selected_obligations: Vec<ForgeQueryGraphObligationInMemorySelectedObligation>,
    execution_statuses: Vec<ForgeQueryGraphObligationExecutionStatus>,
    selection_digest: String,
    proof_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationInMemorySelectedObligation {
    obligation_kind: ForgeQueryGraphObligationKind,
    support_lane: ForgeQueryGraphObligationSupportLane,
    support_status: ForgeQueryGraphObligationSupportStatus,
    rule_identity_digest: String,
    registration_digest: String,
    execution_budget_digest: String,
    row_digest: String,
}

impl ForgeQueryGraphObligationInMemoryProof {
    pub fn from_selection(selection: ForgeQueryGraphObligationSelection) -> Self {
        let selected_obligations = selection
            .matched_registrations()
            .iter()
            .map(ForgeQueryGraphObligationInMemorySelectedObligation::from_registration)
            .collect::<Vec<_>>();
        let execution_statuses = selection
            .matched_support_postures()
            .map(status_for_support_posture)
            .collect::<Vec<_>>();
        let selected_obligation_digests = selected_obligations
            .iter()
            .map(|obligation| obligation.row_digest.as_str())
            .collect::<Vec<_>>();
        let proof_digest = kit_digest(
            "in-memory-proof",
            std::iter::once(selection.selection_digest())
                .chain(selected_obligation_digests.iter().copied()),
        );
        Self {
            selected_obligation_count: selection.matched_obligation_count(),
            selected_obligations,
            execution_statuses,
            selection_digest: selection.selection_digest().to_string(),
            proof_digest,
        }
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub fn selected_obligations(&self) -> &[ForgeQueryGraphObligationInMemorySelectedObligation] {
        &self.selected_obligations
    }

    pub fn selected_registration_digests(&self) -> impl Iterator<Item = &str> {
        self.selected_obligations
            .iter()
            .map(|obligation| obligation.registration_digest())
    }

    pub fn execution_statuses(&self) -> &[ForgeQueryGraphObligationExecutionStatus] {
        &self.execution_statuses
    }

    pub fn selection_digest(&self) -> &str {
        &self.selection_digest
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }
}

impl ForgeQueryGraphObligationInMemorySelectedObligation {
    fn from_registration(registration: &ForgeQueryGraphObligationRegistration) -> Self {
        let obligation_kind = registration.kind();
        let support_lane = registration.support_posture().lane();
        let support_status = registration.support_posture().status();
        let rule_identity_digest = registration.rule_identity().identity_digest().to_string();
        let registration_digest = registration.registration_digest().to_string();
        let execution_budget_digest = registration.execution_budget().budget_digest().to_string();
        let row_digest = kit_digest(
            "in-memory-selected-obligation",
            [
                obligation_kind.as_str(),
                support_lane.as_str(),
                support_status.as_str(),
                rule_identity_digest.as_str(),
                registration_digest.as_str(),
                execution_budget_digest.as_str(),
            ],
        );
        Self {
            obligation_kind,
            support_lane,
            support_status,
            rule_identity_digest,
            registration_digest,
            execution_budget_digest,
            row_digest,
        }
    }

    pub fn obligation_kind(&self) -> ForgeQueryGraphObligationKind {
        self.obligation_kind
    }

    pub fn support_lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub fn support_status(&self) -> ForgeQueryGraphObligationSupportStatus {
        self.support_status
    }

    pub fn rule_identity_digest(&self) -> &str {
        &self.rule_identity_digest
    }

    pub fn registration_digest(&self) -> &str {
        &self.registration_digest
    }

    pub fn execution_budget_digest(&self) -> &str {
        &self.execution_budget_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

fn status_for_support_posture(
    posture: &ForgeQueryGraphObligationSupportPosture,
) -> ForgeQueryGraphObligationExecutionStatus {
    match posture.status() {
        ForgeQueryGraphObligationSupportStatus::Supported => {
            ForgeQueryGraphObligationExecutionStatus::Executed
        }
        ForgeQueryGraphObligationSupportStatus::Unsupported => {
            ForgeQueryGraphObligationExecutionStatus::Unsupported
        }
        ForgeQueryGraphObligationSupportStatus::NotApplicable => {
            ForgeQueryGraphObligationExecutionStatus::NotApplicableAfterStateLoad
        }
        ForgeQueryGraphObligationSupportStatus::DiagnosticOnly => {
            ForgeQueryGraphObligationExecutionStatus::DiagnosticOnly
        }
        ForgeQueryGraphObligationSupportStatus::DeferredToBackstop => {
            ForgeQueryGraphObligationExecutionStatus::DeferredToBackstop
        }
    }
}
