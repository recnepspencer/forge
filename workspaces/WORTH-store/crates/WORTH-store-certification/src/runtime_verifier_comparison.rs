use crate::{RuntimeVerifierParityTrace, RuntimeVerifierRelationship};
use worth_store_physical_format::{
    OfflineVerifierLayoutObservation, PhysicalReference, RuntimeLayoutObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeVerifierComparisonClassification {
    Equivalent,
    MissingInRuntime,
    MissingInVerifier,
    ReferenceOrderMismatch,
    TraversalCountMismatch,
    SemanticDecodeAttempted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeVerifierComparisonReport {
    classification: RuntimeVerifierComparisonClassification,
    compared_references: u32,
    runtime_reference_count: u32,
    offline_reference_count: u32,
    runtime_semantic_decode_attempts: u32,
    offline_semantic_decode_attempts: u32,
    parity_trace: RuntimeVerifierParityTrace,
}

impl RuntimeVerifierComparisonReport {
    pub const fn classification(&self) -> RuntimeVerifierComparisonClassification {
        self.classification
    }

    pub const fn compared_references(&self) -> u32 {
        self.compared_references
    }

    pub const fn runtime_reference_count(&self) -> u32 {
        self.runtime_reference_count
    }

    pub const fn offline_reference_count(&self) -> u32 {
        self.offline_reference_count
    }

    pub const fn runtime_semantic_decode_attempts(&self) -> u32 {
        self.runtime_semantic_decode_attempts
    }

    pub const fn offline_semantic_decode_attempts(&self) -> u32 {
        self.offline_semantic_decode_attempts
    }

    pub const fn parity_trace(&self) -> RuntimeVerifierParityTrace {
        self.parity_trace
    }

    const fn equivalent(
        compared_references: u32,
        runtime_semantic_decode_attempts: u32,
        offline_semantic_decode_attempts: u32,
    ) -> Self {
        Self {
            classification: RuntimeVerifierComparisonClassification::Equivalent,
            compared_references,
            runtime_reference_count: compared_references,
            offline_reference_count: compared_references,
            runtime_semantic_decode_attempts,
            offline_semantic_decode_attempts,
            parity_trace: RuntimeVerifierParityTrace::new(
                RuntimeVerifierRelationship::RuntimeMustMatchVerifier,
            ),
        }
    }

    const fn mismatch(
        classification: RuntimeVerifierComparisonClassification,
        runtime_reference_count: u32,
        offline_reference_count: u32,
        runtime_semantic_decode_attempts: u32,
        offline_semantic_decode_attempts: u32,
    ) -> Self {
        Self {
            classification,
            compared_references: 0,
            runtime_reference_count,
            offline_reference_count,
            runtime_semantic_decode_attempts,
            offline_semantic_decode_attempts,
            parity_trace: RuntimeVerifierParityTrace::new(
                RuntimeVerifierRelationship::RuntimeMustDisagreeWithVerifier,
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeVerifierComparisonDenial {
    report: RuntimeVerifierComparisonReport,
}

impl RuntimeVerifierComparisonDenial {
    pub const fn classification(&self) -> RuntimeVerifierComparisonClassification {
        self.report.classification()
    }

    pub const fn report(&self) -> &RuntimeVerifierComparisonReport {
        &self.report
    }

    pub const fn parity_trace(&self) -> RuntimeVerifierParityTrace {
        self.report.parity_trace()
    }

    const fn new(report: RuntimeVerifierComparisonReport) -> Self {
        Self { report }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRuntimeVerifierComparison;

impl PhysicalRuntimeVerifierComparison {
    pub fn compare(
        runtime: &RuntimeLayoutObservation,
        offline: &OfflineVerifierLayoutObservation,
    ) -> Result<RuntimeVerifierComparisonReport, RuntimeVerifierComparisonDenial> {
        let runtime_count = runtime.discovered_references().len() as u32;
        let offline_count = offline.discovered_references().len() as u32;
        if runtime.semantic_decode_attempts() != 0 || offline.semantic_decode_attempts() != 0 {
            return Err(Self::denial(
                RuntimeVerifierComparisonClassification::SemanticDecodeAttempted,
                runtime,
                offline,
                runtime_count,
                offline_count,
            ));
        }
        if runtime.traversal() != offline.traversal() {
            return Err(Self::denial(
                RuntimeVerifierComparisonClassification::TraversalCountMismatch,
                runtime,
                offline,
                runtime_count,
                offline_count,
            ));
        }
        if runtime.discovered_references() == offline.discovered_references() {
            return Ok(RuntimeVerifierComparisonReport::equivalent(
                offline_count,
                runtime.semantic_decode_attempts(),
                offline.semantic_decode_attempts(),
            ));
        }
        Err(Self::denial(
            classify_reference_mismatch(
                runtime.discovered_references(),
                offline.discovered_references(),
            ),
            runtime,
            offline,
            runtime_count,
            offline_count,
        ))
    }

    fn denial(
        classification: RuntimeVerifierComparisonClassification,
        runtime: &RuntimeLayoutObservation,
        offline: &OfflineVerifierLayoutObservation,
        runtime_count: u32,
        offline_count: u32,
    ) -> RuntimeVerifierComparisonDenial {
        RuntimeVerifierComparisonDenial::new(RuntimeVerifierComparisonReport::mismatch(
            classification,
            runtime_count,
            offline_count,
            runtime.semantic_decode_attempts(),
            offline.semantic_decode_attempts(),
        ))
    }
}

fn classify_reference_mismatch(
    runtime: &[PhysicalReference],
    offline: &[PhysicalReference],
) -> RuntimeVerifierComparisonClassification {
    if offline.iter().any(|reference| !runtime.contains(reference)) {
        return RuntimeVerifierComparisonClassification::MissingInRuntime;
    }
    if runtime.iter().any(|reference| !offline.contains(reference)) {
        return RuntimeVerifierComparisonClassification::MissingInVerifier;
    }
    RuntimeVerifierComparisonClassification::ReferenceOrderMismatch
}
