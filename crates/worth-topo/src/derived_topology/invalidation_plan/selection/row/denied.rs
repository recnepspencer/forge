use serde::Serialize;

use crate::derived_topology::invalidation_plan::catalog::{
    DerivedTopologyLegalityReceiptPosture, DerivedTopologyProductFamilyIdentity,
    DerivedTopologyProductFamilyRecord, DerivedTopologyQueryReceiptPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationDenialKind {
    MissingQuerySupport,
    MissingLegalitySupport,
}

impl DerivedInvalidationDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingQuerySupport => "missing_query_support",
            Self::MissingLegalitySupport => "missing_legality_support",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDenialRow {
    kind: DerivedInvalidationDenialKind,
    family_identity: DerivedTopologyProductFamilyIdentity,
    family_digest: String,
    required_query_posture: Option<DerivedTopologyQueryReceiptPosture>,
    required_legality_posture: Option<DerivedTopologyLegalityReceiptPosture>,
    denial_digest: String,
}

impl DerivedInvalidationDenialRow {
    pub(crate) fn missing_query_support(family: &DerivedTopologyProductFamilyRecord) -> Self {
        Self::new(
            DerivedInvalidationDenialKind::MissingQuerySupport,
            family,
            Some(family.query_receipt_posture()),
            None,
        )
    }

    pub(crate) fn missing_legality_support(family: &DerivedTopologyProductFamilyRecord) -> Self {
        Self::new(
            DerivedInvalidationDenialKind::MissingLegalitySupport,
            family,
            None,
            Some(family.legality_receipt_posture()),
        )
    }

    fn new(
        kind: DerivedInvalidationDenialKind,
        family: &DerivedTopologyProductFamilyRecord,
        required_query_posture: Option<DerivedTopologyQueryReceiptPosture>,
        required_legality_posture: Option<DerivedTopologyLegalityReceiptPosture>,
    ) -> Self {
        let denial_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-denial-row:v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("family:{}", family.identity().as_str()),
            format!("family-digest:{}", family.family_digest()),
            format!(
                "query:{}",
                required_query_posture
                    .map(DerivedTopologyQueryReceiptPosture::as_str)
                    .unwrap_or("not-applicable")
            ),
            format!(
                "legality:{}",
                required_legality_posture
                    .map(DerivedTopologyLegalityReceiptPosture::as_str)
                    .unwrap_or("not-applicable")
            ),
        ]);
        Self {
            kind,
            family_identity: family.identity(),
            family_digest: family.family_digest().to_string(),
            required_query_posture,
            required_legality_posture,
            denial_digest,
        }
    }

    pub const fn kind(&self) -> DerivedInvalidationDenialKind {
        self.kind
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub const fn required_query_posture(&self) -> Option<DerivedTopologyQueryReceiptPosture> {
        self.required_query_posture
    }

    pub const fn required_legality_posture(&self) -> Option<DerivedTopologyLegalityReceiptPosture> {
        self.required_legality_posture
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}
