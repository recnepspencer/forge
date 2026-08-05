use super::{BridgeConditionalDenial, BridgeConditionalDenialKind};

pub(super) struct BridgeConditionalSemanticObservationPlan {
    ordinals: Vec<usize>,
    packet: crate::snapshot::SnapshotReadPacket,
}

impl BridgeConditionalSemanticObservationPlan {
    pub(super) fn ordinals(&self) -> &[usize] {
        &self.ordinals
    }

    pub(super) fn packet(&self) -> &crate::snapshot::SnapshotReadPacket {
        &self.packet
    }
}

pub(super) fn compile_semantic_observation_plan(
    contract: &super::BridgeConditionalContract,
    registrations: &[crate::correspondence::BridgeSemanticCorrespondenceRegistration],
) -> Result<Option<BridgeConditionalSemanticObservationPlan>, BridgeConditionalDenial> {
    if !matches!(
        contract.condition(),
        super::BridgeConditionalCondition::DeltaThreshold(_)
            | super::BridgeConditionalCondition::RuntimePredicate
    ) {
        return Ok(None);
    }
    let condition_dependencies = contract.condition_dependency_ordinals();
    let mut ordinals = Vec::new();
    let mut reads = Vec::new();
    for registration in registrations.iter().filter(|registration| {
        condition_dependencies.is_empty()
            || condition_dependencies
                .iter()
                .any(|ordinal| registration.dependency().dependency_ordinal() == *ordinal)
    }) {
        let dependency = registration.dependency();
        let record = dependency.source_record_identity.ok_or_else(|| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SnapshotAdmission,
                "semantic condition observations require an exact source-record dependency",
            )
        })?;
        ordinals.push(dependency.dependency_ordinal());
        reads.push(
            crate::snapshot::SnapshotReadRequest::from_native_subscription_slice_relational_record(
                record.bridge_entity_identity(),
                record,
                crate::snapshot::SnapshotReadContract::new(dependency.contract().clone()),
                worth_foundational::facade::AspectLocator::new(
                    worth_foundational::facade::LocatorAuthority::Authoritative,
                    dependency.contract().key().clone(),
                ),
                None,
                dependency.projection_mask().clone(),
                crate::mapping::SubscriptionSliceKind::SignalField,
            ),
        );
    }
    if reads.is_empty() {
        return Err(BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::SnapshotAdmission,
            "semantic condition lowering produced no authoritative observation reads",
        ));
    }
    Ok(Some(BridgeConditionalSemanticObservationPlan {
        ordinals,
        packet: crate::snapshot::SnapshotReadPacket::new(reads),
    }))
}
