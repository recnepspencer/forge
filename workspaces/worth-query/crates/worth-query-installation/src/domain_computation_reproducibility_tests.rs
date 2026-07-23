use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn every_locked_reproducibility_class_requires_its_semantic_authority_shape() {
    use WorthQueryArtifactComparisonAuthority as Authority;
    use WorthQueryArtifactDeterminismPosture as Determinism;
    use WorthQueryArtifactReproducibilityClass as Class;
    let valid = [
        contract(
            Class::ExactDeterministic,
            Determinism::Deterministic,
            Authority::ExactCanonicalValue,
            &[],
            &[],
        ),
        contract(
            Class::SeededDeterministic,
            Determinism::SeededDeterministic,
            Authority::ExactCanonicalValue,
            &[],
            &["seed"],
        ),
        contract(
            Class::CanonicalReduction,
            Determinism::Deterministic,
            Authority::CanonicalReduction {
                family: "canonical.reduction".into(),
            },
            &[],
            &[],
        ),
        contract(
            Class::DomainComparator,
            Determinism::EntropyDependent,
            Authority::RegisteredDomainComparator {
                family: "domain.comparator".into(),
            },
            &["environment"],
            &["random-seed"],
        ),
        contract(
            Class::IntervalOrErrorBound,
            Determinism::EnvironmentDependent,
            Authority::RegisteredErrorBoundComparator {
                family: "domain.error-bound".into(),
            },
            &["numeric-environment"],
            &[],
        ),
        contract(
            Class::Distributional,
            Determinism::EntropyDependent,
            Authority::RegisteredDistributionTest {
                family: "distribution.test".into(),
            },
            &[],
            &["seed"],
        ),
        contract(
            Class::NonReplayable,
            Determinism::Nondeterministic,
            Authority::NotComparable,
            &[],
            &["external-entropy"],
        ),
    ];

    for reproducibility in valid {
        base_builder()
            .reproducibility(reproducibility)
            .compatibility(active_compatibility())
            .finish()
            .unwrap();
    }
}

#[test]
fn contradictory_determinism_and_missing_seed_dependencies_are_rejected() {
    use WorthQueryArtifactComparisonAuthority as Authority;
    use WorthQueryArtifactDeterminismPosture as Determinism;
    use WorthQueryArtifactReproducibilityClass as Class;
    for reproducibility in [
        contract(
            Class::ExactDeterministic,
            Determinism::Nondeterministic,
            Authority::ExactCanonicalValue,
            &[],
            &["entropy"],
        ),
        contract(
            Class::SeededDeterministic,
            Determinism::SeededDeterministic,
            Authority::ExactCanonicalValue,
            &[],
            &[],
        ),
        contract(
            Class::IntervalOrErrorBound,
            Determinism::EnvironmentDependent,
            Authority::RegisteredDomainComparator {
                family: "wrong.authority".into(),
            },
            &["environment"],
            &[],
        ),
    ] {
        let denial = base_builder()
            .reproducibility(reproducibility)
            .compatibility(active_compatibility())
            .finish()
            .unwrap_err();
        assert_eq!(
            denial.kind(),
            WorthQueryArtifactContractValidationDenialKind::ReproducibilityAuthorityMismatch
        );
    }
}

fn contract(
    class: WorthQueryArtifactReproducibilityClass,
    determinism: WorthQueryArtifactDeterminismPosture,
    authority: WorthQueryArtifactComparisonAuthority,
    environment: &[&str],
    entropy: &[&str],
) -> WorthQueryArtifactReproducibilityContract {
    WorthQueryArtifactReproducibilityContract::new(
        class,
        determinism,
        authority,
        environment.iter().copied(),
        entropy.iter().copied(),
    )
}
