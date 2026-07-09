use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::{S6ClosedS7PlacementAdmissionSeed, S7PlacementReadinessNonClaim};

#[derive(Debug)]
pub struct S6S7PlacementAdmissionAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

pub fn admit_s6_s7_placement_handoff(
    authority: S6S7PlacementAdmissionAuthority,
) -> S6ClosedS7PlacementAdmissionSeed {
    let _current_authority = authority.into_current_authority();
    S6ClosedS7PlacementAdmissionSeed::from_closed_s6_readiness(
        S7PlacementReadinessNonClaim::required(),
    )
}

impl S6S7PlacementAdmissionAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    fn into_current_authority(self) -> StoreCurrentAuthorityWitness {
        self.current_authority
    }
}
