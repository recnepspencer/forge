use worth_query_decl::facade::application_query::ApplicationQueryLiveCauseBinding;

use crate::{
    estate::{EmergencyAccessId, EstateCaseId},
    schema::{
        BankSchema, EmergencyAccess, EstateCase, EstateEmergencyAccessActivityEffect,
        EstateEmergencyAccessActivityEvent,
    },
};

use super::EstateEmergencyAccessActivityQuery;

pub struct EstateEmergencyAccessActivityLiveCause;

impl
    ApplicationQueryLiveCauseBinding<
        BankSchema,
        EstateEmergencyAccessActivityQuery,
        EstateCase,
        EmergencyAccess,
    > for EstateEmergencyAccessActivityLiveCause
{
    type Effect = EstateEmergencyAccessActivityEffect;
    type Payload = EstateEmergencyAccessActivityEvent;
    type ScopeIdentity = EstateCaseId;
    type TargetIdentity = EmergencyAccessId;

    fn effect() -> worth_query_decl::facade::application_schema::ApplicationEffectRef<
        BankSchema,
        Self::Effect,
        Self::Payload,
    > {
        EstateEmergencyAccessActivityEffect::reference()
    }

    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
        payload.estate
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.access
    }
}
