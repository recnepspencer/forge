use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use worth_relational::facade::runtime::{CustomInvariantRegistration, InvariantFailureEffect};

use super::operating_world_selector::WorthQueryGraphObligationOperatingWorldSelector;
use super::support_posture::WorthQueryGraphObligationSupportPosture;
use super::touch_selector::WorthQueryGraphTouchSelector;
use crate::runtime::{
    WorthQueryGraphObligationExecutionBudget, WorthQueryGraphObligationKind,
    WorthQueryGraphObligationRuleIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationRegistration {
    kind: WorthQueryGraphObligationKind,
    rule_identity: WorthQueryGraphObligationRuleIdentity,
    touch_selector: WorthQueryGraphTouchSelector,
    operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    support_posture: WorthQueryGraphObligationSupportPosture,
    registration_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationRegistration {
    pub fn new(
        kind: WorthQueryGraphObligationKind,
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        let mut registration = Self {
            kind,
            rule_identity,
            touch_selector,
            operating_world_selector,
            support_posture: WorthQueryGraphObligationSupportPosture::default_selection_posture(),
            registration_digest: worth_query_evidence_identity(
                WorthQueryEvidenceScope::GraphObligationRegistration,
            )
            .seal(),
        };
        registration.registration_digest = registration.build_digest();
        registration
    }

    pub fn with_support_posture(
        mut self,
        support_posture: WorthQueryGraphObligationSupportPosture,
    ) -> Self {
        self.support_posture = support_posture;
        self.registration_digest = self.build_digest();
        self
    }

    pub fn with_execution_budget(
        self,
        execution_budget: WorthQueryGraphObligationExecutionBudget,
    ) -> Self {
        let support_posture = self
            .support_posture
            .clone()
            .with_execution_budget(execution_budget);
        self.with_support_posture(support_posture)
    }

    pub fn schema_contract_validator(
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            WorthQueryGraphObligationKind::SchemaContractValidator,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn blocking_invariant(
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            WorthQueryGraphObligationKind::BlockingInvariant,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn advisory_obligation(
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            WorthQueryGraphObligationKind::AdvisoryObligation,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn preflight_sequencing_obligation(
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            WorthQueryGraphObligationKind::PreflightSequencingObligation,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn capability_gap_screen(
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            WorthQueryGraphObligationKind::CapabilityGapScreen,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn operating_context_gate(
        rule_identity: WorthQueryGraphObligationRuleIdentity,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            WorthQueryGraphObligationKind::OperatingContextGate,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn custom_invariant(
        custom_invariant: &CustomInvariantRegistration,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        match custom_invariant.failure_effect() {
            InvariantFailureEffect::AuditOnly => Self::advisory_obligation(
                custom_invariant_rule_identity(custom_invariant),
                touch_selector,
                operating_world_selector,
            ),
            _ => Self::blocking_invariant(
                custom_invariant_rule_identity(custom_invariant),
                touch_selector,
                operating_world_selector,
            ),
        }
    }

    pub fn kind(&self) -> WorthQueryGraphObligationKind {
        self.kind
    }

    pub fn rule_identity(&self) -> &WorthQueryGraphObligationRuleIdentity {
        &self.rule_identity
    }

    pub fn touch_selector(&self) -> &WorthQueryGraphTouchSelector {
        &self.touch_selector
    }

    pub fn operating_world_selector(&self) -> WorthQueryGraphObligationOperatingWorldSelector {
        self.operating_world_selector
    }

    pub fn support_posture(&self) -> &WorthQueryGraphObligationSupportPosture {
        &self.support_posture
    }

    pub fn execution_budget(&self) -> &WorthQueryGraphObligationExecutionBudget {
        self.support_posture.execution_budget()
    }

    pub fn registration_digest(&self) -> &str {
        self.registration_digest.as_str()
    }

    pub(crate) fn registration_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.registration_digest
    }

    fn build_digest(&self) -> WorthQueryEvidenceIdentity {
        let operating_world_selector_digest = self.operating_world_selector.selector_digest();
        worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationRegistration)
            .field_shape(WorthQueryEvidenceTag::new("kind"), self.kind.as_str())
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("rule"),
                self.rule_identity.identity_evidence_digest(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("touch_selector"),
                self.touch_selector.selector_evidence_digest(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("operating_world_selector"),
                &operating_world_selector_digest,
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("support_posture"),
                self.support_posture.posture_evidence_digest(),
            )
            .seal()
    }
}

fn custom_invariant_rule_identity(
    custom_invariant: &CustomInvariantRegistration,
) -> WorthQueryGraphObligationRuleIdentity {
    let identity = &custom_invariant.descriptor().identity;
    let obligation_name = format!(
        "{}.{}",
        identity.rule_id.as_str(),
        custom_invariant.execution_point().diagnostic_label()
    );
    WorthQueryGraphObligationRuleIdentity::new(
        "relational-custom-invariant",
        obligation_name,
        format!(
            "v{}.{}",
            identity.semantic_version.major, identity.semantic_version.minor
        ),
    )
    .expect("custom invariant registration validates non-empty rule identity")
}
