#![forbid(unsafe_code)]

pub mod aspect_native {
    pub use forge_store_aspect_native::{
        canonical_basis_source_owner_for_family, certify_canonical_basis_field_role,
        certify_canonical_basis_source, deny_basis_free_digest_comparison, deny_basis_free_parity,
        deny_basis_free_reuse, deny_basis_free_suppression, StoreAspectAuthorityInput,
        StoreAspectBoundaryFact, StoreAspectBoundaryLocator, StoreAspectContractAdmission,
        StoreAspectFieldBoundaryLocator, StoreAspectIdentity, StoreAspectNativeDenial,
        StoreAspectPatchAuthorityInput, StoreAspectPatchBoundaryFact,
        StoreAspectValueBoundaryLocator, StoreBoundaryArtifactBoundaryLocator,
        StoreCanonicalBasisConstruction, StoreCanonicalBasisConstructionDenial,
        StoreCanonicalBasisConstructionOutcome, StoreCanonicalBasisDomainMismatch,
        StoreCanonicalBasisFamily, StoreCanonicalBasisFieldRole, StoreCanonicalBasisLane,
        StoreCanonicalBasisSourceDenial, StoreCanonicalBasisSourceKind,
        StoreCanonicalBasisSourceOwner, StoreCompletedBoundaryReceiptEvidence,
        StoreDiagnosticExplanationBundleEvidence, StoreDiagnosticSupportReportEvidence,
        StoreDigestAuthority, StoreDigestAuthorityDenial, StoreDigestAuthorityOutcome,
        StoreDigestEquivalenceBasis, StoreDigestEquivalenceDecision, StoreDigestEquivalenceDenial,
        StoreDigestEquivalenceOperation, StoreDigestEquivalenceOutcome, StoreDigestEvidence,
        StoreEquivalenceBasisIdentity, StoreExecutedBoundaryReceiptEvidence,
        StorePerformanceReceiptEvidence, StorePhysicalBoundaryWitness,
        StoreValidatedAspectValueAdmission, STORE_CANONICAL_BASIS_SOURCE_OWNERS,
    };
    pub use forge_store_contracts::{
        StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    };
}

pub mod certification {
    pub use forge_store_certification::{
        certify_store_json_residue_inventory, StoreCertificationProgram, StoreJsonAuthorityRisk,
        StoreJsonResidueClassification, StoreJsonResidueDenial, StoreJsonResidueInventory,
        StoreJsonResidueOccurrence, StoreJsonResidueTokenKind, StoreJsonResidueZone,
    };
}

pub mod contracts {
    pub use forge_store_contracts::{
        DerivedAccuracyClass, DurableArtifactClass, RoadmapScope, StableArtifactId, StableDigest,
        StoreContractError, StoreContractResult,
    };
}

pub mod physical_format {
    pub use forge_store_physical_format::{
        PhysicalExtentId, PhysicalGeneration, PhysicalPageId, PhysicalReference, PhysicalSegmentId,
    };
}

pub mod terminal_projection {
    pub use forge_store_aspect_native::{
        project_store_boundary_fact_to_terminal_json,
        readmit_external_terminal_projection_document_as_store_aspect_state,
        readmit_terminal_json_projection_as_store_aspect_state, StoreTerminalChecksumAlgorithm,
        StoreTerminalChecksumScope, StoreTerminalDocumentChecksum, StoreTerminalJsonProjection,
        StoreTerminalJsonReadmission, StoreTerminalJsonReadmissionOutcome,
        StoreTerminalProjectionDenial, StoreTerminalProjectionDisplayLabel,
        StoreTerminalProjectionDocumentBytes, StoreTerminalProjectionText,
    };
}

pub use forge_store_contracts::{
    DerivedAccuracyClass, DurableArtifactClass, RoadmapScope, StableArtifactId, StableDigest,
    StoreContractError, StoreContractResult,
};

pub use forge_store_physical_format::{
    PhysicalExtentId, PhysicalGeneration, PhysicalPageId, PhysicalReference, PhysicalSegmentId,
};
