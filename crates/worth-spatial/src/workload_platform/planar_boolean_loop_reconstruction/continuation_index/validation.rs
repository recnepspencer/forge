use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopOverlapChainLineageMap;

use super::counters::PlanarBooleanFragmentContinuationCounters;
use super::denial::{
    PlanarBooleanFragmentContinuationDenial, PlanarBooleanFragmentContinuationDenialKind as Kind,
};
use super::input::PlanarBooleanFragmentContinuationIndexInput;

pub(crate) fn validate_fragment_continuation_input(
    input: &PlanarBooleanFragmentContinuationIndexInput<'_>,
    counters: &mut PlanarBooleanFragmentContinuationCounters,
) -> Result<(), PlanarBooleanFragmentContinuationDenial> {
    if input.request().request_identity() != input.source_provenance().request_identity() {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanFragmentContinuationDenial::new(
            Kind::ForeignRequestLineage,
            input.request().request_identity(),
            *counters,
            "fragment continuation indexing requires provenance from the same admitted loop reconstruction request",
        ));
    }
    if input.split_fragments().fragment_set_identity()
        != input
            .source_provenance()
            .fragment_membership_map()
            .fragment_set_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanFragmentContinuationDenial::new(
            Kind::ForeignFragmentSet,
            input.split_fragments().fragment_set_identity(),
            *counters,
            "fragment continuation indexing requires the fragment-authoritative split fragment set from source provenance recovery",
        ));
    }
    if input.overlap_chains().chain_set_identity()
        != input
            .source_provenance()
            .overlap_chain_lineage_map()
            .overlap_chain_set_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanFragmentContinuationDenial::new(
            Kind::ForeignOverlapChainSet,
            input.overlap_chains().chain_set_identity(),
            *counters,
            "fragment continuation indexing requires the overlap-chain-authoritative set from source provenance recovery",
        ));
    }
    if input.split_vertices().split_vertex_identity_set_identity()
        != input.split_fragments().split_vertex_identity_set_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanFragmentContinuationDenial::new(
            Kind::ForeignSplitVertexSet,
            input.split_vertices().split_vertex_identity_set_identity(),
            *counters,
            "fragment continuation indexing requires the split vertex set that authored the split fragments",
        ));
    }
    if input.overlap_chains().split_edge_fragment_set_identity()
        != input.split_fragments().fragment_set_identity()
    {
        counters.rejected_foreign_lineage();
        return Err(PlanarBooleanFragmentContinuationDenial::new(
            Kind::ForeignSourceProvenanceBundle,
            input.overlap_chains().split_edge_fragment_set_identity(),
            *counters,
            "fragment continuation indexing requires overlap chains that bind to the same fragment set as continuation recovery",
        ));
    }
    validate_unique_split_vertices(input, counters)?;
    validate_overlap_chain_references(
        input.source_provenance().overlap_chain_lineage_map(),
        input,
        counters,
    )?;
    Ok(())
}

fn validate_unique_split_vertices(
    input: &PlanarBooleanFragmentContinuationIndexInput<'_>,
    counters: &mut PlanarBooleanFragmentContinuationCounters,
) -> Result<(), PlanarBooleanFragmentContinuationDenial> {
    let mut seen = BTreeSet::new();
    for vertex in input.split_vertices().vertices() {
        counters.consumed_split_vertex();
        if !seen.insert(vertex.split_vertex_identity().to_string()) {
            counters.rejected_duplicate_slot();
            return Err(PlanarBooleanFragmentContinuationDenial::new(
                Kind::DuplicateSplitVertexIdentity,
                vertex.split_vertex_identity(),
                *counters,
                "fragment continuation indexing requires unique split vertex identities",
            ));
        }
    }
    Ok(())
}

fn validate_overlap_chain_references(
    overlap_lineage: &PlanarBooleanLoopOverlapChainLineageMap,
    input: &PlanarBooleanFragmentContinuationIndexInput<'_>,
    counters: &mut PlanarBooleanFragmentContinuationCounters,
) -> Result<(), PlanarBooleanFragmentContinuationDenial> {
    let chain_identities = input
        .overlap_chains()
        .chains()
        .iter()
        .map(|chain| chain.chain_identity())
        .collect::<BTreeSet<_>>();
    let fragment_identities = input
        .source_provenance()
        .fragment_membership_map()
        .rows()
        .iter()
        .map(|row| row.fragment_identity())
        .collect::<BTreeSet<_>>();
    for row in overlap_lineage.rows() {
        counters.consumed_overlap_chain();
        if !chain_identities.contains(row.chain_identity()) {
            counters.rejected_dangling_reference();
            return Err(PlanarBooleanFragmentContinuationDenial::new(
                Kind::MissingOverlapChainBinding,
                row.chain_identity(),
                *counters,
                "fragment continuation indexing requires every overlap-chain lineage row to bind a real overlap chain",
            ));
        }
        for fragment_identity in row.fragment_identities() {
            if !fragment_identities.contains(fragment_identity.as_str()) {
                counters.rejected_dangling_reference();
                return Err(PlanarBooleanFragmentContinuationDenial::new(
                    Kind::MissingFragmentMembership,
                    fragment_identity,
                    *counters,
                    "fragment continuation indexing requires overlap-chain lineage to reference only fragment-membership-owned fragments",
                ));
            }
        }
    }
    Ok(())
}
