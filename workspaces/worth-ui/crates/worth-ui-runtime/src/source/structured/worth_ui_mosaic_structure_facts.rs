use crate::capability::{
    AdmittedCapability, MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId,
    MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicSizingContractDescriptor,
    MosaicSizingContractId, MosaicStateSlotDescriptor, MosaicStateSlotId, SurfaceDescriptor,
    SurfaceId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiMosaicSizingContractProjectionDenial {
    ContradictoryRootSizingContracts {
        observed: Vec<MosaicSizingContractId>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiMosaicStructureFacts {
    root_regions: Vec<WorthUiMosaicRegionFacts>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiMosaicRegionFacts {
    region: AdmittedCapability<MosaicRegionKindId>,
    descriptor: MosaicRegionKindDescriptor,
    sizing_contract: Option<(
        AdmittedCapability<MosaicSizingContractId>,
        MosaicSizingContractDescriptor,
    )>,
    state_slot: Option<(
        AdmittedCapability<MosaicStateSlotId>,
        MosaicStateSlotDescriptor,
    )>,
    child_regions: Vec<WorthUiMosaicRegionFacts>,
    mounts: Vec<WorthUiMosaicMountFacts>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiMosaicMountFacts {
    surface: AdmittedCapability<SurfaceId>,
    descriptor: SurfaceDescriptor,
    placement_policy: Option<(
        AdmittedCapability<MosaicPlacementPolicyId>,
        MosaicPlacementPolicyDescriptor,
    )>,
    state_slot: Option<(
        AdmittedCapability<MosaicStateSlotId>,
        MosaicStateSlotDescriptor,
    )>,
}

impl WorthUiMosaicStructureFacts {
    pub(crate) fn new(root_regions: Vec<WorthUiMosaicRegionFacts>) -> Self {
        Self { root_regions }
    }

    pub(crate) fn root_regions(&self) -> &[WorthUiMosaicRegionFacts] {
        &self.root_regions
    }

    pub(crate) fn unique_root_sizing_contract_id(
        &self,
    ) -> Result<Option<MosaicSizingContractId>, WorthUiMosaicSizingContractProjectionDenial> {
        let mut observed = self
            .root_regions
            .iter()
            .filter_map(|region| {
                region
                    .sizing_contract()
                    .map(|(contract, _)| contract.id().clone())
            })
            .collect::<Vec<_>>();
        observed.sort_by(|left, right| left.as_str().cmp(right.as_str()));
        observed.dedup_by(|left, right| left.as_str() == right.as_str());

        match observed.len() {
            0 => Ok(None),
            1 => Ok(observed.into_iter().next()),
            _ => Err(
                WorthUiMosaicSizingContractProjectionDenial::ContradictoryRootSizingContracts {
                    observed,
                },
            ),
        }
    }
}

impl WorthUiMosaicRegionFacts {
    pub(crate) fn new(
        region: AdmittedCapability<MosaicRegionKindId>,
        descriptor: MosaicRegionKindDescriptor,
        sizing_contract: Option<(
            AdmittedCapability<MosaicSizingContractId>,
            MosaicSizingContractDescriptor,
        )>,
        state_slot: Option<(
            AdmittedCapability<MosaicStateSlotId>,
            MosaicStateSlotDescriptor,
        )>,
        child_regions: Vec<WorthUiMosaicRegionFacts>,
        mounts: Vec<WorthUiMosaicMountFacts>,
    ) -> Self {
        Self {
            region,
            descriptor,
            sizing_contract,
            state_slot,
            child_regions,
            mounts,
        }
    }

    pub(crate) fn region(&self) -> &AdmittedCapability<MosaicRegionKindId> {
        &self.region
    }

    pub(crate) fn descriptor(&self) -> &MosaicRegionKindDescriptor {
        &self.descriptor
    }

    pub(crate) fn sizing_contract(
        &self,
    ) -> Option<&(
        AdmittedCapability<MosaicSizingContractId>,
        MosaicSizingContractDescriptor,
    )> {
        self.sizing_contract.as_ref()
    }

    pub(crate) fn state_slot(
        &self,
    ) -> Option<&(
        AdmittedCapability<MosaicStateSlotId>,
        MosaicStateSlotDescriptor,
    )> {
        self.state_slot.as_ref()
    }

    pub(crate) fn child_regions(&self) -> &[WorthUiMosaicRegionFacts] {
        &self.child_regions
    }

    pub(crate) fn mounts(&self) -> &[WorthUiMosaicMountFacts] {
        &self.mounts
    }
}

impl WorthUiMosaicMountFacts {
    pub(crate) fn new(
        surface: AdmittedCapability<SurfaceId>,
        descriptor: SurfaceDescriptor,
        placement_policy: Option<(
            AdmittedCapability<MosaicPlacementPolicyId>,
            MosaicPlacementPolicyDescriptor,
        )>,
        state_slot: Option<(
            AdmittedCapability<MosaicStateSlotId>,
            MosaicStateSlotDescriptor,
        )>,
    ) -> Self {
        Self {
            surface,
            descriptor,
            placement_policy,
            state_slot,
        }
    }

    pub(crate) fn surface(&self) -> &AdmittedCapability<SurfaceId> {
        &self.surface
    }

    pub(crate) fn descriptor(&self) -> &SurfaceDescriptor {
        &self.descriptor
    }

    pub(crate) fn placement_policy(
        &self,
    ) -> Option<&(
        AdmittedCapability<MosaicPlacementPolicyId>,
        MosaicPlacementPolicyDescriptor,
    )> {
        self.placement_policy.as_ref()
    }

    pub(crate) fn state_slot(
        &self,
    ) -> Option<&(
        AdmittedCapability<MosaicStateSlotId>,
        MosaicStateSlotDescriptor,
    )> {
        self.state_slot.as_ref()
    }
}
