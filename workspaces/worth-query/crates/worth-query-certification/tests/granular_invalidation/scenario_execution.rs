use std::collections::BTreeSet;

use super::world::{
    DeclaredDependency, DeclaredLocality, GranularInvalidationScenario,
    GranularInvalidationWorldDefinition,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CrossRuntimeOracleCounters {
    pub relational_changes: usize,
    pub bridge_bucket_probes: usize,
    pub bridge_candidates: usize,
    pub bridge_rejections: usize,
    pub signal_seeds: usize,
    pub query_impact_probes: usize,
    pub admitted_impacts: usize,
    pub maintenance_operations: usize,
    pub consumer_publications: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossRuntimeOracleProjection {
    pub relational: BTreeSet<String>,
    pub bridge: BTreeSet<String>,
    pub signal: BTreeSet<String>,
    pub impacts: BTreeSet<String>,
    pub maintenance: BTreeSet<String>,
    pub deliveries: BTreeSet<String>,
    pub exclusions: BTreeSet<String>,
    pub counters: CrossRuntimeOracleCounters,
}

/// Independently projects semantic necessity without consulting production
/// indexes, receipts, counters, or observed work.
pub fn project_scenario_oracle(
    world: &GranularInvalidationWorldDefinition,
) -> CrossRuntimeOracleProjection {
    let mut projected = empty_projection();
    for mutation in &world.mutations {
        projected.counters.relational_changes += 1;
        projected.relational.insert(format!(
            "{}:{}:{}:{}",
            mutation.identity,
            mutation.aspect,
            mutation
                .relational_record_identity
                .terminal_projection_for_reporting(),
            mutation.field
        ));
        if !mutation.current {
            projected.counters.bridge_rejections += 1;
            continue;
        }
        let mut roles = BTreeSet::new();
        let mut suppressed = false;
        for dependency in &world.dependencies {
            projected.counters.bridge_bucket_probes += 1;
            if !matches_mutation(dependency, mutation) {
                continue;
            }
            projected.counters.bridge_candidates += 1;
            let bridge = canonical_dependency(mutation.identity, dependency);
            projected.bridge.insert(bridge.clone());
            projected.signal.insert(format!(
                "{}:query-signal:{}:{}",
                mutation.identity,
                dependency.query_signal_mapping,
                dependency.query_signal_partition
            ));
            projected.counters.query_impact_probes += 1;
            for role in dependency.roles {
                projected.impacts.insert(format!("{bridge}:{role}"));
                roles.insert(*role);
                projected.counters.admitted_impacts += 1;
            }
            if dependency.performs_signal {
                projected
                    .signal
                    .insert(format!(
                        "{}:performed-signal:{}",
                        mutation.identity, dependency.performed_signal_partition
                    ));
                projected.counters.signal_seeds += 1;
                let conditional = "conditional-eligibility-or-semantic-cleanliness";
                projected.impacts.insert(format!("{bridge}:{conditional}"));
                roles.insert(conditional);
                projected.counters.admitted_impacts += 1;
            }
            if mutation.magnitude <= dependency.tolerance {
                suppressed = true;
                projected.exclusions.insert(format!("suppressed:{bridge}"));
            }
        }
        if suppressed || roles.is_empty() {
            continue;
        }
        let maintenance = format!(
            "maintenance:strategies={}:scope=exact-record",
            strategies(&roles).join("|")
        );
        projected.maintenance.insert(maintenance.clone());
        let role_identity = roles.iter().copied().collect::<Vec<_>>().join("|");
        let delivery = if world.scenario
            == GranularInvalidationScenario::SharedLeaseDisclosureNoninterference
        {
            format!(
                "delivery:{maintenance}:roles:{role_identity}:purpose=desk-risk-monitoring:disclosure=desk-risk-public"
            )
        } else {
            format!("delivery:{maintenance}:roles:{role_identity}")
        };
        projected.deliveries.insert(delivery);
    }
    if world.scenario == GranularInvalidationScenario::SharedLeaseDisclosureNoninterference {
        projected.exclusions.insert("authorization-denied:1".into());
    }
    if world.scenario == GranularInvalidationScenario::CorrespondenceRebindRestore {
        projected.exclusions.extend([
            "rebind:stale-batch".into(),
            "rebind:old-binding".into(),
        ]);
    }
    projected.counters.maintenance_operations = projected.maintenance.len();
    projected.counters.consumer_publications = projected.deliveries.len();
    projected
}

fn matches_mutation(
    dependency: &DeclaredDependency,
    mutation: &super::world::GranularInvalidationMutation,
) -> bool {
    dependency.aspect == mutation.aspect
        && dependency.field == mutation.field
        && match dependency.locality {
            DeclaredLocality::Unscoped => true,
            DeclaredLocality::WholePartition(partition) => partition == mutation.partition,
            DeclaredLocality::ExactDetail(partition, detail) => {
                partition == mutation.partition && detail == mutation.detail
            }
        }
}

fn canonical_dependency(mutation: &str, dependency: &DeclaredDependency) -> String {
    let locality = match dependency.locality {
        DeclaredLocality::Unscoped => "whole-graph".to_owned(),
        DeclaredLocality::WholePartition(partition) => format!("partition={partition}"),
        DeclaredLocality::ExactDetail(..) => "managed-record".to_owned(),
    };
    format!(
        "{mutation}:dependency:{}:{}:entity-field:{}:{locality}:{}",
        dependency.ordinal, dependency.aspect, dependency.aspect, dependency.field
    )
}

fn strategies(roles: &BTreeSet<&str>) -> Vec<&'static str> {
    let mut values = Vec::new();
    if roles.contains("projected-value")
        || roles.contains("conditional-eligibility-or-semantic-cleanliness")
    {
        values.push("projection");
    }
    if roles.contains("selection-or-membership") {
        values.push("membership");
    }
    if roles.contains("ordering") || roles.contains("grouping") {
        values.push("ordering-or-grouping");
    }
    if roles.contains("window-boundary") {
        values.push("window");
    }
    values
}

fn empty_projection() -> CrossRuntimeOracleProjection {
    CrossRuntimeOracleProjection {
        relational: BTreeSet::new(),
        bridge: BTreeSet::new(),
        signal: BTreeSet::new(),
        impacts: BTreeSet::new(),
        maintenance: BTreeSet::new(),
        deliveries: BTreeSet::new(),
        exclusions: BTreeSet::new(),
        counters: CrossRuntimeOracleCounters::default(),
    }
}
