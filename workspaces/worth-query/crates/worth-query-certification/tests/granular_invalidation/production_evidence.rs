use std::collections::{BTreeMap, BTreeSet};

use worth_query::facade::domain::{
    WorthQueryGranularAdmissionCounters, WorthQueryGranularMaintenanceCounters,
    WorthQuerySemanticDependencyRole,
};
use worth_query_execution::facade::primary_graph::{
    WorthQueryBridgeGranularDeliveryCounters, WorthQueryGranularInvalidationInstallation,
};
use worth_runtime_bridge::facade::BridgeDiagnosticsTier;
use worth_signal::facade::adapters::SignalInvalidationRealizedCounters;

use super::world::GranularInvalidationScenario;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CertificationComparatorPolicy {
    Exact,
    Tolerance {
        epsilon: u64,
        provider_identity: &'static str,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CertificationExecutionLane {
    Scheduled,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CrossRuntimeObservedIdentities {
    pub relational: BTreeSet<String>,
    pub bridge: BTreeSet<String>,
    pub signal: BTreeSet<String>,
    pub impacts: BTreeSet<String>,
    pub maintenance: BTreeSet<String>,
    pub deliveries: BTreeSet<String>,
    pub exclusions: BTreeSet<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OwnerPerformedCounterRows(BTreeMap<String, u64>);

impl OwnerPerformedCounterRows {
    pub fn observe(
        &mut self,
        bridge: WorthQueryBridgeGranularDeliveryCounters,
        signal: SignalInvalidationRealizedCounters,
        admission: WorthQueryGranularAdmissionCounters,
        maintenance: Option<WorthQueryGranularMaintenanceCounters>,
    ) {
        add_rows(&mut self.0, "bridge", bridge_rows(bridge));
        add_rows(
            &mut self.0,
            "signal",
            signal
                .values()
                .iter()
                .enumerate()
                .map(|(ordinal, value)| (format!("row.{ordinal:02}"), *value)),
        );
        add_rows(&mut self.0, "query.admission", admission_rows(admission));
        if let Some(maintenance) = maintenance {
            add_rows(
                &mut self.0,
                "query.maintenance",
                maintenance_rows(maintenance),
            );
        }
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = (&str, u64)> {
        self.0.iter().map(|(name, value)| (name.as_str(), *value))
    }

    pub fn value(&self, row: &str) -> u64 {
        self.0.get(row).copied().unwrap_or_default()
    }

    pub fn sum_prefix(&self, prefix: &str) -> u64 {
        self.0
            .iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .map(|(_, value)| *value)
            .sum()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerformedScenarioEvidence {
    scenario: GranularInvalidationScenario,
    seed: u64,
    policy: CertificationComparatorPolicy,
    diagnostics_tier: BridgeDiagnosticsTier,
    execution_lane: CertificationExecutionLane,
    runtime_ordinal: u64,
    runtime_generation: u64,
    direct_truth_deliveries: usize,
    performed_signal_deliveries: usize,
    identities: CrossRuntimeObservedIdentities,
    counters: OwnerPerformedCounterRows,
}

pub struct PerformedScenarioEvidenceParts<'a> {
    pub scenario: GranularInvalidationScenario,
    pub seed: u64,
    pub policy: CertificationComparatorPolicy,
    pub diagnostics_tier: BridgeDiagnosticsTier,
    pub execution_lane: CertificationExecutionLane,
    pub batch_installation: &'a WorthQueryGranularInvalidationInstallation,
    pub current_installation: &'a WorthQueryGranularInvalidationInstallation,
    pub observer: super::performed_identity_observer::PerformedIdentityObserver,
    pub counters: OwnerPerformedCounterRows,
}

impl PerformedScenarioEvidence {
    pub fn from_performed(parts: PerformedScenarioEvidenceParts<'_>) -> Result<Self, &'static str> {
        if !parts
            .current_installation
            .is_same_current_runtime_as(parts.batch_installation)
        {
            return Err("scenario mixed primary runtime installations");
        }
        let binding = parts.batch_installation.binding_identity();
        let (identities, direct_truth_deliveries, performed_signal_deliveries) =
            parts.observer.finish();
        Ok(Self {
            scenario: parts.scenario,
            seed: parts.seed,
            policy: parts.policy,
            diagnostics_tier: parts.diagnostics_tier,
            execution_lane: parts.execution_lane,
            runtime_ordinal: binding.runtime_ordinal(),
            runtime_generation: binding.generation(),
            direct_truth_deliveries,
            performed_signal_deliveries,
            identities,
            counters: parts.counters,
        })
    }

    pub const fn scenario(&self) -> GranularInvalidationScenario {
        self.scenario
    }
    pub const fn seed(&self) -> u64 {
        self.seed
    }
    pub fn policy(&self) -> &CertificationComparatorPolicy {
        &self.policy
    }
    pub const fn diagnostics_tier(&self) -> BridgeDiagnosticsTier {
        self.diagnostics_tier
    }
    pub const fn execution_lane(&self) -> CertificationExecutionLane {
        self.execution_lane
    }
    pub const fn runtime_ordinal(&self) -> u64 {
        self.runtime_ordinal
    }
    pub const fn runtime_generation(&self) -> u64 {
        self.runtime_generation
    }
    pub const fn direct_truth_deliveries(&self) -> usize {
        self.direct_truth_deliveries
    }
    pub const fn performed_signal_deliveries(&self) -> usize {
        self.performed_signal_deliveries
    }
    pub fn identities(&self) -> &CrossRuntimeObservedIdentities {
        &self.identities
    }
    pub fn counters(&self) -> &OwnerPerformedCounterRows {
        &self.counters
    }

    pub fn with_faulted_scenario(mut self, scenario: GranularInvalidationScenario) -> Self {
        self.scenario = scenario;
        self
    }

    pub fn with_faulted_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn with_faulted_policy(mut self, policy: CertificationComparatorPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn with_faulted_tier(mut self, tier: BridgeDiagnosticsTier) -> Self {
        self.diagnostics_tier = tier;
        self
    }

    pub fn with_faulted_runtime_generation(mut self, generation: u64) -> Self {
        self.runtime_generation = generation;
        self
    }

    pub fn with_faulted_direct_truth_count(mut self, count: usize) -> Self {
        self.direct_truth_deliveries = count;
        self
    }

    pub fn with_faulted_signal_identity(mut self, identity: &str) -> Self {
        self.identities.signal.insert(identity.to_owned());
        self
    }
}

fn add_rows(
    target: &mut BTreeMap<String, u64>,
    owner: &str,
    rows: impl IntoIterator<Item = (String, u64)>,
) {
    for (name, value) in rows {
        *target.entry(format!("{owner}.{name}")).or_default() += value;
    }
}

fn bridge_rows(value: WorthQueryBridgeGranularDeliveryCounters) -> Vec<(String, u64)> {
    [
        ("source-load-attempts", value.source_load_attempts),
        ("source-envelopes-loaded", value.source_envelopes_loaded),
        (
            "allocation-lock-attempts",
            value.allocation_registry_lock_attempts,
        ),
        (
            "allocation-source-checks",
            value.allocation_source_set_checks,
        ),
        ("signal-basis-checks", value.signal_basis_target_checks),
        (
            "signal-capability-admissions",
            value.signal_capability_admissions,
        ),
        ("failed-deliveries", value.failed_deliveries),
        ("truth-targets-admitted", value.truth_targets_admitted),
        ("correspondence-lookups", value.correspondence_lookups),
        ("semantic-match-checks", value.semantic_match_checks),
        ("aspect-rejections", value.aspect_rejections),
        ("binding-rejections", value.binding_rejections),
        ("change-kind-rejections", value.change_kind_rejections),
        ("locality-rejections", value.locality_rejections),
        ("projection-rejections", value.projection_rejections),
        ("relevant-change-checks", value.relevant_change_checks),
        (
            "projection-paths-inspected",
            value.projection_paths_inspected,
        ),
        (
            "source-widening-checks",
            value.source_widening_target_checks,
        ),
        ("signal-seeds-emitted", value.signal_seeds_emitted),
        ("node-fan-out", value.node_fan_out),
        ("slots-touched", value.slots_touched),
    ]
    .into_iter()
    .map(|(name, value)| (name.to_owned(), value as u64))
    .collect()
}

fn admission_rows(value: WorthQueryGranularAdmissionCounters) -> Vec<(String, u64)> {
    let mut rows = vec![
        ("delivery-changes", value.delivery_changes_examined()),
        ("locality-entries", value.locality_entries_examined()),
        ("impact-index-probes", value.impact_index_probes()),
        (
            "candidate-deliveries",
            value.candidate_deliveries_returned(),
        ),
        ("candidate-roles", value.candidate_roles_returned()),
        (
            "candidate-rejections",
            value.candidates_rejected_before_admission(),
        ),
        ("admitted-impacts", value.admitted_impacts()),
    ]
    .into_iter()
    .map(|(name, value)| (name.to_owned(), value as u64))
    .collect::<Vec<_>>();
    for role in dependency_roles() {
        rows.push((
            format!("role.{role:?}"),
            value.admitted_role_count(role) as u64,
        ));
    }
    rows
}

fn maintenance_rows(value: WorthQueryGranularMaintenanceCounters) -> Vec<(String, u64)> {
    [
        ("operations", value.maintenance_operations()),
        ("coalesced-impacts", value.coalesced_impacts()),
        ("projected-fields", value.projected_fields()),
        ("prior-field-comparisons", value.prior_field_comparisons()),
        ("membership-rows", value.membership_rows()),
        ("ordering-keys", value.ordering_keys()),
        ("aggregate-groups", value.aggregate_groups()),
        ("window-rows", value.window_rows()),
        ("bounded-reexecution-rows", value.bounded_reexecution_rows()),
        ("explicit-rebinds", value.explicit_rebinds()),
        ("replacements", value.replacements()),
        ("retirements", value.retirements()),
        ("suppressions", value.suppressions()),
        (
            "authorization-revalidations",
            value.authorization_revalidations(),
        ),
        ("authorization-denials", value.authorization_denials()),
        ("consumer-publications", value.consumer_publications()),
        (
            "retained-backpressure",
            value.retained_backpressure_deliveries(),
        ),
        (
            "dropped-backpressure",
            value.dropped_backpressure_deliveries(),
        ),
        (
            "terminated-backpressure",
            value.terminated_backpressure_deliveries(),
        ),
        ("debt-backpressure", value.debt_backpressure_deliveries()),
    ]
    .into_iter()
    .map(|(name, value)| (name.to_owned(), value as u64))
    .collect()
}

fn dependency_roles() -> [WorthQuerySemanticDependencyRole; 10] {
    use WorthQuerySemanticDependencyRole as Role;
    [
        Role::OperationalIdentity,
        Role::SelectionOrMembership,
        Role::Ordering,
        Role::ProjectedValue,
        Role::Grouping,
        Role::WindowBoundary,
        Role::SupportAndLifecycle,
        Role::ConditionalEligibilityOrSemanticCleanliness,
        Role::InstalledDomainInvariant,
        Role::AdvisoryOnlyContext,
    ]
}
