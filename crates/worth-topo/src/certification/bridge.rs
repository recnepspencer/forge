use std::collections::BTreeMap;

use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeSignalInvalidationDelivery, BridgeTruthViewEvaluationRequest,
    InvalidationSink, SignalBridgeSinkError, TruthBranchIdentity, TruthCommitIdentity,
};

use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::shared::{
    canonical_milestone_one_primitive_families, digest_rows, primitive_family_name,
};
use crate::certification::support::reporting::{
    BridgeFamilyCoverageReport, BridgeFamilyCoverageRow, BridgeProofReport,
};
use crate::certification::BridgeTraceAnchor;
use crate::projection::runtime_boundary::bridge::build_milestone_one_bridge;
use crate::test_support::primitive_corpus::bridge_cases::milestone_one_bridge_proof_cases;
use crate::test_support::primitive_corpus::validated_topology::committed_primitive_input;

use std::sync::Arc;

#[derive(Clone)]
struct BridgeCertificationSink;

impl InvalidationSink for BridgeCertificationSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

pub(crate) fn build_bridge_family_coverage_report(
    family_rows: &[(String, usize, usize)],
) -> BridgeFamilyCoverageReport {
    let family_map = family_rows
        .iter()
        .map(|(family, routed_case_count, historical_evaluation_count)| {
            (
                family.as_str(),
                (*routed_case_count, *historical_evaluation_count),
            )
        })
        .collect::<BTreeMap<_, _>>();

    BridgeFamilyCoverageReport {
        rows: canonical_milestone_one_primitive_families()
            .into_iter()
            .map(|family| BridgeFamilyCoverageRow {
                family: family.to_string(),
                routed_case_count: family_map.get(family).map(|row| row.0).unwrap_or(0),
                historical_evaluation_count: family_map.get(family).map(|row| row.1).unwrap_or(0),
                proof_complete: family_map
                    .get(family)
                    .map(|row| row.0 > 0 && row.1 > 0)
                    .unwrap_or(false),
            })
            .collect(),
    }
}

pub(crate) fn certify_milestone_one_bridge_proof(
    stem: &str,
) -> Result<BridgeProofReport, MilestoneOneCertificationError> {
    let proof_cases = milestone_one_bridge_proof_cases();
    let mut proved_families = Vec::with_capacity(proof_cases.len());
    let mut source_branch = None;
    let mut source_commit = None;
    let mut source_snapshot = None;
    let mut route_rows = Vec::new();
    let mut historical_rows = Vec::new();
    let mut route_identities = Vec::new();
    let mut invalidation_identities = Vec::new();
    let mut snapshot_identities = Vec::new();
    let mut historical_record_identities = Vec::new();
    let mut family_rows = Vec::with_capacity(proof_cases.len());

    for (index, primitive) in proof_cases.iter().enumerate() {
        let mut runtime = crate::validation::reference_integrity::milestone_one_runtime_builder()
            .map_err(|error| {
                MilestoneOneCertificationError::ReadView(format!(
                    " milestone one bridge proof could not build runtime builder: {error:?}"
                ))
            })?
            .build();
        let commit_input =
            committed_primitive_input(&mut runtime, &format!("{stem}.case.{index}"), primitive)?;
        let commit = commit_input.commits().last().ok_or_else(|| {
            MilestoneOneCertificationError::ReadView(
                " milestone one bridge proof requires a committed topology mutation".to_string(),
            )
        })?;
        let family = primitive_family_name(primitive).to_string();
        let branch_id = commit_input.branch_id().0.clone();
        let commit_id = commit.outcome.commit.commit_id.0;
        let bridge_runtime = Arc::new(runtime);
        let bridge =
            build_milestone_one_bridge(Arc::clone(&bridge_runtime), BridgeCertificationSink)
                .map_err(|error| {
                    MilestoneOneCertificationError::ReadView(format!(
                        " milestone one bridge proof could not build bridge: {error:?}"
                    ))
                })?;
        let _route = bridge
            .route(TruthCommitIdentity::from_relational_commit_id(commit_id))
            .map_err(|error| {
                MilestoneOneCertificationError::ReadView(format!(
                    " milestone one bridge proof could not route committed truth: {error:?}"
                ))
            })?;
        let evaluation = bridge
            .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
                TruthBranchIdentity::from_relational_branch_id(branch_id.clone()),
            ))
            .map_err(|error| {
                MilestoneOneCertificationError::ReadView(format!(
                    " milestone one bridge proof could not evaluate branch head: {error:?}"
                ))
            })?;
        let route_records = bridge.diagnostics().route_records();
        let historical_records = bridge.diagnostics().historical_evaluation_records();

        family_rows.push((
            family.clone(),
            route_records.len(),
            historical_records.len(),
        ));
        proved_families.push(family);
        source_branch = Some(branch_id);
        source_commit = Some(commit_id.to_string());
        source_snapshot = Some(
            evaluation
                .snapshot_identity()
                .evidence_identity()
                .as_str()
                .to_string(),
        );

        route_rows.extend(route_records.iter().map(|record| {
            let route_identity = record.route_identity().evidence_identity();
            let invalidation_identity = record.invalidation_identity().evidence_identity();
            let source_snapshot_identity = record.source_snapshot().evidence_identity();
            let source_branch_identity = record.source_branch().evidence_identity();
            let source_commit_identity = record.source_commit().evidence_identity();
            route_identities.push(route_identity.as_str().to_string());
            invalidation_identities.push(invalidation_identity.as_str().to_string());
            snapshot_identities.push(source_snapshot_identity.as_str().to_string());
            format!(
                "route:{}:{}:{}:{}:{}",
                route_identity.as_str(),
                source_branch_identity.as_str(),
                source_commit_identity.as_str(),
                source_snapshot_identity.as_str(),
                record.invalidation_targets().len()
            )
        }));
        historical_rows.extend(historical_records.iter().map(|record| {
            let record_identity = record.record_identity().evidence_identity();
            let branch_identity = record.decision_log().branch_identity().evidence_identity();
            let commit_identity = record
                .decision_log()
                .commit_identity()
                .map(|identity| identity.evidence_identity());
            let snapshot_identity = record
                .decision_log()
                .snapshot_identity()
                .evidence_identity();
            historical_record_identities.push(record_identity.as_str().to_string());
            snapshot_identities.push(snapshot_identity.as_str().to_string());
            format!(
                "historical:{}:{}:{}:{}:{:?}",
                record_identity.as_str(),
                branch_identity.as_str(),
                commit_identity
                    .as_ref()
                    .map(|identity| identity.as_str())
                    .unwrap_or("none"),
                snapshot_identity.as_str(),
                record.decision_log().materialization_path()
            )
        }));
    }
    let bridge_routing_digest = digest_rows(route_rows.into_iter());
    let bridge_historical_evaluation_digest = digest_rows(historical_rows.into_iter());
    let route_record_count = family_rows.iter().map(|row| row.1).sum::<usize>();
    let historical_evaluation_record_count = family_rows.iter().map(|row| row.2).sum::<usize>();
    let family_coverage_report = build_bridge_family_coverage_report(&family_rows);

    Ok(BridgeProofReport {
        proof_case_count: proof_cases.len(),
        proved_families,
        family_coverage_report,
        bridge_trace_anchor: BridgeTraceAnchor::new(
            route_identities,
            invalidation_identities,
            snapshot_identities,
            historical_record_identities,
        ),
        bridge_routing_digest,
        bridge_historical_evaluation_digest,
        route_record_count,
        historical_evaluation_record_count,
        source_branch: source_branch.unwrap_or_else(|| "main".to_string()),
        source_commit: source_commit.unwrap_or_default(),
        source_snapshot: source_snapshot.unwrap_or_default(),
    })
}
