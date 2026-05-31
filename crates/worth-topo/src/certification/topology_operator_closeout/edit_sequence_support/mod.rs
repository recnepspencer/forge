use schema::facade::platform::authority::{MutationOrigin, RawTopologyIntent};

use crate::topology_operators::application::TopologyDeclarationContractPayload;
use crate::topology_operators::{
    naming_edit_continuity_matrix_for_contracts, topology_edit_families_for_contracts,
    NamingEditContinuityMatrix, TopologyCreateInnerLoopOnExistingFaceDeclaration,
    TopologyDetachBoundaryMembershipDeclaration, TopologyEditContract,
    TopologyEditDerivedFallbackPolicy, TopologyEditDigest, TopologyEditFamily,
    TopologyEditNamingOutcome, TopologyOperatorDigest,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyRetireTopologyEntityDeclaration,
    TopologyRewireLoopSuccessorProgramDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
};

#[derive(Clone)]
pub(super) enum TopologyCloseoutDeclaration {
    CreateInnerLoopOnExistingFace(TopologyCreateInnerLoopOnExistingFaceDeclaration),
    DetachBoundaryMembership(TopologyDetachBoundaryMembershipDeclaration),
    RetireTopologyEntity(TopologyRetireTopologyEntityDeclaration),
    RehomeAllOwnedHalfEdgesToNewWire(TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration),
    RewireLoopSuccessorProgram(TopologyRewireLoopSuccessorProgramDeclaration),
    SplitConnectedHalfEdgeSetToNewWire(TopologySplitConnectedHalfEdgeSetToNewWireDeclaration),
}

impl TopologyDeclarationContractPayload for TopologyCloseoutDeclaration {
    const SEMANTIC_FAMILY_KEY: &'static str = "topology.closeout_declaration";

    fn into_contracts(self) -> Vec<TopologyEditContract> {
        match self {
            Self::CreateInnerLoopOnExistingFace(declaration) => declaration.into_contracts(),
            Self::DetachBoundaryMembership(declaration) => declaration.into_contracts(),
            Self::RetireTopologyEntity(declaration) => declaration.into_contracts(),
            Self::RehomeAllOwnedHalfEdgesToNewWire(declaration) => declaration.into_contracts(),
            Self::RewireLoopSuccessorProgram(declaration) => declaration.into_contracts(),
            Self::SplitConnectedHalfEdgeSetToNewWire(declaration) => declaration.into_contracts(),
        }
    }
}

pub(super) fn aggregate_topology_edit_digest_for_contract_sets(
    contract_sets: impl IntoIterator<Item = Vec<TopologyEditContract>>,
) -> TopologyEditDigest {
    let contract_sets = contract_sets.into_iter().collect::<Vec<_>>();
    let rows = contract_sets.iter().flatten().map(contract_digest_row);
    let contract_count = contract_sets.iter().map(Vec::len).sum();
    let family_count = contract_sets
        .iter()
        .map(|contracts| topology_edit_families_for_contracts(contracts).len())
        .sum();
    let changed_scope_count = contract_sets
        .iter()
        .flatten()
        .map(|contract| contract.changed_scopes().len())
        .sum();
    let naming_scope_count = contract_sets
        .iter()
        .flatten()
        .map(|contract| contract.naming_scopes().len())
        .sum();
    let derived_region_count = contract_sets
        .iter()
        .flatten()
        .map(|contract| contract.derived_regions().len())
        .sum();
    let fallback_policy_count = contract_sets.iter().map(Vec::len).sum();
    let fallback_rejection_policy_count = contract_sets
        .iter()
        .flatten()
        .filter(|contract| {
            contract.derived_fallback_policy()
                == TopologyEditDerivedFallbackPolicy::RejectAnyFallback
        })
        .count();
    digest_rows(rows).with_counts(
        contract_count,
        family_count,
        changed_scope_count,
        naming_scope_count,
        derived_region_count,
        fallback_policy_count,
        fallback_rejection_policy_count,
    )
}

pub(super) fn aggregate_naming_edit_continuity_matrix_for_contract_sets(
    contract_sets: impl IntoIterator<Item = Vec<TopologyEditContract>>,
) -> NamingEditContinuityMatrix {
    let rows = contract_sets
        .into_iter()
        .flat_map(|contracts| naming_edit_continuity_matrix_for_contracts(&contracts).rows)
        .collect::<Vec<_>>();
    naming_edit_continuity_matrix_from_rows(rows)
}

pub(super) fn aggregate_topology_edit_digest_for_declarations<D>(
    declarations: impl IntoIterator<Item = D>,
) -> TopologyEditDigest
where
    D: TopologyDeclarationContractPayload,
{
    aggregate_topology_edit_digest_for_contract_sets(declaration_contract_sets(declarations))
}

pub(super) fn aggregate_naming_edit_continuity_matrix_for_declarations<D>(
    declarations: impl IntoIterator<Item = D>,
) -> NamingEditContinuityMatrix
where
    D: TopologyDeclarationContractPayload,
{
    aggregate_naming_edit_continuity_matrix_for_contract_sets(declaration_contract_sets(
        declarations,
    ))
}

pub(super) fn topology_edit_families_for_declarations<D>(
    declarations: impl IntoIterator<Item = D>,
) -> Vec<TopologyEditFamily>
where
    D: TopologyDeclarationContractPayload,
{
    declarations
        .into_iter()
        .flat_map(|declaration| declaration.semantic_families())
        .collect()
}

pub(super) fn branch_local_raw_topology_intent_from_contracts(
    contracts: Vec<TopologyEditContract>,
) -> RawTopologyIntent {
    let mutations = contracts
        .into_iter()
        .flat_map(|contract| contract.lowered_mutations().to_vec())
        .collect();
    RawTopologyIntent::new(mutations, MutationOrigin::BranchLocalApplication)
}

pub(super) fn branch_local_raw_topology_intent_for_declaration<D>(
    declaration: D,
) -> RawTopologyIntent
where
    D: TopologyDeclarationContractPayload,
{
    branch_local_raw_topology_intent_from_contracts(declaration.into_contracts())
}

fn declaration_contract_sets<D>(
    declarations: impl IntoIterator<Item = D>,
) -> Vec<Vec<TopologyEditContract>>
where
    D: TopologyDeclarationContractPayload,
{
    declarations
        .into_iter()
        .map(|declaration| declaration.into_contracts())
        .collect()
}

fn naming_edit_continuity_matrix_from_rows(
    rows: Vec<crate::topology_operators::TopologyEditNamingRow>,
) -> NamingEditContinuityMatrix {
    let preserved_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Preserved)
        .count();
    let ambiguous_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Ambiguous)
        .count();
    let rejected_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Rejected)
        .count();
    NamingEditContinuityMatrix {
        rows,
        preserved_count,
        ambiguous_count,
        rejected_count,
    }
}

fn contract_digest_row(contract: &TopologyEditContract) -> String {
    serde_json::to_string(contract).expect("topology edit contracts should serialize")
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> TopologyOperatorDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    TopologyOperatorDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}

trait WithCounts {
    fn with_counts(
        self,
        contract_count: usize,
        family_count: usize,
        changed_scope_count: usize,
        naming_scope_count: usize,
        derived_region_count: usize,
        fallback_policy_count: usize,
        fallback_rejection_policy_count: usize,
    ) -> TopologyEditDigest;
}

impl WithCounts for TopologyOperatorDigest {
    fn with_counts(
        self,
        contract_count: usize,
        family_count: usize,
        changed_scope_count: usize,
        naming_scope_count: usize,
        derived_region_count: usize,
        fallback_policy_count: usize,
        fallback_rejection_policy_count: usize,
    ) -> TopologyEditDigest {
        TopologyEditDigest {
            digest: self,
            contract_count,
            family_count,
            changed_scope_count,
            naming_scope_count,
            derived_region_count,
            fallback_policy_count,
            fallback_rejection_policy_count,
        }
    }
}
