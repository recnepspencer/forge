use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChain, PlanarBooleanSplitEdgeFragment,
};

use super::bundle::PlanarBooleanLoopSourceProvenanceBundle;
use super::counters::PlanarBooleanLoopSourceProvenanceCounters;
use super::denial::{
    PlanarBooleanLoopSourceProvenanceDenial, PlanarBooleanLoopSourceProvenanceDenialKind as Kind,
};
use super::fragment_membership::{
    PlanarBooleanFragmentMembershipMap, PlanarBooleanFragmentMembershipRow,
};
use super::identity::{
    fragment_membership_identity, fragment_membership_map_identity, overlap_chain_lineage_identity,
    overlap_chain_lineage_map_identity, provenance_bundle_identity, source_loop_carrier_identity,
    source_loop_carrier_set_identity,
};
use super::input::PlanarBooleanLoopSourceProvenanceRecoveryInput;
use super::overlap_chain_lineage::{
    PlanarBooleanLoopOverlapChainLineageMap, PlanarBooleanLoopOverlapChainLineageRow,
};
use super::source_loop_carriers::{
    PlanarBooleanLoopSourceCarrierRow, PlanarBooleanLoopSourceCarrierSet,
};
use super::validation::validate_loop_source_provenance_input;

pub(crate) fn recover_loop_source_provenance(
    input: PlanarBooleanLoopSourceProvenanceRecoveryInput<'_>,
) -> Result<PlanarBooleanLoopSourceProvenanceBundle, PlanarBooleanLoopSourceProvenanceDenial> {
    let mut counters = PlanarBooleanLoopSourceProvenanceCounters::default();
    validate_loop_source_provenance_input(&input, &mut counters)?;
    let fragment_index = index_fragments(&input, &mut counters)?;
    let overlap_index = index_overlap_chains(&input, &mut counters)?;
    let source_loop_carriers = recover_source_loop_carriers(&input, &mut counters)?;
    let fragment_membership = recover_fragment_membership(
        &input,
        &fragment_index,
        &source_loop_carriers,
        &mut counters,
    )?;
    let overlap_chain_lineage =
        recover_overlap_chain_lineage(&input, &overlap_index, &fragment_membership, &mut counters)?;
    let bundle_identity = provenance_bundle_identity(
        input.request().request_identity(),
        &source_loop_carriers,
        &fragment_membership,
        &overlap_chain_lineage,
    );
    Ok(PlanarBooleanLoopSourceProvenanceBundle::new(
        bundle_identity,
        input.request().request_identity().to_string(),
        input.split_ledger_receipt().receipt_identity().to_string(),
        source_loop_carriers,
        fragment_membership,
        overlap_chain_lineage,
        counters,
    ))
}

fn recover_source_loop_carriers(
    input: &PlanarBooleanLoopSourceProvenanceRecoveryInput<'_>,
    counters: &mut PlanarBooleanLoopSourceProvenanceCounters,
) -> Result<PlanarBooleanLoopSourceCarrierSet, PlanarBooleanLoopSourceProvenanceDenial> {
    let mut rows = Vec::new();
    let mut seen_carriers = BTreeSet::new();
    for chain in input.split_ledger().chains() {
        counters.consumed_split_chain();
        if chain.carrier_identity().is_empty() {
            counters.rejected_dangling_reference();
            return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
                Kind::MissingLedgerChainCarrier,
                chain.chain_identity(),
                *counters,
                "split-ledger chains must carry carrier identity for loop provenance recovery",
            ));
        }
        if !seen_carriers.insert(chain.carrier_identity().to_string()) {
            continue;
        }
        let recovered = input
            .recovered_source_carriers()
            .carrier_for_identity(chain.carrier_identity())
            .ok_or_else(|| {
                counters.rejected_dangling_reference();
                PlanarBooleanLoopSourceProvenanceDenial::new(
                    Kind::MissingRecoveredSourceCarrier,
                    chain.carrier_identity(),
                    *counters,
                    "loop provenance recovery requires every split-ledger chain carrier to exist in recovered source carriers",
                )
            })?;
        let row = PlanarBooleanLoopSourceCarrierRow::new(
            source_loop_carrier_identity(
                input.request().request_identity(),
                input.split_ledger_receipt().receipt_identity(),
                recovered.recovered_carrier_identity(),
                recovered.source_loop_identity(),
            ),
            recovered.recovered_carrier_identity().to_string(),
            recovered.carrier_identity().to_string(),
            recovered.source_face_identity().to_string(),
            recovered.source_loop_identity().to_string(),
            recovered.source_edge_identity().to_string(),
            recovered.start_source_endpoint_identity().to_string(),
            [
                recovered.start_point_2d()[0].to_bits(),
                recovered.start_point_2d()[1].to_bits(),
            ],
            recovered.end_source_endpoint_identity().to_string(),
            [
                recovered.end_point_2d()[0].to_bits(),
                recovered.end_point_2d()[1].to_bits(),
            ],
            recovered.loop_role(),
        );
        counters.recovered_source_carrier();
        rows.push(row);
    }
    rows.sort_by(|left, right| {
        left.source_loop_identity()
            .cmp(right.source_loop_identity())
            .then_with(|| {
                left.source_edge_identity()
                    .cmp(right.source_edge_identity())
            })
            .then_with(|| left.carrier_identity().cmp(right.carrier_identity()))
    });
    Ok(PlanarBooleanLoopSourceCarrierSet::new(
        source_loop_carrier_set_identity(
            input.request().request_identity(),
            input.split_ledger_receipt().receipt_identity(),
            &rows,
        ),
        input.request().request_identity().to_string(),
        input.split_ledger_receipt().receipt_identity().to_string(),
        rows,
    ))
}

fn recover_fragment_membership(
    input: &PlanarBooleanLoopSourceProvenanceRecoveryInput<'_>,
    fragment_index: &BTreeMap<String, FragmentBinding<'_>>,
    source_loop_carriers: &PlanarBooleanLoopSourceCarrierSet,
    counters: &mut PlanarBooleanLoopSourceProvenanceCounters,
) -> Result<PlanarBooleanFragmentMembershipMap, PlanarBooleanLoopSourceProvenanceDenial> {
    let mut rows = Vec::new();
    let mut seen_fragments = BTreeSet::new();
    for chain in input.split_ledger().chains() {
        let source_carrier = source_loop_carriers
            .carrier_for_identity(chain.carrier_identity())
            .ok_or_else(|| {
                counters.rejected_dangling_reference();
                PlanarBooleanLoopSourceProvenanceDenial::new(
                    Kind::MissingRecoveredSourceCarrier,
                    chain.carrier_identity(),
                    *counters,
                    "fragment membership recovery requires a source-loop carrier for every ledger chain carrier",
                )
            })?;
        for fragment_identity in chain.fragment_identities() {
            if !seen_fragments.insert(fragment_identity.clone()) {
                continue;
            }
            let binding = fragment_index.get(fragment_identity).ok_or_else(|| {
                counters.rejected_dangling_reference();
                PlanarBooleanLoopSourceProvenanceDenial::new(
                    Kind::MissingLedgerFragment,
                    fragment_identity,
                    *counters,
                    "loop provenance recovery requires every split-ledger fragment identity to exist in the split-fragment set",
                )
            })?;
            if binding.carrier_identity != source_carrier.carrier_identity() {
                counters.rejected_foreign_lineage();
                return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
                    Kind::ForeignFragmentLineage,
                    fragment_identity,
                    *counters,
                    "split-ledger fragment lineage must preserve the carrier identity bound by the ledger chain",
                ));
            }
            let row = PlanarBooleanFragmentMembershipRow::new(
                fragment_membership_identity(
                    input.request().request_identity(),
                    fragment_identity,
                    source_carrier.source_loop_carrier_identity(),
                ),
                fragment_identity.clone(),
                source_carrier.carrier_identity().to_string(),
                source_carrier.source_loop_carrier_identity().to_string(),
                source_carrier.recovered_carrier_identity().to_string(),
                source_carrier.source_face_identity().to_string(),
                source_carrier.source_loop_identity().to_string(),
                source_carrier.source_edge_identity().to_string(),
                binding.fragment.local_frame_identity().to_string(),
                binding.fragment.precision_basis_identity().to_string(),
                binding.fragment.source_senses().to_vec(),
            );
            counters.recovered_fragment_membership();
            rows.push(row);
        }
    }
    rows.sort_by(|left, right| left.fragment_identity().cmp(right.fragment_identity()));
    Ok(PlanarBooleanFragmentMembershipMap::new(
        fragment_membership_map_identity(input.request().request_identity(), &rows),
        input.request().request_identity().to_string(),
        input.split_fragments().fragment_set_identity().to_string(),
        rows,
    ))
}

fn recover_overlap_chain_lineage(
    input: &PlanarBooleanLoopSourceProvenanceRecoveryInput<'_>,
    overlap_index: &BTreeMap<String, &PlanarBooleanOverlapEdgeChain>,
    fragment_membership: &PlanarBooleanFragmentMembershipMap,
    counters: &mut PlanarBooleanLoopSourceProvenanceCounters,
) -> Result<PlanarBooleanLoopOverlapChainLineageMap, PlanarBooleanLoopSourceProvenanceDenial> {
    let mut rows = Vec::new();
    let mut seen_chains = BTreeSet::new();
    for chain in input.split_ledger().chains() {
        for overlap_chain_identity in chain.overlap_chain_identities() {
            if !seen_chains.insert(overlap_chain_identity.clone()) {
                continue;
            }
            let overlap_chain = overlap_index.get(overlap_chain_identity).ok_or_else(|| {
                counters.rejected_dangling_reference();
                PlanarBooleanLoopSourceProvenanceDenial::new(
                    Kind::MissingLedgerOverlapChain,
                    overlap_chain_identity,
                    *counters,
                    "loop provenance recovery requires every split-ledger overlap chain identity to exist in the overlap-chain set",
                )
            })?;
            let mut fragment_identities = Vec::new();
            let mut member_identities = Vec::new();
            let mut source_loop_identities = Vec::new();
            let mut source_edge_identities = Vec::new();
            let mut boundary_roles = Vec::new();
            for member in overlap_chain.members() {
                let membership = fragment_membership
                    .membership_for_fragment_identity(member.fragment_identity())
                    .ok_or_else(|| {
                        counters.rejected_dangling_reference();
                        PlanarBooleanLoopSourceProvenanceDenial::new(
                            Kind::OverlapChainMemberMissingFragmentMembership,
                            member.member_identity(),
                            *counters,
                            "overlap-chain lineage requires fragment membership for every overlap-chain member fragment",
                        )
                    })?;
                member_identities.push(member.member_identity().to_string());
                fragment_identities.push(member.fragment_identity().to_string());
                source_loop_identities.push(membership.source_loop_identity().to_string());
                source_edge_identities.push(membership.source_edge_identity().to_string());
                boundary_roles.push(member.boundary_role());
            }
            let row = PlanarBooleanLoopOverlapChainLineageRow::new(
                overlap_chain_lineage_identity(
                    input.request().request_identity(),
                    overlap_chain.chain_identity(),
                    &fragment_identities,
                ),
                overlap_chain.chain_identity().to_string(),
                member_identities,
                fragment_identities,
                source_loop_identities,
                source_edge_identities,
                boundary_roles,
            );
            counters.recovered_overlap_chain_lineage();
            rows.push(row);
        }
    }
    rows.sort_by(|left, right| left.chain_identity().cmp(right.chain_identity()));
    Ok(PlanarBooleanLoopOverlapChainLineageMap::new(
        overlap_chain_lineage_map_identity(input.request().request_identity(), &rows),
        input.request().request_identity().to_string(),
        input.overlap_chains().chain_set_identity().to_string(),
        rows,
    ))
}

fn index_fragments<'a>(
    input: &'a PlanarBooleanLoopSourceProvenanceRecoveryInput<'a>,
    counters: &mut PlanarBooleanLoopSourceProvenanceCounters,
) -> Result<BTreeMap<String, FragmentBinding<'a>>, PlanarBooleanLoopSourceProvenanceDenial> {
    let mut fragment_index = BTreeMap::new();
    for schedule in input.split_fragments().schedules() {
        for fragment in schedule.fragments() {
            let binding = FragmentBinding {
                fragment,
                carrier_identity: schedule.carrier_identity(),
            };
            if fragment_index
                .insert(fragment.fragment_identity().to_string(), binding)
                .is_some()
            {
                counters.rejected_dangling_reference();
                return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
                    Kind::DuplicateFragmentIdentity,
                    fragment.fragment_identity(),
                    *counters,
                    "loop provenance recovery requires fragment identities to be unique",
                ));
            }
        }
    }
    Ok(fragment_index)
}

fn index_overlap_chains<'a>(
    input: &'a PlanarBooleanLoopSourceProvenanceRecoveryInput<'a>,
    counters: &mut PlanarBooleanLoopSourceProvenanceCounters,
) -> Result<
    BTreeMap<String, &'a PlanarBooleanOverlapEdgeChain>,
    PlanarBooleanLoopSourceProvenanceDenial,
> {
    let mut overlap_index = BTreeMap::new();
    for chain in input.overlap_chains().chains() {
        if overlap_index
            .insert(chain.chain_identity().to_string(), chain)
            .is_some()
        {
            counters.rejected_dangling_reference();
            return Err(PlanarBooleanLoopSourceProvenanceDenial::new(
                Kind::DuplicateOverlapChainIdentity,
                chain.chain_identity(),
                *counters,
                "loop provenance recovery requires overlap-chain identities to be unique",
            ));
        }
    }
    Ok(overlap_index)
}

struct FragmentBinding<'a> {
    fragment: &'a PlanarBooleanSplitEdgeFragment,
    carrier_identity: &'a str,
}
