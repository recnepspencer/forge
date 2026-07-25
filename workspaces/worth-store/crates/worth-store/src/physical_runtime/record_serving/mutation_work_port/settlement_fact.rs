use crate::physical_runtime::{
    PhysicalEffectIdentity, PhysicalWorkEffectFate, PhysicalWorkIdentity,
    PhysicalWorkRecoveryDisposition, SettledPhysicalWork,
};

use super::super::publication::RecordPublicationWorkSettlement;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime) struct CanonicalRecordMutationSettlement {
    identity: PhysicalWorkIdentity,
    publication: RecordPublicationWorkSettlement,
}

impl CanonicalRecordMutationSettlement {
    pub(super) fn from_settled(settled: &SettledPhysicalWork) -> Self {
        Self {
            identity: settled.intent().identity(),
            publication: RecordPublicationWorkSettlement::from_canonical(
                settled.effect_identity(),
                settled.evidence().fate(),
                settled.recovery_disposition(),
            ),
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
        self.publication.effect()
    }

    pub(in crate::physical_runtime::record_serving) const fn effect_fate(
        self,
    ) -> PhysicalWorkEffectFate {
        self.publication.effect_fate()
    }

    pub(in crate::physical_runtime::record_serving) const fn recovery(
        self,
    ) -> PhysicalWorkRecoveryDisposition {
        self.publication.recovery()
    }

    pub(in crate::physical_runtime::record_serving) const fn publication(
        self,
    ) -> RecordPublicationWorkSettlement {
        self.publication
    }
}
