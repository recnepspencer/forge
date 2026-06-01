use forge_relational::facade::identity::{EntityId, PartitionId};
use schema::facade::platform::aspects::{Aspect, DiagnosticsAspect, NamingAspect, TopologyAspect};
use schema::facade::platform::entities::TopologyEntityKind;

use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::application::TopologyMutationApplicationError;
use crate::topology_operators::{
    topology_mutation_digest_for_records, BoundaryMembershipKind, ShellOrWireMembershipKind,
    TopologyAttachBoundaryMembershipDeclaration, TopologyCreateTopologyEntityDeclaration,
    TopologyDeclaredMutationSequence, TopologyDeclaredMutationSequenceBuilder,
    TopologyDerivedRegion, TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationNamingScope,
    TopologyMutationRejectionClass, TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    TopologyRetireTopologyEntityDeclaration, TopologyWireRehomeHalfEdgeMember,
};

#[test]
fn create_topology_entity_record_is_topology_only_and_naming_aware() {
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "m3.contract.vertex",
        TopologyEntityKind::Vertex,
    );
    let sequence = declaration.into_mutation_sequence();
    let members = sequence.members().collect::<Vec<_>>();
    let record = members[0].record();

    assert_eq!(record.family, TopologyMutationFamily::CreateTopologyEntity);
    assert!(record
        .touched_aspects()
        .contains(&Aspect::Topology(TopologyAspect::Structure)));
    assert!(record
        .touched_aspects()
        .contains(&Aspect::Naming(NamingAspect::PersistentName)));
    assert!(record
        .touched_aspects()
        .contains(&Aspect::Diagnostics(DiagnosticsAspect::Decisions)));
    assert_eq!(
        record.changed_scopes(),
        &[
            TopologyMutationChangedScope::Entity,
            TopologyMutationChangedScope::Naming,
        ]
    );
    assert_eq!(
        record.naming_scopes(),
        &[TopologyMutationNamingScope::EditedEntityNames]
    );
    assert_eq!(
        record.derived_regions(),
        &[
            TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
            TopologyDerivedRegion::NamingContinuityRegion,
        ]
    );
    assert_eq!(
        record.derived_fallback_policy(),
        TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback
    );
}

#[test]
fn boundary_membership_record_exposes_boundary_scope_and_regions() {
    let declaration = TopologyAttachBoundaryMembershipDeclaration::new(
        "m3.boundary.loop".to_string(),
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        schema::facade::platform::authority::EntityReference::Existing(EntityId::new(
            PartitionId::main(),
            1,
            1,
        )),
        schema::facade::platform::authority::EntityReference::Existing(EntityId::new(
            PartitionId::main(),
            2,
            1,
        )),
    );
    let sequence = declaration.into_mutation_sequence();
    let members = sequence.members().collect::<Vec<_>>();
    let record = members[0].record();

    assert_eq!(
        record.family,
        TopologyMutationFamily::AttachBoundaryMembership
    );
    assert!(record
        .touched_aspects()
        .contains(&Aspect::Topology(TopologyAspect::Boundary)));
    assert!(record
        .changed_scopes()
        .contains(&TopologyMutationChangedScope::Loop));
    assert!(record
        .derived_regions()
        .contains(&TopologyDerivedRegion::LoopRegion));
}

#[test]
fn declared_mutation_sequence_digest_is_deterministic_for_same_declarations() {
    let sequence = TopologyDeclaredMutationSequence::concatenate([
        TopologyCreateTopologyEntityDeclaration::new(
            "m3.digest.vertex",
            TopologyEntityKind::Vertex,
        )
        .into_mutation_sequence(),
        TopologyAttachBoundaryMembershipDeclaration::new(
            "m3.digest.loop",
            BoundaryMembershipKind::LoopOwnsHalfEdge,
            EntityId::new(PartitionId::main(), 1, 1),
            EntityId::new(PartitionId::main(), 2, 1),
        )
        .into_mutation_sequence(),
    ]);

    let left_digest = sequence.topology_mutation_digest().clone();
    let right_digest = sequence.topology_mutation_digest().clone();
    assert_eq!(left_digest, right_digest);
    assert_eq!(left_digest.mutation_record_count, 2);
    assert_eq!(left_digest.family_count, 2);
    assert_eq!(left_digest.changed_scope_count, 5);
    assert_eq!(left_digest.naming_scope_count, 2);
    assert_eq!(left_digest.derived_region_count, 5);
    assert_eq!(left_digest.fallback_policy_count, 2);
    assert_eq!(left_digest.fallback_rejection_policy_count, 0);
}

#[test]
fn single_record_digest_tracks_locality_only_fallback_policy() {
    let mut builder = TopologyDeclaredMutationSequenceBuilder::builder();
    builder.attach_boundary_membership(
        "m3.digest.local_only.loop",
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        EntityId::new(PartitionId::main(), 1, 1),
        EntityId::new(PartitionId::main(), 2, 1),
    );
    let record = builder
        .finish()
        .members()
        .next()
        .expect("single-member boundary membership sequence")
        .record()
        .clone()
        .with_derived_fallback_policy(TopologyMutationDerivedFallbackPolicy::RejectAnyFallback);
    let digest = topology_mutation_digest_for_records(&[record]);

    assert_eq!(digest.fallback_policy_count, 1);
    assert_eq!(digest.fallback_rejection_policy_count, 1);
}

#[test]
fn declared_mutation_sequence_continuity_matrix_counts_naming_outcomes() {
    let sequence = TopologyDeclaredMutationSequence::concatenate([
        TopologyCreateTopologyEntityDeclaration::new(
            "m3.naming.vertex",
            TopologyEntityKind::Vertex,
        )
        .into_mutation_sequence(),
        TopologyAttachBoundaryMembershipDeclaration::new(
            "m3.naming.loop",
            BoundaryMembershipKind::LoopOwnsHalfEdge,
            EntityId::new(PartitionId::main(), 1, 1),
            EntityId::new(PartitionId::main(), 2, 1),
        )
        .into_mutation_sequence(),
    ]);

    let matrix = sequence.naming_continuity_matrix().clone();

    assert_eq!(matrix.rows.len(), 2);
    assert_eq!(matrix.preserved_count, 1);
    assert_eq!(matrix.ambiguous_count, 1);
    assert_eq!(matrix.rejected_count, 0);
    assert_eq!(
        matrix.rows[0].outcome,
        TopologyMutationNamingOutcome::Preserved
    );
    assert_eq!(
        matrix.rows[1].outcome,
        TopologyMutationNamingOutcome::Ambiguous
    );
}

#[test]
fn continuity_matrix_exposes_overall_outcome_class() {
    let ambiguous = TopologyAttachBoundaryMembershipDeclaration::new(
        "m3.naming.ambiguous.loop".to_string(),
        BoundaryMembershipKind::LoopOwnsHalfEdge,
        schema::facade::platform::authority::EntityReference::Existing(EntityId::new(
            PartitionId::main(),
            1,
            1,
        )),
        schema::facade::platform::authority::EntityReference::Existing(EntityId::new(
            PartitionId::main(),
            2,
            1,
        )),
    )
    .naming_continuity_matrix();
    assert_eq!(
        ambiguous.outcome_class(),
        TopologyMutationNamingOutcome::Ambiguous
    );
    assert_eq!(
        ambiguous.rejection_class(),
        Some(TopologyMutationRejectionClass::NamingContinuityAmbiguous)
    );

    let rejected = TopologyRetireTopologyEntityDeclaration::new(
        EntityId::new(PartitionId::main(), 3, 1),
        TopologyEntityKind::Loop,
    )
    .naming_continuity_matrix();
    assert_eq!(
        rejected.outcome_class(),
        TopologyMutationNamingOutcome::Rejected
    );
    assert_eq!(
        rejected.rejection_class(),
        Some(TopologyMutationRejectionClass::NamingContinuityRejected)
    );
}

#[test]
fn topology_mutation_rejection_taxonomy_matches_milestone_three_spec() {
    assert_eq!(
        TopologyMutationRejectionClass::ALL,
        [
            TopologyMutationRejectionClass::OutOfClassEdit,
            TopologyMutationRejectionClass::InvariantBlocked,
            TopologyMutationRejectionClass::NamingContinuityAmbiguous,
            TopologyMutationRejectionClass::NamingContinuityRejected,
            TopologyMutationRejectionClass::ScopeLocalizationUnavailable,
            TopologyMutationRejectionClass::DerivedFallbackExceeded,
        ]
    );
    assert_eq!(
        TopologyMutationRejectionClass::ScopeLocalizationUnavailable.as_str(),
        "ScopeLocalizationUnavailable"
    );
    assert_eq!(
        TopologyMutationRejectionClass::DerivedFallbackExceeded.as_str(),
        "DerivedFallbackExceeded"
    );
}

#[test]
fn missing_authoritative_scope_reports_scope_localization_unavailable() {
    let missing_entity = EntityId::new(PartitionId::main(), 99, 1);
    let error = TopologyMutationApplicationError::MissingExistingEntityBinding(missing_entity);

    assert_eq!(
        error.rejection_class(),
        Some(TopologyMutationRejectionClass::ScopeLocalizationUnavailable),
        "a missing live authority binding means the mutation application runner cannot localize the requested scope, not that a specific invariant was proven false"
    );
}

#[test]
fn declaration_mutation_sequence_preserves_created_entity_kinds_and_digest_metadata() {
    let declaration = TopologyCreateTopologyEntityDeclaration::new(
        "m3.sequence.vertex",
        TopologyEntityKind::Vertex,
    );

    let sequence = declaration.into_mutation_sequence();

    assert_eq!(
        sequence.created_entity_kinds().get("m3.sequence.vertex"),
        Some(&TopologyEntityKind::Vertex)
    );
    assert_eq!(
        sequence.families(),
        &[TopologyMutationFamily::CreateTopologyEntity]
    );
    assert_eq!(sequence.topology_mutation_digest().mutation_record_count, 1);
    assert_eq!(sequence.naming_continuity_matrix().preserved_count, 1);
    assert_eq!(sequence.naming_report().rows.len(), 1);
}

#[test]
fn grouped_declaration_mutation_sequence_keeps_multi_record_order_and_families() {
    let declaration = TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        "m3.sequence.wire",
        EntityId::new(PartitionId::main(), 99, 1),
        vec![
            TopologyWireRehomeHalfEdgeMember::new(
                "m3.sequence.wire.1",
                EntityId::new(PartitionId::main(), 10, 1),
            ),
            TopologyWireRehomeHalfEdgeMember::new(
                "m3.sequence.wire.2",
                EntityId::new(PartitionId::main(), 11, 1),
            ),
        ],
    );

    let sequence = declaration.into_mutation_sequence();
    let members = sequence.members().collect::<Vec<_>>();
    let mut expected = TopologyDeclaredMutationSequenceBuilder::builder();
    expected
        .create_topology_entity("m3.sequence.wire", TopologyEntityKind::Wire)
        .attach_shell_or_wire_membership(
            "m3.sequence.wire.1",
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
            schema::facade::topology_authoring::created_ref("m3.sequence.wire"),
            EntityId::new(PartitionId::main(), 10, 1),
        )
        .attach_shell_or_wire_membership(
            "m3.sequence.wire.2",
            ShellOrWireMembershipKind::WireOwnsHalfEdge,
            schema::facade::topology_authoring::created_ref("m3.sequence.wire"),
            EntityId::new(PartitionId::main(), 11, 1),
        )
        .retire_topology_entity(
            EntityId::new(PartitionId::main(), 99, 1),
            TopologyEntityKind::Wire,
        );
    let expected = expected.finish();
    let expected_members = expected.members().collect::<Vec<_>>();

    assert_eq!(members.len(), 4);
    assert_eq!(*members[0].record(), *expected_members[0].record());
    assert_eq!(*members[1].record(), *expected_members[1].record());
    assert_eq!(sequence.families().len(), 4);
    assert_eq!(
        sequence.created_entity_kinds().get("m3.sequence.wire"),
        Some(&TopologyEntityKind::Wire)
    );
}
