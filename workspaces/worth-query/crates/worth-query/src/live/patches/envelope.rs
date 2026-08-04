use super::super::promotion::{LiveQueryFamily, LiveQueryPlan};
use super::super::refresh::{CoalescingDecision, RefreshFallback};
use super::bounded_materialization::BoundedMaterializationPatch;
use super::detail::{DetailPatch, SuppressionReason};
use super::ordered_collection::OrderedCollectionPatch;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LivePatchDigest(String);

impl LivePatchDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(in crate::live) fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LivePatchPayload {
    Detail(DetailPatch),
    OrderedCollection(OrderedCollectionPatch),
    BoundedMaterialization(BoundedMaterializationPatch),
    Suppressed(SuppressionReason),
    Refresh(RefreshFallback),
    ProgressAdvance { ordinal: u64 },
    Coalesced(CoalescingDecision),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LivePatchEnvelope {
    pub(in crate::live) query_digest: String,
    pub(in crate::live) result_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) replay_digest: String,
    pub(in crate::live) basis_digest: String,
    pub(in crate::live) subscription_digest: String,
    pub(in crate::live) family: LiveQueryFamily,
    pub(in crate::live) payload: LivePatchPayload,
}

impl LivePatchEnvelope {
    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn subscription_digest(&self) -> &str {
        &self.subscription_digest
    }

    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn payload(&self) -> &LivePatchPayload {
        &self.payload
    }
}

impl LiveQueryPlan {
    pub(in crate::live) fn patch_digest(&self, extra_parts: &[String]) -> LivePatchDigest {
        let mut digest_parts = vec![
            format!("query:{}", self.descriptor.query_digest().as_str()),
            format!("family:{}", self.descriptor.family().as_str()),
        ];
        digest_parts.extend(extra_parts.iter().cloned());
        LivePatchDigest::from_parts(&digest_parts)
    }
}
