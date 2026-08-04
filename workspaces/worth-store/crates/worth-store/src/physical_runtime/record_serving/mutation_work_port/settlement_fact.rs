use crate::physical_runtime::{
    PhysicalEffectIdentity, PhysicalWorkEffectFate, PhysicalWorkIdentity,
    PhysicalWorkRecoveryDisposition, SettledPhysicalWork,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct CanonicalRecordMutationSettlement {
    identity: PhysicalWorkIdentity,
    effect: Option<PhysicalEffectIdentity>,
    effect_fate: PhysicalWorkEffectFate,
    recovery: PhysicalWorkRecoveryDisposition,
}

impl CanonicalRecordMutationSettlement {
    pub(super) fn from_settled(settled: &SettledPhysicalWork) -> Self {
        Self {
            identity: settled.intent().identity(),
            effect: settled.effect_identity(),
            effect_fate: settled.evidence().fate(),
            recovery: settled.recovery_disposition(),
        }
    }

    pub(in crate::physical_runtime::record_serving) const fn identity(
        self,
    ) -> PhysicalWorkIdentity {
        self.identity
    }

    pub(in crate::physical_runtime::record_serving) const fn effect(
        self,
    ) -> Option<PhysicalEffectIdentity> {
        self.effect
    }

    pub(in crate::physical_runtime::record_serving) const fn effect_fate(
        self,
    ) -> PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub(in crate::physical_runtime::record_serving) const fn recovery(
        self,
    ) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }
}
