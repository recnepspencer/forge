use worth_query_installation::facade::{
    ApplicationSchema, ApplicationSchemaMember, WorthQueryInstalledApplicationSchema,
};
use worth_relational::facade::bridge::RuntimeBridgeRelationalSource;
use worth_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration, BridgeRuntimePolicy,
    BridgeSourceAdapter, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeTruthViewSelector, CoarseRoutingMode, InvalidationSink, MappingSelector, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SnapshotReadContract,
    SnapshotReadSource, SourceDeclaration, SourceDeclarationIdentity, TruthPatchScope,
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

pub(super) fn install_application_bridge<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    source: RuntimeBridgeRelationalSource,
) -> Result<RuntimeBridge, WorthQueryPrimaryGraphInstallationDenial>
where
    Schema: ApplicationSchema,
{
    let mut mappings = application_mappings(schema)?;
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
        .register_mapping(first);
    mappings
        .fold(builder, |builder, mapping| {
            builder.register_mapping(mapping)
        })
        .build()
        .map_err(|error| bridge_denial(format!("{error:?}")))
}

fn application_mappings<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
) -> Result<impl Iterator<Item = BridgeMappingRegistration>, WorthQueryPrimaryGraphInstallationDenial>
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
        .map(|(entity, aspect, field, scalar_family)| {
            let aspect_key = worth_foundational::facade::AspectKey::new(aspect)
                .ok_or_else(|| bridge_denial(aspect))?;
            let field_key = worth_foundational::facade::FieldKey::new(field)
                .ok_or_else(|| bridge_denial(field))?;
            let identity = format!(
                "application-field:{}:{}:{entity}:{aspect}:{field}",
                schema.owner(),
                schema.schema_name(),
            );
            Ok(BridgeMappingRegistration::new(
                BridgeMappingId::from_stable_name(identity),
                TruthPatchScope::for_entity_field(
                    MappingSelector::exact(entity.as_str()),
                    aspect_key.clone(),
                    field_key,
                ),
                SnapshotReadContract::scalar(aspect_key, *scalar_family),
                SignalInvalidationScope::from_stable_name(format!(
                    "application-field-signal:{entity}:{aspect}:{field}"
                )),
                CoarseRoutingMode::Direct,
            ))
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
