use std::collections::BTreeSet;

use super::counters::PlanarBooleanLoopIdentityMintingCounters;
use super::denial::{
    PlanarBooleanLoopIdentityMintingDenial, PlanarBooleanLoopIdentityMintingDenialKind,
};
use super::support::PlanarBooleanLoopNamingAuthoritySupport;
use super::support_index::IndexedLoopIdentityInputRow;

pub(crate) fn validate_name_row_lineage(
    loop_row: &IndexedLoopIdentityInputRow<'_>,
    naming_support: &PlanarBooleanLoopNamingAuthoritySupport,
    source_edge_identity: &str,
    counters: &mut PlanarBooleanLoopIdentityMintingCounters,
) -> Result<(), PlanarBooleanLoopIdentityMintingDenial> {
    let allowed = loop_row
        .source_loop_identities()
        .iter()
        .flat_map(|source_loop_identity| {
            naming_support
                .source_edges_for_source_loop(source_loop_identity)
                .into_iter()
                .flat_map(|edges| edges.iter().cloned())
        })
        .collect::<BTreeSet<_>>();
    if allowed.contains(source_edge_identity) {
        return Ok(());
    }
    counters.denied_foreign_lineage();
    Err(PlanarBooleanLoopIdentityMintingDenial::new(
        PlanarBooleanLoopIdentityMintingDenialKind::ForeignNamingLineage,
        loop_row.tracked_loop_identity().to_string(),
        *counters,
        "loop identity minting denies split naming rows whose source-edge lineage falls outside the source loops proven for the admitted loop",
    ))
}
