use super::super::error::{EvidenceLookupFamilyCatalogError, EvidenceLookupFamilyCatalogErrorKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupEvidenceClass {
    BooleanStageReceipt,
    SpatialTouchEvidence,
    TopologyDerivedReceiptReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupEvidenceClassSet {
    classes: Vec<EvidenceLookupEvidenceClass>,
}

impl EvidenceLookupEvidenceClassSet {
    pub(crate) fn new(
        classes: Vec<EvidenceLookupEvidenceClass>,
    ) -> Result<Self, EvidenceLookupFamilyCatalogError> {
        if classes.is_empty() {
            return Err(EvidenceLookupFamilyCatalogError::new(
                EvidenceLookupFamilyCatalogErrorKind::EmptyEvidenceClassSet,
            ));
        }
        if has_duplicate_class(&classes) {
            return Err(EvidenceLookupFamilyCatalogError::new(
                EvidenceLookupFamilyCatalogErrorKind::DuplicateEvidenceClass,
            ));
        }
        Ok(Self { classes })
    }

    pub fn classes(&self) -> &[EvidenceLookupEvidenceClass] {
        &self.classes
    }

    pub fn class_count(&self) -> usize {
        self.classes.len()
    }
}

fn has_duplicate_class(classes: &[EvidenceLookupEvidenceClass]) -> bool {
    for (index, class) in classes.iter().enumerate() {
        if classes[index + 1..].contains(class) {
            return true;
        }
    }
    false
}
