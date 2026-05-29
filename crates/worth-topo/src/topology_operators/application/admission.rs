use std::collections::BTreeMap;

use schema::facade::TopologyEntityKind;

use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryBindingIndex, TopologyQueryEditFamilySupportStatus, TopologyRuntimeSupport,
};
use crate::topology_operators::local_rewrites::boundary_wiring::{
    supports_admitted_loop_successor_program, supports_admitted_relation_create_program,
};
use crate::topology_operators::local_rewrites::sheet_wire_laminar::supports_admitted_shell_or_wire_create_program;
use crate::topology_operators::{TopologyEditContract, TopologyEditFamily};

pub(super) fn unsupported_families(
    support: &TopologyRuntimeSupport,
    bindings: &TopologyQueryBindingIndex,
    families: &[TopologyEditFamily],
    contracts: &[TopologyEditContract],
) -> Vec<TopologyEditFamily> {
    let admitted_relation_create_program = supports_admitted_relation_create_program(contracts);
    let admitted_shell_or_wire_create_program =
        supports_admitted_shell_or_wire_create_program(bindings, contracts);
    let admitted_loop_successor_program =
        supports_admitted_loop_successor_program(bindings, contracts);

    let mut unsupported = Vec::new();
    for family in families.iter().copied() {
        let supported = match support.query_edit_family_support_status(family) {
            TopologyQueryEditFamilySupportStatus::Admitted => true,
            TopologyQueryEditFamilySupportStatus::Denied => false,
            TopologyQueryEditFamilySupportStatus::PartiallyAdmittedByLane => {
                (family == TopologyEditFamily::AttachBoundaryMembership
                    && admitted_relation_create_program)
                    || (family == TopologyEditFamily::AttachShellOrWireMembership
                        && admitted_shell_or_wire_create_program)
                    || (family == TopologyEditFamily::RewireLoopSuccessor
                        && admitted_loop_successor_program)
            }
        };
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
            crate::topology_operators::TopologyEditAction::CreateTopologyEntity {
                create_key,
                kind,
                ..
            } => Some((create_key.as_str().to_string(), *kind)),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::{CreateKey, EntityReference, TopologyEntityKind};

    use super::*;

    fn entity_id(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }

    #[test]
    fn snapshot_support_denies_authoritative_create_even_without_bindings() {
        let support = TopologyRuntimeSupport::snapshot_read_only();
        let bindings = TopologyQueryBindingIndex::default();
        let contracts = [TopologyEditContract::create_topology_entity(
            "new-loop",
            TopologyEntityKind::Loop,
        )];
        let unsupported = unsupported_families(
            &support,
            &bindings,
            &[TopologyEditFamily::CreateTopologyEntity],
            &contracts,
        );

        assert_eq!(unsupported, vec![TopologyEditFamily::CreateTopologyEntity]);
    }

    #[test]
    fn current_head_support_admits_face_inner_loop_lane_from_contract_shape() {
        let support = TopologyRuntimeSupport::current_head_authoritative();
        let bindings = TopologyQueryBindingIndex::default();
        let contracts = [
            TopologyEditContract::create_topology_entity("new-loop", TopologyEntityKind::Loop),
            TopologyEditContract::attach_boundary_membership(
                "attach-inner-loop",
                crate::topology_operators::BoundaryMembershipKind::FaceInnerLoop,
                EntityReference::Existing(entity_id(1)),
                EntityReference::Created(CreateKey::new("new-loop")),
            ),
        ];
        let unsupported = unsupported_families(
            &support,
            &bindings,
            &[TopologyEditFamily::AttachBoundaryMembership],
            &contracts,
        );

        assert!(unsupported.is_empty());
    }

    #[test]
    fn current_head_support_keeps_shell_membership_lane_denied_without_bindings() {
        let support = TopologyRuntimeSupport::current_head_authoritative();
        let bindings = TopologyQueryBindingIndex::default();
        let contracts = [
            TopologyEditContract::create_topology_entity("new-wire", TopologyEntityKind::Wire),
            TopologyEditContract::attach_shell_or_wire_membership(
                "attach-wire-half-edge",
                crate::topology_operators::ShellOrWireMembershipKind::WireOwnsHalfEdge,
                EntityReference::Created(CreateKey::new("new-wire")),
                EntityReference::Existing(entity_id(2)),
            ),
        ];
        let unsupported = unsupported_families(
            &support,
            &bindings,
            &[TopologyEditFamily::AttachShellOrWireMembership],
            &contracts,
        );

        assert_eq!(
            unsupported,
            vec![TopologyEditFamily::AttachShellOrWireMembership]
        );
    }
}
