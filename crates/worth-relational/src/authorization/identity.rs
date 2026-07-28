use sha2::{Digest, Sha256};
use worth_foundational::facade::{
    prepare_aspect_value_identity_basis, AspectFieldLocator, LocatorAuthority,
};

use crate::identity::data::{EntityId, RelationId};
use crate::snapshots::data::{SnapshotHandle, SnapshotReadPolicy};
use crate::transactions::data::RecordRef;

use super::{
    RelationalAuthorizationDecision, RelationalAuthorizationEffectTarget,
    RelationalAuthorizationObservationIdentity, RelationalAuthorizationObservationPlan,
    RelationalAuthorizationPathEffect, RelationalAuthorizationPathObservation,
    RelationalAuthorizationPlanIdentity, RelationalAuthorizationTraversalDirection,
};

pub(super) fn observation_plan_identity(
    plan: &RelationalAuthorizationObservationPlan,
) -> RelationalAuthorizationPlanIdentity {
    let mut hash = Sha256::new();
    hash_text(
        &mut hash,
        "worth-relational.authorization-observation-plan.v1",
    );
    hash_snapshot(&mut hash, plan.snapshot());
    hash_entity(&mut hash, plan.principal());
    hash_entity(&mut hash, plan.scope());
    hash_u64(&mut hash, u64::from(plan.principal_kind().as_u32()));
    hash_u64(&mut hash, u64::from(plan.scope_kind().as_u32()));
    hash_u64(&mut hash, plan.paths().len() as u64);
    for path in plan.paths() {
        hash_u8(
            &mut hash,
            match path.effect() {
                RelationalAuthorizationPathEffect::Allow => 1,
                RelationalAuthorizationPathEffect::Deny => 2,
            },
        );
        hash_u64(&mut hash, path.traversals().len() as u64);
        for traversal in path.traversals() {
            hash_u64(&mut hash, u64::from(traversal.relation_kind().as_u32()));
            hash_u64(&mut hash, u64::from(traversal.from_kind().as_u32()));
            hash_u64(&mut hash, u64::from(traversal.to_kind().as_u32()));
            hash_u8(
                &mut hash,
                match traversal.direction() {
                    RelationalAuthorizationTraversalDirection::Forward => 1,
                    RelationalAuthorizationTraversalDirection::Reverse => 2,
                },
            );
        }
        hash_u64(&mut hash, path.predicates().len() as u64);
        for predicate in path.predicates() {
            hash_u64(&mut hash, predicate.traversal_ordinal() as u64);
            hash_u64(&mut hash, u64::from(predicate.entity_kind().as_u32()));
            hash_locator(&mut hash, predicate.field());
            hash_text(
                &mut hash,
                prepare_aspect_value_identity_basis(predicate.expected()).as_str(),
            );
        }
    }
    hash_u64(&mut hash, plan.proposed_effects().len() as u64);
    for target in plan.proposed_effects() {
        hash_effect_target(&mut hash, target);
    }
    RelationalAuthorizationPlanIdentity(hash.finalize().into())
}

pub(super) fn observation_evidence_identity(
    plan_identity: RelationalAuthorizationPlanIdentity,
    decision: RelationalAuthorizationDecision,
    paths: &[RelationalAuthorizationPathObservation],
) -> RelationalAuthorizationObservationIdentity {
    let mut hash = Sha256::new();
    hash_text(
        &mut hash,
        "worth-relational.authorization-observation-evidence.v1",
    );
    hash.update(plan_identity.bytes());
    hash_u8(
        &mut hash,
        match decision {
            RelationalAuthorizationDecision::Allowed => 1,
            RelationalAuthorizationDecision::Denied => 2,
        },
    );
    hash_u64(&mut hash, paths.len() as u64);
    for path in paths {
        hash_u8(
            &mut hash,
            match path.effect() {
                RelationalAuthorizationPathEffect::Allow => 1,
                RelationalAuthorizationPathEffect::Deny => 2,
            },
        );
        hash_u8(&mut hash, u8::from(path.matched()));
        hash_u8(&mut hash, u8::from(path.exhaustive()));
        hash_u64(&mut hash, path.entities().len() as u64);
        for entity in path.entities() {
            hash_entity(&mut hash, *entity);
        }
        hash_u64(&mut hash, path.relations().len() as u64);
        for relation in path.relations() {
            hash_relation(&mut hash, *relation);
        }
        hash_u64(&mut hash, path.adjacency_lists().len() as u64);
        for adjacency in path.adjacency_lists() {
            hash_entity(&mut hash, adjacency.entity());
            hash_u64(&mut hash, u64::from(adjacency.relation_kind().as_u32()));
            hash_u8(
                &mut hash,
                match adjacency.direction() {
                    RelationalAuthorizationTraversalDirection::Forward => 1,
                    RelationalAuthorizationTraversalDirection::Reverse => 2,
                },
            );
        }
        hash_u64(&mut hash, path.fields().len() as u64);
        for (entity, locator) in path.fields() {
            hash_entity(&mut hash, *entity);
            hash_locator(&mut hash, locator);
        }
    }
    RelationalAuthorizationObservationIdentity(hash.finalize().into())
}

fn hash_snapshot(hash: &mut Sha256, snapshot: &SnapshotHandle) {
    hash_u64(hash, snapshot.runtime_instance_id);
    hash_u64(hash, snapshot.snapshot_id.0);
    hash_u64(hash, snapshot.version_id.as_u64());
    hash_u8(
        hash,
        match snapshot.read_policy {
            SnapshotReadPolicy::ImmutablePinned => 1,
            SnapshotReadPolicy::ImmutablePinnedNoLazyMutation => 2,
        },
    );
}

fn hash_effect_target(hash: &mut Sha256, target: &RelationalAuthorizationEffectTarget) {
    match target.record_ref() {
        RecordRef::Entity(entity) => {
            hash_u8(hash, 1);
            hash_entity(hash, *entity);
        }
        RecordRef::Relation(relation) => {
            hash_u8(hash, 2);
            hash_relation(hash, *relation);
        }
    }
    match target.field_locator() {
        Some(locator) => {
            hash_u8(hash, 1);
            hash_locator(hash, locator);
        }
        None => hash_u8(hash, 0),
    }
}

fn hash_locator(hash: &mut Sha256, locator: &AspectFieldLocator) {
    hash_u8(
        hash,
        match locator.aspect().authority() {
            LocatorAuthority::Authoritative => 1,
            LocatorAuthority::Derived => 2,
            LocatorAuthority::Projected => 3,
            LocatorAuthority::SupportOnly => 4,
            LocatorAuthority::Planned => 5,
            LocatorAuthority::ReceiptBearing => 6,
        },
    );
    hash_text(hash, locator.aspect().aspect_key().as_str());
    hash_u64(hash, locator.field_path().fields().len() as u64);
    for field in locator.field_path().fields() {
        hash_text(hash, field.as_str());
    }
}

fn hash_entity(hash: &mut Sha256, entity: EntityId) {
    hash_record_parts(
        hash,
        entity.partition_value(),
        entity.local_slot_value(),
        entity.generation_value(),
    );
}

fn hash_relation(hash: &mut Sha256, relation: RelationId) {
    hash_record_parts(
        hash,
        relation.partition_value(),
        relation.local_slot_value(),
        relation.generation_value(),
    );
}

fn hash_record_parts(hash: &mut Sha256, partition: u32, slot: u64, generation: u32) {
    hash_u64(hash, u64::from(partition));
    hash_u64(hash, slot);
    hash_u64(hash, u64::from(generation));
}

fn hash_text(hash: &mut Sha256, value: &str) {
    hash_u64(hash, value.len() as u64);
    hash.update(value.as_bytes());
}

fn hash_u64(hash: &mut Sha256, value: u64) {
    hash.update(value.to_le_bytes());
}

fn hash_u8(hash: &mut Sha256, value: u8) {
    hash.update([value]);
}
