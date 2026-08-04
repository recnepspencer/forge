use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalDigestId};
use worth_query_declaration::facade::application_query::ApplicationQueryResultSlotKey;
use worth_query_installation::facade::WorthQueryInstalledApplicationQueryIdentity;
use worth_relational::facade::identity::EntityId;

use super::installed_contract::WorthQueryAdmittedApplicationDisclosureContract;
use super::receipt::WorthQueryApplicationDisclosureReceipt;
use crate::domain_computation::authorization::WorthQueryRetainedCapabilityAuthorization;
use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;

pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryPendingApplicationQueryGovernance
{
    pub(super) capability_name: String,
    pub(super) capability_type: String,
    pub(super) disclosure_value: AspectValue,
    pub(super) authorization: WorthQueryRetainedCapabilityAuthorization,
}

impl WorthQueryPendingApplicationQueryGovernance {
    pub(in crate::domain_computation::primary_graph::application_query) fn new(
        capability_name: impl Into<String>,
        capability_type: impl Into<String>,
        disclosure_value: AspectValue,
        authorization: WorthQueryRetainedCapabilityAuthorization,
    ) -> Self {
        Self {
            capability_name: capability_name.into(),
            capability_type: capability_type.into(),
            disclosure_value,
            authorization,
        }
    }
}

pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryApplicationInternalComputationAuthority
{
    runtime: WorthQueryRuntimeAuthorityIdentity,
    query: WorthQueryInstalledApplicationQueryIdentity,
    parameters: CanonicalDigestId,
    principal: EntityId,
    scope: EntityId,
    disclosure_value: AspectValue,
}

#[derive(Clone, Debug)]
pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryApplicationDisclosureDecision
{
    classification: String,
    disclosed: BTreeMap<ApplicationQueryResultSlotKey, AspectValue>,
    omitted: BTreeMap<ApplicationQueryResultSlotKey, AspectValue>,
}

pub(in crate::domain_computation::primary_graph::application_query) enum WorthQueryApplicationQueryGovernance
{
    Public,
    Governed {
        capability_name: String,
        capability_type: String,
        disclosure_value: AspectValue,
        computation: WorthQueryApplicationInternalComputationAuthority,
        disclosure: WorthQueryApplicationDisclosureDecision,
        authorization: WorthQueryRetainedCapabilityAuthorization,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph::application_query) enum WorthQueryApplicationQueryGovernanceDenialKind
{
    Required,
    CapabilityMismatch,
    InternalComputationDenied,
}

pub(in crate::domain_computation::primary_graph::application_query) fn admit_application_query_governance(
    contract: WorthQueryAdmittedApplicationDisclosureContract,
    pending: Option<WorthQueryPendingApplicationQueryGovernance>,
    binding: WorthQueryApplicationGovernanceBinding,
) -> Result<WorthQueryApplicationQueryGovernance, WorthQueryApplicationQueryGovernanceDenialKind> {
    match (contract, pending) {
        (WorthQueryAdmittedApplicationDisclosureContract::Public, None) => {
            Ok(WorthQueryApplicationQueryGovernance::Public)
        }
        (
            WorthQueryAdmittedApplicationDisclosureContract::Governed {
                classification,
                capability_name,
                capability_type,
                result_rules,
                internal_rules,
            },
            Some(pending),
        ) => {
            if capability_name != pending.capability_name
                || capability_type != pending.capability_type
            {
                return Err(WorthQueryApplicationQueryGovernanceDenialKind::CapabilityMismatch);
            }
            if internal_rules
                .iter()
                .any(|rule| rule.disclosure_value() != &pending.disclosure_value)
            {
                return Err(
                    WorthQueryApplicationQueryGovernanceDenialKind::InternalComputationDenied,
                );
            }
            let mut disclosed = BTreeMap::new();
            let mut omitted = BTreeMap::new();
            for (slot, rule) in result_rules {
                if rule.disclosure_value() == &pending.disclosure_value {
                    disclosed.insert(slot, rule.disclosure_value().clone());
                } else {
                    omitted.insert(slot, rule.disclosure_value().clone());
                }
            }
            Ok(WorthQueryApplicationQueryGovernance::Governed {
                capability_name: pending.capability_name,
                capability_type: pending.capability_type,
                disclosure_value: pending.disclosure_value.clone(),
                computation: WorthQueryApplicationInternalComputationAuthority {
                    runtime: binding.runtime,
                    query: binding.query,
                    parameters: binding.parameters,
                    principal: binding.principal,
                    scope: binding.scope,
                    disclosure_value: pending.disclosure_value,
                },
                disclosure: WorthQueryApplicationDisclosureDecision {
                    classification,
                    disclosed,
                    omitted,
                },
                authorization: pending.authorization,
            })
        }
        _ => Err(WorthQueryApplicationQueryGovernanceDenialKind::Required),
    }
}

pub(in crate::domain_computation::primary_graph::application_query) struct WorthQueryApplicationGovernanceBinding
{
    pub(super) runtime: WorthQueryRuntimeAuthorityIdentity,
    pub(super) query: WorthQueryInstalledApplicationQueryIdentity,
    pub(super) parameters: CanonicalDigestId,
    pub(super) principal: EntityId,
    pub(super) scope: EntityId,
}

impl WorthQueryApplicationGovernanceBinding {
    pub(in crate::domain_computation::primary_graph::application_query) fn new(
        runtime: WorthQueryRuntimeAuthorityIdentity,
        query: WorthQueryInstalledApplicationQueryIdentity,
        parameters: CanonicalDigestId,
        principal: EntityId,
        scope: EntityId,
    ) -> Self {
        Self {
            runtime,
            query,
            parameters,
            principal,
            scope,
        }
    }
}

impl WorthQueryApplicationQueryGovernance {
    pub(in crate::domain_computation::primary_graph::application_query) fn into_pending(
        self,
    ) -> Option<WorthQueryPendingApplicationQueryGovernance> {
        match self {
            Self::Public => None,
            Self::Governed {
                capability_name,
                capability_type,
                disclosure_value,
                computation: _,
                disclosure: _,
                authorization,
            } => Some(WorthQueryPendingApplicationQueryGovernance {
                capability_name,
                capability_type,
                disclosure_value,
                authorization,
            }),
        }
    }

    pub(super) fn disclosure(&self) -> Option<&WorthQueryApplicationDisclosureDecision> {
        match self {
            Self::Public => None,
            Self::Governed { disclosure, .. } => Some(disclosure),
        }
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn is_disclosed(
        &self,
        slot: &ApplicationQueryResultSlotKey,
    ) -> bool {
        self.disclosure()
            .is_none_or(|disclosure| disclosure.is_disclosed(slot))
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn omission(
        &self,
        slot: &ApplicationQueryResultSlotKey,
    ) -> Option<(&str, &AspectValue)> {
        self.disclosure().and_then(|disclosure| {
            disclosure
                .omission(slot)
                .map(|value| (disclosure.classification.as_str(), value))
        })
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn authorization_mut(
        &mut self,
    ) -> Option<&mut WorthQueryRetainedCapabilityAuthorization> {
        match self {
            Self::Public => None,
            Self::Governed { authorization, .. } => Some(authorization),
        }
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn authorization(
        &self,
    ) -> Option<&WorthQueryRetainedCapabilityAuthorization> {
        match self {
            Self::Public => None,
            Self::Governed { authorization, .. } => Some(authorization),
        }
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn installed_capability_identity(
        &self,
    ) -> Option<[u8; 32]> {
        self.authorization()
            .map(WorthQueryRetainedCapabilityAuthorization::installed_capability_identity)
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn authorization_belongs_to_session(
        &self,
        session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    ) -> bool {
        self.authorization()
            .is_none_or(|authorization| authorization.belongs_to_session(session))
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn receipt(
        &self,
    ) -> WorthQueryApplicationDisclosureReceipt {
        match self {
            Self::Public => WorthQueryApplicationDisclosureReceipt::public(),
            Self::Governed {
                disclosure,
                authorization,
                ..
            } => WorthQueryApplicationDisclosureReceipt::governed(
                disclosure.classification.clone(),
                disclosure.disclosed.values().cloned().collect(),
                disclosure.omitted.values().cloned().collect(),
                authorization.capability_authority_identity(),
                authorization.decision_identity(),
                authorization.exact_fact_count(),
            ),
        }
    }

    pub(in crate::domain_computation::primary_graph::application_query) fn computation_matches(
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
                computation.runtime == runtime
                    && &computation.query == query
                    && &computation.parameters == parameters
                    && computation.principal == principal
                    && computation.scope == scope
                    && !matches!(computation.disclosure_value, AspectValue::Null)
            }
        }
    }
}

impl WorthQueryApplicationDisclosureDecision {
    pub(super) fn is_disclosed(&self, slot: &ApplicationQueryResultSlotKey) -> bool {
        self.disclosed.contains_key(slot)
    }

    pub(super) fn omission(&self, slot: &ApplicationQueryResultSlotKey) -> Option<&AspectValue> {
        self.omitted.get(slot)
    }
}
