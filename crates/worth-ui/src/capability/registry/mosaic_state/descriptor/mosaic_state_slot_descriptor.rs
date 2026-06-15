use crate::capability::MosaicStateSlotId;

use super::{
    MosaicStateOwnerIdentity, MosaicStatePersistencePolicy, MosaicStateReplacementRule,
    MosaicStateSlotKind, MosaicStateTruthPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MosaicStateSlotDescriptor {
    id: MosaicStateSlotId,
    kind: MosaicStateSlotKind,
    owner_identity: Option<MosaicStateOwnerIdentity>,
    persistence_policy: Option<MosaicStatePersistencePolicy>,
    replacement_rule: Option<MosaicStateReplacementRule>,
    truth_posture: Option<MosaicStateTruthPosture>,
    label: Option<String>,
}

impl MosaicStateSlotDescriptor {
    pub fn new(id: MosaicStateSlotId, kind: MosaicStateSlotKind) -> Self {
        Self {
            id,
            kind,
            owner_identity: None,
            persistence_policy: None,
            replacement_rule: None,
            truth_posture: None,
            label: None,
        }
    }

    pub fn with_owner_identity(mut self, identity: MosaicStateOwnerIdentity) -> Self {
        self.owner_identity = Some(identity);
        self
    }

    pub fn with_persistence_policy(mut self, policy: MosaicStatePersistencePolicy) -> Self {
        self.persistence_policy = Some(policy);
        self
    }

    pub fn with_replacement_rule(mut self, rule: MosaicStateReplacementRule) -> Self {
        self.replacement_rule = Some(rule);
        self
    }

    pub fn with_truth_posture(mut self, posture: MosaicStateTruthPosture) -> Self {
        self.truth_posture = Some(posture);
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn id(&self) -> &MosaicStateSlotId {
        &self.id
    }

    pub fn kind(&self) -> &MosaicStateSlotKind {
        &self.kind
    }

    pub fn owner_identity(&self) -> Option<&MosaicStateOwnerIdentity> {
        self.owner_identity.as_ref()
    }

    pub fn persistence_policy(&self) -> Option<&MosaicStatePersistencePolicy> {
        self.persistence_policy.as_ref()
    }

    pub fn replacement_rule(&self) -> Option<&MosaicStateReplacementRule> {
        self.replacement_rule.as_ref()
    }

    pub fn truth_posture(&self) -> Option<&MosaicStateTruthPosture> {
        self.truth_posture.as_ref()
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}
