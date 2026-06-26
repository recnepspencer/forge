mod diagnostic_projection;
mod execution;
mod witness_input;
mod witness_intake;
mod witness_row;

pub use diagnostic_projection::WorthTopologyLoopWiringDiagnosticProjection;
pub use witness_input::{
    WorthTopologyLoopWiringHalfEdgeWitnessRow, WorthTopologyLoopWiringLoopWitnessRow,
    WorthTopologyLoopWiringWitnessInput,
};
pub use witness_intake::{
    WorthTopologyLoopWiringAdmittedLocalFacts, WorthTopologyLoopWiringWitnessIntakeReceipt,
};
pub use witness_row::{WorthTopologyLoopWiringViolationKind, WorthTopologyLoopWiringWitnessRow};

pub(in crate::validator_invariant_catalog) use diagnostic_projection::loop_wiring_diagnostic_projection;
pub(in crate::validator_invariant_catalog) use execution::execute_loop_wiring_obligation;
pub(in crate::validator_invariant_catalog) use witness_intake::admit_loop_wiring_witness_input;
