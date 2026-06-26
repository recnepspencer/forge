use crate::capability::MosaicPlacementPolicyId;

use super::{
    MosaicPlacementAction, MosaicPlacementConflictBehavior, MosaicPlacementEligibility,
    MosaicPlacementPersistence, MosaicPlacementReloadReconciliation, MosaicPlacementSource,
    MosaicPlacementSupport, MosaicPlacementTarget, MosaicStableIdentityBehavior,
};

/// Declarative runtime-owned mosaic placement policy supplied by an application.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MosaicPlacementPolicyDescriptor {
    id: MosaicPlacementPolicyId,
    action: MosaicPlacementAction,
    source: Option<MosaicPlacementSource>,
    target: Option<MosaicPlacementTarget>,
    persistence: Option<MosaicPlacementPersistence>,
    stable_identity_behavior: Option<MosaicStableIdentityBehavior>,
    conflict_behavior: Option<MosaicPlacementConflictBehavior>,
    reload_reconciliation: Option<MosaicPlacementReloadReconciliation>,
    support: Option<MosaicPlacementSupport>,
    label: Option<String>,
}

impl MosaicPlacementPolicyDescriptor {
    pub fn new(id: MosaicPlacementPolicyId, action: MosaicPlacementAction) -> Self {
        Self {
            id,
            action,
            source: None,
            target: None,
            persistence: None,
            stable_identity_behavior: None,
            conflict_behavior: None,
            reload_reconciliation: None,
            support: None,
            label: None,
        }
    }

    pub fn with_source(mut self, source: MosaicPlacementSource) -> Self {
        self.source = Some(source);
        self
    }

    pub fn with_target(mut self, target: MosaicPlacementTarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn with_eligibility(mut self, eligibility: MosaicPlacementEligibility) -> Self {
        self.source = Some(eligibility.source().clone());
        self.target = Some(eligibility.target().clone());
        self
    }

    pub fn with_persistence(mut self, persistence: MosaicPlacementPersistence) -> Self {
        self.persistence = Some(persistence);
        self
    }

    pub fn with_stable_identity_behavior(
        mut self,
        stable_identity_behavior: MosaicStableIdentityBehavior,
    ) -> Self {
        self.stable_identity_behavior = Some(stable_identity_behavior);
        self
    }

    pub fn with_conflict_behavior(
        mut self,
        conflict_behavior: MosaicPlacementConflictBehavior,
    ) -> Self {
        self.conflict_behavior = Some(conflict_behavior);
        self
    }

    pub fn with_reload_reconciliation(
        mut self,
        reload_reconciliation: MosaicPlacementReloadReconciliation,
    ) -> Self {
        self.reload_reconciliation = Some(reload_reconciliation);
        self
    }

    pub fn with_support(mut self, support: MosaicPlacementSupport) -> Self {
        self.support = Some(support);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn id(&self) -> &MosaicPlacementPolicyId {
        &self.id
    }

    pub fn action(&self) -> &MosaicPlacementAction {
        &self.action
    }

    pub fn source(&self) -> Option<&MosaicPlacementSource> {
        self.source.as_ref()
    }

    pub fn target(&self) -> Option<&MosaicPlacementTarget> {
        self.target.as_ref()
    }

    pub fn eligibility(&self) -> Option<MosaicPlacementEligibility> {
        Some(MosaicPlacementEligibility::new(
            self.source.clone()?,
            self.target.clone()?,
        ))
    }

    pub fn persistence(&self) -> Option<&MosaicPlacementPersistence> {
        self.persistence.as_ref()
    }

    pub fn stable_identity_behavior(&self) -> Option<&MosaicStableIdentityBehavior> {
        self.stable_identity_behavior.as_ref()
    }

    pub fn conflict_behavior(&self) -> Option<&MosaicPlacementConflictBehavior> {
        self.conflict_behavior.as_ref()
    }

    pub fn reload_reconciliation(&self) -> Option<&MosaicPlacementReloadReconciliation> {
        self.reload_reconciliation.as_ref()
    }

    pub fn support(&self) -> Option<&MosaicPlacementSupport> {
        self.support.as_ref()
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}
