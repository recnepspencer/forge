use worth_foundational::facade::{FoundationalPerformanceCounterName, RetentionDeliveryProfile};

use crate::domain_computation_artifact_fixture::{
    active_compatibility, base_builder, CandidateArtifactFamily, CandidateComparatorFamily,
};
use crate::facade::*;

#[test]
fn evidence_schema_drift_conflicts_atomically_for_every_required_dimension() {
    let baseline = contract(
        domain_counter(
            WorthQueryStructuralCounterUnit::Comparisons,
            WorthQueryStructuralCounterAggregation::Independent,
            WorthQueryStructuralCounterRequiredness::OptionalSidecar,
            WorthQueryStructuralCounterReplayPosture::NotCompared,
        ),
        RetentionDeliveryProfile::Retained,
    );
    let drifted = [
        contract(
            domain_counter(
                WorthQueryStructuralCounterUnit::Iterations,
                WorthQueryStructuralCounterAggregation::Independent,
                WorthQueryStructuralCounterRequiredness::OptionalSidecar,
                WorthQueryStructuralCounterReplayPosture::NotCompared,
            ),
            RetentionDeliveryProfile::Retained,
        ),
        contract(
            domain_counter(
                WorthQueryStructuralCounterUnit::Comparisons,
                WorthQueryStructuralCounterAggregation::SumOf(vec![counter("elements")]),
                WorthQueryStructuralCounterRequiredness::OptionalSidecar,
                WorthQueryStructuralCounterReplayPosture::NotCompared,
            ),
            RetentionDeliveryProfile::Retained,
        ),
        contract(
            domain_counter(
                WorthQueryStructuralCounterUnit::Comparisons,
                WorthQueryStructuralCounterAggregation::Independent,
                WorthQueryStructuralCounterRequiredness::RequiredCore,
                WorthQueryStructuralCounterReplayPosture::NotCompared,
            ),
            RetentionDeliveryProfile::Retained,
        ),
        contract(
            domain_counter(
                WorthQueryStructuralCounterUnit::Comparisons,
                WorthQueryStructuralCounterAggregation::Independent,
                WorthQueryStructuralCounterRequiredness::OptionalSidecar,
                WorthQueryStructuralCounterReplayPosture::Exact,
            ),
            RetentionDeliveryProfile::Retained,
        ),
        contract(
            domain_counter(
                WorthQueryStructuralCounterUnit::Comparisons,
                WorthQueryStructuralCounterAggregation::Independent,
                WorthQueryStructuralCounterRequiredness::OptionalSidecar,
                WorthQueryStructuralCounterReplayPosture::NotCompared,
            ),
            RetentionDeliveryProfile::Durable,
        ),
    ];

    for changed in drifted {
        assert_ne!(baseline.identity(), changed.identity());
        assert_conflicts_without_mutating_the_clean_index(baseline.clone(), changed);
    }
}

fn assert_conflicts_without_mutating_the_clean_index(
    baseline: WorthQueryPortableArtifactContract,
    changed: WorthQueryPortableArtifactContract,
) {
    let runtime = WorthQueryInstallationRuntimeIdentity::fresh();
    let canonical = admit_for_owner("worth.alpha", baseline);
    let denial = WorthQueryInstalledPackageIndex::build(
        runtime.retained(),
        WorthQueryInstallationGeneration::initial(),
        [canonical.clone(), admit_for_owner("worth.beta", changed)],
    )
    .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryInstalledPackageIndexDenialKind::ConflictingArtifactContract
    );

    let clean = WorthQueryInstalledPackageIndex::build(
        runtime,
        WorthQueryInstallationGeneration::initial(),
        [canonical],
    )
    .unwrap();
    assert_eq!(clean.installed_artifact_contract_count(), 1);
}

fn contract(
    domain_counter: WorthQueryStructuralCounterSchema,
    decision_retention: RetentionDeliveryProfile,
) -> WorthQueryPortableArtifactContract {
    let counters = WorthQueryStructuralCounterContract::declare([
        foundation(
            "bytes",
            WorthQueryStructuralCounterRole::Bytes,
            WorthQueryStructuralCounterUnit::Bytes,
        ),
        foundation(
            "elements",
            WorthQueryStructuralCounterRole::Elements,
            WorthQueryStructuralCounterUnit::Elements,
        ),
        foundation(
            "work",
            WorthQueryStructuralCounterRole::StructuralWork,
            WorthQueryStructuralCounterUnit::Operations,
        ),
        domain_counter,
    ]);
    let decisions = WorthQueryDecisionRecordContract::declared([WorthQueryDecisionSchema::new(
        WorthQueryDecisionIdentity::new(
            WorthQueryDecisionKind::new("candidate-rejected").unwrap(),
            WorthQueryDecisionReasonFamily::new("routing-reason").unwrap(),
            WorthQueryArtifactKeyFamily::new("candidate-key").unwrap(),
        ),
        WorthQueryDecisionCausalParentShape::OptionalSingle,
        WorthQueryDecisionPayloadVersion::new(1),
        WorthQueryDecisionGovernance::new(
            WorthQueryArtifactClassification::Internal,
            decision_retention,
        ),
    )]);
    base_builder()
        .counters(counters)
        .decisions(decisions)
        .compatibility(active_compatibility())
        .finish()
        .unwrap()
}

fn domain_counter(
    unit: WorthQueryStructuralCounterUnit,
    aggregation: WorthQueryStructuralCounterAggregation,
    requiredness: WorthQueryStructuralCounterRequiredness,
    replay: WorthQueryStructuralCounterReplayPosture,
) -> WorthQueryStructuralCounterSchema {
    schema(
        "candidate-comparisons",
        WorthQueryStructuralCounterRole::DomainWork,
        unit,
        aggregation,
        requiredness,
        replay,
    )
}

fn foundation(
    name: &str,
    role: WorthQueryStructuralCounterRole,
    unit: WorthQueryStructuralCounterUnit,
) -> WorthQueryStructuralCounterSchema {
    schema(
        name,
        role,
        unit,
        WorthQueryStructuralCounterAggregation::Independent,
        WorthQueryStructuralCounterRequiredness::RequiredCore,
        WorthQueryStructuralCounterReplayPosture::Exact,
    )
}

fn schema(
    name: &str,
    role: WorthQueryStructuralCounterRole,
    unit: WorthQueryStructuralCounterUnit,
    aggregation: WorthQueryStructuralCounterAggregation,
    requiredness: WorthQueryStructuralCounterRequiredness,
    replay: WorthQueryStructuralCounterReplayPosture,
) -> WorthQueryStructuralCounterSchema {
    WorthQueryStructuralCounterSchema::new(
        counter(name),
        role,
        unit,
        aggregation,
        WorthQueryStructuralCounterMonotonicity::NonDecreasing,
        WorthQueryStructuralCounterScope::ArtifactOccurrence,
        WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence,
        requiredness,
        replay,
    )
}

fn admit_for_owner(
    owner: &str,
    contract: WorthQueryPortableArtifactContract,
) -> WorthQueryAdmittedPortableDomainPackage {
    let package =
        WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(owner, 1, 0))
            .artifact_contract(contract)
            .validate()
            .unwrap();
    WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
        .artifact_version::<CandidateArtifactFamily>(
            WorthQueryArtifactSchemaVersion::new(2),
            WorthQueryArtifactProtocolVersion::new(1),
            WorthQueryArtifactVersionSupport::Admitted,
        )
        .artifact_comparator::<CandidateComparatorFamily>(
            WorthQueryInstallationSupportStatus::Admitted,
        )
        .admit(package)
        .unwrap()
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
