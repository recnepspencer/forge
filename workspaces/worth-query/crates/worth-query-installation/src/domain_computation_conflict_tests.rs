use worth_foundational::facade::RetentionDeliveryProfile;

use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn every_required_conflict_dimension_fails_installation_atomically() {
    let baseline = base_builder()
        .compatibility(active_compatibility())
        .finish()
        .unwrap();
    let drifted = [
        base_builder()
            .ownership(WorthQueryArtifactOwnershipContract::domain_payload(
                "different-owner",
                "provider",
            ))
            .compatibility(active_compatibility())
            .finish()
            .unwrap(),
        base_builder()
            .reproducibility(WorthQueryArtifactReproducibilityContract::new(
                WorthQueryArtifactReproducibilityClass::CanonicalReduction,
                WorthQueryArtifactDeterminismPosture::Deterministic,
                WorthQueryArtifactComparisonAuthority::CanonicalReduction {
                    family: "canonical.reduction".into(),
                },
                std::iter::empty::<String>(),
                std::iter::empty::<String>(),
            ))
            .compatibility(active_compatibility())
            .finish()
            .unwrap(),
        base_builder()
            .lifecycle(WorthQueryArtifactLifecycleContract::ExternallyDurable)
            .compatibility(active_compatibility())
            .finish()
            .unwrap(),
        base_builder()
            .governance(WorthQueryArtifactGovernanceContract::new(
                ["external-auditor"],
                WorthQueryArtifactClassification::Confidential,
                WorthQueryArtifactRedactionPosture::DomainRedactorRequired,
                RetentionDeliveryProfile::Durable,
                WorthQueryArtifactDeletionPosture::DomainControlled,
                WorthQueryArtifactLegalHoldPosture::RequiredWhenDirected,
            ))
            .compatibility(active_compatibility())
            .finish()
            .unwrap(),
        base_builder()
            .evidence(WorthQueryArtifactEvidenceContract::new(
                "different-basis",
                "provenance",
                "dependency",
                "invalidation",
                "equivalence",
            ))
            .compatibility(active_compatibility())
            .finish()
            .unwrap(),
    ];

    for contract in drifted {
        assert_ne!(baseline.identity(), contract.identity());
        let runtime = WorthQueryInstallationRuntimeIdentity::fresh();
        let canonical = admit_for_owner("worth.alpha", baseline.clone());
        let denial = WorthQueryInstalledPackageIndex::build(
            runtime.retained(),
            WorthQueryInstallationGeneration::initial(),
            [canonical.clone(), admit_for_owner("worth.beta", contract)],
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
}

#[test]
fn different_package_owners_may_share_an_identical_global_artifact_contract() {
    let contract = base_builder()
        .compatibility(active_compatibility())
        .finish()
        .unwrap();
    let index = WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [
            admit_for_owner("worth.alpha", contract.clone()),
            admit_for_owner("worth.beta", contract),
        ],
    )
    .unwrap();

    assert_eq!(index.installed_artifact_contract_count(), 2);
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
        .artifact_comparator::<CanonicalReductionComparatorFamily>(
            WorthQueryInstallationSupportStatus::Admitted,
        )
        .admit(package)
        .unwrap()
}

struct CanonicalReductionComparatorFamily;

impl WorthQueryArtifactComparatorFamily for CanonicalReductionComparatorFamily {
    const SEMANTIC_FAMILY: &'static str = "canonical.reduction";
}
