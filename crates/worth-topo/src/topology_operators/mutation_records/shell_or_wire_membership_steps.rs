use std::collections::BTreeSet;

use forge_relational::facade::identity::RelationId;
use schema::facade::platform::aspects::{Aspect, DiagnosticsAspect, TopologyAspect};
use schema::facade::platform::authority::{CreateKey, EntityReference, TopologyMutation};
use schema::facade::platform::relations::RelationKind;

use crate::topology_operators::declared_mutation_sequence_builder::TopologyDeclaredMutationSequenceBuilder;

use super::records::TopologyDeclaredMutationAction;
use super::{
    ShellOrWireMembershipKind, TopologyDeclaredMutationRecord, TopologyDerivedRegion,
    TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy, TopologyMutationFamily,
    TopologyMutationNamingScope,
};

impl TopologyDeclaredMutationSequenceBuilder {
    pub(crate) fn attach_shell_or_wire_membership(
        &mut self,
        create_key: impl Into<String>,
        kind: ShellOrWireMembershipKind,
        owner: impl Into<EntityReference>,
        member: impl Into<EntityReference>,
    ) -> &mut Self {
        let create_key = CreateKey::new(create_key.into());
        let owner = owner.into();
        let member = member.into();
        let changed_scope = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyMutationChangedScope::Shell,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyMutationChangedScope::Wire,
        };
        let derived_region = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyDerivedRegion::ShellRegion,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyDerivedRegion::WireRegion,
        };
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::AttachShellOrWireMembership,
            action: TopologyDeclaredMutationAction::AttachShellOrWireMembership {
                create_key: create_key.clone(),
                kind,
                owner: owner.clone(),
                member: member.clone(),
            },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Ownership),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                changed_scope,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                derived_region,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::CreateRelation {
                create_key,
                kind: RelationKind::Topology(kind.relation_kind()),
                source: owner,
                target: member,
            }],
        });
        self
    }

    pub(crate) fn detach_shell_or_wire_membership(
        &mut self,
        relation_id: RelationId,
        kind: ShellOrWireMembershipKind,
    ) -> &mut Self {
        let changed_scope = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyMutationChangedScope::Shell,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyMutationChangedScope::Wire,
        };
        let derived_region = match kind {
            ShellOrWireMembershipKind::RegionOwnsShell
            | ShellOrWireMembershipKind::ShellOwnsFace => TopologyDerivedRegion::ShellRegion,
            ShellOrWireMembershipKind::WireOwnsHalfEdge => TopologyDerivedRegion::WireRegion,
        };
        self.records.push(TopologyDeclaredMutationRecord {
            family: TopologyMutationFamily::DetachShellOrWireMembership,
            action: TopologyDeclaredMutationAction::DetachShellOrWireMembership {
                relation_id,
                kind,
            },
            touched_aspects: BTreeSet::from([
                Aspect::Topology(TopologyAspect::Ownership),
                Aspect::Diagnostics(DiagnosticsAspect::Decisions),
            ]),
            changed_scopes: vec![
                TopologyMutationChangedScope::Relation,
                changed_scope,
                TopologyMutationChangedScope::LocalNeighborhood,
            ],
            naming_scopes: vec![TopologyMutationNamingScope::AdjacentEntityNames],
            derived_regions: vec![
                derived_region,
                TopologyDerivedRegion::MutationLocalNeighborhoodRegion,
                TopologyDerivedRegion::NamingContinuityRegion,
            ],
            derived_fallback_policy: TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
            lowered_mutations: vec![TopologyMutation::RemoveRelation { relation_id }],
        });
        self
    }
}
