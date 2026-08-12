use worth_query_installation::facade::ApplicationOperationDecisionReadTarget;

use super::super::fact::WorthQueryApplicationFactKey;
use super::WorthQueryApplicationReadAttempt;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
    WorthQueryApplicationEntityIdentity,
};

impl<Schema, Operation, Input, Scope, Phase>
    WorthQueryApplicationReadAttempt<Schema, Operation, Input, Scope, Phase>
{
    pub(super) fn admit_target(
        &self,
        target: &ApplicationOperationDecisionReadTarget,
    ) -> Result<(), WorthQueryApplicationAttemptDenial> {
        if self
            .admission
            .allowed_graph_contract()
            .decision_reads()
            .contains(target)
        {
            Ok(())
        } else {
            Err(denial(
                WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                self.admission.operation(),
            ))
        }
    }

    pub(super) fn admit_fact_key(
        &self,
        key: &WorthQueryApplicationFactKey,
    ) -> Result<(), WorthQueryApplicationAttemptDenial> {
        let budget = self
            .admission
            .allowed_graph_contract()
            .decision_fact_budget();
        if !self.facts.contains_key(key) && self.facts.len() >= budget {
            Err(denial(
                WorthQueryApplicationAttemptDenialKind::DecisionFactBudgetExceeded,
                self.admission.operation(),
            ))
        } else {
            Ok(())
        }
    }

    pub(super) fn validate_identity_authority<Entity>(
        &self,
        entity: &str,
        identity: &WorthQueryApplicationEntityIdentity<Schema, Entity>,
    ) -> Result<(), WorthQueryApplicationAttemptDenial> {
        if identity.runtime_authority() != self.admission.runtime_authority()
            || identity.binding_identity() != self.admission.binding_identity()
            || identity.entity_name() != entity
        {
            return Err(denial(
                WorthQueryApplicationAttemptDenialKind::StaleEntityIdentity,
                entity,
            ));
        }
        if !self.read_scope.admits(identity.entity_id()) {
            return Err(denial(
                WorthQueryApplicationAttemptDenialKind::OutsideRealizedReadScope,
                entity,
            ));
        }
        Ok(())
    }

    pub(super) fn validate_identity_freshness<Entity>(
        &self,
        entity: &str,
        identity: &WorthQueryApplicationEntityIdentity<Schema, Entity>,
    ) -> Result<(), WorthQueryApplicationAttemptDenial> {
        self.lease
            .handle()
            .with_runtime(|runtime| {
                self.entity_resolution
                    .at_snapshot(
                        runtime,
                        self.lease.snapshot(),
                        super::super::super::WorthQueryPrincipalResolutionMode::Ordinary,
                    )
                    .and_then(|truth| truth.validate_entity_freshness(identity))
            })
            .map_err(|_| {
                denial(
                    WorthQueryApplicationAttemptDenialKind::StaleEntityIdentity,
                    entity,
                )
            })
    }

    pub(super) fn field_layout(
        &self,
        entity: &str,
        aspect: &str,
        field: &str,
    ) -> Result<worth_foundational::facade::AspectFieldLocator, WorthQueryApplicationAttemptDenial>
    {
        self.layout
            .field_locator(entity, aspect, field)
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryApplicationAttemptDenialKind::UndeclaredDecisionRead,
                    field,
                )
            })
    }
}

fn denial(
    kind: WorthQueryApplicationAttemptDenialKind,
    subject: impl Into<String>,
) -> WorthQueryApplicationAttemptDenial {
    WorthQueryApplicationAttemptDenial::new(kind, subject)
}
