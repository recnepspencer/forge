use crate::{ProtocolFamily, SharedFrontierDenial, SharedFrontierModel};

use super::{
    CanonicalProtocolAction, CanonicalProtocolTrace, ProtocolCheckStatistics, ProtocolCheckVerdict,
    ProtocolCounterexample, ReceiptLossClassification,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolExecutionOutcome {
    LegalProtocolExecution {
        trace: CanonicalProtocolTrace,
        statistics: ProtocolCheckStatistics,
    },
    IllegalRuntimeTransition {
        trace: CanonicalProtocolTrace,
        action_index: usize,
        denial: SharedFrontierDenial,
    },
    ReceiptOmissionDefect(ReceiptLossClassification),
    CrashLostNonAuthoritativeTrace,
    NoOwnerTransition,
    UnsupportedBackendProfile,
    BoundExhausted,
    CounterexampleFound(ProtocolCounterexample),
    DeadlockFound(ProtocolCounterexample),
}

pub fn adjudicate_shared_frontier_trace(
    verdict: ProtocolCheckVerdict,
    trace: CanonicalProtocolTrace,
) -> ProtocolExecutionOutcome {
    if trace.protocol() != ProtocolFamily::SharedFrontiers {
        return ProtocolExecutionOutcome::IllegalRuntimeTransition {
            trace,
            action_index: 0,
            denial: SharedFrontierDenial::IllegalTransition,
        };
    }
    let mut model = SharedFrontierModel::initial();
    for (action_index, action) in trace.actions().iter().copied().enumerate() {
        let CanonicalProtocolAction::SharedFrontier(action) = action else {
            return ProtocolExecutionOutcome::IllegalRuntimeTransition {
                trace,
                action_index,
                denial: SharedFrontierDenial::IllegalTransition,
            };
        };
        if let Err(denial) = model.apply(action) {
            return ProtocolExecutionOutcome::IllegalRuntimeTransition {
                trace,
                action_index,
                denial,
            };
        }
    }

    match verdict {
        ProtocolCheckVerdict::CheckedWithinBounds { statistics, .. } => {
            ProtocolExecutionOutcome::LegalProtocolExecution { trace, statistics }
        }
        ProtocolCheckVerdict::DeadlockFound { counterexample, .. } => {
            ProtocolExecutionOutcome::DeadlockFound(counterexample)
        }
        ProtocolCheckVerdict::BoundExhausted { .. } => ProtocolExecutionOutcome::BoundExhausted,
        ProtocolCheckVerdict::CounterexampleFound { counterexample, .. } => {
            ProtocolExecutionOutcome::CounterexampleFound(counterexample)
        }
        ProtocolCheckVerdict::UnsupportedBackendAssumptions => {
            ProtocolExecutionOutcome::UnsupportedBackendProfile
        }
    }
}

pub const fn receipt_loss_outcome(
    classification: ReceiptLossClassification,
) -> ProtocolExecutionOutcome {
    match classification {
        ReceiptLossClassification::NoOwnerTransition => ProtocolExecutionOutcome::NoOwnerTransition,
        ReceiptLossClassification::CrashLostNonAuthoritativeTrace => {
            ProtocolExecutionOutcome::CrashLostNonAuthoritativeTrace
        }
        defect => ProtocolExecutionOutcome::ReceiptOmissionDefect(defect),
    }
}
