mod legal_execution;
pub(in crate::courtroom::protocol_models) mod ordinary_execution;
mod owner_coverage;
mod report;
mod report_evidence;

pub use legal_execution::{
    run_checked_protocol_program, CheckedProtocolExecution, CheckedProtocolExecutionReport,
    CheckedProtocolProgramFailure,
};
pub use ordinary_execution::{OrdinaryProtocolExecutionDenial, OrdinaryProtocolExecutionSuite};
pub use report::{
    adjudicate_protocol_law_closeout, ExactOwnerMappingEvidence, ProtocolCloseoutCounters,
    ProtocolCloseoutDenial, ProtocolCloseoutRow, ProtocolLawCloseoutReport, ProtocolResidualRisk,
};
pub use report_evidence::{
    CounterexampleDiagnosticEvidence, CounterexampleDiagnosticEvidenceDenial,
};

#[cfg(test)]
mod tests;
