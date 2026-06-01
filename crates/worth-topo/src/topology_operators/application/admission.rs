#[cfg(test)]
use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryBindingIndex, TopologyQueryMutationFamilySupportStatus, TopologyRuntimeSupport,
};
#[cfg(test)]
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
#[cfg(test)]
use crate::topology_operators::local_rewrites::boundary_wiring::{
    supports_admitted_loop_successor_program, supports_admitted_relation_create_program,
};
#[cfg(test)]
use crate::topology_operators::local_rewrites::sheet_wire_laminar::supports_admitted_shell_or_wire_create_program;
#[cfg(test)]
use crate::topology_operators::TopologyDeclaredMutationSequenceBuilder;
#[cfg(test)]
use crate::topology_operators::{TopologyDeclaredMutationSequence, TopologyMutationFamily};

#[cfg(test)]
pub(crate) fn unsupported_mutation_sequence_families(
    support: &TopologyRuntimeSupport,
    bindings: &TopologyQueryBindingIndex,
    sequence: &TopologyDeclaredMutationSequence,
) -> Vec<TopologyMutationFamily> {
    let admitted_relation_create_program = supports_admitted_relation_create_program(sequence);
    let admitted_shell_or_wire_create_program =
        supports_admitted_shell_or_wire_create_program(bindings, sequence);
    let admitted_loop_successor_program =
        supports_admitted_loop_successor_program(bindings, sequence);

    collect_unsupported_families(
        support,
        sequence.families().iter().copied(),
        admitted_relation_create_program,
        admitted_shell_or_wire_create_program,
        admitted_loop_successor_program,
    )
}

#[cfg(test)]
fn collect_unsupported_families(
    support: &TopologyRuntimeSupport,
    families: impl IntoIterator<Item = TopologyMutationFamily>,
    admitted_relation_create_program: bool,
    admitted_shell_or_wire_create_program: bool,
    admitted_loop_successor_program: bool,
) -> Vec<TopologyMutationFamily> {
    let mut unsupported = Vec::new();
    for family in families {
        let supported = match support.query_mutation_family_support_status(family) {
            TopologyQueryMutationFamilySupportStatus::Admitted => true,
            TopologyQueryMutationFamilySupportStatus::Denied => false,
            TopologyQueryMutationFamilySupportStatus::PartiallyAdmittedByLane => {
                (family == TopologyMutationFamily::AttachBoundaryMembership
                    && admitted_relation_create_program)
                    || (family == TopologyMutationFamily::AttachShellOrWireMembership
                        && admitted_shell_or_wire_create_program)
                    || (family == TopologyMutationFamily::RewireLoopSuccessor
                        && admitted_loop_successor_program)
            }
        };
        if !supported && !unsupported.contains(&family) {
            unsupported.push(family);
        }
    }
    unsupported
}

#[cfg(test)]
pub(crate) fn unsupported_declaration_families<D>(
    support: &TopologyRuntimeSupport,
    bindings: &TopologyQueryBindingIndex,
    declaration: &D,
) -> Vec<TopologyMutationFamily>
where
    D: TopologyDeclarationMutationPayload,
{
    let sequence = declaration.clone().into_mutation_sequence();
    unsupported_mutation_sequence_families(support, bindings, &sequence)
}
#[cfg(test)]
mod tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use schema::facade::platform::authority::{CreateKey, EntityReference};
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::*;

    fn entity_id(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }

    #[test]
    fn snapshot_support_denies_authoritative_create_even_without_bindings() {
        let support = TopologyRuntimeSupport::snapshot_read_only();
        let bindings = TopologyQueryBindingIndex::default();
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder.create_topology_entity("new-loop", TopologyEntityKind::Loop);
        let sequence = builder.finish();
        let unsupported = unsupported_mutation_sequence_families(&support, &bindings, &sequence);

        assert_eq!(
            unsupported,
            vec![TopologyMutationFamily::CreateTopologyEntity]
        );
    }

    #[test]
    fn current_head_support_admits_face_inner_loop_lane_from_contract_shape() {
        let support = TopologyRuntimeSupport::current_head_authoritative();
        let bindings = TopologyQueryBindingIndex::default();
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder
            .create_topology_entity("new-loop", TopologyEntityKind::Loop)
            .attach_boundary_membership(
                "attach-inner-loop",
                crate::topology_operators::BoundaryMembershipKind::FaceInnerLoop,
                EntityReference::Existing(entity_id(1)),
                EntityReference::Created(CreateKey::new("new-loop")),
            );
        let sequence = builder.finish();
        let unsupported = unsupported_mutation_sequence_families(&support, &bindings, &sequence);

        assert!(unsupported.is_empty());
    }

    #[test]
    fn current_head_support_keeps_shell_membership_lane_denied_without_bindings() {
        let support = TopologyRuntimeSupport::current_head_authoritative();
        let bindings = TopologyQueryBindingIndex::default();
        let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
        builder
            .create_topology_entity("new-wire", TopologyEntityKind::Wire)
            .attach_shell_or_wire_membership(
                "attach-wire-half-edge",
                crate::topology_operators::ShellOrWireMembershipKind::WireOwnsHalfEdge,
                EntityReference::Created(CreateKey::new("new-wire")),
                EntityReference::Existing(entity_id(2)),
            );
        let sequence = builder.finish();
        let unsupported = unsupported_mutation_sequence_families(&support, &bindings, &sequence);

        assert_eq!(
            unsupported,
            vec![TopologyMutationFamily::AttachShellOrWireMembership]
        );
    }
}
