use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeSubscriptionPreviewResidueCategory;
use crate::subscription::{
    BridgePreviewActiveSubscriptionIdentity, BridgeSubscriptionCounters,
    BridgeSubscriptionPreviewResidueScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewDiscardResidueRejectionKind {
    PreviewActiveMismatch,
    PreviewResidueScopeMismatch,
    MissingResidueCategory,
    DuplicateResidueCategory,
    NonzeroResidue,
}

impl BridgeSubscriptionPreviewDiscardResidueRejectionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewActiveMismatch => "preview_active_mismatch",
            Self::PreviewResidueScopeMismatch => "preview_residue_scope_mismatch",
            Self::MissingResidueCategory => "missing_residue_category",
            Self::DuplicateResidueCategory => "duplicate_residue_category",
            Self::NonzeroResidue => "nonzero_residue",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewResidueCategoryCount {
    category: BridgeSubscriptionPreviewResidueCategory,
    residue_count: usize,
}

impl BridgeSubscriptionPreviewResidueCategoryCount {
    pub(super) fn new(
        category: BridgeSubscriptionPreviewResidueCategory,
        residue_count: usize,
    ) -> Self {
        Self {
            category,
            residue_count,
        }
    }

    pub fn category(&self) -> BridgeSubscriptionPreviewResidueCategory {
        self.category
    }

    pub fn residue_count(&self) -> usize {
        self.residue_count
    }

    fn canonical_basis(&self) -> String {
        format!("{}={}", self.category.as_str(), self.residue_count)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeSubscriptionPreviewDiscardResidueRejectionContext {
    PreviewActiveMismatch {
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        index_preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    },
    PreviewResidueScopeMismatch {
        preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
        index_preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    },
    MissingResidueCategory {
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        missing_category: BridgeSubscriptionPreviewResidueCategory,
    },
    DuplicateResidueCategory {
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        duplicate_category: BridgeSubscriptionPreviewResidueCategory,
    },
    NonzeroResidue {
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        nonzero_categories: Arc<[BridgeSubscriptionPreviewResidueCategoryCount]>,
    },
}

impl BridgeSubscriptionPreviewDiscardResidueRejectionContext {
    pub(super) fn preview_active_mismatch(
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        index_preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
    ) -> Self {
        Self::PreviewActiveMismatch {
            preview_active_subscription_identity,
            index_preview_active_subscription_identity,
        }
    }

    pub(super) fn preview_residue_scope_mismatch(
        preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
        index_preview_residue_scope_identity: BridgeSubscriptionPreviewResidueScopeIdentity,
    ) -> Self {
        Self::PreviewResidueScopeMismatch {
            preview_residue_scope_identity,
            index_preview_residue_scope_identity,
        }
    }

    pub(super) fn missing_category(
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        missing_category: BridgeSubscriptionPreviewResidueCategory,
    ) -> Self {
        Self::MissingResidueCategory {
            preview_active_subscription_identity,
            missing_category,
        }
    }

    pub(super) fn duplicate_category(
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        duplicate_category: BridgeSubscriptionPreviewResidueCategory,
    ) -> Self {
        Self::DuplicateResidueCategory {
            preview_active_subscription_identity,
            duplicate_category,
        }
    }

    pub(super) fn nonzero_residue(
        preview_active_subscription_identity: BridgePreviewActiveSubscriptionIdentity,
        nonzero_categories: Vec<BridgeSubscriptionPreviewResidueCategoryCount>,
    ) -> Self {
        Self::NonzeroResidue {
            preview_active_subscription_identity,
            nonzero_categories: Arc::from(nonzero_categories),
        }
    }

    pub fn missing_category_value(&self) -> Option<BridgeSubscriptionPreviewResidueCategory> {
        match self {
            Self::MissingResidueCategory {
                missing_category, ..
            } => Some(*missing_category),
            _ => None,
        }
    }

    pub fn duplicate_category_value(&self) -> Option<BridgeSubscriptionPreviewResidueCategory> {
        match self {
            Self::DuplicateResidueCategory {
                duplicate_category, ..
            } => Some(*duplicate_category),
            _ => None,
        }
    }

    pub fn nonzero_categories(&self) -> &[BridgeSubscriptionPreviewResidueCategoryCount] {
        match self {
            Self::NonzeroResidue {
                nonzero_categories, ..
            } => nonzero_categories.as_ref(),
            _ => &[],
        }
    }

    fn canonical_basis(&self) -> String {
        match self {
            Self::PreviewActiveMismatch {
                preview_active_subscription_identity,
                index_preview_active_subscription_identity,
            } => format!(
                "preview-active={}|index-preview-active={}",
                preview_active_subscription_identity.as_str(),
                index_preview_active_subscription_identity.as_str()
            ),
            Self::PreviewResidueScopeMismatch {
                preview_residue_scope_identity,
                index_preview_residue_scope_identity,
            } => format!(
                "preview-scope={}|index-scope={}",
                preview_residue_scope_identity.as_str(),
                index_preview_residue_scope_identity.as_str()
            ),
            Self::MissingResidueCategory {
                preview_active_subscription_identity,
                missing_category,
            } => format!(
                "preview-active={}|missing-category={}",
                preview_active_subscription_identity.as_str(),
                missing_category.as_str()
            ),
            Self::DuplicateResidueCategory {
                preview_active_subscription_identity,
                duplicate_category,
            } => format!(
                "preview-active={}|duplicate-category={}",
                preview_active_subscription_identity.as_str(),
                duplicate_category.as_str()
            ),
            Self::NonzeroResidue {
                preview_active_subscription_identity,
                nonzero_categories,
            } => format!(
                "preview-active={}|nonzero={}",
                preview_active_subscription_identity.as_str(),
                nonzero_categories
                    .iter()
                    .map(BridgeSubscriptionPreviewResidueCategoryCount::canonical_basis)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionPreviewDiscardResidueRejection {
    rejection_kind: BridgeSubscriptionPreviewDiscardResidueRejectionKind,
    rejection_context: BridgeSubscriptionPreviewDiscardResidueRejectionContext,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionPreviewDiscardResidueRejection {
    pub(super) fn new(
        rejection_kind: BridgeSubscriptionPreviewDiscardResidueRejectionKind,
        rejection_context: BridgeSubscriptionPreviewDiscardResidueRejectionContext,
        nonzero_residue: bool,
        residue_check_count: usize,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-preview-discard-residue-rejection|kind={}|context={}",
            rejection_kind.as_str(),
            rejection_context.canonical_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rejection_kind,
            rejection_context,
            counters: BridgeSubscriptionCounters::from_subscription_preview_discard_rejection(
                nonzero_residue,
                residue_check_count,
            ),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-preview-discard-residue-rejection:sha256:{digest:x}"
            )),
        }
    }

    pub fn rejection_kind(&self) -> BridgeSubscriptionPreviewDiscardResidueRejectionKind {
        self.rejection_kind
    }

    pub fn rejection_context(&self) -> &BridgeSubscriptionPreviewDiscardResidueRejectionContext {
        &self.rejection_context
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
