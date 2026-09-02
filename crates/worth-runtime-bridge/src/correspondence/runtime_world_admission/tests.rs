use super::RuntimeWorldCorrespondenceAdmissionDenial;

#[test]
fn admission_denial_names_generation_rebind_without_a_relational_adapter() {
    let drift = RuntimeWorldCorrespondenceAdmissionDenial::InstalledGenerationDrift {
        expected_generation: 8,
        actual_generation: 7,
    };

    assert!(matches!(
        drift,
        RuntimeWorldCorrespondenceAdmissionDenial::InstalledGenerationDrift {
            expected_generation: 8,
            actual_generation: 7,
        }
    ));
    assert_ne!(
        drift,
        RuntimeWorldCorrespondenceAdmissionDenial::InstalledCorrespondenceNotCurrent
    );
}
