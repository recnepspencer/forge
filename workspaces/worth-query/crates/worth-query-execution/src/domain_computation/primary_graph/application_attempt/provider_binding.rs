use std::collections::BTreeMap;
use std::sync::Arc;

use worth_query_installation::facade::{
    InstalledCorrectionMechanism, InstalledPreImageDemand, WorthQueryInstalledAftermathContract,
};
use worth_relational::facade::identity::{EntityId, RelationId};
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, DeleteEntityIntent, DeleteRelationIntent, EntityMutationIntent,
    EntityReference, EntitySpec, MutationIntent, RelationMutationIntent, RelationSpec,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};

use super::effect_program::WorthQueryApplicationRealizedEffect;
use super::fact::WorthQueryApplicationObservedFact;
use super::{
    WorthQueryAdmittedApplicationEmissionBatch, WorthQueryApplicationAttemptDenial,
    WorthQueryApplicationAttemptDenialKind, WorthQueryApplicationEmission,
};
use crate::domain_computation::{
    WorthQueryProvisionalEffectAction, WorthQueryProvisionalEffectStep,
};

pub(super) struct WorthQueryPreparedApplicationProviderAttempt {
    pub(super) facts: Vec<WorthQueryApplicationObservedFact>,
    pub(super) steps: Vec<WorthQueryProvisionalEffectStep>,
    pub(super) batch: WorkerIntentBatch,
    pub(super) emissions: WorthQueryAdmittedApplicationEmissionBatch,
    pub(super) preimage_demand: Option<worth_query_installation::facade::InstalledPreImageDemand>,
}

/// Derive the retention demand from the admitted operation's compiled aftermath
/// (Q8.26-C1).
///
/// The demand is a property of the installed contract — `InstalledRecordedInverse`
/// carries it non-optionally — so nothing about retention is a caller's to supply.
/// It was previously attached through a public `with_preimage_demand` builder each
/// operation had to remember to call, which made retention opt-in to the very party
/// it constrains: an operation could declare `RecordedInverse`, never attach the
/// demand, and commit with nothing retained and no diagnostic.
pub(super) fn installed_preimage_demand(
    aftermath: Option<&WorthQueryInstalledAftermathContract>,
) -> Option<InstalledPreImageDemand> {
    match aftermath?.mechanism()? {
        InstalledCorrectionMechanism::RecordedInverse(inverse) => {
            Some(inverse.preimage_demand().clone())
        }
        InstalledCorrectionMechanism::Compensation(_) => None,
    }
}

pub(super) fn prepare_provider_attempt(
    facts: Vec<WorthQueryApplicationObservedFact>,
    effects: Vec<WorthQueryApplicationRealizedEffect>,
    expected_emission_retained_bytes: u64,
    emission_retained_bytes_ceiling: u64,
    preimage_demand: Option<worth_query_installation::facade::InstalledPreImageDemand>,
) -> Result<WorthQueryPreparedApplicationProviderAttempt, WorthQueryApplicationAttemptDenial> {
    let symbols = created_entity_symbols(&effects);
    let mut steps = Vec::new();
    let mut intents = Vec::new();
    let mut emissions = Vec::new();
    for effect in effects {
        lower_effect(
            &facts,
            &symbols,
            effect,
            &mut steps,
            &mut intents,
            &mut emissions,
        )?;
    }
    let batch = intents.into_iter().fold(
        WorkerIntentBatch::new("application-provider-attempt"),
        WorkerIntentBatch::push,
    );
    let emissions = WorthQueryAdmittedApplicationEmissionBatch::admit(
        emissions,
        emission_retained_bytes_ceiling,
    )
    .map_err(|_| retained_bytes_denial())?;
    if emissions.retained_bytes() != expected_emission_retained_bytes {
        return Err(retained_bytes_denial());
    }
    Ok(WorthQueryPreparedApplicationProviderAttempt {
        facts,
        steps,
        batch,
        emissions,
        preimage_demand,
    })
}

fn lower_effect(
    facts: &[WorthQueryApplicationObservedFact],
    symbols: &BTreeMap<EntityReference, Arc<str>>,
    effect: WorthQueryApplicationRealizedEffect,
    steps: &mut Vec<WorthQueryProvisionalEffectStep>,
    intents: &mut Vec<MutationIntent>,
    emissions: &mut Vec<WorthQueryApplicationEmission>,
) -> Result<(), WorthQueryApplicationAttemptDenial> {
    match effect {
        WorthQueryApplicationRealizedEffect::CreateEntity { kind, key, fields } => {
            let symbol = created_entity_symbol(kind, &key);
            steps.push(effect_step(WorthQueryProvisionalEffectAction::Create {
                symbolic_identity: symbol,
            })?);
            intents.push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: worth_relational::facade::identity::PartitionId::main(),
                kind_id: kind,
                client_key: worth_relational::facade::symbols::ClientKey::raw(key),
                fields: AspectFieldPatch::from(fields),
            })));
        }
        WorthQueryApplicationRealizedEffect::UpdateEntity {
            entity_id, fields, ..
        } => {
            for locator in fields.keys() {
                let target = field_fact_identity(facts, entity_id, locator)?;
                steps.push(effect_step(WorthQueryProvisionalEffectAction::Replace {
                    target_identity: target.into(),
                })?);
            }
            intents.push(MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                UpdateEntityFieldsIntent {
                    entity_id,
                    fields: AspectFieldPatch::from(fields),
                },
            )));
        }
        WorthQueryApplicationRealizedEffect::DeleteEntity { entity_id } => {
            let target = entity_fact_identity(facts, entity_id)?;
            steps.push(effect_step(WorthQueryProvisionalEffectAction::Retire {
                target_identity: target.into(),
            })?);
            intents.push(MutationIntent::Entity(EntityMutationIntent::Delete(
                DeleteEntityIntent { entity_id },
            )));
        }
        WorthQueryApplicationRealizedEffect::CreateRelation {
            kind,
            key,
            from,
            to,
        } => {
            let dependencies = [&from, &to]
                .into_iter()
                .filter_map(|reference| symbols.get(reference).cloned());
            let step = effect_step(WorthQueryProvisionalEffectAction::Create {
                symbolic_identity: format!("application-create-relation:{}:{key}", kind.as_u32())
                    .into(),
            })?
            .with_symbolic_dependencies(dependencies)
            .map_err(|_| progression_denial())?;
            steps.push(step);
            intents.push(MutationIntent::Create(CreateIntent::Relation(
                RelationSpec {
                    partition_id: worth_relational::facade::identity::PartitionId::main(),
                    kind_id: kind,
                    client_key: worth_relational::facade::symbols::ClientKey::raw(key),
                    source: from,
                    target: to,
                    fields: AspectFieldPatch::default(),
                },
            )));
        }
        WorthQueryApplicationRealizedEffect::DeleteRelation { relation_id } => {
            let target = relation_fact_identity(facts, relation_id)?;
            steps.push(effect_step(WorthQueryProvisionalEffectAction::Retire {
                target_identity: target.into(),
            })?);
            intents.push(MutationIntent::Relation(RelationMutationIntent::Delete(
                DeleteRelationIntent { relation_id },
            )));
        }
        WorthQueryApplicationRealizedEffect::Emit(emission) => emissions.push(emission),
    }
    Ok(())
}

fn created_entity_symbols(
    effects: &[WorthQueryApplicationRealizedEffect],
) -> BTreeMap<EntityReference, Arc<str>> {
    effects
        .iter()
        .filter_map(|effect| {
            let WorthQueryApplicationRealizedEffect::CreateEntity { kind, key, .. } = effect else {
                return None;
            };
            let reference = EntityReference::Created(
                worth_relational::facade::transactions::CreatedEntityRef {
                    partition_id: worth_relational::facade::identity::PartitionId::main(),
                    kind_id: *kind,
                    client_key: worth_relational::facade::symbols::ClientKey::raw(key.clone()),
                },
            );
            Some((reference, created_entity_symbol(*kind, key)))
        })
        .collect()
}

fn created_entity_symbol(kind: worth_relational::facade::identity::KindId, key: &str) -> Arc<str> {
    Arc::from(format!("application-create-entity:{}:{key}", kind.as_u32()))
}

fn effect_step(
    action: WorthQueryProvisionalEffectAction,
) -> Result<WorthQueryProvisionalEffectStep, WorthQueryApplicationAttemptDenial> {
    WorthQueryProvisionalEffectStep::new("mutation", action).map_err(|_| progression_denial())
}

fn field_fact_identity(
    facts: &[WorthQueryApplicationObservedFact],
    entity_id: EntityId,
    locator: &worth_foundational::facade::AspectFieldLocator,
) -> Result<String, WorthQueryApplicationAttemptDenial> {
    facts
        .iter()
        .find(|fact| {
            matches!(
                fact,
                WorthQueryApplicationObservedFact::Field {
                    entity_id: observed,
                    locator: observed_locator,
                    ..
                } if *observed == entity_id && observed_locator == locator
            )
        })
        .map(WorthQueryApplicationObservedFact::locator_identity)
        .ok_or_else(progression_denial)
}

fn entity_fact_identity(
    facts: &[WorthQueryApplicationObservedFact],
    entity_id: EntityId,
) -> Result<String, WorthQueryApplicationAttemptDenial> {
    facts
        .iter()
        .find(|fact| {
            matches!(
                fact,
                WorthQueryApplicationObservedFact::Entity {
                    entity_id: observed,
                    ..
                } if *observed == entity_id
            )
        })
        .map(WorthQueryApplicationObservedFact::locator_identity)
        .ok_or_else(progression_denial)
}

fn relation_fact_identity(
    facts: &[WorthQueryApplicationObservedFact],
    relation_id: RelationId,
) -> Result<String, WorthQueryApplicationAttemptDenial> {
    facts
        .iter()
        .find(|fact| match fact {
            WorthQueryApplicationObservedFact::Relation {
                matching_relations, ..
            } => matching_relations.contains(&relation_id),
            WorthQueryApplicationObservedFact::Adjacency { relations, .. } => relations
                .iter()
                .any(|relation| relation.relation_id == relation_id),
            _ => false,
        })
        .map(WorthQueryApplicationObservedFact::locator_identity)
        .ok_or_else(progression_denial)
}

fn progression_denial() -> WorthQueryApplicationAttemptDenial {
    WorthQueryApplicationAttemptDenial::new(
        WorthQueryApplicationAttemptDenialKind::IncompleteEffectBasis,
        "provider progression",
    )
}

fn retained_bytes_denial() -> WorthQueryApplicationAttemptDenial {
    WorthQueryApplicationAttemptDenial::new(
        WorthQueryApplicationAttemptDenialKind::RetainedEffectBytesExceeded,
        "application emission batch",
    )
}
