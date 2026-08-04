use worth_foundational::facade::{AspectValue, CanonicalDigestId};
use worth_query_declaration::facade::application_query::{
    ApplicationQueryObservableInfluence, ApplicationQueryResultSlotKey,
};
use worth_query_installation::facade::WorthQueryInstalledApplicationQueryIdentity;
use worth_relational::facade::identity::EntityId;

use super::super::WorthQueryApplicationDisclosureReceipt;
use super::{
    WorthQueryApplicationDisclosedFieldAdmission, WorthQueryApplicationInternalFieldAdmission,
    WorthQueryApplicationQueryGovernance, WorthQueryPendingApplicationQueryGovernance,
};
use crate::domain_computation::authorization::WorthQueryRetainedCapabilityAuthorization;
use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;
use crate::domain_computation::provider_session::{
    WorthQueryGraphWorkSessionIdentity, WorthQueryManagedGraphWorkSession,
};
impl WorthQueryApplicationQueryGovernance {
    pub(in crate::domain_computation) fn into_pending(
        self,
    ) -> Option<WorthQueryPendingApplicationQueryGovernance> {
        match self {
            Self::Public => None,
            Self::Governed {
                capability_name,
                capability_type,
                disclosure_value,
                authorization,
                ..
            } => Some(WorthQueryPendingApplicationQueryGovernance {
                capability_name,
                capability_type,
                disclosure_value,
                authorization,
            }),
        }
    }

    pub(in crate::domain_computation) fn is_disclosed(
        &self,
        slot: &ApplicationQueryResultSlotKey,
    ) -> bool {
        match self {
            Self::Public => true,
            Self::Governed { disclosure, .. } => disclosure.disclosed.contains_key(slot),
        }
    }

    pub(in crate::domain_computation) fn omission(
        &self,
        slot: &ApplicationQueryResultSlotKey,
    ) -> Option<(&str, &AspectValue)> {
        match self {
            Self::Public => None,
            Self::Governed { disclosure, .. } => disclosure
                .omitted
                .get(slot)
                .map(|rule| (disclosure.classification.as_str(), rule.disclosure_value())),
        }
    }

    pub(in crate::domain_computation) fn admit_disclosed_field(
        &self,
        slot: &ApplicationQueryResultSlotKey,
        field: (&str, &str, &str),
    ) -> Option<WorthQueryApplicationDisclosedFieldAdmission<'_>> {
        match self {
            Self::Public => Some(WorthQueryApplicationDisclosedFieldAdmission { field: None }),
            Self::Governed { disclosure, .. } => {
                let admitted = disclosure.disclosed.get(slot)?.field()?;
                admitted
                    .matches(field)
                    .then_some(WorthQueryApplicationDisclosedFieldAdmission {
                        field: Some(admitted),
                    })
            }
        }
    }

    pub(in crate::domain_computation) fn admit_internal_field(
        &self,
        field: (&str, &str, &str),
        surface: ApplicationQueryObservableInfluence,
    ) -> Option<WorthQueryApplicationInternalFieldAdmission<'_>> {
        match self {
            Self::Public => Some(WorthQueryApplicationInternalFieldAdmission { field: None }),
            Self::Governed { computation, .. } => {
                let key = (
                    field.0.to_string(),
                    field.1.to_string(),
                    field.2.to_string(),
                );
                let rules = computation.internal_field_rules.get(&key)?;
                let admitted = rules.iter().all(|rule| rule.influence().permits(surface));
                let field = rules
                    .first()?
                    .field()
                    .filter(|field| field.matches(field_tuple(&key)))?;
                admitted
                    .then_some(WorthQueryApplicationInternalFieldAdmission { field: Some(field) })
            }
        }
    }

    pub(in crate::domain_computation) fn authorization_mut(
        &mut self,
    ) -> Option<&mut WorthQueryRetainedCapabilityAuthorization> {
        match self {
            Self::Public => None,
            Self::Governed { authorization, .. } => Some(authorization),
        }
    }

    pub(in crate::domain_computation) fn authorization(
        &self,
    ) -> Option<&WorthQueryRetainedCapabilityAuthorization> {
        match self {
            Self::Public => None,
            Self::Governed { authorization, .. } => Some(authorization),
        }
    }

    pub(in crate::domain_computation) fn authorization_belongs_to_session(
        &self,
        session: WorthQueryGraphWorkSessionIdentity,
    ) -> bool {
        self.authorization()
            .is_none_or(|authorization| authorization.belongs_to_session(session))
    }

    pub(in crate::domain_computation) fn receipt(&self) -> WorthQueryApplicationDisclosureReceipt {
        match self {
            Self::Public => WorthQueryApplicationDisclosureReceipt::public(),
            Self::Governed {
                disclosure,
                authorization,
                ..
            } => WorthQueryApplicationDisclosureReceipt::governed(
                disclosure.classification.clone(),
                disclosure
                    .disclosed
                    .iter()
                    .map(|(slot, rule)| (*slot, rule.disclosure_value().clone()))
                    .collect(),
                disclosure
                    .omitted
                    .iter()
                    .map(|(slot, rule)| (*slot, rule.disclosure_value().clone()))
                    .collect(),
                authorization.capability_authority_identity(),
                authorization.decision_identity(),
                authorization.exact_fact_count(),
            ),
        }
    }

    pub(in crate::domain_computation) fn computation_matches(
        &self,
        graph_work: &WorthQueryManagedGraphWorkSession,
        runtime: WorthQueryRuntimeAuthorityIdentity,
        query: &WorthQueryInstalledApplicationQueryIdentity,
        parameters: &CanonicalDigestId,
        principal: EntityId,
        scope: EntityId,
    ) -> bool {
        match self {
            Self::Public => true,
            Self::Governed { computation, .. } => {
                computation
                    .binding
                    .matches(graph_work, runtime, query, parameters, principal, scope)
                    && !matches!(computation.disclosure_value, AspectValue::Null)
            }
        }
    }

    pub(in crate::domain_computation) fn readmission_matches(
        &self,
        runtime: WorthQueryRuntimeAuthorityIdentity,
        query: &WorthQueryInstalledApplicationQueryIdentity,
        parameters: &CanonicalDigestId,
        principal: EntityId,
        scope: EntityId,
    ) -> bool {
        match self {
            Self::Public => true,
            Self::Governed { computation, .. } => {
                computation.binding.runtime == runtime
                    && &computation.binding.query == query
                    && &computation.binding.parameters == parameters
                    && computation.binding.principal == principal
                    && computation.binding.scope == scope
            }
        }
    }
}

fn field_tuple(field: &(String, String, String)) -> (&str, &str, &str) {
    (&field.0, &field.1, &field.2)
}
