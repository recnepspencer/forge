use crate::runtime::WorthUiQuerySupportStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQuerySupportDenialKind {
    Deferred,
    Unsupported,
    LiveRebindDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQuerySupportDenialReceipt {
    kind: WorthUiQuerySupportDenialKind,
    support_status: WorthUiQuerySupportStatus,
    support_receipt_digest: u64,
    runtime_hook_count: usize,
    denied_binding_count: usize,
}

impl WorthUiQuerySupportDenialReceipt {
    pub(crate) fn support_not_admitted(
        support_status: WorthUiQuerySupportStatus,
        support_receipt_digest: u64,
        runtime_hook_count: usize,
    ) -> Option<Self> {
        let kind = match support_status {
            WorthUiQuerySupportStatus::Supported => return None,
            WorthUiQuerySupportStatus::Deferred => WorthUiQuerySupportDenialKind::Deferred,
            WorthUiQuerySupportStatus::Unsupported => WorthUiQuerySupportDenialKind::Unsupported,
        };
        Some(Self {
            kind,
            support_status,
            support_receipt_digest,
            runtime_hook_count,
            denied_binding_count: 0,
        })
    }

    pub(crate) fn live_rebind_denied(
        support_receipt_digest: u64,
        runtime_hook_count: usize,
        denied_binding_count: usize,
    ) -> Self {
        Self {
            kind: WorthUiQuerySupportDenialKind::LiveRebindDenied,
            support_status: WorthUiQuerySupportStatus::Supported,
            support_receipt_digest,
            runtime_hook_count,
            denied_binding_count,
        }
    }

    pub fn kind(&self) -> WorthUiQuerySupportDenialKind {
        self.kind
    }

    pub fn support_status(&self) -> WorthUiQuerySupportStatus {
        self.support_status
    }

    pub fn support_receipt_digest(&self) -> u64 {
        self.support_receipt_digest
    }

    pub fn runtime_hook_count(&self) -> usize {
        self.runtime_hook_count
    }

    pub fn denied_binding_count(&self) -> usize {
        self.denied_binding_count
    }
}
