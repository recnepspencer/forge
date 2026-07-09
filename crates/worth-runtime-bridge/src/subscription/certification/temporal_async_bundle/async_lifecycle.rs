use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::source::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncCompletionReceipt,
    BridgeAsyncCompletionSupersessionReceipt, BridgeAsyncDeniedCompletionReceipt,
    BridgeAsyncForwardCausalityReceipt, BridgeAsyncWritebackCausalityTransferReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationAsyncSectionInput {
    request_identities: Arc<[AdmittedBridgeAsyncRequestIdentity]>,
    completion_receipts: Arc<[BridgeAsyncCompletionReceipt]>,
    denied_completion_receipts: Arc<[BridgeAsyncDeniedCompletionReceipt]>,
    supersession_receipts: Arc<[BridgeAsyncCompletionSupersessionReceipt]>,
    forward_causality_receipts: Arc<[BridgeAsyncForwardCausalityReceipt]>,
    writeback_receipts: Arc<[BridgeAsyncWritebackCausalityTransferReceipt]>,
}

impl BridgeTemporalAsyncCertificationAsyncSectionInput {
    pub fn new(
        request_identities: Vec<AdmittedBridgeAsyncRequestIdentity>,
        completion_receipts: Vec<BridgeAsyncCompletionReceipt>,
        denied_completion_receipts: Vec<BridgeAsyncDeniedCompletionReceipt>,
        supersession_receipts: Vec<BridgeAsyncCompletionSupersessionReceipt>,
        forward_causality_receipts: Vec<BridgeAsyncForwardCausalityReceipt>,
        writeback_receipts: Vec<BridgeAsyncWritebackCausalityTransferReceipt>,
    ) -> Self {
        Self {
            request_identities: request_identities.into(),
            completion_receipts: completion_receipts.into(),
            denied_completion_receipts: denied_completion_receipts.into(),
            supersession_receipts: supersession_receipts.into(),
            forward_causality_receipts: forward_causality_receipts.into(),
            writeback_receipts: writeback_receipts.into(),
        }
    }

    pub(crate) fn request_identities(&self) -> &[AdmittedBridgeAsyncRequestIdentity] {
        self.request_identities.as_ref()
    }

    pub(crate) fn completion_receipts(&self) -> &[BridgeAsyncCompletionReceipt] {
        self.completion_receipts.as_ref()
    }

    pub(crate) fn denied_completion_receipts(&self) -> &[BridgeAsyncDeniedCompletionReceipt] {
        self.denied_completion_receipts.as_ref()
    }

    pub(crate) fn supersession_receipts(&self) -> &[BridgeAsyncCompletionSupersessionReceipt] {
        self.supersession_receipts.as_ref()
    }

    pub(crate) fn forward_causality_receipts(&self) -> &[BridgeAsyncForwardCausalityReceipt] {
        self.forward_causality_receipts.as_ref()
    }

    pub(crate) fn writeback_receipts(&self) -> &[BridgeAsyncWritebackCausalityTransferReceipt] {
        self.writeback_receipts.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeTemporalAsyncCertificationAsyncLifecycleSection {
    bridge_owner: Arc<str>,
    signal_owner: Arc<str>,
    request_identity_digests: Arc<[Arc<str>]>,
    completion_receipt_digests: Arc<[Arc<str>]>,
    denied_completion_receipt_digests: Arc<[Arc<str>]>,
    supersession_receipt_digests: Arc<[Arc<str>]>,
    forward_causality_receipt_digests: Arc<[Arc<str>]>,
    writeback_receipt_digests: Arc<[Arc<str>]>,
    semantic_digest: Arc<str>,
    digest: Arc<str>,
}

impl BridgeTemporalAsyncCertificationAsyncLifecycleSection {
    pub(crate) fn collect(input: &BridgeTemporalAsyncCertificationAsyncSectionInput) -> Self {
        let request_identity_digests = arc_digests(
            input
                .request_identities()
                .iter()
                .map(|identity| identity.digest()),
        );
        let completion_receipt_digests = arc_digests(
            input
                .completion_receipts()
                .iter()
                .map(|receipt| receipt.digest()),
        );
        let denied_completion_receipt_digests = arc_digests(
            input
                .denied_completion_receipts()
                .iter()
                .map(|receipt| receipt.digest()),
        );
        let supersession_receipt_digests = arc_digests(
            input
                .supersession_receipts()
                .iter()
                .map(|receipt| receipt.digest()),
        );
        let forward_causality_receipt_digests = arc_digests(
            input
                .forward_causality_receipts()
                .iter()
                .map(|receipt| receipt.digest()),
        );
        let writeback_receipt_digests = arc_digests(
            input
                .writeback_receipts()
                .iter()
                .map(|receipt| receipt.digest()),
        );
        let semantic_basis = format!(
            "bridge-temporal-async-certification-async-section|requests={}|completions={}|denied={}|supersession={}|forward={}|writeback={}",
            join(&request_identity_digests),
            join(&completion_receipt_digests),
            join(&denied_completion_receipt_digests),
            join(&supersession_receipt_digests),
            join(&forward_causality_receipt_digests),
            join(&writeback_receipt_digests),
        );
        let semantic_digest = Sha256::digest(semantic_basis.as_bytes());
        let digest = Sha256::digest(
            format!("{semantic_basis}|bridge-owner=worth-runtime-bridge|signal-owner=worth-signal")
                .as_bytes(),
        );
        Self {
            bridge_owner: Arc::from("worth-runtime-bridge"),
            signal_owner: Arc::from("worth-signal"),
            request_identity_digests,
            completion_receipt_digests,
            denied_completion_receipt_digests,
            supersession_receipt_digests,
            forward_causality_receipt_digests,
            writeback_receipt_digests,
            semantic_digest: Arc::from(format!(
                "bridge-temporal-async-certification-async-section-semantic:sha256:{semantic_digest:x}"
            )),
            digest: Arc::from(format!(
                "bridge-temporal-async-certification-async-section:sha256:{digest:x}"
            )),
        }
    }

    pub fn bridge_owner(&self) -> &str {
        self.bridge_owner.as_ref()
    }

    pub fn signal_owner(&self) -> &str {
        self.signal_owner.as_ref()
    }

    pub fn request_identity_count(&self) -> usize {
        self.request_identity_digests.len()
    }

    pub fn semantic_digest(&self) -> &str {
        self.semantic_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn arc_digests<'a>(values: impl Iterator<Item = &'a str>) -> Arc<[Arc<str>]> {
    values
        .map(|value| Arc::from(value.to_owned()))
        .collect::<Vec<_>>()
        .into()
}

fn join(values: &[Arc<str>]) -> String {
    values
        .iter()
        .map(|value| value.as_ref())
        .collect::<Vec<_>>()
        .join(",")
}
