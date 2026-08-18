use std::collections::{BTreeMap, BTreeSet};

use super::comparison::ObservedSupplyChainState;
use super::handles::SupplyChainSemanticHandles;
use super::production_world::ProductionSeededSupplyChainWorld;
use super::program::CompiledSupplyChainProgram;
use super::schema::{
    BookingStatus, CargoLotRecord, EntityRecord, InspectionRecord, InspectionResult,
    PortCallRecord, PortRecord, RelationEdge, TerminalRecord, VesselRecord, VoyageRecord,
};
use super::semantic_key::{BranchLabel, EntityKey, EntityKind, RelationKey};
use worth_foundational::facade::{
    AspectKey, AspectValue, AuthoritativeRecordAspectState, ContractValidatedAspectValueView,
    InternedString,
};
use worth_relational::facade::identity::{EntityId, KindId, RelationId};
use worth_relational::facade::runtime::RelationalReadView;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ObservationError {
    SnapshotUnavailable,
    UnknownEntityIdentity(EntityId),
    UnknownRelationIdentity(RelationId),
    MissingEntity(EntityKey),
    MissingRelation(RelationKey),
    WrongEntityKind {
        key: EntityKey,
        observed: KindId,
    },
    WrongRelationKind {
        key: RelationKey,
        observed: KindId,
    },
    UnknownRelationEndpoint(RelationKey),
    MissingAspect {
        entity: EntityKey,
        name: String,
    },
    InvalidAspect {
        entity: EntityKey,
        name: String,
        detail: String,
    },
}

pub(crate) fn observe(
    world: &ProductionSeededSupplyChainWorld,
) -> Result<ObservedSupplyChainState, ObservationError> {
    if world.handles.snapshot.runtime_instance_id != world.handles.branch.runtime_instance_id
        || world.handles.snapshot.branch_id != world.handles.branch.branch_id
    {
        return Err(ObservationError::SnapshotUnavailable);
    }
    let Some(view) = world
        .runtime
        .read_truth()
        .read_snapshot(&world.handles.snapshot)
    else {
        return Err(ObservationError::SnapshotUnavailable);
    };
    let entities = observe_entities(&world.program, &world.handles, &view)?;
    let relations = observe_relations(&world.handles, &view)?;
    Ok(ObservedSupplyChainState {
        schema: world.program.definition().schema.version,
        relation_vector: relations.values().copied().collect(),
        entities,
        relations,
        absent_entities: BTreeSet::new(),
        absent_relations: BTreeSet::new(),
        branch: BranchLabel::Operating,
        parent: None,
        lineage: vec![BranchLabel::Operating],
        accepted: Vec::new(),
        history: Vec::new(),
    })
}

fn observe_entities(
    program: &CompiledSupplyChainProgram,
    handles: &SupplyChainSemanticHandles,
    view: &RelationalReadView,
) -> Result<BTreeMap<EntityKey, EntityRecord>, ObservationError> {
    let mut entities = BTreeMap::new();
    for record in view.entities() {
        let Some(key) = handles.entity_key(record.entity_id) else {
            return Err(ObservationError::UnknownEntityIdentity(record.entity_id));
        };
        let expected_kind = key.kind;
        if record.kind.kind_id != super::program::entity_kind_id(expected_kind) {
            return Err(ObservationError::WrongEntityKind {
                key,
                observed: record.kind.kind_id,
            });
        }
        let Some(state) = record.authoritative_aspect_state.as_ref() else {
            return Err(ObservationError::MissingAspect {
                entity: key,
                name: "record".to_owned(),
            });
        };
        let value = decode_entity(key, state)?;
        entities.insert(key, value);
    }
    for key in program.definition().entities.keys() {
        if !entities.contains_key(key) {
            return Err(ObservationError::MissingEntity(*key));
        }
    }
    Ok(entities)
}

fn observe_relations(
    handles: &SupplyChainSemanticHandles,
    view: &RelationalReadView,
) -> Result<BTreeMap<RelationKey, RelationEdge>, ObservationError> {
    let mut relations = BTreeMap::new();
    for record in view.relations() {
        let Some(key) = handles.relation_key(record.relation_id) else {
            return Err(ObservationError::UnknownRelationIdentity(
                record.relation_id,
            ));
        };
        if record.kind.kind_id != super::program::relation_kind_id(key.kind) {
            return Err(ObservationError::WrongRelationKind {
                key,
                observed: record.kind.kind_id,
            });
        }
        let (Some(source), Some(target)) = (
            handles.entity_key(record.source),
            handles.entity_key(record.target),
        ) else {
            return Err(ObservationError::UnknownRelationEndpoint(key));
        };
        relations.insert(
            key,
            RelationEdge {
                key,
                source,
                target,
            },
        );
    }
    for key in handles.relations.keys() {
        if !relations.contains_key(key) {
            return Err(ObservationError::MissingRelation(*key));
        }
    }
    Ok(relations)
}

fn decode_entity(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
) -> Result<EntityRecord, ObservationError> {
    match key.kind {
        EntityKind::Port => Ok(EntityRecord::Port(PortRecord {
            code: u16_value(key, state, "port_code")?,
            name: string_value(key, state, "name")?,
            region: debug_region(key, state, "region")?,
            posture: debug_posture(key, state, "posture")?,
        })),
        EntityKind::Terminal => Ok(EntityRecord::Terminal(TerminalRecord {
            name: string_value(key, state, "name")?,
            capacity: super::schema::Capacity(u32_value(key, state, "capacity")?),
            posture: debug_posture(key, state, "posture")?,
        })),
        EntityKind::Berth => Ok(EntityRecord::Berth(super::schema::BerthRecord {
            name: string_value(key, state, "name")?,
            depth: super::schema::DepthMeters(u16_value(key, state, "depth")?),
            capacity: super::schema::Capacity(u32_value(key, state, "capacity")?),
            posture: debug_posture(key, state, "posture")?,
        })),
        EntityKind::Vessel => Ok(EntityRecord::Vessel(VesselRecord {
            call_sign: string_value(key, state, "call_sign")?,
            class: debug_class(key, state, "class")?,
            capacity: super::schema::Capacity(u32_value(key, state, "capacity")?),
            posture: debug_posture(key, state, "posture")?,
        })),
        EntityKind::Voyage => Ok(EntityRecord::Voyage(VoyageRecord {
            status: debug_status(key, state, "status")?,
            departure: super::schema::Minute(u32_value(key, state, "departure")?),
            arrival: super::schema::Minute(u32_value(key, state, "arrival")?),
            revision: u16_value(key, state, "revision")?,
        })),
        EntityKind::PortCall => Ok(EntityRecord::PortCall(PortCallRecord {
            sequence: u16_value(key, state, "sequence")?,
            revision: u16_value(key, state, "revision")?,
        })),
        EntityKind::CargoLot => Ok(EntityRecord::CargoLot(CargoLotRecord {
            mass: super::schema::MassTons(u32_value(key, state, "mass")?),
            customer_code: super::schema::CargoCode(string_value(key, state, "customer_code")?),
            hazard: debug_hazard(key, state, "hazard")?,
            booking: debug_booking(key, state, "booking")?,
        })),
        EntityKind::Inspection => Ok(EntityRecord::Inspection(InspectionRecord {
            result: debug_result(key, state, "result")?,
            minute: super::schema::Minute(u32_value(key, state, "minute")?),
        })),
    }
}

fn value<'a>(
    key: EntityKey,
    state: &'a AuthoritativeRecordAspectState,
    name: &str,
) -> Result<&'a AspectValue, ObservationError> {
    let aspect = AspectKey::new(name).expect("canonical Supply Chain aspect key");
    let Some(entry) = state.get(&aspect) else {
        return Err(ObservationError::MissingAspect {
            entity: key,
            name: name.to_owned(),
        });
    };
    match entry.view() {
        ContractValidatedAspectValueView::Scalar(value) => Ok(value),
        ContractValidatedAspectValueView::Struct(_) => Err(ObservationError::InvalidAspect {
            entity: key,
            name: name.to_owned(),
            detail: "expected scalar value".to_owned(),
        }),
    }
}

fn string_value(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<String, ObservationError> {
    match value(key, state, name)? {
        AspectValue::String(InternedString::Raw(value)) => Ok(value.clone()),
        _ => Err(invalid(key, name, "expected raw string")),
    }
}

fn u16_value(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<u16, ObservationError> {
    match value(key, state, name)? {
        AspectValue::UInt64(value) => (*value)
            .try_into()
            .map_err(|_| invalid(key, name, "UInt64 value exceeds u16")),
        _ => Err(invalid(key, name, "expected UInt64")),
    }
}

fn u32_value(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<u32, ObservationError> {
    match value(key, state, name)? {
        AspectValue::UInt64(value) => (*value)
            .try_into()
            .map_err(|_| invalid(key, name, "UInt64 value exceeds u32")),
        _ => Err(invalid(key, name, "expected UInt64")),
    }
}

fn invalid(key: EntityKey, name: &str, detail: &str) -> ObservationError {
    ObservationError::InvalidAspect {
        entity: key,
        name: name.to_owned(),
        detail: detail.to_owned(),
    }
}

fn debug_text(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<String, ObservationError> {
    string_value(key, state, name)
}

fn debug_region(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<super::schema::Region, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "NorthReach" => Ok(super::schema::Region::NorthReach),
        "SouthReach" => Ok(super::schema::Region::SouthReach),
        other => other
            .strip_prefix("Generated(")
            .and_then(|v| v.strip_suffix(')'))
            .and_then(|v| v.parse().ok())
            .map(super::schema::Region::Generated)
            .ok_or_else(|| invalid(key, name, "unknown region")),
    }
}

fn debug_posture(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<super::schema::OperatingPosture, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Open" => Ok(super::schema::OperatingPosture::Open),
        "Maintenance" => Ok(super::schema::OperatingPosture::Maintenance),
        "Retired" => Ok(super::schema::OperatingPosture::Retired),
        _ => Err(invalid(key, name, "unknown operating posture")),
    }
}

fn debug_class(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<super::schema::VesselClass, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Feeder" => Ok(super::schema::VesselClass::Feeder),
        "Panamax" => Ok(super::schema::VesselClass::Panamax),
        "HeavyLift" => Ok(super::schema::VesselClass::HeavyLift),
        _ => Err(invalid(key, name, "unknown vessel class")),
    }
}

fn debug_status(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<super::schema::VoyageStatus, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Planned" => Ok(super::schema::VoyageStatus::Planned),
        "Delayed" => Ok(super::schema::VoyageStatus::Delayed),
        "Rerouted" => Ok(super::schema::VoyageStatus::Rerouted),
        "Held" => Ok(super::schema::VoyageStatus::Held),
        _ => Err(invalid(key, name, "unknown voyage status")),
    }
}

fn debug_hazard(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<super::schema::HazardClass, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "General" => Ok(super::schema::HazardClass::General),
        "Medical" => Ok(super::schema::HazardClass::Medical),
        "Industrial" => Ok(super::schema::HazardClass::Industrial),
        "HazardousV2" => Ok(super::schema::HazardClass::HazardousV2),
        _ => Err(invalid(key, name, "unknown hazard class")),
    }
}

fn debug_booking(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<BookingStatus, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Available" => Ok(BookingStatus::Available),
        "Booked" => Ok(BookingStatus::Booked),
        "Held" => Ok(BookingStatus::Held),
        _ => Err(invalid(key, name, "unknown booking status")),
    }
}

fn debug_result(
    key: EntityKey,
    state: &AuthoritativeRecordAspectState,
    name: &str,
) -> Result<InspectionResult, ObservationError> {
    match debug_text(key, state, name)?.as_str() {
        "Pending" => Ok(InspectionResult::Pending),
        "Passed" => Ok(InspectionResult::Passed),
        "Flagged" => Ok(InspectionResult::Flagged),
        _ => Err(invalid(key, name, "unknown inspection result")),
    }
}
