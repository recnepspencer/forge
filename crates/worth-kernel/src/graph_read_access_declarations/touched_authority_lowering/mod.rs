mod lowered_authority;
mod lowering_errors;
mod lowering_record;
mod lowering_summary;
mod operating_world;
mod query_touch_descriptor;
mod source_family;

pub use lowered_authority::WorthGraphReadLoweredTouchedAuthority;
pub use lowering_errors::{
    WorthGraphReadTouchedAuthorityLoweringError, WorthGraphReadTouchedAuthorityLoweringErrorKind,
};
pub use lowering_record::WorthGraphReadLoweredAuthorityRecord;
pub use lowering_summary::WorthGraphReadTouchedAuthorityLoweringSummary;
pub use source_family::WorthGraphReadTouchedAuthoritySourceFamily;

use crate::graph_read_access_inventory::WorthGraphReadDeclarationCandidate;

pub(crate) fn lower_touched_authority_for_catalog_candidate(
    candidate: &WorthGraphReadDeclarationCandidate,
) -> Result<WorthGraphReadLoweredAuthorityRecord, WorthGraphReadTouchedAuthorityLoweringError> {
    let lowered_authority = WorthGraphReadLoweredTouchedAuthority::from_candidate(candidate)?;
    Ok(WorthGraphReadLoweredAuthorityRecord::from_candidate(
        candidate,
        lowered_authority,
    ))
}
