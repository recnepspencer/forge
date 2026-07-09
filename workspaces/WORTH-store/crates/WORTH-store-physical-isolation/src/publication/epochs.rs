use super::PhysicalPublicationDenial;
use crate::{CurrentPhysicalRoot, ManifestEpoch, RootEpoch};

#[derive(Debug, Clone, Copy)]
pub struct RootPublicationEpoch {
    old: RootEpoch,
    new: RootEpoch,
}

#[derive(Debug, Clone, Copy)]
pub struct ManifestPublicationEpoch {
    old: ManifestEpoch,
    new: ManifestEpoch,
}

#[derive(Debug, Clone, Copy)]
pub struct PublicationEpochPair {
    root: RootPublicationEpoch,
    manifest: ManifestPublicationEpoch,
}

impl PartialEq for RootPublicationEpoch {
    fn eq(&self, other: &Self) -> bool {
        self.old.get() == other.old.get() && self.new.get() == other.new.get()
    }
}

impl Eq for RootPublicationEpoch {}

impl PartialEq for ManifestPublicationEpoch {
    fn eq(&self, other: &Self) -> bool {
        self.old.get() == other.old.get() && self.new.get() == other.new.get()
    }
}

impl Eq for ManifestPublicationEpoch {}

impl PartialEq for PublicationEpochPair {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root && self.manifest == other.manifest
    }
}

impl Eq for PublicationEpochPair {}

impl RootPublicationEpoch {
    pub fn advance_from(
        old_root: CurrentPhysicalRoot,
        new_root: CurrentPhysicalRoot,
    ) -> Result<Self, PhysicalPublicationDenial> {
        if new_root.epoch().get() <= old_root.epoch().get() {
            return Err(PhysicalPublicationDenial::StaleRootPublicationEpoch);
        }
        Ok(Self {
            old: old_root.epoch(),
            new: new_root.epoch(),
        })
    }

    pub const fn old(self) -> RootEpoch {
        self.old
    }

    pub const fn new(self) -> RootEpoch {
        self.new
    }
}

impl ManifestPublicationEpoch {
    pub fn advance_from(
        old_root: CurrentPhysicalRoot,
        new_root: CurrentPhysicalRoot,
    ) -> Result<Self, PhysicalPublicationDenial> {
        if new_root.manifest_epoch().get() <= old_root.manifest_epoch().get() {
            return Err(PhysicalPublicationDenial::StaleManifestPublicationEpoch);
        }
        Ok(Self {
            old: old_root.manifest_epoch(),
            new: new_root.manifest_epoch(),
        })
    }

    pub const fn old(self) -> ManifestEpoch {
        self.old
    }

    pub const fn new(self) -> ManifestEpoch {
        self.new
    }
}

impl PublicationEpochPair {
    pub fn advance_from(
        old_root: CurrentPhysicalRoot,
        new_root: CurrentPhysicalRoot,
    ) -> Result<Self, PhysicalPublicationDenial> {
        Ok(Self {
            root: RootPublicationEpoch::advance_from(old_root, new_root)?,
            manifest: ManifestPublicationEpoch::advance_from(old_root, new_root)?,
        })
    }

    pub const fn root(self) -> RootPublicationEpoch {
        self.root
    }

    pub const fn manifest(self) -> ManifestPublicationEpoch {
        self.manifest
    }
}
