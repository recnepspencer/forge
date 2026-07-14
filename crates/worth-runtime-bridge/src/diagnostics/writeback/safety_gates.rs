use crate::writeback::{
    BridgeWritebackLoopPreventionReport, BridgeWritebackStrategyCoherenceReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackLoopPreventionExplanation {
    report: BridgeWritebackLoopPreventionReport,
}

impl BridgeWritebackLoopPreventionExplanation {
    pub fn from_report(report: &BridgeWritebackLoopPreventionReport) -> Self {
        Self {
            report: report.clone(),
        }
    }

    pub fn report(&self) -> &BridgeWritebackLoopPreventionReport {
        &self.report
    }

    pub fn loop_prevention_digest(&self) -> &str {
        self.report.digest()
    }

    pub fn disposition(&self) -> crate::writeback::BridgeWritebackLoopDisposition {
        self.report.disposition()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackStrategyCoherenceExplanation {
    report: BridgeWritebackStrategyCoherenceReport,
}

impl BridgeWritebackStrategyCoherenceExplanation {
    pub fn from_report(report: &BridgeWritebackStrategyCoherenceReport) -> Self {
        Self {
            report: report.clone(),
        }
    }

    pub fn report(&self) -> &BridgeWritebackStrategyCoherenceReport {
        &self.report
    }

    pub fn coherence_digest(&self) -> &str {
        self.report.digest()
    }

    pub fn disposition(&self) -> crate::writeback::BridgeWritebackStrategyCoherenceDisposition {
        self.report.disposition()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.report.effect_intent_digest()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.report.effect_intent_patch_canonical_basis()
    }
}
