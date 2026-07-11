// --- S6/S7 production handoffs (non-claim readiness seeds) ---
pub use crate::handoffs::{
    blob_background_pressure_kind, blob_compaction_background_pressure_shape,
    blob_ingest_background_pressure_shape, blob_migration_background_pressure_shape,
    AdmittedBlobChunkSecurity, BlobBackgroundPressureKind, BlobChunkSecurityPermission,
    BlobHarnessAccessMode, BlobHarnessActorMix, BlobHarnessChunkSizeClass,
    BlobHarnessChunkTopology, BlobHarnessFailurePoint, BlobHarnessPlacementClass,
    BlobHarnessSecurityScopeClass, BlobHarnessSizeClass, BlobHarnessTopologyDenial,
    BlobReclaimPolicyEvidence, BlobReclaimPolicyScopeDenial,
};
