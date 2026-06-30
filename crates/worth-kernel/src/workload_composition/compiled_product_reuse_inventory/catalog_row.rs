use super::classification::{
    CompiledProductReuseAuthorityKind as AuthorityKind,
    CompiledProductReuseDisposition as Disposition, CompiledProductReuseOwner as Owner,
    CompiledProductReuseReplacementPhase as Phase,
    CompiledProductReuseSemanticCategory as Category,
    CompiledProductReuseSemanticDistinction as Distinction,
};
use super::row::{
    CompiledProductReuseInventoryRow, CompiledProductReuseSurfaceIdentity as Surface,
};
use super::source_scan::CompiledProductReuseScanPattern as Pattern;

#[allow(clippy::too_many_arguments)]
pub(super) fn row(
    surface_identity: Surface,
    source_path: &'static str,
    surface_name: &'static str,
    authority_surface: &'static str,
    semantic_category: Category,
    semantic_distinction: Distinction,
    authority_kind: AuthorityKind,
    disposition: Disposition,
    owner: Owner,
    replacement_phase: Phase,
    blocker: &'static str,
    removal_trigger: &'static str,
    ordinary_path: bool,
    certification_only: bool,
    cap: Option<usize>,
    scan_pattern: Pattern,
    secondary_scan_pattern: Option<Pattern>,
) -> CompiledProductReuseInventoryRow {
    CompiledProductReuseInventoryRow::new(
        surface_identity,
        source_path,
        surface_name,
        authority_surface,
        semantic_category,
        semantic_distinction,
        authority_kind,
        disposition,
        owner,
        replacement_phase,
        blocker,
        removal_trigger,
        ordinary_path,
        certification_only,
        cap,
        scan_pattern,
        secondary_scan_pattern,
    )
}
