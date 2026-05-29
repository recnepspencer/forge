pub(crate) mod closeout;
pub(crate) mod fallback;
pub(crate) mod ledger;
pub(crate) mod no_n_plus_one;
#[allow(dead_code)]
pub(crate) mod parity;
pub(crate) mod report;
mod report_surface;
mod surface;

pub use closeout::{
    TopologyDomainQueryCloseoutReport, TopologyDomainQueryCloseoutRow,
    TopologyDomainQueryCloseoutStatus, TopologyDomainQueryPhaseThreeBlocker,
    TopologyDomainQueryPhaseThreeBlockerRow, TopologyDomainQueryPhaseThreeBlockerStatus,
};
pub use fallback::TopologyDomainQueryFallbackPosture;
pub use ledger::TopologyDomainQueryProofReport;
pub use no_n_plus_one::{
    TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus,
};
pub use parity::{
    TopologyDomainQueryParityAggregateReport, TopologyDomainQueryParityAggregateRow,
    TopologyDomainQueryParityKind,
};
pub use report::{
    TopologyDomainQueryAggregateReport, TopologyDomainQueryDebtRow,
    TopologyDomainQueryExecutionAggregateRow, TopologyDomainQueryExecutionEngine,
    TopologyDomainQueryFamilyAggregateRow, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport,
};




