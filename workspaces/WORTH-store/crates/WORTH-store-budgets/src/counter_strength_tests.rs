use super::CounterEvidenceStrength;

#[test]
fn counter_strength_substitution_matrix_is_explicit() {
    for actual in STRENGTHS {
        for required in STRENGTHS {
            assert_eq!(
                actual.satisfies(required),
                expected_satisfaction(actual, required),
                "{actual:?} incorrectly satisfied {required:?}"
            );
        }
    }
}

const STRENGTHS: [CounterEvidenceStrength; 6] = [
    CounterEvidenceStrength::Exact,
    CounterEvidenceStrength::Bounded,
    CounterEvidenceStrength::Sampled,
    CounterEvidenceStrength::Derived,
    CounterEvidenceStrength::CertificationOnly,
    CounterEvidenceStrength::Unavailable,
];

const fn expected_satisfaction(
    actual: CounterEvidenceStrength,
    required: CounterEvidenceStrength,
) -> bool {
    match required {
        CounterEvidenceStrength::Exact => matches!(actual, CounterEvidenceStrength::Exact),
        CounterEvidenceStrength::Bounded => {
            matches!(
                actual,
                CounterEvidenceStrength::Exact | CounterEvidenceStrength::Bounded
            )
        }
        CounterEvidenceStrength::Sampled => {
            matches!(
                actual,
                CounterEvidenceStrength::Exact
                    | CounterEvidenceStrength::Bounded
                    | CounterEvidenceStrength::Sampled
            )
        }
        CounterEvidenceStrength::Derived => {
            matches!(
                actual,
                CounterEvidenceStrength::Exact | CounterEvidenceStrength::Derived
            )
        }
        CounterEvidenceStrength::CertificationOnly => {
            !matches!(actual, CounterEvidenceStrength::Unavailable)
        }
        CounterEvidenceStrength::Unavailable => {
            matches!(actual, CounterEvidenceStrength::Unavailable)
        }
    }
}
