use crate::graph_read_access_inventory::{
    WorthGraphReadAccessInventoryRowIdentity, WorthGraphReadDeclarationCandidate,
    WorthGraphReadReadFamilyTarget,
};

use super::lowered_authority::WorthGraphReadLoweredTouchedAuthority;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadLoweredAuthorityRecord {
    read_family_target: WorthGraphReadReadFamilyTarget,
    lowered_authority: WorthGraphReadLoweredTouchedAuthority,
    source_row_identity: WorthGraphReadAccessInventoryRowIdentity,
}

impl WorthGraphReadLoweredAuthorityRecord {
    pub(crate) fn from_candidate(
        candidate: &WorthGraphReadDeclarationCandidate,
        lowered_authority: WorthGraphReadLoweredTouchedAuthority,
    ) -> Self {
        Self {
            read_family_target: candidate.read_family_target(),
            lowered_authority,
            source_row_identity: candidate.inventory_row_identity().clone(),
        }
    }

    pub const fn read_family_target(&self) -> WorthGraphReadReadFamilyTarget {
        self.read_family_target
    }

    pub fn lowered_authority(&self) -> &WorthGraphReadLoweredTouchedAuthority {
        &self.lowered_authority
    }

    pub fn query_touch_descriptor_digest(&self) -> &str {
        self.lowered_authority.query_touch_descriptor_digest()
    }

    pub fn operating_world_digest(&self) -> &str {
        self.lowered_authority.operating_world_digest()
    }

    pub fn source_row_identity(&self) -> &WorthGraphReadAccessInventoryRowIdentity {
        &self.source_row_identity
    }

    pub const fn claims_read_declaration_authority(&self) -> bool {
        true
    }

    pub const fn claims_selected_obligation_is_declaration_authority(&self) -> bool {
        false
    }

    pub const fn claims_execution_authority(&self) -> bool {
        false
    }
}
