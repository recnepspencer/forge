use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;

use super::decision::EvidenceLookupIndexReuseDecision;
use super::denial::EvidenceLookupIndexRebuildDenial;
use super::posture::EvidenceLookupReuseDecisionPosture;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceLookupIndexReuseResolution {
    Reused {
        decision: EvidenceLookupIndexReuseDecision,
        product: EvidenceLookupIndexProduct,
    },
    Rebuilt {
        decision: EvidenceLookupIndexReuseDecision,
        product: EvidenceLookupIndexProduct,
    },
    Denied {
        decision: EvidenceLookupIndexReuseDecision,
        denial: EvidenceLookupIndexRebuildDenial,
    },
}

impl EvidenceLookupIndexReuseResolution {
    pub const fn posture(&self) -> EvidenceLookupReuseDecisionPosture {
        match self {
            Self::Reused { decision, .. }
            | Self::Rebuilt { decision, .. }
            | Self::Denied { decision, .. } => decision.posture(),
        }
    }

    pub const fn decision(&self) -> &EvidenceLookupIndexReuseDecision {
        match self {
            Self::Reused { decision, .. }
            | Self::Rebuilt { decision, .. }
            | Self::Denied { decision, .. } => decision,
        }
    }

    pub const fn product(&self) -> Option<&EvidenceLookupIndexProduct> {
        match self {
            Self::Reused { product, .. } | Self::Rebuilt { product, .. } => Some(product),
            Self::Denied { .. } => None,
        }
    }

    pub const fn denial(&self) -> Option<&EvidenceLookupIndexRebuildDenial> {
        match self {
            Self::Denied { denial, .. } => Some(denial),
            Self::Reused { .. } | Self::Rebuilt { .. } => None,
        }
    }
}
