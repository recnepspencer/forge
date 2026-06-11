mod alignment;
mod bundle;
mod compile_fail;
mod hostile;
mod parity;
mod surface;

#[cfg(test)]
pub(crate) use alignment::docs_coverage_alignment_audit_from_audit;
pub use alignment::ForgeQueryPlatformEntryAlignmentAudit;
pub use bundle::{
    certify_platform_entry_closeout, ForgeQueryPlatformEntryCloseoutBundle,
    ForgeQueryPlatformEntryCloseoutOutput,
};
pub use compile_fail::{
    forge_query_platform_entry_compile_fail_boundary_digest,
    forge_query_platform_entry_compile_fail_manifest, ForgeQueryPlatformEntryCompileFailAudit,
    ForgeQueryPlatformEntryCompileFailManifest, ForgeQueryPlatformEntryUiProofKind,
    ForgeQueryPlatformEntryUiProofRow,
};
pub use hostile::{
    forge_query_platform_entry_hostile_manifest, ForgeQueryPlatformEntryHostileAudit,
    ForgeQueryPlatformEntryHostileDivergenceClass, ForgeQueryPlatformEntryHostileManifest,
    ForgeQueryPlatformEntryHostileRow,
};
pub use parity::{
    forge_query_platform_entry_parity_manifest, ForgeQueryPlatformEntryParityAssertionClass,
    ForgeQueryPlatformEntryParityAudit, ForgeQueryPlatformEntryParityLane,
    ForgeQueryPlatformEntryParityManifest, ForgeQueryPlatformEntryParityRow,
};
pub use surface::{
    forge_query_platform_entry_closeout_surface, ForgeQueryPlatformEntryCloseoutSurface,
};

#[cfg(test)]
mod tests;
