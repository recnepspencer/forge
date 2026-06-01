use std::collections::BTreeSet;

use forge_relational::facade::identity::RelationId;
use schema::facade::platform::aspects::{Aspect, DiagnosticsAspect, TopologyAspect};
use schema::facade::platform::authority::{CreateKey, EntityReference, TopologyMutation};
use schema::facade::platform::relations::RelationKind;

use crate::topology_operators::declared_mutation_sequence_builder::TopologyDeclaredMutationSequenceBuilder;

use super::records::TopologyDeclaredMutationAction;
use super::{
    BoundaryMembershipKind, TopologyDeclaredMutationRecord, TopologyDerivedRegion,
    TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy, TopologyMutationFamily,
    TopologyMutationNamingScope,
};

impl TopologyDeclaredMutationSequenceBuilder {
    pub(crate) fn attach_boundary_membership(
        &mut self,
        create_key: impl Into<String>,
        kind: BoundaryMembershipKind,
        owner: impl Into<EntityReference>,
        member: impl Into<EntityReference>,
    ) -> &mut Self {
        let owner = owner.into();
        let member = member.into();
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        let lowered_mutations = vec![TopologyMutation::CreateRelation {
            create_key: CreateKey::new(create_key.into()),
            kind: RelationKind::Topology(kind.relation_kind()),
            source: owner.clone(),
            target: member.clone(),
        }];
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::AttachBoundaryMembership,
            action: TopologyDeclaredMutationAction::AttachBoundaryMembership {
                create_key: match &lowered_mutations[0] {
                    TopologyMutation::CreateRelation { create_key, .. } => create_key.clone(),
                    _ => unreachable!(),
                },
                kind,
                owner,
                member,
            },
            touched_aspects,
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                TopologyMutationChangedScope::Loop,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations,
        });
        self
    }

    pub(crate) fn detach_boundary_membership(
        &mut self,
        relation_id: RelationId,
        kind: BoundaryMembershipKind,
    ) -> &mut Self {
        let touched_aspects = BTreeSet::from([
            Aspect::Topology(TopologyAspect::Boundary),
            Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        ]);
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::DetachBoundaryMembership,
            action: TopologyDeclaredMutationAction::DetachBoundaryMembership { relation_id, kind },
            touched_aspects,
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                TopologyMutationChangedScope::Loop,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                TopologyDerivedRegion::LoopRegion,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveRelation { relation_id }],
        });
        self
    }
}
