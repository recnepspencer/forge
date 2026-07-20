use crate::error::BridgeBuildError;
use crate::mapping::SubscriptionSliceKind;
use std::collections::BTreeMap;

use super::ids::BridgeAspectRegistrationId;
use super::registration::BridgeAspectRegistration;
use super::types::{SliceWideningPolicy, TruthDeltaSurfaceKind};
use super::validation::{
    canonical_aspect_registration_order, validate_registration_set, validate_registration_values,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenAspectRegistration {
    registration: BridgeAspectRegistration,
    identity_basis: std::sync::Arc<str>,
}

impl FrozenAspectRegistration {
    fn new(registration: BridgeAspectRegistration) -> Self {
        let identity_basis = std::sync::Arc::from(format!(
            "aspect-registration|id={}|scope={}|snapshot={}|surface={}|slice={}|widening={}|source-precision={}",
            registration.registration_id().as_str(),
            truth_scope_basis(registration.truth_scope()),
            registration.snapshot_read_contract().canonical_basis(),
            surface_name(registration.truth_surface_kind()),
            slice_name(registration.subscription_slice_kind()),
            widening_name(registration.widening_policy()),
            source_precision_name(registration.source_precision_policy()),
        ));
        Self {
            registration,
            identity_basis,
        }
    }

    pub(crate) fn registration_id(&self) -> &BridgeAspectRegistrationId {
        self.registration.registration_id()
    }

    pub(crate) fn truth_surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.registration.truth_surface_kind()
    }

    pub(crate) fn truth_scope(&self) -> &crate::mapping::TruthPatchScope {
        self.registration.truth_scope()
    }

    pub(crate) fn snapshot_read_contract(&self) -> &crate::snapshot::SnapshotReadContract {
        self.registration.snapshot_read_contract()
    }

    pub(crate) fn subscription_slice_kind(&self) -> &SubscriptionSliceKind {
        self.registration.subscription_slice_kind()
    }

    pub(crate) fn widening_policy(&self) -> SliceWideningPolicy {
        self.registration.widening_policy()
    }

    pub(crate) fn source_precision_policy(
        &self,
    ) -> super::BridgeAuthoritativeSourcePrecisionPolicy {
        self.registration.source_precision_policy()
    }

    pub(crate) fn identity_basis(&self) -> &std::sync::Arc<str> {
        &self.identity_basis
    }
}

fn truth_scope_basis(scope: &crate::mapping::TruthPatchScope) -> String {
    use crate::mapping::{AspectKeySelector, MappingSelector};
    let entity = match scope.entity_selector() {
        MappingSelector::Any => "any".to_string(),
        MappingSelector::Exact(value) => format!("exact#{}:{value}", value.len()),
    };
    let aspect = match scope.aspect_selector() {
        AspectKeySelector::Any => "any".to_string(),
        AspectKeySelector::Exact(value) => {
            format!("exact#{}:{}", value.as_str().len(), value.as_str())
        }
    };
    format!(
        "entity={entity}|aspect={aspect}|target={}",
        scope.target_selector().canonical_basis()
    )
}

fn surface_name(surface: TruthDeltaSurfaceKind) -> &'static str {
    match surface {
        TruthDeltaSurfaceKind::AuthoritativeAspect => "authoritative-aspect",
        TruthDeltaSurfaceKind::EntityField => "entity-field",
        TruthDeltaSurfaceKind::EntityRelationEndpoint => "entity-relation-endpoint",
        TruthDeltaSurfaceKind::EntityRegion => "entity-region",
        TruthDeltaSurfaceKind::EntityPartition => "entity-partition",
        TruthDeltaSurfaceKind::EntityFacet => "entity-facet",
        TruthDeltaSurfaceKind::LifecycleTransition => "lifecycle-transition",
    }
}

fn slice_name(slice: &SubscriptionSliceKind) -> &'static str {
    match slice {
        SubscriptionSliceKind::SignalAspect => "signal-aspect",
        SubscriptionSliceKind::SignalField => "signal-field",
        SubscriptionSliceKind::SignalLens => "signal-lens",
        SubscriptionSliceKind::SignalRegion => "signal-region",
        SubscriptionSliceKind::SignalPartition => "signal-partition",
        SubscriptionSliceKind::SignalFacet => "signal-facet",
        SubscriptionSliceKind::SignalLifecycle => "signal-lifecycle",
        SubscriptionSliceKind::RegisteredCoarseWidening => "registered-coarse-widening",
    }
}

fn widening_name(widening: SliceWideningPolicy) -> &'static str {
    match widening {
        SliceWideningPolicy::Disallow => "disallow",
        SliceWideningPolicy::RegisteredEntityCoarseWidening => "entity-coarse",
        SliceWideningPolicy::RegisteredAspectCoarseWidening => "aspect-coarse",
        SliceWideningPolicy::RegisteredSurfaceCoarseWidening => "surface-coarse",
        SliceWideningPolicy::RegisteredPartitionWidening => "partition",
    }
}

fn source_precision_name(policy: super::BridgeAuthoritativeSourcePrecisionPolicy) -> &'static str {
    use super::BridgeAuthoritativeSourcePrecisionPolicy as Policy;
    use crate::input::envelope::BridgeAspectChangeWideningCause as Cause;
    match policy {
        Policy::ExactOnly => "exact-only",
        Policy::AdmitDeclared(Cause::FieldToWholeAspect) => "field-to-whole-aspect",
        Policy::AdmitDeclared(Cause::AspectToEntity) => "aspect-to-entity",
        Policy::AdmitDeclared(Cause::SurfaceBroadening) => "surface-broadening",
        Policy::AdmitDeclared(Cause::OpaquePayloadToWholeAspect) => {
            "opaque-payload-to-whole-aspect"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FrozenAspectMappingRegistry {
    registrations: Vec<FrozenAspectRegistration>,
    by_id: BTreeMap<BridgeAspectRegistrationId, usize>,
}

impl FrozenAspectMappingRegistry {
    pub(crate) fn freeze(
        mut registrations: Vec<BridgeAspectRegistration>,
    ) -> Result<Self, BridgeBuildError> {
        registrations.sort_by(canonical_aspect_registration_order);
        validate_registration_values(&registrations)?;
        validate_registration_set(&registrations)?;

        let registrations = registrations
            .into_iter()
            .map(FrozenAspectRegistration::new)
            .collect::<Vec<_>>();
        let by_id = registrations
            .iter()
            .enumerate()
            .map(|(index, registration)| (registration.registration_id().clone(), index))
            .collect();
        Ok(Self {
            registrations,
            by_id,
        })
    }

    pub(crate) fn registrations(&self) -> &[FrozenAspectRegistration] {
        &self.registrations
    }

    pub(crate) fn by_id(
        &self,
        id: &BridgeAspectRegistrationId,
    ) -> Option<&FrozenAspectRegistration> {
        self.by_id.get(id).map(|index| &self.registrations[*index])
    }

    pub(crate) fn rebuilt_id_index_has_exact_parity(&self) -> bool {
        self.registrations
            .iter()
            .enumerate()
            .map(|(index, registration)| (registration.registration_id().clone(), index))
            .collect::<BTreeMap<_, _>>()
            == self.by_id
    }
}
