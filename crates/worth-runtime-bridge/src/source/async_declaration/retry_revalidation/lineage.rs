use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::request_identity::AdmittedBridgeAsyncRequestIdentity;
use super::class::BridgeAsyncForwardCausalityClass;
use super::counters::BridgeAsyncForwardCausalityCounters;
use super::receipt::{BridgeAsyncForwardCausalityIdentity, BridgeAsyncForwardCausalityReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncRetryLineage {
    causality_identity: BridgeAsyncForwardCausalityIdentity,
    prior_request: AdmittedBridgeAsyncRequestIdentity,
    newer_request: AdmittedBridgeAsyncRequestIdentity,
    class: BridgeAsyncForwardCausalityClass,
    counters: BridgeAsyncForwardCausalityCounters,
    receipt: BridgeAsyncForwardCausalityReceipt,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeAsyncRevalidationLineage {
    causality_identity: BridgeAsyncForwardCausalityIdentity,
    prior_request: AdmittedBridgeAsyncRequestIdentity,
    newer_request: AdmittedBridgeAsyncRequestIdentity,
    class: BridgeAsyncForwardCausalityClass,
    counters: BridgeAsyncForwardCausalityCounters,
    receipt: BridgeAsyncForwardCausalityReceipt,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeAsyncRetryLineage {
    pub(crate) fn new(
        prior_request: AdmittedBridgeAsyncRequestIdentity,
        newer_request: AdmittedBridgeAsyncRequestIdentity,
        class: BridgeAsyncForwardCausalityClass,
        counters: BridgeAsyncForwardCausalityCounters,
        canonical_basis: Arc<str>,
    ) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let causality_identity = BridgeAsyncForwardCausalityIdentity::admit_bridge_owned(format!(
            "bridge-async-forward-causality-id:sha256:{digest:x}"
        ));
        let receipt = BridgeAsyncForwardCausalityReceipt::new(
            &causality_identity,
            class,
            Arc::from(format!(
                "bridge-async-forward-causality-receipt|class={class:?}|prior={}|newer={}",
                prior_request.request_identity().as_str(),
                newer_request.request_identity().as_str(),
            )),
        );
        Self {
            causality_identity,
            prior_request,
            newer_request,
            class,
            counters,
            receipt,
            canonical_basis,
            digest: Arc::from(format!("bridge-async-forward-causality:sha256:{digest:x}")),
        }
    }

    pub fn causality_identity(&self) -> &str {
        self.causality_identity.as_str()
    }

    pub fn prior_request(&self) -> &AdmittedBridgeAsyncRequestIdentity {
        &self.prior_request
    }

    pub fn newer_request(&self) -> &AdmittedBridgeAsyncRequestIdentity {
        &self.newer_request
    }

    pub fn class(&self) -> BridgeAsyncForwardCausalityClass {
        self.class
    }

    pub fn counters(&self) -> &BridgeAsyncForwardCausalityCounters {
        &self.counters
    }

    pub fn receipt(&self) -> &BridgeAsyncForwardCausalityReceipt {
        &self.receipt
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

impl BridgeAsyncRevalidationLineage {
    pub(crate) fn new(
        prior_request: AdmittedBridgeAsyncRequestIdentity,
        newer_request: AdmittedBridgeAsyncRequestIdentity,
        class: BridgeAsyncForwardCausalityClass,
        counters: BridgeAsyncForwardCausalityCounters,
        canonical_basis: Arc<str>,
    ) -> Self {
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let causality_identity = BridgeAsyncForwardCausalityIdentity::admit_bridge_owned(format!(
            "bridge-async-forward-causality-id:sha256:{digest:x}"
        ));
        let receipt = BridgeAsyncForwardCausalityReceipt::new(
            &causality_identity,
            class,
            Arc::from(format!(
                "bridge-async-forward-causality-receipt|class={class:?}|prior={}|newer={}",
                prior_request.request_identity().as_str(),
                newer_request.request_identity().as_str(),
            )),
        );
        Self {
            causality_identity,
            prior_request,
            newer_request,
            class,
            counters,
            receipt,
            canonical_basis,
            digest: Arc::from(format!("bridge-async-forward-causality:sha256:{digest:x}")),
        }
    }

    pub fn causality_identity(&self) -> &str {
        self.causality_identity.as_str()
    }

    pub fn prior_request(&self) -> &AdmittedBridgeAsyncRequestIdentity {
        &self.prior_request
    }

    pub fn newer_request(&self) -> &AdmittedBridgeAsyncRequestIdentity {
        &self.newer_request
    }

    pub fn class(&self) -> BridgeAsyncForwardCausalityClass {
        self.class
    }

    pub fn counters(&self) -> &BridgeAsyncForwardCausalityCounters {
        &self.counters
    }

    pub fn receipt(&self) -> &BridgeAsyncForwardCausalityReceipt {
        &self.receipt
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
