use worth_query_installation::facade::{
    WorthQueryPortablePackageManifest, WorthQueryPortablePackageRecordFamily as Family,
};

use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::WorthQueryUntrustedPortablePackageRecordFrame;

pub(super) struct CanonicalArchiveInventory<'a> {
    manifest: &'a WorthQueryPortablePackageManifest,
    next_index: u32,
    family_index: usize,
    remaining_in_family: u32,
}

impl<'a> CanonicalArchiveInventory<'a> {
    pub(super) fn new(manifest: &'a WorthQueryPortablePackageManifest) -> Self {
        Self {
            manifest,
            next_index: 0,
            family_index: 0,
            remaining_in_family: manifest.family_count(Family::ALL[0]),
        }
    }

    pub(super) fn admit(
        &mut self,
        frame: &WorthQueryUntrustedPortablePackageRecordFrame,
    ) -> Result<(), Denial> {
        if frame.canonical_index() != self.next_index {
            return Err(Denial::new(Kind::NonCanonicalRecordSequence));
        }
        let expected = self
            .expected_family()
            .ok_or_else(|| Denial::new(Kind::InvalidFamilyCount))?;
        if frame.family() != expected {
            return Err(Denial::new(Kind::RecordFamilyInventoryMismatch));
        }
        self.remaining_in_family -= 1;
        self.next_index = self
            .next_index
            .checked_add(1)
            .ok_or_else(|| Denial::new(Kind::RecordBudgetExceeded))?;
        Ok(())
    }

    pub(super) fn finish(mut self) -> Result<(), Denial> {
        if self.next_index != self.manifest.record_count() || self.expected_family().is_some() {
            return Err(Denial::new(Kind::InvalidFamilyCount));
        }
        Ok(())
    }

    fn expected_family(&mut self) -> Option<Family> {
        while self.remaining_in_family == 0 {
            self.family_index += 1;
            let family = Family::ALL.get(self.family_index).copied()?;
            self.remaining_in_family = self.manifest.family_count(family);
        }
        Family::ALL.get(self.family_index).copied()
    }
}
