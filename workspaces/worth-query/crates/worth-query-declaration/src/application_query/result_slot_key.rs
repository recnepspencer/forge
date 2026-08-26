use worth_foundational::facade::ScalarAspectType;

use super::{ApplicationQueryCardinality, ApplicationQueryResultTraversalDirection};
use crate::application_schema::ApplicationFieldPresence;
use crate::portable_identity::WorthQueryPortableTypeIdentity;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApplicationQueryResultSlotKey {
    query: WorthQueryPortableTypeIdentity,
    slot: WorthQueryPortableTypeIdentity,
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
        value_type: WorthQueryPortableTypeIdentity,
        presence: ApplicationFieldPresence,
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
    pub value_type: WorthQueryPortableTypeIdentity,
    pub presence: ApplicationFieldPresence,
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
    pub const fn query_identity(self) -> WorthQueryPortableTypeIdentity {
        self.query
    }

    pub const fn slot_identity(self) -> WorthQueryPortableTypeIdentity {
        self.slot
    }

    pub(super) const fn field(
        query: WorthQueryPortableTypeIdentity,
        slot: WorthQueryPortableTypeIdentity,
        contract: ApplicationQueryResultFieldSlotContract,
    ) -> Self {
        Self {
            query,
            slot,
            contract: ApplicationQueryResultSlotContract::Field {
                entity: contract.entity,
                aspect: contract.aspect,
                field: contract.field,
                output_name: contract.output_name,
                scalar_family: contract.scalar_family,
                value_type: contract.value_type,
                presence: contract.presence,
            },
        }
    }

    pub(super) const fn relation(
        query: WorthQueryPortableTypeIdentity,
        slot: WorthQueryPortableTypeIdentity,
        contract: ApplicationQueryResultRelationSlotContract,
    ) -> Self {
        Self {
            query,
            slot,
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
