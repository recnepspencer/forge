use std::any::TypeId;

use worth_foundational::facade::ScalarAspectType;

use super::{ApplicationQueryCardinality, ApplicationQueryResultTraversalDirection};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApplicationQueryResultSlotKey {
    query: TypeId,
    slot: TypeId,
    contract: ApplicationQueryResultSlotContract,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum ApplicationQueryResultSlotContract {
    Field {
        entity: &'static str,
        aspect: &'static str,
        field: &'static str,
        output_name: &'static str,
        scalar_family: ScalarAspectType,
        value_type: &'static str,
    },
    Relation {
        relation: &'static str,
        from: &'static str,
        to: &'static str,
        direction: ApplicationQueryResultTraversalDirection,
        output_name: &'static str,
        cardinality: ApplicationQueryCardinality,
    },
}

pub(super) struct ApplicationQueryResultFieldSlotContract {
    pub entity: &'static str,
    pub aspect: &'static str,
    pub field: &'static str,
    pub output_name: &'static str,
    pub scalar_family: ScalarAspectType,
    pub value_type: &'static str,
}

pub(super) struct ApplicationQueryResultRelationSlotContract {
    pub relation: &'static str,
    pub from: &'static str,
    pub to: &'static str,
    pub direction: ApplicationQueryResultTraversalDirection,
    pub output_name: &'static str,
    pub cardinality: ApplicationQueryCardinality,
}

impl ApplicationQueryResultSlotKey {
    pub(super) fn field<Query: 'static, Slot: 'static>(
        contract: ApplicationQueryResultFieldSlotContract,
    ) -> Self {
        Self {
            query: TypeId::of::<Query>(),
            slot: TypeId::of::<Slot>(),
            contract: ApplicationQueryResultSlotContract::Field {
                entity: contract.entity,
                aspect: contract.aspect,
                field: contract.field,
                output_name: contract.output_name,
                scalar_family: contract.scalar_family,
                value_type: contract.value_type,
            },
        }
    }

    pub(super) fn relation<Query: 'static, Slot: 'static>(
        contract: ApplicationQueryResultRelationSlotContract,
    ) -> Self {
        Self {
            query: TypeId::of::<Query>(),
            slot: TypeId::of::<Slot>(),
            contract: ApplicationQueryResultSlotContract::Relation {
                relation: contract.relation,
                from: contract.from,
                to: contract.to,
                direction: contract.direction,
                output_name: contract.output_name,
                cardinality: contract.cardinality,
            },
        }
    }
}
