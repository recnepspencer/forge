use worth_query::facade::domain::{
    WorthQueryAdmittedInvalidationObservation, WorthQueryConditionalDependencyInstallation,
    WorthQueryGranularNoChange, WorthQueryMaintenanceScope, WorthQueryMaintenanceStrategy,
    WorthQueryPrimaryGranularMaintenancePerformed, WorthQueryPublishedSharedPrimaryInvalidation,
    WorthQuerySemanticDependencyRole,
};
use worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts;

use crate::production_evidence::CrossRuntimeObservedIdentities;
use crate::world::GranularInvalidationMutation;

#[derive(Default)]
pub struct PerformedIdentityObserver {
    observed: CrossRuntimeObservedIdentities,
    direct_truth_deliveries: usize,
    performed_signal_deliveries: usize,
}

impl PerformedIdentityObserver {
    pub fn observe_lower_truth(
        &mut self,
        mutation: &GranularInvalidationMutation,
        owner_record: RelationalBridgeRecordIdentityParts,
        batch: &worth_query_execution::facade::primary_graph::WorthQueryGranularInvalidationDeliveryBatch,
    ) -> Result<(), &'static str> {
        let observed = batch.bridge_deliveries().iter().find_map(|delivery| {
            delivery
                .truth()
                .change_set()
                .changes()
                .iter()
                .find_map(|delivered| {
                    let change = delivered.semantic_change()?;
                    let record = delivered.relational_record_identity()?;
                    (change.aspect_key().as_str() == mutation.aspect
                        && field_path(change.field_path()) == mutation.field
                        && record == owner_record)
                        .then(|| relational_identity(mutation.identity, change, record))
                })
        });
        self.observed
            .relational
            .insert(observed.ok_or("lower truth does not retain the named source mutation")?);
        Ok(())
    }

    pub fn observe_impacts(
        &mut self,
        mutation: &GranularInvalidationMutation,
        owner_record: RelationalBridgeRecordIdentityParts,
        impacts: &[WorthQueryAdmittedInvalidationObservation],
        signal_installations: &[WorthQueryConditionalDependencyInstallation],
    ) -> Result<(), &'static str> {
        for impact in impacts {
            self.observe_impact(mutation, owner_record, impact, signal_installations)?;
        }
        Ok(())
    }

    pub fn observe_suppression(
        &mut self,
        mutation: &GranularInvalidationMutation,
        owner_record: RelationalBridgeRecordIdentityParts,
        suppressed: &WorthQueryGranularNoChange,
        signal_installations: &[WorthQueryConditionalDependencyInstallation],
    ) -> Result<(), &'static str> {
        self.observe_impacts(
            mutation,
            owner_record,
            suppressed.impact_observations(),
            signal_installations,
        )?;
        for impact in suppressed.impact_observations() {
            self.observed.exclusions.insert(format!(
                "suppressed:{}",
                bridge_identity(mutation.identity, impact)
            ));
        }
        Ok(())
    }

    pub fn observe_primary_publication(
        &mut self,
        performed: &WorthQueryPrimaryGranularMaintenancePerformed,
    ) {
        for delivery in performed.deliveries() {
            let maintenance = maintenance_identity(delivery.strategies(), delivery.scope());
            self.observed.maintenance.insert(maintenance.clone());
            self.observed.deliveries.insert(format!(
                "delivery:{maintenance}:roles:{}",
                role_identity(delivery.roles())
            ));
        }
    }

    pub fn observe_shared_publication(
        &mut self,
        publication: &WorthQueryPublishedSharedPrimaryInvalidation,
    ) {
        let maintenance = maintenance_identity(publication.strategies(), publication.scope());
        self.observed.maintenance.insert(maintenance.clone());
        let authority = publication.consumer_delivery_authority();
        self.observed.deliveries.insert(format!(
            "delivery:{maintenance}:roles:{}:purpose={}:disclosure={}",
            role_identity(publication.roles()),
            authority.purpose_identity(),
            authority.disclosure_identity()
        ));
    }

    pub fn observe_authorization_denials(&mut self, count: usize) {
        if count > 0 {
            self.observed
                .exclusions
                .insert(format!("authorization-denied:{count}"));
        }
    }

    pub fn observe_rebind_denial(&mut self, owner: &'static str) {
        self.observed.exclusions.insert(format!("rebind:{owner}"));
    }

    pub fn finish(self) -> (CrossRuntimeObservedIdentities, usize, usize) {
        (
            self.observed,
            self.direct_truth_deliveries,
            self.performed_signal_deliveries,
        )
    }

    fn observe_impact(
        &mut self,
        mutation: &GranularInvalidationMutation,
        owner_record: RelationalBridgeRecordIdentityParts,
        impact: &WorthQueryAdmittedInvalidationObservation,
        signal_installations: &[WorthQueryConditionalDependencyInstallation],
    ) -> Result<(), &'static str> {
        let change_set = impact.truth().change_set();
        let mut matched_change = false;
        for delivered in change_set.changes() {
            let Some(change) = delivered.semantic_change() else {
                continue;
            };
            let Some(record) = delivered.relational_record_identity() else {
                continue;
            };
            if change.aspect_key().as_str() != mutation.aspect
                || field_path(change.field_path()) != mutation.field
                || record != owner_record
            {
                continue;
            }
            matched_change = true;
            self.observed
                .relational
                .insert(relational_identity(mutation.identity, change, record));
        }
        if !matched_change {
            return Err("performed impact does not retain the named source mutation");
        }
        let bridge = bridge_identity(mutation.identity, impact);
        self.direct_truth_deliveries += 1;
        self.observed.bridge.insert(bridge.clone());
        let ordinal = change_set.dependency().dependency_ordinal();
        let installation = signal_installations
            .get(ordinal)
            .ok_or("performed impact has no matching Query Signal installation")?;
        for target in installation.signal_targets() {
            self.observed.signal.insert(format!(
                "{}:query-signal:{}:{}",
                mutation.identity,
                target.aspect_registration_identity(),
                target.partition().0
            ));
        }
        for role in impact.roles() {
            self.observed
                .impacts
                .insert(format!("{bridge}:{}", role.canonical_name()));
        }
        if impact.performed_signal_binding().is_some() {
            self.performed_signal_deliveries += 1;
            let partitions = change_set
                .basis()
                .signal_partitions()
                .iter()
                .map(|partition| partition.0.as_str())
                .collect::<Vec<_>>()
                .join("|");
            self.observed.signal.insert(format!(
                "{}:performed-signal:{partitions}",
                mutation.identity
            ));
        }
        Ok(())
    }
}

fn relational_identity(
    mutation: &str,
    change: &worth_runtime_bridge::facade::BridgeSemanticAspectChange,
    record: RelationalBridgeRecordIdentityParts,
) -> String {
    format!(
        "{mutation}:{}:{}:{}",
        change.aspect_key().as_str(),
        record.terminal_projection_for_reporting(),
        field_path(change.field_path())
    )
}

fn bridge_identity(mutation: &str, impact: &WorthQueryAdmittedInvalidationObservation) -> String {
    let dependency = impact.truth().change_set().dependency();
    let paths = if dependency.projection_mask().is_whole_aspect() {
        "whole".to_owned()
    } else {
        dependency
            .projection_mask()
            .paths()
            .iter()
            .map(|path| field_path(Some(path)))
            .collect::<Vec<_>>()
            .join("|")
    };
    format!(
        "{mutation}:dependency:{}:{}:{}:{}:{paths}",
        dependency.dependency_ordinal(),
        dependency.contract().key().as_str(),
        dependency.binding().canonical_name(),
        locality_identity(dependency.locality())
    )
}

fn locality_identity(locality: &worth_runtime_bridge::facade::BridgeSemanticLocality) -> String {
    match locality {
        worth_runtime_bridge::facade::BridgeSemanticLocality::SourceRecord => "record".to_owned(),
        worth_runtime_bridge::facade::BridgeSemanticLocality::ManagedSourceRecord => {
            "managed-record".to_owned()
        }
        worth_runtime_bridge::facade::BridgeSemanticLocality::SourcePartition(partition) => {
            format!("partition={}", partition.as_str())
        }
        worth_runtime_bridge::facade::BridgeSemanticLocality::WholeLogicalGraph => {
            "whole-graph".to_owned()
        }
    }
}

fn maintenance_identity(
    strategies: &[WorthQueryMaintenanceStrategy],
    scope: &WorthQueryMaintenanceScope,
) -> String {
    format!(
        "maintenance:strategies={}:scope={}",
        strategies
            .iter()
            .map(|strategy| strategy_name(*strategy))
            .collect::<Vec<_>>()
            .join("|"),
        match scope {
            WorthQueryMaintenanceScope::ExactSourceRecord { .. } => "exact-record",
            WorthQueryMaintenanceScope::SourcePartition(_) => "partition",
            WorthQueryMaintenanceScope::WholeLogicalGraph => "whole-graph",
        }
    )
}

fn strategy_name(strategy: WorthQueryMaintenanceStrategy) -> &'static str {
    match strategy {
        WorthQueryMaintenanceStrategy::Suppression => "suppression",
        WorthQueryMaintenanceStrategy::LocalProjectionPatch => "projection",
        WorthQueryMaintenanceStrategy::MembershipSplice => "membership",
        WorthQueryMaintenanceStrategy::StableReorderOrRegroup => "ordering-or-grouping",
        WorthQueryMaintenanceStrategy::WindowRefill => "window",
        WorthQueryMaintenanceStrategy::BoundedReexecution => "bounded-reexecution",
        WorthQueryMaintenanceStrategy::ExplicitRebind => "rebind",
        WorthQueryMaintenanceStrategy::Replacement => "replacement",
        WorthQueryMaintenanceStrategy::Retirement => "retirement",
    }
}

fn role_identity(roles: &[WorthQuerySemanticDependencyRole]) -> String {
    let mut names = roles
        .iter()
        .map(|role| role.canonical_name())
        .collect::<Vec<_>>();
    names.sort_unstable();
    names.join("|")
}

fn field_path(path: Option<&worth_foundational::facade::CanonicalFieldPath>) -> String {
    path.map(|path| {
        path.fields()
            .iter()
            .map(|field| field.as_str())
            .collect::<Vec<_>>()
            .join(".")
    })
    .unwrap_or_else(|| "whole".to_owned())
}
