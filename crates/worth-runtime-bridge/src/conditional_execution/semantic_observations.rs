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
        (
            worth_signal::facade::NodeId,
            usize,
            Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
        ),
        worth_foundational::facade::ContractValidatedAspectArtifact,
    >,
    managed_source_record: Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
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
    let packet = plan.packet(managed_source_record)?;
    let result = snapshot.read_packet(&packet).map_err(|error| {
        BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::SnapshotAdmission,
            format!("conditional semantic observation failed: {error}"),
        )
    })?;
    let validated = crate::snapshot::validate_snapshot_read_result_contract(&packet, result)
        .map_err(|error| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SnapshotAdmission,
                format!("conditional semantic observation violated its contract: {error}"),
            )
        })?;
    Ok(assemble_observations(
        plan,
        lowering.signal_node(),
        previous,
        managed_source_record,
        validated
            .records()
            .iter()
            .map(|record| record.validated_value_posture().cloned()),
    ))
}

fn assemble_observations(
    plan: &super::semantic_observation_plan::BridgeConditionalSemanticObservationPlan,
    signal_node: worth_signal::facade::NodeId,
    previous: &std::collections::BTreeMap<
        (
            worth_signal::facade::NodeId,
            usize,
            Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
        ),
        worth_foundational::facade::ContractValidatedAspectArtifact,
    >,
    managed_source_record: Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
    current: impl IntoIterator<
        Item = Option<worth_foundational::facade::ContractValidatedAspectArtifact>,
    >,
) -> Vec<BridgeConditionalSemanticObservation> {
    let mut current_records = current.into_iter();
    plan.ordinals()
        .map(|ordinal| {
            let current = current_records
                .next()
                .expect("validated packet retains every requested observation posture");
            BridgeConditionalSemanticObservation::new(
                ordinal,
                previous
                    .get(&(
                        signal_node,
                        ordinal,
                        plan.baseline_record(ordinal, managed_source_record),
                    ))
                    .cloned(),
                current,
                plan.projection_mask(ordinal)
                    .expect("compiled observation ordinal retains its projection mask")
                    .clone(),
            )
        })
        .collect()
}

fn admit_observation_snapshot<'a>(
    condition: &BridgeConditionalCondition,
    snapshot: Option<&'a TruthSnapshotContext>,
) -> Result<Option<&'a TruthSnapshotContext>, BridgeConditionalDenial> {
    if !matches!(
        condition,
        BridgeConditionalCondition::DeltaThreshold(_)
            | BridgeConditionalCondition::RuntimePredicate
            | BridgeConditionalCondition::TemporalWake
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authoritative_clear_materializes_an_explicit_absent_current_observation() {
        let plan = super::super::semantic_observation_plan::BridgeConditionalSemanticObservationPlan::managed_test_plan();
        let record =
            crate::relational_identity::RelationalBridgeRecordIdentityParts::entity(1, 7, 2);
        let signal_node = worth_signal::facade::SignalGraph::new().node().build();
        let observations = assemble_observations(
            &plan,
            signal_node,
            &Default::default(),
            Some(record),
            [None],
        );

        assert_eq!(observations.len(), 1);
        assert!(observations[0].previous().is_none());
        assert!(observations[0].current().is_none());
    }
}
