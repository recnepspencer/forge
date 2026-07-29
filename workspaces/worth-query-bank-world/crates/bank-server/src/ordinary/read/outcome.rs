use bank_domain::model::ReadOutcome;
use worth_query_host::facade::domain::WorthQueryApplicationOperationInstallationDenialKind;
use worth_query_host::facade::primary_graph::{
    WorthQueryEntityResolutionDenialKind, WorthQueryInvariantProjectionWork,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryOrdinaryReadMetadata,
    WorthQueryOrdinaryReadVersion,
};

use crate::{BankActivityCursorDenial, BankProjectionDenial};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BankReadDenial {
    Scope(WorthQueryEntityResolutionDenialKind),
    Installation(WorthQueryApplicationOperationInstallationDenialKind),
    Authorization(WorthQueryOperationAuthorizationDenialKind),
    ActivityCursor(BankActivityCursorDenial),
    Projection(BankProjectionDenial),
    ProjectionWorkBudgetExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BankReadMetadata {
    version: WorthQueryOrdinaryReadVersion,
    work: WorthQueryInvariantProjectionWork,
    result_count: usize,
    truncated: bool,
}

impl From<WorthQueryOrdinaryReadMetadata> for BankReadMetadata {
    fn from(metadata: WorthQueryOrdinaryReadMetadata) -> Self {
        Self {
            version: metadata.version(),
            work: metadata.work(),
            result_count: metadata.result_count(),
            truncated: metadata.truncated(),
        }
    }
}

impl BankReadMetadata {
    pub const fn version(self) -> WorthQueryOrdinaryReadVersion {
        self.version
    }

    pub const fn work(self) -> WorthQueryInvariantProjectionWork {
        self.work
    }

    pub const fn result_count(self) -> usize {
        self.result_count
    }

    pub const fn truncated(self) -> bool {
        self.truncated
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankReadResult<Output> {
    output: Output,
    metadata: BankReadMetadata,
}

impl<Output> BankReadResult<Output> {
    pub(super) const fn new(output: Output, metadata: BankReadMetadata) -> Self {
        Self { output, metadata }
    }

    pub const fn output(&self) -> &Output {
        &self.output
    }

    pub const fn metadata(&self) -> BankReadMetadata {
        self.metadata
    }

    pub fn into_output(self) -> Output {
        self.output
    }

    pub(super) fn map_output<Mapped>(
        self,
        map: impl FnOnce(Output, BankReadMetadata) -> Mapped,
    ) -> BankReadResult<Mapped> {
        let output = map(self.output, self.metadata);
        BankReadResult {
            output,
            metadata: self.metadata,
        }
    }
}

pub type BankReadOutcome<Output> = ReadOutcome<BankReadResult<Output>, BankReadDenial>;
