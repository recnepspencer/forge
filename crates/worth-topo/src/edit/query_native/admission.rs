use std::collections::BTreeMap;

use forge_query::facade::ForgeQueryEntity;
use worth_schema::facade::WorthTopologyEntityKind;

use super::relation_boundary::supports_admitted_relation_create_workflow;
use super::relation_shell_or_wire::supports_admitted_shell_or_wire_create_workflow;
use super::relation_successor::supports_admitted_loop_successor_workflow;
use crate::edit::{WorthTopologyEditContract, WorthTopologyEditFamily};

pub(super) fn unsupported_families(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
    families: &[WorthTopologyEditFamily],
    contracts: &[WorthTopologyEditContract],
) -> Vec<WorthTopologyEditFamily> {
    let admitted_relation_create_workflow = supports_admitted_relation_create_workflow(contracts);
    let admitted_shell_or_wire_create_workflow =
        supports_admitted_shell_or_wire_create_workflow(entity_rows, relation_rows, contracts);
    let admitted_loop_successor_workflow =
        supports_admitted_loop_successor_workflow(entity_rows, relation_rows, contracts);

    let mut unsupported = Vec::new();
    for family in families.iter().copied() {
        let supported = family == WorthTopologyEditFamily::CreateTopologyEntity
            || (family == WorthTopologyEditFamily::AttachBoundaryMembership
                && admitted_relation_create_workflow)
            || (family == WorthTopologyEditFamily::AttachShellOrWireMembership
                && admitted_shell_or_wire_create_workflow)
            || (family == WorthTopologyEditFamily::RewireLoopSuccessor
                && admitted_loop_successor_workflow)
            || family == WorthTopologyEditFamily::DetachBoundaryMembership
            || family == WorthTopologyEditFamily::DetachRadialAdjacency
            || family == WorthTopologyEditFamily::DetachShellOrWireMembership
            || family == WorthTopologyEditFamily::RewireLoopEndpoint
            || family == WorthTopologyEditFamily::SpliceRadialAdjacency
            || family == WorthTopologyEditFamily::RetireTopologyEntity;
        if !supported && !unsupported.contains(&family) {
            unsupported.push(family);
        }
    }
    unsupported
}

pub(super) fn planned_created_entity_kinds(
    contracts: &[WorthTopologyEditContract],
) -> BTreeMap<String, WorthTopologyEntityKind> {
    contracts
        .iter()
        .filter_map(|contract| match &contract.action {
            crate::edit::WorthTopologyEditAction::CreateTopologyEntity {
                create_key, kind, ..
            } => Some((create_key.as_str().to_string(), *kind)),
            _ => None,
        })
        .collect()
}
