use crate::WorthServerProductOperationBaseDigest;

use super::WorthServerDurableProductMutationCompletion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDurableProductMutationDisposition {
    Committed,
    PreviouslyCommitted,
}

impl WorthServerDurableProductMutationDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Committed => "committed",
            Self::PreviouslyCommitted => "previously-committed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDurableProductMutationReceipt {
    disposition: WorthServerDurableProductMutationDisposition,
    request_digest: String,
    completion_digest: String,
    next_basis: WorthServerProductOperationBaseDigest,
    product_commit_digest: String,
}

impl WorthServerDurableProductMutationReceipt {
    pub(super) fn from_completion(
        completion: &WorthServerDurableProductMutationCompletion,
        disposition: WorthServerDurableProductMutationDisposition,
    ) -> Self {
        Self {
            disposition,
            request_digest: completion.request_digest().to_string(),
            completion_digest: completion.canonical_digest().to_string(),
            next_basis: completion.next_basis().clone(),
            product_commit_digest: completion.product_commit_digest().to_string(),
        }
    }

    pub fn disposition(&self) -> WorthServerDurableProductMutationDisposition {
        self.disposition
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn completion_digest(&self) -> &str {
        &self.completion_digest
    }

    pub fn next_basis(&self) -> &WorthServerProductOperationBaseDigest {
        &self.next_basis
    }

    pub fn product_commit_digest(&self) -> &str {
        &self.product_commit_digest
    }
}
