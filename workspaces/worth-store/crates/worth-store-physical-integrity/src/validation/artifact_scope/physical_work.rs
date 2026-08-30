use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::PhysicalWorkObligationIdentity;

use super::{PhysicalArtifactScope, PhysicalArtifactScopeIdentity};
use crate::localization::PhysicalByteRange;

impl PhysicalArtifactScope {
    pub const fn physical_work_obligation(
        store: StableStoreIdentity,
        identity: PhysicalWorkObligationIdentity,
        range: PhysicalByteRange,
    ) -> Self {
        Self::new(
            store,
            PhysicalArtifactScopeIdentity::PhysicalWorkObligation(identity),
            range,
        )
    }

    pub const fn physical_work_obligation_identity(self) -> Option<PhysicalWorkObligationIdentity> {
        match self.identity {
            PhysicalArtifactScopeIdentity::PhysicalWorkObligation(identity) => Some(identity),
            _ => None,
        }
    }
}
