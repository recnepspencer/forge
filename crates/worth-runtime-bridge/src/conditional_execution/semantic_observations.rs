use super::{
    BridgeConditionalCondition, BridgeConditionalDenial, BridgeConditionalDenialKind,
    BridgeConditionalSemanticObservation, BridgeInstalledConditionalLowering,
};

type TruthSnapshotContext =
    crate::snapshot::AdmittedSnapshotContext<Box<dyn crate::snapshot::TruthSnapshotReader>>;

pub(super) fn read_condition_observations(
    snapshot: Option<&TruthSnapshotContext>,
    lowering: &BridgeInstalledConditionalLowering,
    previous: &std::collections::BTreeMap<
        (worth_signal::facade::NodeId, usize),
        worth_foundational::facade::ContractValidatedAspectArtifact,
    >,
) -> Result<Vec<BridgeConditionalSemanticObservation>, BridgeConditionalDenial> {
    let condition = lowering.contract.condition();
    let Some(snapshot) = admit_observation_snapshot(condition, snapshot)? else {
        return Ok(Vec::new());
    };
    let plan = lowering.semantic_observation_plan.as_ref().ok_or_else(|| {
        BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::SnapshotAdmission,
            "installed semantic condition lost its compiled observation plan",
        )
    })?;
    let result = snapshot.read_packet(plan.packet()).map_err(|error| {
        BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::SnapshotAdmission,
            format!("conditional semantic observation failed: {error}"),
        )
    })?;
    let validated = crate::snapshot::validate_snapshot_read_result_contract(plan.packet(), result)
        .map_err(|error| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SnapshotAdmission,
                format!("conditional semantic observation violated its contract: {error}"),
            )
        })?;
    Ok(plan
        .ordinals()
        .iter()
        .copied()
        .zip(validated.records().iter())
        .map(|(ordinal, record)| {
            BridgeConditionalSemanticObservation::new(
                ordinal,
                previous.get(&(lowering.signal_node(), ordinal)).cloned(),
                record.validated_value().clone(),
            )
        })
        .collect())
}

fn admit_observation_snapshot<'a>(
    condition: &BridgeConditionalCondition,
    snapshot: Option<&'a TruthSnapshotContext>,
) -> Result<Option<&'a TruthSnapshotContext>, BridgeConditionalDenial> {
    if !matches!(
        condition,
        BridgeConditionalCondition::DeltaThreshold(_)
            | BridgeConditionalCondition::RuntimePredicate
    ) {
        return Ok(None);
    }
    if snapshot.is_none() && matches!(condition, BridgeConditionalCondition::DeltaThreshold(_)) {
        return Err(BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::SnapshotAdmission,
            "a typed delta threshold requires an admitted truth snapshot",
        ));
    }
    Ok(snapshot)
}
