use std::collections::{BTreeMap, BTreeSet};

use super::handles::{EntityHandle, RelationHandle};
use super::{
    observe_supply_chain_observation, BranchLabel, DeltaId, DeltaPrecondition, EntityKey,
    EntityRecord, ObservationError, ObservedSupplyChainState, RelationKey, SchemaVersion,
    SupplyChainSemanticHandles,
};
use worth_foundational::facade::{AspectKey, AspectValue, FieldKey, InternedString};
use worth_relational::facade::branch::RelationalBranchIdentity;
use worth_relational::facade::history::{BranchId, RelationalCommitReceipt};
use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::snapshots::SnapshotHandle;
use worth_relational::facade::transactions::{
    planned_single_field_locator, AspectFieldPatch, EntityMutationIntent, EntityReference,
    MutationIntent, RelationMutationIntent, UpdateEntityFieldsIntent,
    UpdateRelationEndpointsIntent, WorkerIntentBatch,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Phase5ProductionDeltaLoweringError {
    UnknownBranch(BranchId),
    BranchBasis(worth_relational::facade::branch::RelationalBranchBasisDenial),
    Observation(ObservationError),
    MissingEntity(EntityKey),
    MissingRelation(RelationKey),
    WrongBranch {
        expected: BranchLabel,
        observed: BranchLabel,
    },
    WrongSchema {
        expected: SchemaVersion,
        observed: SchemaVersion,
    },
    DuplicateDelta(DeltaId),
    UnsupportedDelta(DeltaId),
    UnexpectedEntity(EntityKey),
    ArithmeticOverflow(EntityKey),
}

/// Lowers semantic intent from the selected branch's actual public owner
/// observation. Expected oracle state never enters this path.
pub(crate) fn lower_phase5_production_delta(
    runtime: &mut RelationalRuntime,
    program: &super::CompiledSupplyChainProgram,
    handles: &SupplyChainSemanticHandles,
    branch_id: &BranchId,
    previously_applied: &BTreeSet<DeltaId>,
    delta: DeltaId,
) -> Result<WorkerIntentBatch, Phase5ProductionDeltaLoweringError> {
    let identity = runtime
        .branch_identity(branch_id)
        .map_err(|_| Phase5ProductionDeltaLoweringError::UnknownBranch(branch_id.clone()))?;
    let (_, basis) = runtime
        .observe_branch(&identity)
        .map_err(Phase5ProductionDeltaLoweringError::BranchBasis)?;
    let observation = basis.observation();
    let branch_handles = handles.for_observation(&observation);
    let observed =
        observe_supply_chain_observation(program, &branch_handles, runtime, &observation)
            .map_err(Phase5ProductionDeltaLoweringError::Observation)?;
    validate_preconditions(&observed, previously_applied, delta)?;

    match delta {
        DeltaId::StormRerouteAurora => lower_storm(handles, &observed),
        DeltaId::MaintainAtlasBerth => lower_maintenance(handles, &observed),
        DeltaId::HoldMedicalCargo => lower_medical_hold(handles),
        DeltaId::RewireAuroraPortCall => lower_rewire(handles, &observed),
        unsupported => Err(Phase5ProductionDeltaLoweringError::UnsupportedDelta(
            unsupported,
        )),
    }
}

fn validate_preconditions(
    observed: &ObservedSupplyChainState,
    previously_applied: &BTreeSet<DeltaId>,
    delta: DeltaId,
) -> Result<(), Phase5ProductionDeltaLoweringError> {
    for precondition in delta.contract().preconditions {
        match precondition {
            DeltaPrecondition::EntityPresent(key) if !observed.entities.contains_key(&key) => {
                return Err(Phase5ProductionDeltaLoweringError::MissingEntity(key));
            }
            DeltaPrecondition::RelationPresent(key) if !observed.relations.contains_key(&key) => {
                return Err(Phase5ProductionDeltaLoweringError::MissingRelation(key));
            }
            DeltaPrecondition::Schema(expected) if observed.schema != expected => {
                return Err(Phase5ProductionDeltaLoweringError::WrongSchema {
                    expected,
                    observed: observed.schema,
                });
            }
            DeltaPrecondition::Branch(expected) if observed.branch != expected => {
                return Err(Phase5ProductionDeltaLoweringError::WrongBranch {
                    expected,
                    observed: observed.branch,
                });
            }
            DeltaPrecondition::DeltaNotAccepted(expected)
                if previously_applied.contains(&expected) =>
            {
                return Err(Phase5ProductionDeltaLoweringError::DuplicateDelta(expected));
            }
            _ => {}
        }
    }
    Ok(())
}

fn lower_storm(
    handles: &SupplyChainSemanticHandles,
    observed: &ObservedSupplyChainState,
) -> Result<WorkerIntentBatch, Phase5ProductionDeltaLoweringError> {
    let voyage_handle = handles.aurora_voyage();
    let voyage_key = voyage_handle.semantic;
    let EntityRecord::Voyage(voyage) = observed_entity(observed, voyage_key)? else {
        return Err(Phase5ProductionDeltaLoweringError::UnexpectedEntity(
            voyage_key,
        ));
    };
    let arrival = voyage.arrival.0.checked_add(30).ok_or(
        Phase5ProductionDeltaLoweringError::ArithmeticOverflow(voyage_key),
    )?;
    let revision = voyage.revision.checked_add(1).ok_or(
        Phase5ProductionDeltaLoweringError::ArithmeticOverflow(voyage_key),
    )?;
    let relation_key = handles.aurora_call_at_port().semantic;
    let source = observed_relation_source(observed, relation_key)?;
    Ok(WorkerIntentBatch::new("phase5-storm-reroute-aurora")
        .push(update_fields(
            voyage_handle,
            [
                (SupplyChainField::Status, text("Rerouted")),
                (SupplyChainField::Arrival, number(arrival)),
                (SupplyChainField::Revision, number(revision)),
            ],
        ))
        .push(update_relation(
            handles,
            handles.aurora_call_at_port(),
            source,
            handles.reroute_port(),
        )))
}

fn lower_maintenance(
    handles: &SupplyChainSemanticHandles,
    observed: &ObservedSupplyChainState,
) -> Result<WorkerIntentBatch, Phase5ProductionDeltaLoweringError> {
    let voyage_handle = handles.aurora_voyage();
    let voyage_key = voyage_handle.semantic;
    let EntityRecord::Voyage(voyage) = observed_entity(observed, voyage_key)? else {
        return Err(Phase5ProductionDeltaLoweringError::UnexpectedEntity(
            voyage_key,
        ));
    };
    let arrival = voyage.arrival.0.checked_add(60).ok_or(
        Phase5ProductionDeltaLoweringError::ArithmeticOverflow(voyage_key),
    )?;
    let revision = voyage.revision.checked_add(1).ok_or(
        Phase5ProductionDeltaLoweringError::ArithmeticOverflow(voyage_key),
    )?;
    let atlas = handles.atlas_berth();
    let assignment = handles.atlas_berth_assignment().semantic;
    let source = observed_relation_source(observed, assignment)?;
    Ok(WorkerIntentBatch::new("phase5-maintain-atlas-berth")
        .push(update_fields(
            atlas,
            [(SupplyChainField::Posture, text("Maintenance"))],
        ))
        .push(update_fields(
            voyage_handle,
            [
                (SupplyChainField::Status, text("Delayed")),
                (SupplyChainField::Arrival, number(arrival)),
                (SupplyChainField::Revision, number(revision)),
            ],
        ))
        .push(update_relation(
            handles,
            handles.atlas_berth_assignment(),
            source,
            handles.maintenance_berth(),
        )))
}

fn lower_medical_hold(
    handles: &SupplyChainSemanticHandles,
) -> Result<WorkerIntentBatch, Phase5ProductionDeltaLoweringError> {
    Ok(
        WorkerIntentBatch::new("phase5-hold-medical-cargo").push(update_fields(
            handles.medical_cargo(),
            [(SupplyChainField::Booking, text("Held"))],
        )),
    )
}

fn lower_rewire(
    handles: &SupplyChainSemanticHandles,
    observed: &ObservedSupplyChainState,
) -> Result<WorkerIntentBatch, Phase5ProductionDeltaLoweringError> {
    let call_handle = handles.aurora_port_call();
    let call_key = call_handle.semantic;
    let EntityRecord::PortCall(call) = observed_entity(observed, call_key)? else {
        return Err(Phase5ProductionDeltaLoweringError::UnexpectedEntity(
            call_key,
        ));
    };
    let revision = call.revision.checked_add(1).ok_or(
        Phase5ProductionDeltaLoweringError::ArithmeticOverflow(call_key),
    )?;
    let relation_key = handles.aurora_call_at_port().semantic;
    let source = observed_relation_source(observed, relation_key)?;
    Ok(WorkerIntentBatch::new("phase5-rewire-aurora-port-call")
        .push(update_fields(
            call_handle,
            [(SupplyChainField::Revision, number(revision))],
        ))
        .push(update_relation(
            handles,
            handles.aurora_call_at_port(),
            source,
            handles.rewire_port(),
        )))
}

fn observed_entity(
    observed: &ObservedSupplyChainState,
    key: EntityKey,
) -> Result<&EntityRecord, Phase5ProductionDeltaLoweringError> {
    observed
        .entities
        .get(&key)
        .ok_or(Phase5ProductionDeltaLoweringError::MissingEntity(key))
}

fn observed_relation_source(
    observed: &ObservedSupplyChainState,
    key: RelationKey,
) -> Result<EntityKey, Phase5ProductionDeltaLoweringError> {
    observed
        .relations
        .get(&key)
        .map(|edge| edge.source)
        .ok_or(Phase5ProductionDeltaLoweringError::MissingRelation(key))
}

fn update_fields<const N: usize>(
    entity: &EntityHandle,
    fields: [(SupplyChainField, AspectValue); N],
) -> MutationIntent {
    let fields = fields
        .into_iter()
        .map(|(field, value)| {
            let name = field.canonical_name();
            (
                planned_single_field_locator(
                    AspectKey::new(name).expect("canonical Supply Chain aspect"),
                    FieldKey::new(name).expect("canonical Supply Chain field"),
                ),
                value,
            )
        })
        .collect::<BTreeMap<_, _>>();
    MutationIntent::Entity(EntityMutationIntent::UpdateFields(
        UpdateEntityFieldsIntent {
            entity_id: entity.id,
            fields: AspectFieldPatch::new(fields),
        },
    ))
}

fn update_relation(
    handles: &SupplyChainSemanticHandles,
    relation: &RelationHandle,
    source: EntityKey,
    target: &EntityHandle,
) -> MutationIntent {
    MutationIntent::Relation(RelationMutationIntent::UpdateEndpoints(
        UpdateRelationEndpointsIntent {
            relation_id: relation.id,
            kind_id: super::relation_kind_id(relation.semantic.kind),
            source: EntityReference::Existing(handles.entities[&source].id),
            target: EntityReference::Existing(target.id),
        },
    ))
}

fn text(value: &str) -> AspectValue {
    AspectValue::String(InternedString::Raw(value.to_owned()))
}

fn number(value: impl Into<u64>) -> AspectValue {
    AspectValue::UInt64(value.into())
}

#[derive(Clone, Copy)]
enum SupplyChainField {
    Arrival,
    Booking,
    Posture,
    Revision,
    Status,
}

impl SupplyChainField {
    const fn canonical_name(self) -> &'static str {
        match self {
            Self::Arrival => "arrival",
            Self::Booking => "booking",
            Self::Posture => "posture",
            Self::Revision => "revision",
            Self::Status => "status",
        }
    }
}

pub(crate) fn commit_main_batch(runtime: &mut RelationalRuntime, batch: WorkerIntentBatch) {
    commit_branch_batch(runtime, BranchId("main".to_owned()), batch);
}

pub(crate) fn fork_supply_chain_branch_from_main(
    runtime: &mut RelationalRuntime,
    branch_id: BranchId,
) {
    let (_, source) = runtime
        .observe_fork_source(&BranchId("main".to_owned()))
        .expect("main remains an admitted fork source");
    runtime
        .fork_branch(branch_id, source)
        .expect("the Supply Chain branch forks from the admitted main basis");
}

pub(crate) fn snapshot_for_supply_chain_identity(
    runtime: &mut RelationalRuntime,
    identity: &RelationalBranchIdentity,
) -> SnapshotHandle {
    let (_, basis) = runtime
        .observe_branch(identity)
        .expect("branch basis is owner-admitted");
    runtime
        .snapshots()
        .snapshot_for_observation(&basis.observation())
        .expect("admitted branch observation opens its exact snapshot")
}

pub(crate) fn head_for_supply_chain_identity(
    runtime: &RelationalRuntime,
    identity: &RelationalBranchIdentity,
) -> RelationalCommitReceipt {
    let (_, basis) = runtime
        .observe_branch(identity)
        .expect("branch basis is owner-admitted");
    runtime
        .history()
        .branch_head_for_observation(&basis.observation())
        .expect("observation belongs to this runtime")
        .cloned()
        .expect("observed Supply Chain branch has a canonical head")
}

pub(crate) fn head_for_supply_chain_branch(
    runtime: &RelationalRuntime,
    branch_id: &BranchId,
) -> RelationalCommitReceipt {
    let identity = runtime
        .branch_identity(branch_id)
        .expect("branch identity is owner-issued");
    head_for_supply_chain_identity(runtime, &identity)
}

pub(crate) fn commit_branch_batch(
    runtime: &mut RelationalRuntime,
    branch_id: BranchId,
    batch: WorkerIntentBatch,
) {
    let identity = runtime
        .branch_identity(&branch_id)
        .expect("branch identity is owner-issued");
    let options = runtime
        .admit_branch_basis(&identity)
        .expect("transaction authority is owner-issued");
    let mut transaction = runtime
        .begin_branch_transaction(
            &options,
            worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
        )
        .expect("owner-admitted transaction context");
    transaction.push_batch(batch);
    transaction
        .commit(runtime)
        .expect("Supply Chain mutation commits through production publication");
}
