use serde::{Deserialize, Serialize};

use crate::indexes::data::DerivedIndexGenerationId;

use super::QueryExecutionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallbackParityMode {
    ProductionAdmissibility,
    SampledParity,
    CertificationParity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexQueryRejectionClass {
    MissingGeneration,
    IncompatibleVersion,
    IncompatibleBranch,
    CorruptIndexEntries,
    UnsupportedScope,
    UnsupportedOrderingContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryAccessPath {
    AuthoritativeStorage,
    DerivedIndexGeneration {
        generation_id: DerivedIndexGenerationId,
    },
    DerivedIndexRejectedStorageFallback {
        rejection: IndexQueryRejectionClass,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackParityVerifiedQueryOutcome {
    pub execution: QueryExecutionOutcome,
    pub access_path: QueryAccessPath,
    pub parity_mode: FallbackParityMode,
    pub parity_basis_digest: String,
}
