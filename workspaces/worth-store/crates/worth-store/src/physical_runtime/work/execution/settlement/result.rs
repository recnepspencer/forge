use super::{PhysicalWorkHealthRevocation, PhysicalWorkSettlementResult};
use crate::physical_runtime::work::SettledPhysicalWork;

impl PhysicalWorkSettlementResult {
    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (
        SettledPhysicalWork,
        Option<PhysicalWorkHealthRevocation>,
        super::super::super::submission::PhysicalEffectActivity,
        Option<super::super::PhysicalResidencyWritebackCompletion>,
    ) {
        (
            self.settled,
            self.health_revocation,
            self.effect_activity,
            self.residency_writeback,
        )
    }
}
