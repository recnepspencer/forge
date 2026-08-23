use std::sync::Arc;

use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledPackageIndex, WorthQueryInstalledPackageIndexRelation,
};

use super::{
    publication::ConditionalRuntimeAffinity, runtime_owners::ConditionalRuntimeOwners,
    WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryConditionalRuntimeInstallationDenialKind,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

#[derive(Clone)]
pub struct WorthQueryConditionalRuntimeReinstallationReceipt {
    lower_runtime_reconstitution:
        worth_runtime_bridge::facade::BridgeConditionalRuntimeReconstitutionReport,
    reconstructed_binding_count: usize,
    reconstructed_intent_count: usize,
    examined_candidate_count: usize,
    projected_record_count: usize,
    projected_field_count: usize,
    total_work_units: usize,
    successor_invalidation_installation: super::super::WorthQueryGranularInvalidationInstallation,
}

impl std::fmt::Debug for WorthQueryConditionalRuntimeReinstallationReceipt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryConditionalRuntimeReinstallationReceipt")
            .field(
                "lower_runtime_reconstitution",
                &self.lower_runtime_reconstitution,
            )
            .field(
                "reconstructed_binding_count",
                &self.reconstructed_binding_count,
            )
            .field(
                "reconstructed_intent_count",
                &self.reconstructed_intent_count,
            )
            .field("total_work_units", &self.total_work_units)
            .finish_non_exhaustive()
    }
}

impl WorthQueryConditionalRuntimeReinstallationReceipt {
    pub const fn lower_runtime_reconstitution(
        &self,
    ) -> worth_runtime_bridge::facade::BridgeConditionalRuntimeReconstitutionReport {
        self.lower_runtime_reconstitution
    }

    pub const fn reconstructed_binding_count(&self) -> usize {
        self.reconstructed_binding_count
    }

    /// Authoritative temporal intents projected during this reconstructive pass.
    pub const fn reconstructed_intent_count(&self) -> usize {
        self.reconstructed_intent_count
    }

    pub const fn examined_candidate_count(&self) -> usize {
        self.examined_candidate_count
    }
    pub const fn projected_record_count(&self) -> usize {
        self.projected_record_count
    }
    pub const fn projected_field_count(&self) -> usize {
        self.projected_field_count
    }
    pub const fn total_work_units(&self) -> usize {
        self.total_work_units
    }

    /// Exact successor installation minted by this reconstructive transition.
    #[doc(hidden)]
    pub const fn successor_invalidation_installation(
        &self,
    ) -> &super::super::WorthQueryGranularInvalidationInstallation {
        &self.successor_invalidation_installation
    }
}

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema + 'static,
{
    /// Rebuilds volatile Bridge/Signal state only when the presented
    /// installation is exactly the installation already owning this runtime.
    pub fn reinstall_conditional_runtime(
        &mut self,
    ) -> Result<
        WorthQueryConditionalRuntimeReinstallationReceipt,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        let current = self.runtime.retain_installed_packages();
        self.reinstall_conditional_runtime_for_installation(current)
    }

    /// Rebuilds derived conditional state for one explicitly presented
    /// installation candidate. A changed generation or meaning cannot inherit
    /// incumbent typed bindings and therefore fails closed with RebindRequired.
    pub fn reinstall_conditional_runtime_for_installation(
        &mut self,
        candidate: Arc<WorthQueryInstalledPackageIndex>,
    ) -> Result<
        WorthQueryConditionalRuntimeReinstallationReceipt,
        WorthQueryConditionalRuntimeInstallationDenial,
    > {
        require_equivalent_installation(self.runtime.installed_packages(), &candidate)?;
        let affinity = ConditionalRuntimeAffinity::for_installation(self, &candidate);
        let mut owners = ConditionalRuntimeOwners::take(self);
        if owners.binding_count() == 0 {
            return Err(rebind(
                "conditional runtime has no installed binding inventory",
            ));
        }
        let mut successor = owners.fresh_bridge().map_err(|error| {
            WorthQueryConditionalRuntimeInstallationDenial::new(
                WorthQueryConditionalRuntimeInstallationDenialKind::BridgeRejected,
                format!("{error:?}"),
            )
        })?;
        let lower_runtime_reconstitution = successor.reconstitution_report().ok_or_else(|| {
            WorthQueryConditionalRuntimeInstallationDenial::new(
                WorthQueryConditionalRuntimeInstallationDenialKind::BridgeRejected,
                "conditional successor omitted Signal/Bridge reconstitution evidence",
            )
        })?;
        let mut prepared =
            isolate_reinstallation(|| owners.prepare_reinstallation(&mut successor, &affinity))?;
        isolate_reinstallation(|| {
            owners.reconcile_prepared_reinstallation(&mut successor, &mut prepared)
        })?;
        let reconstructed_binding_count = owners.binding_count();
        owners.commit_reinstallation(successor, prepared);
        let reconstructed_intent_count = owners.retained_resource_counts().intents;
        let work = owners.reconstruction_work();
        owners.clear_maintenance_failure();
        owners.advance_granular_invalidation_generation();
        let successor_invalidation_installation = owners.granular_invalidation_installation();
        Ok(WorthQueryConditionalRuntimeReinstallationReceipt {
            lower_runtime_reconstitution,
            reconstructed_binding_count,
            reconstructed_intent_count,
            examined_candidate_count: work.examined_candidates,
            projected_record_count: work.projected_records,
            projected_field_count: work.projected_fields,
            total_work_units: work.total_work_units,
            successor_invalidation_installation,
        })
    }
}

fn require_equivalent_installation(
    current: &WorthQueryInstalledPackageIndex,
    candidate: &WorthQueryInstalledPackageIndex,
) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
    match current.relation_to(candidate) {
        WorthQueryInstalledPackageIndexRelation::EquivalentGeneration => Ok(()),
        WorthQueryInstalledPackageIndexRelation::ExactSuccessor => Err(rebind(
            "successor installation requires fresh typed conditional bindings",
        )),
        WorthQueryInstalledPackageIndexRelation::SameGenerationMeaningChanged => Err(rebind(
            "candidate installation changed meaning within the current generation",
        )),
        WorthQueryInstalledPackageIndexRelation::ForeignRuntime => {
            Err(rebind("candidate installation belongs to another runtime"))
        }
        WorthQueryInstalledPackageIndexRelation::NonSuccessorGeneration => Err(rebind(
            "candidate installation is not the exact current or successor generation",
        )),
    }
}

fn isolate_reinstallation<Output>(
    action: impl FnOnce() -> Result<Output, WorthQueryConditionalRuntimeInstallationDenial>,
) -> Result<Output, WorthQueryConditionalRuntimeInstallationDenial> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(action)).unwrap_or_else(|_| {
        Err(WorthQueryConditionalRuntimeInstallationDenial::new(
            WorthQueryConditionalRuntimeInstallationDenialKind::ReconstructionIntent,
            "conditional runtime reinstallation callback panicked",
        ))
    })
}

fn rebind(detail: impl Into<String>) -> WorthQueryConditionalRuntimeInstallationDenial {
    WorthQueryConditionalRuntimeInstallationDenial::new(
        WorthQueryConditionalRuntimeInstallationDenialKind::RebindRequired,
        detail,
    )
}
