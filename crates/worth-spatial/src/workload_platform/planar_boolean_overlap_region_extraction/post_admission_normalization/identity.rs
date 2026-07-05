use super::rows::PlanarBooleanOverlapRegionCanonicalWindingSourceKind;

pub(super) fn canonical_row_identity(
    request_identity: &str,
    source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
    source_identity: &str,
) -> String {
    format!(
        "post-admission-canonical-winding:{}:{}:{}",
        request_identity,
        match source_kind {
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion => {
                "admitted-region"
            }
            PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome => {
                "boundary-only"
            }
        },
        source_identity
    )
}

pub(super) fn canonical_set_identity(request_identity: &str, row_count: usize) -> String {
    format!("post-admission-canonical-winding-set:{request_identity}:{row_count}")
}
