use super::{RelationalBranchRoot, RelationalBranchRootCaptureDenial};

#[derive(Debug, Clone)]
pub(crate) struct RelationalBranchRootIdentityIssuer {
    next_root_id: u64,
    next_schema_authority_id: u64,
    next_region_id: u64,
    next_reachability_id: u64,
}

impl Default for RelationalBranchRootIdentityIssuer {
    fn default() -> Self {
        Self {
            next_root_id: 1,
            next_schema_authority_id: 1,
            next_region_id: 1,
            next_reachability_id: 1,
        }
    }
}

impl RelationalBranchRootIdentityIssuer {
    pub(super) const fn next_reachability_id(&self) -> u64 {
        self.next_reachability_id
    }

    pub(crate) fn observe_root(&mut self, root: &RelationalBranchRoot) {
        self.next_root_id = self.next_root_id.max(root.id.saturating_add(1));
        self.next_schema_authority_id = self
            .next_schema_authority_id
            .max(root.schema_authority().allocation_id().saturating_add(1));
        let next_region_id = root
            .storage_regions()
            .map(|region| region.region_id.saturating_add(1))
            .max()
            .unwrap_or(self.next_region_id);
        self.next_region_id = self.next_region_id.max(next_region_id);
        let next_reachability_id = root
            .regions
            .allocation_observations()
            .into_iter()
            .map(|node| node.node_id.saturating_add(1))
            .max()
            .unwrap_or(self.next_reachability_id);
        self.next_reachability_id = self.next_reachability_id.max(next_reachability_id);
    }

    pub(crate) fn validate_capture_capacity(
        &self,
        touched_regions: usize,
    ) -> Result<(), RelationalBranchRootCaptureDenial> {
        self.next_root_id
            .checked_add(1)
            .ok_or(RelationalBranchRootCaptureDenial::RootIdentityExhausted)?;
        self.next_schema_authority_id
            .checked_add(1)
            .ok_or(RelationalBranchRootCaptureDenial::SchemaAuthorityIdentityExhausted)?;
        self.next_region_id
            .checked_add(touched_regions as u64)
            .ok_or(RelationalBranchRootCaptureDenial::RegionIdentityExhausted)?;
        let maximum_path_nodes = (touched_regions as u64)
            .checked_mul(33)
            .and_then(|nodes| nodes.checked_add(1))
            .ok_or(RelationalBranchRootCaptureDenial::ReachabilityIdentityExhausted)?;
        self.next_reachability_id
            .checked_add(maximum_path_nodes)
            .ok_or(RelationalBranchRootCaptureDenial::ReachabilityIdentityExhausted)?;
        Ok(())
    }

    pub(super) fn issue_root_id(&mut self) -> Result<u64, RelationalBranchRootCaptureDenial> {
        let issued = self.next_root_id;
        self.next_root_id = self
            .next_root_id
            .checked_add(1)
            .ok_or(RelationalBranchRootCaptureDenial::RootIdentityExhausted)?;
        Ok(issued)
    }

    pub(super) fn issue_schema_authority_id(
        &mut self,
    ) -> Result<u64, RelationalBranchRootCaptureDenial> {
        let issued = self.next_schema_authority_id;
        self.next_schema_authority_id = self
            .next_schema_authority_id
            .checked_add(1)
            .ok_or(RelationalBranchRootCaptureDenial::SchemaAuthorityIdentityExhausted)?;
        Ok(issued)
    }

    pub(super) fn issue_region_id(&mut self) -> Result<u64, RelationalBranchRootCaptureDenial> {
        let issued = self.next_region_id;
        self.next_region_id = self
            .next_region_id
            .checked_add(1)
            .ok_or(RelationalBranchRootCaptureDenial::RegionIdentityExhausted)?;
        Ok(issued)
    }

    pub(crate) fn issue_reachability_id(
        &mut self,
    ) -> Result<u64, RelationalBranchRootCaptureDenial> {
        let issued = self.next_reachability_id;
        self.next_reachability_id = self
            .next_reachability_id
            .checked_add(1)
            .ok_or(RelationalBranchRootCaptureDenial::ReachabilityIdentityExhausted)?;
        Ok(issued)
    }
}
