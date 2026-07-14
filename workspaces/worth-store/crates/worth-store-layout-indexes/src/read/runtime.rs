use super::{
    denial::LayoutReadAdmissionDenied,
    request::{PageLookupRequest, WalLookupRequest},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutReadRuntime;

pub const fn layout_read_runtime() -> LayoutReadRuntime {
    LayoutReadRuntime
}

impl LayoutReadRuntime {
    pub fn prepare_page_lookup(
        self,
        request: PageLookupRequest<'_>,
    ) -> Result<crate::BTreeLookupReadinessOutcome, LayoutReadAdmissionDenied> {
        super::page_lookup::prepare(request)
    }

    pub fn execute_page_lookup(
        self,
        request: PageLookupRequest<'_>,
    ) -> Result<crate::BTreeLookupExecutionOutcome, LayoutReadAdmissionDenied> {
        super::page_lookup::execute(request)
    }

    pub fn execute_wal_lookup(
        self,
        request: WalLookupRequest<'_>,
    ) -> Result<crate::BaselineLsmLookupExecution, LayoutReadAdmissionDenied> {
        super::wal_lookup::execute(request)
    }

    pub fn prepare_wal_lookup(
        self,
        request: WalLookupRequest<'_>,
    ) -> Result<crate::BaselineLsmLookupAdmissionOutcome, LayoutReadAdmissionDenied> {
        super::wal_lookup::prepare(request)
    }
}
