use forge_query::facade::{
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationSupportStatus,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorthTopologyLegalitySelectionDenialKind {
    MissingAccessReceipt,
    SupportPostureDenied,
    BudgetExceeded,
}

impl WorthTopologyLegalitySelectionDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingAccessReceipt => "missing-access-receipt",
            Self::SupportPostureDenied => "support-posture-denied",
            Self::BudgetExceeded => "budget-exceeded",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLegalitySelectionDenial {
    kind: WorthTopologyLegalitySelectionDenialKind,
    query_rule_identity_digest: String,
    support_status: Option<ForgeQueryGraphObligationSupportStatus>,
    registration_digest: Option<String>,
    execution_budget_digest: Option<String>,
    denial_digest: String,
}

impl WorthTopologyLegalitySelectionDenial {
    pub(super) fn support_posture_denied(
        registration: &ForgeQueryGraphObligationRegistration,
    ) -> Self {
        let support_status = registration.support_posture().status();
        let denial_digest = [
            "worth-topo-legality-selection-denial-v1",
            WorthTopologyLegalitySelectionDenialKind::SupportPostureDenied.as_str(),
            registration.rule_identity().identity_digest(),
            support_status.as_str(),
            registration.registration_digest(),
        ]
        .join("|");
        Self {
            kind: WorthTopologyLegalitySelectionDenialKind::SupportPostureDenied,
            query_rule_identity_digest: registration.rule_identity().identity_digest().to_string(),
            support_status: Some(support_status),
            registration_digest: Some(registration.registration_digest().to_string()),
            execution_budget_digest: Some(
                registration.execution_budget().budget_digest().to_string(),
            ),
            denial_digest,
        }
    }

    pub(super) fn missing_access_receipt(
        registration: &ForgeQueryGraphObligationRegistration,
        seed_digest: &str,
    ) -> Self {
        let denial_digest = [
            "worth-topo-legality-selection-denial-v1",
            WorthTopologyLegalitySelectionDenialKind::MissingAccessReceipt.as_str(),
            registration.rule_identity().identity_digest(),
            registration.registration_digest(),
            registration.execution_budget().budget_digest(),
            seed_digest,
        ]
        .join("|");
        Self {
            kind: WorthTopologyLegalitySelectionDenialKind::MissingAccessReceipt,
            query_rule_identity_digest: registration.rule_identity().identity_digest().to_string(),
            support_status: Some(registration.support_posture().status()),
            registration_digest: Some(registration.registration_digest().to_string()),
            execution_budget_digest: Some(
                registration.execution_budget().budget_digest().to_string(),
            ),
            denial_digest,
        }
    }

    pub(super) fn budget_exceeded(
        registration: &ForgeQueryGraphObligationRegistration,
        observed_state_scope: usize,
        max_state_scope: usize,
    ) -> Self {
        let support_status = registration.support_posture().status();
        let execution_budget = registration.execution_budget();
        let denial_digest = vec![
            "worth-topo-legality-selection-denial-v1".to_string(),
            WorthTopologyLegalitySelectionDenialKind::BudgetExceeded
                .as_str()
                .to_string(),
            registration.rule_identity().identity_digest().to_string(),
            support_status.as_str().to_string(),
            execution_budget.cost_class().as_str().to_string(),
            execution_budget
                .budget_exceeded_policy()
                .as_str()
                .to_string(),
            format!("observed-state-scope:{observed_state_scope}"),
            format!("max-state-scope:{max_state_scope}"),
            execution_budget.budget_digest().to_string(),
            registration.registration_digest().to_string(),
        ]
        .join("|");
        Self {
            kind: WorthTopologyLegalitySelectionDenialKind::BudgetExceeded,
            query_rule_identity_digest: registration.rule_identity().identity_digest().to_string(),
            support_status: Some(support_status),
            registration_digest: Some(registration.registration_digest().to_string()),
            execution_budget_digest: Some(execution_budget.budget_digest().to_string()),
            denial_digest,
        }
    }

    pub const fn kind(&self) -> WorthTopologyLegalitySelectionDenialKind {
        self.kind
    }

    pub fn query_rule_identity_digest(&self) -> &str {
        &self.query_rule_identity_digest
    }

    pub const fn support_status(&self) -> Option<ForgeQueryGraphObligationSupportStatus> {
        self.support_status
    }

    pub fn registration_digest(&self) -> Option<&str> {
        self.registration_digest.as_deref()
    }

    pub fn execution_budget_digest(&self) -> Option<&str> {
        self.execution_budget_digest.as_deref()
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}
