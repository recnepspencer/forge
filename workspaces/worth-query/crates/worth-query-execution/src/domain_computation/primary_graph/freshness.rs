use super::authenticated_principal::WorthQueryAuthenticatedPrincipal;
use super::observations::{
    observe_exact_principal_target, observe_mapping, WorthQueryPrincipalMappingObservation,
    WorthQueryPrincipalTargetObservation,
};
use super::resolution_denial::{resolution_denial, WorthQueryPrincipalResolutionDenialKind};
use super::schema_layout::WorthQueryPrimaryPrincipalBindingLayout;
use super::WorthQueryPrincipalResolutionDenial;

#[derive(Clone, Debug, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryPrincipalFreshnessEvidence {
    mapping: WorthQueryPrincipalMappingObservation,
    target: WorthQueryPrincipalTargetObservation,
}

impl WorthQueryPrincipalFreshnessEvidence {
    pub(super) fn new(
        mapping: WorthQueryPrincipalMappingObservation,
        target: WorthQueryPrincipalTargetObservation,
    ) -> Self {
        Self { mapping, target }
    }

    pub(super) fn matches(
        &self,
        mapping: &WorthQueryPrincipalMappingObservation,
        target: &WorthQueryPrincipalTargetObservation,
    ) -> bool {
        self.mapping == *mapping && self.target == *target
    }

    pub(in crate::domain_computation) fn remains_current_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        layout: &WorthQueryPrimaryPrincipalBindingLayout,
        binding: &str,
    ) -> bool {
        let Ok(mapping) =
            observe_mapping(runtime, snapshot, self.mapping.entity_id, layout, binding)
        else {
            return false;
        };
        let Ok(target) = observe_exact_principal_target(
            runtime,
            snapshot,
            self.target.relation_id,
            self.target.target,
            layout,
            binding,
        ) else {
            return false;
        };
        mapping.enabled && self.matches(&mapping, &target)
    }
}

pub(in crate::domain_computation) fn validate_freshness_at_snapshot<
    Schema,
    Principal,
    PrincipalIdentity,
>(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    layout: &WorthQueryPrimaryPrincipalBindingLayout,
    expected_identity: &worth_foundational::facade::AspectValue,
) -> Result<(), WorthQueryPrincipalResolutionDenial> {
    let mapping = observe_mapping(
        runtime,
        snapshot,
        principal.mapping_entity_id(),
        layout,
        principal.binding(),
    )?;
    let target = observe_exact_principal_target(
        runtime,
        snapshot,
        principal.target_relation_id(),
        principal.principal_entity_id(),
        layout,
        principal.binding(),
    )?;
    if !mapping.enabled
        || &mapping.identity != expected_identity
        || target.source != principal.mapping_entity_id()
        || target.target != principal.principal_entity_id()
        || !principal.freshness().matches(&mapping, &target)
    {
        return Err(resolution_denial(
            WorthQueryPrincipalResolutionDenialKind::StalePrincipalProof,
            principal.binding(),
        ));
    }
    Ok(())
}
