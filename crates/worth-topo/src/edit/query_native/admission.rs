use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryEntity;
use schema::facade::TopologyEntityKind;

use super::relation_boundary::supports_admitted_relation_create_workflow;
use super::relation_shell_or_wire::supports_admitted_shell_or_wire_create_workflow;
use super::relation_successor::supports_admitted_loop_successor_workflow;
use crate::edit::{TopologyEditContract, TopologyEditFamily};

pub(super) fn unsupported_families(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    families: &[TopologyEditFamily],
    contracts: &[TopologyEditContract],
) -> Vec<TopologyEditFamily> {
    let admitted_relation_create_workflow = supports_admitted_relation_create_workflow(contracts);
    let admitted_shell_or_wire_create_workflow =
        supports_admitted_shell_or_wire_create_workflow(entity_rows, relation_rows, contracts);
    let admitted_loop_successor_workflow =
        supports_admitted_loop_successor_workflow(entity_rows, relation_rows, contracts);

    let mut unsupported = Vec::new();
    for family in families.iter().copied() {
        let supported = family == TopologyEditFamily::CreateTopologyEntity
            || (family == TopologyEditFamily::AttachBoundaryMembership
                && admitted_relation_create_workflow)
            || (family == TopologyEditFamily::AttachShellOrWireMembership
                && admitted_shell_or_wire_create_workflow)
            || (family == TopologyEditFamily::RewireLoopSuccessor
                && admitted_loop_successor_workflow)
            || family == TopologyEditFamily::DetachBoundaryMembership
            || family == TopologyEditFamily::DetachRadialAdjacency
            || family == TopologyEditFamily::DetachShellOrWireMembership
            || family == TopologyEditFamily::RewireLoopEndpoint
            || family == TopologyEditFamily::SpliceRadialAdjacency
            || family == TopologyEditFamily::RetireTopologyEntity;
        if !supported && !unsupported.contains(&family) {
            unsupported.push(family);
        }
    }
    unsupported
}

pub(super) fn planned_created_entity_kinds(
    contracts: &[TopologyEditContract],
) -> BTreeMap<String, TopologyEntityKind> {
    contracts
        .iter()
        .filter_map(|contract| match &contract.action {
            crate::edit::TopologyEditAction::CreateTopologyEntity {
                create_key, kind, ..
            } => Some((create_key.as_str().to_string(), *kind)),
            _ => None,
        })
        .collect()
}
