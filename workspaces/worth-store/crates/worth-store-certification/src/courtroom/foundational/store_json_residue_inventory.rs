mod classification;

use crate::{
    courtroom::foundational::store_json_residue_scan::scan_current_store_json_residue,
    StoreJsonResidueClassification, StoreJsonResidueDenial, StoreJsonResidueOccurrence,
    StoreJsonResidueZone,
};

use classification::classify_store_json_residue_occurrences;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreJsonResidueInventory {
    classified: Vec<StoreJsonResidueClassification>,
}

impl StoreJsonResidueInventory {
    pub(crate) fn from_current_sources() -> Result<Self, StoreJsonResidueDenial> {
        Self::from_occurrences(scan_current_store_json_residue()?)
    }

    pub(crate) fn from_occurrences(
        occurrences: Vec<StoreJsonResidueOccurrence>,
    ) -> Result<Self, StoreJsonResidueDenial> {
        let classified = classify_store_json_residue_occurrences(occurrences)?;
        Ok(Self { classified })
    }

    pub fn classified(&self) -> &[StoreJsonResidueClassification] {
        &self.classified
    }

    pub fn contains_zone(&self, zone: StoreJsonResidueZone) -> bool {
        self.classified
            .iter()
            .any(|classification| classification.zone() == zone)
    }

    pub fn dedicated_workspace_classified(
        &self,
    ) -> impl Iterator<Item = &StoreJsonResidueClassification> {
        self.classified.iter().filter(|classification| {
            classification
                .occurrence()
                .path()
                .starts_with("workspaces/worth-store/")
        })
    }
}
