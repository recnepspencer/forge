use serde::Serialize;

use super::classification::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationAuthorityOwner,
    DerivedInvalidationOldAuthorityKind, DerivedInvalidationProductCategory,
    DerivedInvalidationReplacementPhase,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationAuthorityInventoryRow {
    source_path: &'static str,
    surface: &'static str,
    product_category: DerivedInvalidationProductCategory,
    authority_kind: DerivedInvalidationOldAuthorityKind,
    disposition: DerivedInvalidationAuthorityDisposition,
    owner: DerivedInvalidationAuthorityOwner,
    blocker: &'static str,
    removal_trigger: &'static str,
    replacement_phase: DerivedInvalidationReplacementPhase,
    ordinary_path: bool,
    certification_or_bootstrap_only: bool,
    cap: Option<usize>,
    row_digest: String,
}

impl DerivedInvalidationAuthorityInventoryRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        source_path: &'static str,
        surface: &'static str,
        product_category: DerivedInvalidationProductCategory,
        authority_kind: DerivedInvalidationOldAuthorityKind,
        disposition: DerivedInvalidationAuthorityDisposition,
        owner: DerivedInvalidationAuthorityOwner,
        blocker: &'static str,
        removal_trigger: &'static str,
        replacement_phase: DerivedInvalidationReplacementPhase,
        ordinary_path: bool,
        certification_or_bootstrap_only: bool,
        cap: Option<usize>,
    ) -> Self {
        let row_digest = digest_strings(vec![
            source_path.to_string(),
            surface.to_string(),
            product_category.as_str().to_string(),
            authority_kind.as_str().to_string(),
            disposition.as_str().to_string(),
            owner.as_str().to_string(),
            blocker.to_string(),
            removal_trigger.to_string(),
            replacement_phase.as_str().to_string(),
            if ordinary_path {
                "ordinary"
            } else {
                "nonordinary"
            }
            .to_string(),
            if certification_or_bootstrap_only {
                "certification_bootstrap"
            } else {
                "not_certification_bootstrap"
            }
            .to_string(),
            cap.map_or("uncapped".to_string(), |cap| format!("cap:{cap}")),
        ]);
        Self {
            source_path,
            surface,
            product_category,
            authority_kind,
            disposition,
            owner,
            blocker,
            removal_trigger,
            replacement_phase,
            ordinary_path,
            certification_or_bootstrap_only,
            cap,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        self.source_path
    }

    pub fn surface(&self) -> &str {
        self.surface
    }

    pub fn product_category(&self) -> DerivedInvalidationProductCategory {
        self.product_category
    }

    pub fn authority_kind(&self) -> DerivedInvalidationOldAuthorityKind {
        self.authority_kind
    }

    pub fn disposition(&self) -> DerivedInvalidationAuthorityDisposition {
        self.disposition
    }

    pub fn owner(&self) -> DerivedInvalidationAuthorityOwner {
        self.owner
    }

    pub fn blocker(&self) -> &str {
        self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        self.removal_trigger
    }

    pub fn replacement_phase(&self) -> DerivedInvalidationReplacementPhase {
        self.replacement_phase
    }

    pub fn ordinary_path(&self) -> bool {
        self.ordinary_path
    }

    pub fn certification_or_bootstrap_only(&self) -> bool {
        self.certification_or_bootstrap_only
    }

    pub fn cap(&self) -> Option<usize> {
        self.cap
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(crate) fn has_blocker_and_trigger(&self) -> bool {
        !self.blocker.trim().is_empty() && !self.removal_trigger.trim().is_empty()
    }
}

pub(crate) fn digest_strings(mut normalized: Vec<String>) -> String {
    normalized.sort_unstable();
    format!("derived-invalidation:{}", normalized.join("|"))
}
