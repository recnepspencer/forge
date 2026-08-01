use std::any::{Any, TypeId};
use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::sync::Arc;

use worth_foundational::facade::{AspectFieldLocator, AspectValue};
use worth_query_declaration::facade::application_schema::ApplicationEffectPayload;
use worth_relational::facade::identity::{EntityId, KindId, RelationId};
use worth_relational::facade::transactions::EntityReference;

use super::super::read_set::WorthQueryCompleteApplicationReadSet;
use super::super::WorthQueryProjectedApplicationMutation;

#[derive(Clone)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryApplicationEmission {
    effect: &'static str,
    payload_type: &'static str,
    payload_type_id: TypeId,
    payload: Arc<dyn Any + Send + Sync>,
    retained_bytes: u64,
    measure_retained_bytes: fn(&(dyn Any + Send + Sync)) -> Option<u64>,
}

impl WorthQueryApplicationEmission {
    pub(super) fn new<Payload>(effect: &'static str, payload: Payload) -> Self
    where
        Payload: ApplicationEffectPayload,
    {
        let retained_bytes = payload.retained_bytes();
        Self {
            effect,
            payload_type: std::any::type_name::<Payload>(),
            payload_type_id: TypeId::of::<Payload>(),
            payload: Arc::new(payload),
            retained_bytes,
            measure_retained_bytes: measure_retained_bytes::<Payload>,
        }
    }

    pub(in crate::domain_computation::primary_graph) fn is_well_formed(&self) -> bool {
        !self.effect.is_empty()
            && !self.payload_type.is_empty()
            && self.payload_type_id == self.payload.as_ref().type_id()
            && (self.measure_retained_bytes)(self.payload.as_ref()) == Some(self.retained_bytes)
    }

    pub(in crate::domain_computation::primary_graph) const fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }

    pub(in crate::domain_computation::primary_graph) fn cloned_payload<Schema, Effect, Payload>(
        &self,
        effect: &worth_query_declaration::facade::application_schema::ApplicationEffectRef<
            Schema,
            Effect,
            Payload,
        >,
    ) -> Option<Payload>
    where
        Payload: ApplicationEffectPayload + Clone,
    {
        (self.effect == effect.name())
            .then(|| self.payload.downcast_ref::<Payload>().cloned())
            .flatten()
    }

    #[cfg(test)]
    pub(crate) const fn effect(&self) -> &'static str {
        self.effect
    }

    #[cfg(test)]
    pub(crate) fn payload<Payload: 'static>(&self) -> Option<&Payload> {
        self.payload.downcast_ref()
    }
}

fn measure_retained_bytes<Payload>(payload: &(dyn Any + Send + Sync)) -> Option<u64>
where
    Payload: ApplicationEffectPayload,
{
    payload
        .downcast_ref::<Payload>()
        .map(ApplicationEffectPayload::retained_bytes)
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryAdmittedApplicationEmissionBatch {
    emissions: Vec<WorthQueryApplicationEmission>,
    retained_bytes: u64,
}

impl WorthQueryAdmittedApplicationEmissionBatch {
    pub(in crate::domain_computation::primary_graph) fn admit(
        emissions: Vec<WorthQueryApplicationEmission>,
        retained_bytes_ceiling: u64,
    ) -> Result<Self, &'static str> {
        let retained_bytes = emissions.iter().try_fold(0_u64, |total, emission| {
            if !emission.is_well_formed() {
                return Err("application commit carried an invalid typed emission");
            }
            total
                .checked_add(emission.retained_bytes())
                .ok_or("application emission retained-byte count overflowed")
        })?;
        if retained_bytes > retained_bytes_ceiling {
            return Err("application emission exceeded installed retained-byte ceiling");
        }
        Ok(Self {
            emissions,
            retained_bytes,
        })
    }

    pub(in crate::domain_computation::primary_graph) fn into_parts(
        self,
    ) -> (Vec<WorthQueryApplicationEmission>, u64) {
        (self.emissions, self.retained_bytes)
    }

    pub(in crate::domain_computation::primary_graph) fn len(&self) -> usize {
        self.emissions.len()
    }

    pub(in crate::domain_computation::primary_graph) const fn retained_bytes(&self) -> u64 {
        self.retained_bytes
    }
}

pub struct WorthQueryApplicationEffectEntity<Schema, Entity> {
    pub(super) reference: EntityReference,
    pub(super) entity: String,
    pub(super) created_effect: Option<usize>,
    pub(super) program: Arc<()>,
    pub(super) _marker: PhantomData<fn() -> (Schema, Entity)>,
}

pub(in crate::domain_computation::primary_graph::application_attempt) enum WorthQueryApplicationRealizedEffect
{
    CreateEntity {
        kind: KindId,
        key: String,
        fields: BTreeMap<AspectFieldLocator, AspectValue>,
    },
    UpdateEntity {
        entity: String,
        entity_id: EntityId,
        fields: BTreeMap<AspectFieldLocator, AspectValue>,
    },
    DeleteEntity {
        entity_id: EntityId,
    },
    CreateRelation {
        kind: KindId,
        key: String,
        from: EntityReference,
        to: EntityReference,
    },
    DeleteRelation {
        relation_id: RelationId,
    },
    Emit(WorthQueryApplicationEmission),
}

pub struct WorthQueryApplicationEffectProgram<Schema, Operation, Input, Scope> {
    pub(in crate::domain_computation::primary_graph::application_attempt) operation:
        super::super::WorthQueryProgressingApplicationOperation<Schema, Operation, Input, Scope>,
    pub(in crate::domain_computation::primary_graph::application_attempt) facts:
        Vec<super::super::fact::WorthQueryApplicationObservedFact>,
    pub(in crate::domain_computation::primary_graph::application_attempt) effects:
        Vec<WorthQueryApplicationRealizedEffect>,
    pub(in crate::domain_computation::primary_graph::application_attempt) emission_retained_bytes:
        u64,
    pub(in crate::domain_computation::primary_graph::application_attempt) emission_retained_bytes_ceiling:
        u64,
}

pub struct WorthQueryApplicationEffectProgramBuilder<Schema, Operation, Input, Scope> {
    pub(super) read_set: WorthQueryCompleteApplicationReadSet<
        Schema,
        Operation,
        Input,
        Scope,
        WorthQueryProjectedApplicationMutation,
    >,
    pub(super) layout: Arc<super::super::super::schema_layout::WorthQueryPrimaryGraphLayout>,
    pub(super) program: Arc<()>,
    pub(super) effects: Vec<WorthQueryApplicationRealizedEffect>,
    pub(super) keys: BTreeSet<(KindId, String)>,
    pub(super) emission_retained_bytes: u64,
    pub(super) emission_retained_bytes_ceiling: u64,
}
