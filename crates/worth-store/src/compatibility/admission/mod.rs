use super::catalog::{CompatibilityFamilyDeclaration, CompatibilityRegistrySnapshot};
use super::decoding::{CompatibilityCheckedArtifact, QuarantinedDecodedArtifact};
use super::manifests::{
    ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion, CompatibilityManifestDigest,
    CompatibilityRecoveredManifestIndex,
};
use crate::failure::StoreErrorKind;
use serde::Serialize;
use std::collections::BTreeMap;

mod adapters;
mod batch;
mod capabilities;
mod checked_artifact;
mod counters;
mod decision;
mod edge_registry;
mod intents;
mod manifest_index;
mod outcomes;
mod plan;
mod planning;
mod receipts;
mod rejection;
mod relation;

pub use adapters::{
    CompatibilityAdapterDigest, CompatibilityAdapterId, DeclaredCompatibilityAdapter,
    DeclaredCompatibilityEdge,
};
pub use batch::CompatibilityAdmissionBatch;
pub use capabilities::{ReaderCapabilitySet, WriterCapabilitySet};
pub(crate) use checked_artifact::check_artifact_with_read_receipt;
pub use counters::CompatibilityAdmissionCounters;
pub use decision::{CompatibilityDecision, CompatibilityRejectionKind};
pub use edge_registry::{CompatibilityEdgeProof, CompatibilityEdgeRegistry};
pub use intents::{CompatibilityReadIntent, CompatibilityWriteIntent};
pub use manifest_index::{CompatibilityManifestIndex, CompatibilityManifestIndexEntry};
pub use outcomes::{CompatibilityReadAdmissionOutcome, CompatibilityWriteAdmissionOutcome};
pub use plan::{CompatibilityAdmissionPlan, CompatibilityBatchScope};
pub(crate) use planning::plan_read_compatibility_for_path;
pub use planning::{plan_read_compatibility, plan_write_compatibility};
pub use receipts::{
    BackwardReadCompatibilityWitness, CompatibilityAdapterParityWitness,
    CompatibilityAdmissionReceipt, DerivedReuseCompatibilityReceipt,
    ForwardReadCompatibilityWitness, ReadCompatibilityReceipt, RestoreCompatibilityReceipt,
    RollingWindowCompatibilityReceipt, SemanticMeaningPreservationWitness,
    UpgradeAdmissionWitness, WriteCompatibilityReceipt,
};
use receipts::{has_stale_receipt_basis, ReceiptKey};
pub use rejection::CompatibilityRejection;
pub use relation::{
    CompatibilityAdapterCostClass, CompatibilityAdmissionPath, CompatibilityRelation,
};
