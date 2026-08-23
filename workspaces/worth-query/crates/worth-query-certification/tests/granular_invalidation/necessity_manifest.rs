use std::collections::BTreeSet;

use super::world::{
    DeclaredDependency, DeclaredLocality, GranularInvalidationScenario,
    GranularInvalidationWorldDefinition,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossRuntimeInvalidationNecessityManifest {
    pub relational: BTreeSet<String>,
    pub bridge: BTreeSet<String>,
    pub signal: BTreeSet<String>,
    pub impacts: BTreeSet<String>,
    pub maintenance: BTreeSet<String>,
    pub deliveries: BTreeSet<String>,
    pub exclusions: BTreeSet<String>,
}

impl CrossRuntimeInvalidationNecessityManifest {
    pub fn derive(world: &GranularInvalidationWorldDefinition) -> Self {
        let mut expected = Self::empty();
        for mutation in &world.mutations {
            expected.relational.insert(format!(
                "{}:{}:{}:{}",
                mutation.identity,
                mutation.aspect,
                mutation
                    .relational_record_identity
                    .terminal_projection_for_reporting(),
                mutation.field
            ));
            if !mutation.current {
                continue;
            }
            let matched = world
                .dependencies
                .iter()
                .filter(|dependency| dependency_matches(dependency, mutation))
                .collect::<Vec<_>>();
            let mut performed_roles = BTreeSet::new();
            for dependency in matched {
                let bridge = bridge_identity(mutation.identity, dependency);
                expected.bridge.insert(bridge.clone());
                expected.signal.insert(format!(
                    "{}:query-signal:{}:{}",
                    mutation.identity,
                    dependency.query_signal_mapping,
                    dependency.query_signal_partition
                ));
                for role in dependency.roles {
                    expected.impacts.insert(format!("{bridge}:{role}"));
                    performed_roles.insert(*role);
                }
                let meaningful = mutation.magnitude > dependency.tolerance;
                if dependency.performs_signal {
                    expected.signal.insert(format!(
                        "{}:performed-signal:{}",
                        mutation.identity, dependency.performed_signal_partition
                    ));
                    expected.impacts.insert(format!(
                        "{bridge}:conditional-eligibility-or-semantic-cleanliness"
                    ));
                    performed_roles.insert("conditional-eligibility-or-semantic-cleanliness");
                }
                if !meaningful {
                    expected.exclusions.insert(format!("suppressed:{bridge}"));
                }
            }
            if expected
                .exclusions
                .iter()
                .any(|identity| identity.starts_with(&format!("suppressed:{}:", mutation.identity)))
            {
                continue;
            }
            if !performed_roles.is_empty() {
                let maintenance = maintenance_identity(&performed_roles);
                expected.maintenance.insert(maintenance.clone());
                expected.deliveries.insert(delivery_identity(
                    world.scenario,
                    &maintenance,
                    &performed_roles,
                ));
            }
        }
        if world.scenario == GranularInvalidationScenario::SharedLeaseDisclosureNoninterference {
            expected.exclusions.insert("authorization-denied:1".into());
        }
        if world.scenario == GranularInvalidationScenario::CorrespondenceRebindRestore {
            expected
                .exclusions
                .extend(["rebind:stale-batch".into(), "rebind:old-binding".into()]);
        }
        expected
    }

    fn empty() -> Self {
        Self {
            relational: BTreeSet::new(),
            bridge: BTreeSet::new(),
            signal: BTreeSet::new(),
            impacts: BTreeSet::new(),
            maintenance: BTreeSet::new(),
            deliveries: BTreeSet::new(),
            exclusions: BTreeSet::new(),
        }
    }
}

pub(super) fn bridge_identity(mutation: &str, dependency: &DeclaredDependency) -> String {
    format!(
        "{mutation}:dependency:{}:{}:entity-field:{}:{}:{}",
        dependency.ordinal,
        dependency.aspect,
        dependency.aspect,
        locality_identity(dependency.locality),
        dependency.field
    )
}

fn dependency_matches(
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

fn locality_identity(locality: DeclaredLocality) -> String {
    match locality {
        DeclaredLocality::Unscoped => "whole-graph".to_owned(),
        DeclaredLocality::WholePartition(partition) => format!("partition={partition}"),
        DeclaredLocality::ExactDetail(..) => "managed-record".to_owned(),
    }
}

fn maintenance_identity(roles: &BTreeSet<&str>) -> String {
    let strategies = strategy_names(roles);
    format!(
        "maintenance:strategies={}:scope=exact-record",
        strategies.join("|")
    )
}

fn delivery_identity(
    scenario: GranularInvalidationScenario,
    maintenance: &str,
    roles: &BTreeSet<&str>,
) -> String {
    let roles = roles.iter().copied().collect::<Vec<_>>().join("|");
    if scenario == GranularInvalidationScenario::SharedLeaseDisclosureNoninterference {
        format!(
            "delivery:{maintenance}:roles:{roles}:purpose=desk-risk-monitoring:disclosure=desk-risk-public"
        )
    } else {
        format!("delivery:{maintenance}:roles:{roles}")
    }
}

fn strategy_names(roles: &BTreeSet<&str>) -> Vec<&'static str> {
    let mut strategies = Vec::new();
    if roles.contains("projected-value")
        || roles.contains("conditional-eligibility-or-semantic-cleanliness")
    {
        strategies.push("projection");
    }
    if roles.contains("selection-or-membership") {
        strategies.push("membership");
    }
    if roles.contains("ordering") || roles.contains("grouping") {
        strategies.push("ordering-or-grouping");
    }
    if roles.contains("window-boundary") {
        strategies.push("window");
    }
    strategies
}
