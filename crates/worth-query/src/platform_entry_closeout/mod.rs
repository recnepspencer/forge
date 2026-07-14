mod alignment;
mod bundle;
mod compile_fail;
mod hostile;
mod parity;
mod surface;

#[cfg(test)]
pub(crate) use alignment::docs_coverage_alignment_audit_from_audit;
pub use alignment::WorthQueryPlatformEntryAlignmentAudit;
pub use bundle::{
    certify_platform_entry_closeout, WorthQueryPlatformEntryCloseoutBundle,
    WorthQueryPlatformEntryCloseoutOutput,
};
pub use compile_fail::{
    worth_query_platform_entry_compile_fail_boundary_digest,
    worth_query_platform_entry_compile_fail_manifest, WorthQueryPlatformEntryCompileFailAudit,
    WorthQueryPlatformEntryCompileFailManifest, WorthQueryPlatformEntryUiProofKind,
    WorthQueryPlatformEntryUiProofRow,
};
pub use hostile::{
    worth_query_platform_entry_hostile_manifest, WorthQueryPlatformEntryHostileAudit,
    WorthQueryPlatformEntryHostileDivergenceClass, WorthQueryPlatformEntryHostileManifest,
    WorthQueryPlatformEntryHostileRow,
};
pub use parity::{
    worth_query_platform_entry_parity_manifest, WorthQueryPlatformEntryParityAssertionClass,
    WorthQueryPlatformEntryParityAudit, WorthQueryPlatformEntryParityLane,
    WorthQueryPlatformEntryParityManifest, WorthQueryPlatformEntryParityRow,
};
pub use surface::{
    worth_query_platform_entry_closeout_surface, WorthQueryPlatformEntryCloseoutSurface,
};

#[cfg(test)]
mod tests;
