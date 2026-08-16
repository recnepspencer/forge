use worth_query_installation::facade::{
    ApplicationSchema, ApplicationSchemaMember, WorthQueryInstalledApplicationSchema,
};
use worth_relational::facade::bridge::RuntimeBridgeRelationalSource;
use worth_runtime_bridge::facade::{
    AspectKeySelector, BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeDeliveryReceipt,
    BridgeMappingId, BridgeMappingRegistration, BridgeOwnedSignalRuntime, BridgeRuntimePolicy,
    BridgeSourceAdapter, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeTruthViewSelector, CoarseRoutingMode, InvalidationSink, MappingSelector, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SliceWideningPolicy,
    SnapshotReadContract, SnapshotReadSource, SourceDeclaration, SourceDeclarationIdentity,
    SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope, TruthPatchTargetSelector,
    TruthSnapshotIdentity, TruthSnapshotReader,
};

use super::{
    WorthQueryPrimaryGraphInstallationDenial, WorthQueryPrimaryGraphInstallationDenialKind,
};

#[derive(Clone)]
struct WorthQueryApplicationBridgeSource {
    source: RuntimeBridgeRelationalSource,
}

struct WorthQueryApplicationInvalidationSink;

struct WorthQueryApplicationFieldMappings {
    routing: BridgeMappingRegistration,
    aspect: BridgeAspectRegistration,
}

pub(super) struct WorthQueryInstalledApplicationBridge {
    ordinary: RuntimeBridge,
    conditional: std::sync::Mutex<Option<BridgeOwnedSignalRuntime>>,
}

impl WorthQueryInstalledApplicationBridge {
    pub(super) fn ordinary(&self) -> &RuntimeBridge {
        &self.ordinary
    }

    pub(super) fn conditional_mut(&mut self) -> &mut BridgeOwnedSignalRuntime {
        self.conditional
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_mut()
            .expect("exclusive conditional runtime access cannot overlap")
    }

    pub(super) fn take_conditional(&mut self) -> BridgeOwnedSignalRuntime {
        self.conditional
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
            .expect("exclusive conditional runtime access cannot overlap")
    }

    pub(super) fn take_conditional_if_present(&self) -> Option<BridgeOwnedSignalRuntime> {
        self.conditional
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take()
    }

    pub(super) fn restore_conditional(&mut self, conditional: BridgeOwnedSignalRuntime) {
        let slot = self
            .conditional
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(slot.replace(conditional).is_none());
    }

    pub(super) fn restore_conditional_shared(&self, conditional: BridgeOwnedSignalRuntime) {
        let mut slot = self
            .conditional
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(slot.replace(conditional).is_none());
    }

    pub(super) fn fresh_conditional_runtime(
        &self,
    ) -> Result<BridgeOwnedSignalRuntime, worth_runtime_bridge::facade::BridgeConditionalDenial>
    {
        BridgeOwnedSignalRuntime::with_owned_signal_graph(self.ordinary.clone())
    }
}

pub(super) fn install_application_bridge<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    layout: &super::schema_layout::WorthQueryPrimaryGraphLayout,
    source: RuntimeBridgeRelationalSource,
) -> Result<WorthQueryInstalledApplicationBridge, WorthQueryPrimaryGraphInstallationDenial>
where
    Schema: ApplicationSchema,
{
    let mut mappings = application_mappings(schema, layout)?;
    let first = mappings.next().ok_or_else(|| {
        bridge_denial("installed application schema has no bridge-readable fields")
    })?;
    let builder = RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::operational())
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source.clone())
        .with_source_adapter(WorthQueryApplicationBridgeSource { source })
        .with_signal_sink(WorthQueryApplicationInvalidationSink)
        .register_source(SourceDeclaration::new(
            SourceDeclarationIdentity::from_stable_name("primary-application-source"),
            BridgeTruthViewSelector::branch_head(
                super::application_branch::primary_truth_branch_identity(),
            ),
            BridgeSourceCapabilitySet::new(vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::BranchRead,
            ]),
        ))
        .register_aspect_mapping(first.aspect)
        .register_mapping(first.routing);
    let ordinary = mappings
        .fold(builder, |builder, mapping| {
            builder
                .register_aspect_mapping(mapping.aspect)
                .register_mapping(mapping.routing)
        })
        .build()
        .map_err(|error| bridge_denial(format!("{error:?}")))?;
    let conditional = BridgeOwnedSignalRuntime::with_owned_signal_graph(ordinary.clone())
        .map_err(|error| bridge_denial(format!("{error:?}")))?;
    Ok(WorthQueryInstalledApplicationBridge {
        ordinary,
        conditional: std::sync::Mutex::new(Some(conditional)),
    })
}

fn application_mappings<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    layout: &super::schema_layout::WorthQueryPrimaryGraphLayout,
) -> Result<
    impl Iterator<Item = WorthQueryApplicationFieldMappings>,
    WorthQueryPrimaryGraphInstallationDenial,
>
where
    Schema: ApplicationSchema,
{
    schema
        .installed_declaration()
        .members()
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Field {
                entity,
                aspect,
                field,
                scalar_family,
                ..
            } => Some((entity, aspect, field, scalar_family)),
            _ => None,
        })
        .map(|(entity, aspect, field, _scalar_family)| {
            let aspect_key = worth_foundational::facade::AspectKey::new(aspect)
                .ok_or_else(|| bridge_denial(aspect))?;
            let contract = layout
                .aspect_contract(entity, &aspect_key)
                .cloned()
                .ok_or_else(|| bridge_denial(aspect))?;
            let field_key = worth_foundational::facade::FieldKey::new(field)
                .ok_or_else(|| bridge_denial(field))?;
            let identity = format!(
                "application-field:{}:{}:{entity}:{aspect}:{field}",
                schema.owner(),
                schema.schema_name(),
            );
            let snapshot = SnapshotReadContract::new(contract);
            let routing = BridgeMappingRegistration::new(
                BridgeMappingId::from_stable_name(identity),
                TruthPatchScope::for_entity_field(
                    MappingSelector::exact(entity.as_str()),
                    aspect_key.clone(),
                    field_key,
                ),
                snapshot.clone(),
                SignalInvalidationScope::from_stable_name(format!(
                    "application-field-signal:{entity}:{aspect}:{field}"
                )),
                CoarseRoutingMode::Direct,
            );
            let aspect = BridgeAspectRegistration::new(
                BridgeAspectRegistrationId::from_stable_name(format!(
                    "application-field:{entity}:{aspect}:{field}"
                )),
                TruthPatchScope::new(
                    MappingSelector::any(),
                    AspectKeySelector::exact(aspect_key),
                    TruthPatchTargetSelector::entity_field(
                        worth_foundational::facade::FieldKey::new(field)
                            .expect("installed schema field keys are validated"),
                    ),
                ),
                snapshot,
                TruthDeltaSurfaceKind::EntityField,
                SubscriptionSliceKind::RegisteredCoarseWidening,
                SliceWideningPolicy::RegisteredEntityCoarseWidening,
            );
            Ok(WorthQueryApplicationFieldMappings { routing, aspect })
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Vec::into_iter)
}

impl BridgeSourceAdapter for WorthQueryApplicationBridgeSource {
    fn declared_capabilities(&self) -> BridgeSourceCapabilitySet {
        BridgeSourceCapabilitySet::new(vec![
            BridgeSourceCapability::SnapshotRead,
            BridgeSourceCapability::BranchRead,
        ])
    }

    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn TruthSnapshotReader>,
        worth_runtime_bridge::facade::RelationalBridgeSourceError,
    > {
        SnapshotReadSource::open_snapshot(&self.source, identity)
    }
}

impl InvalidationSink for WorthQueryApplicationInvalidationSink {
    fn deliver_invalidation(
        &self,
        delivery: worth_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn bridge_denial(subject: impl Into<String>) -> WorthQueryPrimaryGraphInstallationDenial {
    WorthQueryPrimaryGraphInstallationDenial::new(
        WorthQueryPrimaryGraphInstallationDenialKind::RuntimeBridgeRejected,
        subject,
    )
}
