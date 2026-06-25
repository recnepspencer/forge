use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use forge_relational::facade::runtime::{CustomInvariantRegistration, InvariantFailureEffect};

use super::operating_world_selector::ForgeQueryGraphObligationOperatingWorldSelector;
use super::support_posture::ForgeQueryGraphObligationSupportPosture;
use super::touch_selector::ForgeQueryGraphTouchSelector;
use crate::runtime::{
    ForgeQueryGraphObligationExecutionBudget, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationRuleIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationRegistration {
    kind: ForgeQueryGraphObligationKind,
    rule_identity: ForgeQueryGraphObligationRuleIdentity,
    touch_selector: ForgeQueryGraphTouchSelector,
    operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    support_posture: ForgeQueryGraphObligationSupportPosture,
    registration_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationRegistration {
    pub fn new(
        kind: ForgeQueryGraphObligationKind,
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        let mut registration = Self {
            kind,
            rule_identity,
            touch_selector,
            operating_world_selector,
            support_posture: ForgeQueryGraphObligationSupportPosture::default_selection_posture(),
            registration_digest: forge_query_evidence_identity(
                ForgeQueryEvidenceScope::GraphObligationRegistration,
            )
            .seal(),
        };
        registration.registration_digest = registration.build_digest();
        registration
    }

    pub fn with_support_posture(
        mut self,
        support_posture: ForgeQueryGraphObligationSupportPosture,
    ) -> Self {
        self.support_posture = support_posture;
        self.registration_digest = self.build_digest();
        self
    }

    pub fn with_execution_budget(
        self,
        execution_budget: ForgeQueryGraphObligationExecutionBudget,
    ) -> Self {
        let support_posture = self
            .support_posture
            .clone()
            .with_execution_budget(execution_budget);
        self.with_support_posture(support_posture)
    }

    pub fn schema_contract_validator(
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            ForgeQueryGraphObligationKind::SchemaContractValidator,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn blocking_invariant(
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            ForgeQueryGraphObligationKind::BlockingInvariant,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn advisory_obligation(
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            ForgeQueryGraphObligationKind::AdvisoryObligation,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn preflight_sequencing_obligation(
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            ForgeQueryGraphObligationKind::PreflightSequencingObligation,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn capability_gap_screen(
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            ForgeQueryGraphObligationKind::CapabilityGapScreen,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn operating_context_gate(
        rule_identity: ForgeQueryGraphObligationRuleIdentity,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self::new(
            ForgeQueryGraphObligationKind::OperatingContextGate,
            rule_identity,
            touch_selector,
            operating_world_selector,
        )
    }

    pub fn custom_invariant(
        custom_invariant: &CustomInvariantRegistration,
        touch_selector: ForgeQueryGraphTouchSelector,
        operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
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

    pub fn kind(&self) -> ForgeQueryGraphObligationKind {
        self.kind
    }

    pub fn rule_identity(&self) -> &ForgeQueryGraphObligationRuleIdentity {
        &self.rule_identity
    }

    pub fn touch_selector(&self) -> &ForgeQueryGraphTouchSelector {
        &self.touch_selector
    }

    pub fn operating_world_selector(&self) -> ForgeQueryGraphObligationOperatingWorldSelector {
        self.operating_world_selector
    }

    pub fn support_posture(&self) -> &ForgeQueryGraphObligationSupportPosture {
        &self.support_posture
    }

    pub fn execution_budget(&self) -> &ForgeQueryGraphObligationExecutionBudget {
        self.support_posture.execution_budget()
    }

    pub fn registration_digest(&self) -> &str {
        self.registration_digest.as_str()
    }

    pub(crate) fn registration_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.registration_digest
    }

    fn build_digest(&self) -> ForgeQueryEvidenceIdentity {
        let operating_world_selector_digest = self.operating_world_selector.selector_digest();
        forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationRegistration)
            .field_shape(ForgeQueryEvidenceTag::new("kind"), self.kind.as_str())
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("rule"),
                self.rule_identity.identity_evidence_digest(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("touch_selector"),
                self.touch_selector.selector_evidence_digest(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("operating_world_selector"),
                &operating_world_selector_digest,
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("support_posture"),
                self.support_posture.posture_evidence_digest(),
            )
            .seal()
    }
}

fn custom_invariant_rule_identity(
    custom_invariant: &CustomInvariantRegistration,
) -> ForgeQueryGraphObligationRuleIdentity {
    let identity = &custom_invariant.descriptor().identity;
    let obligation_name = format!(
        "{}.{}",
        identity.rule_id.as_str(),
        custom_invariant.execution_point().diagnostic_label()
    );
    ForgeQueryGraphObligationRuleIdentity::new(
        "relational-custom-invariant",
        obligation_name,
        format!(
            "v{}.{}",
            identity.semantic_version.major, identity.semantic_version.minor
        ),
    )
    .expect("custom invariant registration validates non-empty rule identity")
}
