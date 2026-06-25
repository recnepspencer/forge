use forge_query::facade::{ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportStatus};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorthTopologySelectedValidatorEnforcementDenialKind {
    MissingSelectedFamily,
    UnexpectedObligationKind,
    UnsupportedObligation,
    MissingSupportOrBudgetProof,
    WitnessInputNotBoundToSelectedObligation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedValidatorEnforcementDenial {
    kind: WorthTopologySelectedValidatorEnforcementDenialKind,
    family: &'static str,
    selected_obligation_digest: Option<String>,
    witness_selected_obligation_digest: Option<String>,
    expected_obligation_kind: Option<ForgeQueryGraphObligationKind>,
    actual_obligation_kind: Option<ForgeQueryGraphObligationKind>,
    support_status: Option<ForgeQueryGraphObligationSupportStatus>,
    denial_digest: String,
}

impl WorthTopologySelectedValidatorEnforcementDenial {
    pub(in crate::validator_invariant_catalog) fn missing_selected_family(
        family: &'static str,
        family_identity_digest: &str,
    ) -> Self {
        Self::new(
            WorthTopologySelectedValidatorEnforcementDenialKind::MissingSelectedFamily,
            family,
            Some(family_identity_digest.to_string()),
            None,
            None,
            None,
            None,
        )
    }

    pub(in crate::validator_invariant_catalog) fn unexpected_obligation_kind(
        family: &'static str,
        selected_obligation_digest: &str,
        actual: ForgeQueryGraphObligationKind,
        expected: ForgeQueryGraphObligationKind,
    ) -> Self {
        Self::new(
            WorthTopologySelectedValidatorEnforcementDenialKind::UnexpectedObligationKind,
            family,
            Some(selected_obligation_digest.to_string()),
            None,
            Some(expected),
            Some(actual),
            None,
        )
    }

    pub(in crate::validator_invariant_catalog) fn unsupported_obligation(
        family: &'static str,
        selected_obligation_digest: &str,
        support_status: ForgeQueryGraphObligationSupportStatus,
    ) -> Self {
        Self::new(
            WorthTopologySelectedValidatorEnforcementDenialKind::UnsupportedObligation,
            family,
            Some(selected_obligation_digest.to_string()),
            None,
            None,
            None,
            Some(support_status),
        )
    }

    pub(in crate::validator_invariant_catalog) fn missing_support_or_budget_proof(
        family: &'static str,
        selected_obligation_digest: &str,
    ) -> Self {
        Self::new(
            WorthTopologySelectedValidatorEnforcementDenialKind::MissingSupportOrBudgetProof,
            family,
            Some(selected_obligation_digest.to_string()),
            None,
            None,
            None,
            None,
        )
    }

    pub(in crate::validator_invariant_catalog) fn witness_input_not_bound(
        family: &'static str,
        selected_obligation_digest: &str,
        witness_selected_obligation_digest: &str,
    ) -> Self {
        Self::new(
            WorthTopologySelectedValidatorEnforcementDenialKind::WitnessInputNotBoundToSelectedObligation,
            family,
            Some(selected_obligation_digest.to_string()),
            Some(witness_selected_obligation_digest.to_string()),
            None,
            None,
            None,
        )
    }

    pub const fn kind(&self) -> WorthTopologySelectedValidatorEnforcementDenialKind {
        self.kind
    }

    pub fn family(&self) -> &'static str {
        self.family
    }

    pub fn selected_obligation_digest(&self) -> Option<&str> {
        self.selected_obligation_digest.as_deref()
    }

    pub fn witness_selected_obligation_digest(&self) -> Option<&str> {
        self.witness_selected_obligation_digest.as_deref()
    }

    pub const fn expected_obligation_kind(&self) -> Option<ForgeQueryGraphObligationKind> {
        self.expected_obligation_kind
    }

    pub const fn actual_obligation_kind(&self) -> Option<ForgeQueryGraphObligationKind> {
        self.actual_obligation_kind
    }

    pub const fn support_status(&self) -> Option<ForgeQueryGraphObligationSupportStatus> {
        self.support_status
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }

    fn new(
        kind: WorthTopologySelectedValidatorEnforcementDenialKind,
        family: &'static str,
        selected_obligation_digest: Option<String>,
        witness_selected_obligation_digest: Option<String>,
        expected_obligation_kind: Option<ForgeQueryGraphObligationKind>,
        actual_obligation_kind: Option<ForgeQueryGraphObligationKind>,
        support_status: Option<ForgeQueryGraphObligationSupportStatus>,
    ) -> Self {
        let denial_digest = format!(
            "worth-topo-selected-validator-enforcement-denial-v1|{:?}|{}|{:?}|{:?}|{:?}|{:?}|{:?}",
            kind,
            family,
            selected_obligation_digest,
            witness_selected_obligation_digest,
            expected_obligation_kind,
            actual_obligation_kind,
            support_status
        );
        Self {
            kind,
            family,
            selected_obligation_digest,
            witness_selected_obligation_digest,
            expected_obligation_kind,
            actual_obligation_kind,
            support_status,
            denial_digest,
        }
    }
}

impl std::fmt::Display for WorthTopologySelectedValidatorEnforcementDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            WorthTopologySelectedValidatorEnforcementDenialKind::MissingSelectedFamily => {
                write!(
                    f,
                    "{} selected plan does not contain the required validator family",
                    self.family
                )
            }
            WorthTopologySelectedValidatorEnforcementDenialKind::UnexpectedObligationKind => {
                write!(
                    f,
                    "{} selected obligation has unexpected Query obligation kind {:?}; expected {:?}",
                    self.family, self.actual_obligation_kind, self.expected_obligation_kind
                )
            }
            WorthTopologySelectedValidatorEnforcementDenialKind::UnsupportedObligation => {
                write!(
                    f,
                    "{} selected obligation has unsupported Query support status {:?}",
                    self.family, self.support_status
                )
            }
            WorthTopologySelectedValidatorEnforcementDenialKind::MissingSupportOrBudgetProof => {
                write!(
                    f,
                    "{} selected obligation is missing support or budget proof",
                    self.family
                )
            }
            WorthTopologySelectedValidatorEnforcementDenialKind::WitnessInputNotBoundToSelectedObligation => {
                write!(
                    f,
                    "{} witness input is not bound to the selected obligation row",
                    self.family
                )
            }
        }
    }
}
