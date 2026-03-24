use crate::logic::runtime::state::subsystems::RuntimeSubsystem;
use crate::schema::data::{
    AspectPlanCatalog, LoweredPayloadSchemaContract, PayloadContractRecordKind,
    PayloadFieldConstraint, RelationIntegrityPlanCatalog,
};
use crate::schema::logic::{lower_aspect_plans, lower_relation_integrity_plans};
use crate::validation::data::{payload_schema_registration, InvariantRegistration};
use crate::validation::logic::FrozenCustomInvariantRegistry;

#[derive(Debug, Clone, Default)]
pub(crate) struct AspectSemanticsSubsystem {
    pub(crate) plans: AspectPlanCatalog,
    pub(crate) relation_integrity_plans: RelationIntegrityPlanCatalog,
    pub(crate) relation_integrity_registrations: Vec<InvariantRegistration>,
    pub(crate) custom_invariant_registries: FrozenCustomInvariantRegistry,
}

impl RuntimeSubsystem for AspectSemanticsSubsystem {
    type Config = crate::config::data::RelationalRuntimeConfig;

    fn new(config: &Self::Config) -> Self {
        let relation_integrity_plans = lower_relation_integrity_plans(&config.schema.registry);
        let payload_schema_registrations = config
            .schema
            .registry
            .entity_kinds
            .iter()
            .filter_map(|(kind_id, registration)| {
                registration.aspect_declarations.payload_schema.as_ref().map(|payload_schema| {
                    payload_schema_registration(LoweredPayloadSchemaContract {
                        contract_id: payload_schema.contract_id.clone(),
                        record_kind: PayloadContractRecordKind::Entity,
                        kind_id: *kind_id,
                        allowed_payload_class: payload_schema.allowed_payload_class,
                        field_constraints: payload_schema
                            .field_constraints
                            .iter()
                            .cloned()
                            .map(|constraint| match constraint {
                                crate::schema::data::PayloadFieldConstraintDeclaration::Required {
                                    field,
                                } => PayloadFieldConstraint::Required { field },
                                crate::schema::data::PayloadFieldConstraintDeclaration::Type {
                                    field,
                                    expected,
                                } => PayloadFieldConstraint::Type { field, expected },
                            })
                            .collect(),
                    })
                })
            })
            .chain(config.schema.registry.relation_kinds.iter().filter_map(
                |(kind_id, registration)| {
                    registration.aspect_declarations.payload_schema.as_ref().map(
                        |payload_schema| {
                            payload_schema_registration(LoweredPayloadSchemaContract {
                                contract_id: payload_schema.contract_id.clone(),
                                record_kind: PayloadContractRecordKind::Relation,
                                kind_id: *kind_id,
                                allowed_payload_class: payload_schema.allowed_payload_class,
                                field_constraints: payload_schema
                                    .field_constraints
                                    .iter()
                                    .cloned()
                                    .map(|constraint| match constraint {
                                        crate::schema::data::PayloadFieldConstraintDeclaration::Required {
                                            field,
                                        } => PayloadFieldConstraint::Required { field },
                                        crate::schema::data::PayloadFieldConstraintDeclaration::Type {
                                            field,
                                            expected,
                                        } => PayloadFieldConstraint::Type { field, expected },
                                    })
                                    .collect(),
                            })
                        },
                    )
                },
            ));
        Self {
            plans: lower_aspect_plans(&config.schema.registry),
            relation_integrity_registrations: relation_integrity_plans
                .relation_plans
                .values()
                .flat_map(crate::validation::data::relation_integrity_registrations_for_plan)
                .chain(payload_schema_registrations)
                .collect(),
            custom_invariant_registries: FrozenCustomInvariantRegistry::default(),
            relation_integrity_plans,
        }
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
