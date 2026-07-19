use crate::runtime::{
    WorthQueryGraphObligationKind, WorthQueryGraphObligationOperatingWorldSelector,
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphObligationSupportPosture, WorthQueryGraphTouchSelector,
};

use super::super::{WorthQueryDomainIdentityName, WorthQueryDomainSemanticVersion};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainGraphObligationDefinition {
    name: WorthQueryDomainIdentityName,
    semantic_version: WorthQueryDomainSemanticVersion,
    kind: WorthQueryGraphObligationKind,
    touch_selector: WorthQueryGraphTouchSelector,
    operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    support_posture: Option<WorthQueryGraphObligationSupportPosture>,
}

impl WorthQueryDomainGraphObligationDefinition {
    pub fn new(
        name: WorthQueryDomainIdentityName,
        semantic_version: WorthQueryDomainSemanticVersion,
        kind: WorthQueryGraphObligationKind,
        touch_selector: WorthQueryGraphTouchSelector,
        operating_world_selector: WorthQueryGraphObligationOperatingWorldSelector,
    ) -> Self {
        Self {
            name,
            semantic_version,
            kind,
            touch_selector,
            operating_world_selector,
            support_posture: None,
        }
    }

    #[must_use]
    pub fn with_support_posture(
        mut self,
        support_posture: WorthQueryGraphObligationSupportPosture,
    ) -> Self {
        self.support_posture = Some(support_posture);
        self
    }

    pub fn name(&self) -> &WorthQueryDomainIdentityName {
        &self.name
    }

    pub fn semantic_version(&self) -> WorthQueryDomainSemanticVersion {
        self.semantic_version
    }

    pub fn kind(&self) -> WorthQueryGraphObligationKind {
        self.kind
    }

    pub fn touch_selector(&self) -> &WorthQueryGraphTouchSelector {
        &self.touch_selector
    }

    pub fn operating_world_selector(&self) -> WorthQueryGraphObligationOperatingWorldSelector {
        self.operating_world_selector
    }

    pub fn support_posture(&self) -> Option<&WorthQueryGraphObligationSupportPosture> {
        self.support_posture.as_ref()
    }

    pub(crate) fn slot_key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.name.as_str(),
            self.semantic_version.major(),
            self.touch_selector.selector_digest(),
            self.operating_world_selector.selector_digest().as_str(),
        )
    }

    pub(crate) fn canonical_part(&self) -> String {
        format!(
            "{}:{}.{}:{}:{}:{}:{}",
            self.name.as_str(),
            self.semantic_version.major(),
            self.semantic_version.minor(),
            self.kind.as_str(),
            self.touch_selector.selector_digest(),
            self.operating_world_selector.selector_digest().as_str(),
            self.support_posture
                .as_ref()
                .map_or("default-selection", |posture| posture.posture_digest()),
        )
    }

    pub(crate) fn lower_with_owner(
        &self,
        domain_owner: &str,
        provenance: crate::runtime::WorthQueryInstalledDomainSubstrateProvenance,
    ) -> WorthQueryGraphObligationRegistration {
        let identity = WorthQueryGraphObligationRuleIdentity::new(
            domain_owner,
            self.name.as_str(),
            format!(
                "{}.{}",
                self.semantic_version.major(),
                self.semantic_version.minor()
            ),
        )
        .expect("typed domain package identity lowers to a valid obligation identity");
        let registration = match self.kind {
            WorthQueryGraphObligationKind::BlockingInvariant => {
                WorthQueryGraphObligationRegistration::blocking_invariant(
                    identity,
                    self.touch_selector.clone(),
                    self.operating_world_selector,
                )
            }
            WorthQueryGraphObligationKind::SchemaContractValidator => {
                WorthQueryGraphObligationRegistration::schema_contract_validator(
                    identity,
                    self.touch_selector.clone(),
                    self.operating_world_selector,
                )
            }
            WorthQueryGraphObligationKind::AdvisoryObligation => {
                WorthQueryGraphObligationRegistration::advisory_obligation(
                    identity,
                    self.touch_selector.clone(),
                    self.operating_world_selector,
                )
            }
            WorthQueryGraphObligationKind::PreflightSequencingObligation => {
                WorthQueryGraphObligationRegistration::preflight_sequencing_obligation(
                    identity,
                    self.touch_selector.clone(),
                    self.operating_world_selector,
                )
            }
            WorthQueryGraphObligationKind::CapabilityGapScreen => {
                WorthQueryGraphObligationRegistration::capability_gap_screen(
                    identity,
                    self.touch_selector.clone(),
                    self.operating_world_selector,
                )
            }
            WorthQueryGraphObligationKind::OperatingContextGate => {
                WorthQueryGraphObligationRegistration::operating_context_gate(
                    identity,
                    self.touch_selector.clone(),
                    self.operating_world_selector,
                )
            }
        };
        let registration = match self.support_posture.clone() {
            Some(posture) => registration.with_support_posture(posture),
            None => registration,
        };
        registration.authorized_by_installed_domain(provenance)
    }
}
