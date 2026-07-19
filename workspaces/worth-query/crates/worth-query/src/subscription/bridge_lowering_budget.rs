use std::collections::BTreeSet;

use super::bridge_family::BridgeSubscriptionDeclarationFamilyKind;
use super::bridge_slice::BridgeSubscriptionSliceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionBridgeFallbackPosture {
    DeniedByDefault,
    CertifiedFallbackAdmitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionBridgeLoweringBudget {
    bridge_family_registry_lookup_limit: usize,
    bridge_slice_registry_lookup_limit: usize,
    bridge_declaration_input_width_limit: usize,
    basis_request_width_limit: usize,
    signal_strategy_request_width_limit: usize,
    admitted_bridge_families: BTreeSet<BridgeSubscriptionDeclarationFamilyKind>,
    admitted_bridge_slices: BTreeSet<BridgeSubscriptionSliceKind>,
    bridge_fallback_posture: QuerySubscriptionBridgeFallbackPosture,
    historical_basis_support: bool,
    preview_basis_support: bool,
}

impl QuerySubscriptionBridgeLoweringBudget {
    pub fn admitted(
        bridge_family_registry_lookup_limit: usize,
        bridge_slice_registry_lookup_limit: usize,
        bridge_declaration_input_width_limit: usize,
        basis_request_width_limit: usize,
        signal_strategy_request_width_limit: usize,
    ) -> Self {
        Self {
            bridge_family_registry_lookup_limit,
            bridge_slice_registry_lookup_limit,
            bridge_declaration_input_width_limit,
            basis_request_width_limit,
            signal_strategy_request_width_limit,
            admitted_bridge_families: [
                BridgeSubscriptionDeclarationFamilyKind::DetailExact,
                BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            ]
            .into_iter()
            .collect(),
            admitted_bridge_slices: [
                BridgeSubscriptionSliceKind::ProjectedField,
                BridgeSubscriptionSliceKind::Membership,
                BridgeSubscriptionSliceKind::Ordering,
                BridgeSubscriptionSliceKind::Grouping,
                BridgeSubscriptionSliceKind::RelationScope,
                BridgeSubscriptionSliceKind::ViewMetadata,
            ]
            .into_iter()
            .collect(),
            bridge_fallback_posture: QuerySubscriptionBridgeFallbackPosture::DeniedByDefault,
            historical_basis_support: true,
            preview_basis_support: false,
        }
    }

    pub fn bridge_family_registry_lookup_limit(&self) -> usize {
        self.bridge_family_registry_lookup_limit
    }

    pub fn bridge_slice_registry_lookup_limit(&self) -> usize {
        self.bridge_slice_registry_lookup_limit
    }

    pub fn bridge_declaration_input_width_limit(&self) -> usize {
        self.bridge_declaration_input_width_limit
    }

    pub fn basis_request_width_limit(&self) -> usize {
        self.basis_request_width_limit
    }

    pub fn signal_strategy_request_width_limit(&self) -> usize {
        self.signal_strategy_request_width_limit
    }

    pub(super) fn admits_bridge_family(
        &self,
        family: &BridgeSubscriptionDeclarationFamilyKind,
    ) -> bool {
        self.admitted_bridge_families.contains(family)
    }

    pub(super) fn admits_bridge_slices(&self, slices: &[BridgeSubscriptionSliceKind]) -> bool {
        slices
            .iter()
            .all(|slice| self.admitted_bridge_slices.contains(slice))
    }

    pub(super) fn bridge_fallback_posture(&self) -> &QuerySubscriptionBridgeFallbackPosture {
        &self.bridge_fallback_posture
    }

    pub(super) fn historical_basis_support(&self) -> bool {
        self.historical_basis_support
    }

    pub(super) fn preview_basis_support(&self) -> bool {
        self.preview_basis_support
    }

    #[cfg(test)]
    pub(crate) fn without_bridge_family_support(mut self) -> Self {
        self.admitted_bridge_families.clear();
        self
    }

    #[cfg(test)]
    pub(crate) fn without_bridge_slice_support(
        mut self,
        slice: BridgeSubscriptionSliceKind,
    ) -> Self {
        self.admitted_bridge_slices.remove(&slice);
        self
    }

    #[cfg(test)]
    pub(crate) fn with_bridge_fallback_support(mut self) -> Self {
        self.bridge_fallback_posture =
            QuerySubscriptionBridgeFallbackPosture::CertifiedFallbackAdmitted;
        self
    }

    #[cfg(test)]
    pub(crate) fn without_historical_basis_support(mut self) -> Self {
        self.historical_basis_support = false;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_preview_basis_support(mut self) -> Self {
        self.preview_basis_support = true;
        self
    }
}
