use worth_store_physical_format::{
    DurablePhysicalRootManifest, DurableRootSelector, RootManifestDenial, RootSelectorDecodeDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRootSourceCandidate {
    selector: DurableRootSelector,
    manifest: DurablePhysicalRootManifest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalRootSlotObservation {
    Absent,
    Rejected {
        denial: PhysicalRootCandidateDenial,
        selector: Option<DurableRootSelector>,
    },
    Admitted(PhysicalRootSourceCandidate),
}

impl PhysicalRootSlotObservation {
    pub const fn rejection(
        &self,
    ) -> Option<(PhysicalRootCandidateDenial, Option<DurableRootSelector>)> {
        match self {
            Self::Rejected { denial, selector } => Some((*denial, *selector)),
            Self::Absent | Self::Admitted(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalRootCandidateDenial {
    SelectorFormat(RootSelectorDecodeDenial),
    ForeignStore,
    WrongRole,
    RootMissing,
    RootFormat(RootManifestDenial),
    RootFormatMismatch,
    RootGenerationMismatch,
}

impl PhysicalRootSourceCandidate {
    pub(super) fn admit(
        selector: DurableRootSelector,
        manifest: DurablePhysicalRootManifest,
        manifest_format: worth_store_physical_format::PhysicalRecordFormatDeclaration,
    ) -> Result<Self, PhysicalRootCandidateDenial> {
        if selector.format() != manifest_format {
            return Err(PhysicalRootCandidateDenial::RootFormatMismatch);
        }
        if selector.root_generation() != manifest.generation() {
            return Err(PhysicalRootCandidateDenial::RootGenerationMismatch);
        }
        Ok(Self { selector, manifest })
    }

    pub const fn selector(&self) -> DurableRootSelector {
        self.selector
    }

    pub const fn manifest(&self) -> &DurablePhysicalRootManifest {
        &self.manifest
    }
}
