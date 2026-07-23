use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

fn profile(
    version: WorthQueryArtifactVersionSupport,
    comparator: WorthQueryInstallationSupportStatus,
) -> WorthQueryInstallationAdmissionProfile {
    version_profile(version).artifact_comparator::<CandidateComparatorFamily>(comparator)
}

fn version_profile(
    version: WorthQueryArtifactVersionSupport,
) -> WorthQueryInstallationAdmissionProfile {
    WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
        .artifact_version::<CandidateArtifactFamily>(
            WorthQueryArtifactSchemaVersion::new(2),
            WorthQueryArtifactProtocolVersion::new(1),
            version,
        )
}

#[test]
fn artifact_version_denials_are_exact_and_precede_comparator_registration() {
    let unsupported = profile(
        WorthQueryArtifactVersionSupport::Unsupported,
        WorthQueryInstallationSupportStatus::Admitted,
    )
    .admit(package(valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    )))
    .unwrap_err();
    assert_eq!(
        unsupported.kind(),
        WorthQueryInstallationAdmissionDenialKind::UnsupportedArtifactVersion
    );

    let retired = profile(
        WorthQueryArtifactVersionSupport::Retired,
        WorthQueryInstallationSupportStatus::Unsupported,
    )
    .admit(package(valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    )))
    .unwrap_err();
    assert_eq!(
        retired.kind(),
        WorthQueryInstallationAdmissionDenialKind::RetiredArtifactVersion
    );
}

#[test]
fn typed_comparator_family_without_installed_support_mints_no_admission() {
    let denial = profile(
        WorthQueryArtifactVersionSupport::Admitted,
        WorthQueryInstallationSupportStatus::Unsupported,
    )
    .admit(package(valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    )))
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryInstallationAdmissionDenialKind::UnsupportedArtifactComparator
    );
    assert_eq!(denial.subject(), "worth.routing.candidate-comparator");
}

#[test]
fn every_registered_reproducibility_authority_requires_exact_installed_support() {
    use WorthQueryArtifactComparisonAuthority as Authority;
    use WorthQueryArtifactDeterminismPosture as Determinism;
    use WorthQueryArtifactReproducibilityClass as Class;
    let registered = [
        (
            reproducible_contract(WorthQueryArtifactReproducibilityContract::new(
                Class::CanonicalReduction,
                Determinism::Deterministic,
                Authority::CanonicalReduction {
                    family: CanonicalReductionFamily::SEMANTIC_FAMILY.into(),
                },
                std::iter::empty::<String>(),
                std::iter::empty::<String>(),
            )),
            version_profile(WorthQueryArtifactVersionSupport::Admitted)
                .artifact_comparator::<CanonicalReductionFamily>(
                    WorthQueryInstallationSupportStatus::Admitted,
                ),
        ),
        (
            reproducible_contract(domain_reproducibility()),
            version_profile(WorthQueryArtifactVersionSupport::Admitted)
                .artifact_comparator::<CandidateComparatorFamily>(
                    WorthQueryInstallationSupportStatus::Admitted,
                ),
        ),
        (
            reproducible_contract(WorthQueryArtifactReproducibilityContract::new(
                Class::IntervalOrErrorBound,
                Determinism::EnvironmentDependent,
                Authority::RegisteredErrorBoundComparator {
                    family: ErrorBoundComparatorFamily::SEMANTIC_FAMILY.into(),
                },
                ["numeric-environment"],
                std::iter::empty::<String>(),
            )),
            version_profile(WorthQueryArtifactVersionSupport::Admitted)
                .artifact_comparator::<ErrorBoundComparatorFamily>(
                    WorthQueryInstallationSupportStatus::Admitted,
                ),
        ),
        (
            reproducible_contract(WorthQueryArtifactReproducibilityContract::new(
                Class::Distributional,
                Determinism::EntropyDependent,
                Authority::RegisteredDistributionTest {
                    family: DistributionTestFamily::SEMANTIC_FAMILY.into(),
                },
                std::iter::empty::<String>(),
                ["seed"],
            )),
            version_profile(WorthQueryArtifactVersionSupport::Admitted)
                .artifact_comparator::<DistributionTestFamily>(
                    WorthQueryInstallationSupportStatus::Admitted,
                ),
        ),
    ];

    for (contract, supported_profile) in registered {
        let family = contract
            .reproducibility()
            .comparison()
            .registered_family()
            .unwrap()
            .to_string();
        let denial = version_profile(WorthQueryArtifactVersionSupport::Admitted)
            .admit(package(contract.clone()))
            .unwrap_err();
        assert_eq!(
            denial.kind(),
            WorthQueryInstallationAdmissionDenialKind::UnsupportedArtifactComparator
        );
        assert_eq!(denial.subject(), family);
        supported_profile.admit(package(contract)).unwrap();
    }
}

#[test]
fn intrinsic_and_nonreplayable_comparison_postures_need_no_registered_comparator() {
    use WorthQueryArtifactComparisonAuthority as Authority;
    use WorthQueryArtifactDeterminismPosture as Determinism;
    use WorthQueryArtifactReproducibilityClass as Class;
    for reproducibility in [
        exact_reproducibility(),
        WorthQueryArtifactReproducibilityContract::new(
            Class::SeededDeterministic,
            Determinism::SeededDeterministic,
            Authority::ExactCanonicalValue,
            std::iter::empty::<String>(),
            ["seed"],
        ),
        WorthQueryArtifactReproducibilityContract::new(
            Class::NonReplayable,
            Determinism::Nondeterministic,
            Authority::NotComparable,
            std::iter::empty::<String>(),
            ["external-entropy"],
        ),
    ] {
        version_profile(WorthQueryArtifactVersionSupport::Admitted)
            .admit(package(reproducible_contract(reproducibility)))
            .unwrap();
    }
}

#[test]
fn contradictory_migration_registration_is_typed_and_order_independent() {
    let admit = |first, second| {
        WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
            .artifact_version::<CandidateArtifactFamily>(
                WorthQueryArtifactSchemaVersion::new(2),
                WorthQueryArtifactProtocolVersion::new(1),
                first,
            )
            .artifact_version::<CandidateArtifactFamily>(
                WorthQueryArtifactSchemaVersion::new(2),
                WorthQueryArtifactProtocolVersion::new(1),
                second,
            )
            .admit(package(valid_contract(
                false,
                WorthQueryArtifactLifecycleContract::ArenaScoped,
                exact_reproducibility(),
            )))
            .unwrap_err()
    };
    let migration = WorthQueryArtifactVersionSupport::MigrationRequired {
        target_schema_version: 3,
        migration_owner: "worth.routing.migration".into(),
    };
    let left = admit(
        migration.clone(),
        WorthQueryArtifactVersionSupport::Admitted,
    );
    let right = admit(WorthQueryArtifactVersionSupport::Admitted, migration);

    assert_eq!(left, right);
    assert_eq!(
        left.kind(),
        WorthQueryInstallationAdmissionDenialKind::AmbiguousArtifactMigration
    );
}

#[test]
fn irrelevant_artifact_support_rows_do_not_fragment_admission_identity() {
    let admit = |extra_status| {
        profile(
            WorthQueryArtifactVersionSupport::Admitted,
            WorthQueryInstallationSupportStatus::Admitted,
        )
        .artifact_comparator::<UnusedComparatorFamily>(extra_status)
        .admit(package(valid_contract(
            false,
            WorthQueryArtifactLifecycleContract::Retained,
            domain_reproducibility(),
        )))
        .unwrap()
    };
    let admitted = admit(WorthQueryInstallationSupportStatus::Admitted);
    let drifted = admit(WorthQueryInstallationSupportStatus::Unsupported);

    assert_eq!(admitted.admission_identity(), drifted.admission_identity());
}

struct UnusedComparatorFamily;

impl WorthQueryArtifactComparatorFamily for UnusedComparatorFamily {
    const SEMANTIC_FAMILY: &'static str = "unused.comparator";
}

struct CanonicalReductionFamily;

impl WorthQueryArtifactComparatorFamily for CanonicalReductionFamily {
    const SEMANTIC_FAMILY: &'static str = "canonical.reduction";
}

struct ErrorBoundComparatorFamily;

impl WorthQueryArtifactComparatorFamily for ErrorBoundComparatorFamily {
    const SEMANTIC_FAMILY: &'static str = "domain.error-bound";
}

struct DistributionTestFamily;

impl WorthQueryArtifactComparatorFamily for DistributionTestFamily {
    const SEMANTIC_FAMILY: &'static str = "distribution.test";
}

fn reproducible_contract(
    reproducibility: WorthQueryArtifactReproducibilityContract,
) -> WorthQueryPortableArtifactContract {
    base_builder()
        .reproducibility(reproducibility)
        .compatibility(active_compatibility())
        .finish()
        .unwrap()
}
